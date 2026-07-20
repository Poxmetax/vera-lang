//! Verification-condition generation + Z3 discharge (Phase 2 thin slice).
//!
//! Scope: Int comparisons / bool ops / ite from `requires`/`ensures` and
//! `{x:Int|pred}` return (and param) refinements. Unsupported forms stay
//! runtime-checked (SPEC §4.4 obligation flow tier 4).

use crate::ast::*;
use crate::smt::{check_smtlib, SatResult, SmtError};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VcError {
    #[error("{0}")]
    Msg(String),
    #[error(transparent)]
    Smt(#[from] SmtError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Discharge {
    /// SMT returned unsat — obligation holds for all inputs under assumptions.
    Proved,
    /// Fragment unsupported or solver unknown/timeout — keep runtime check.
    RuntimeChecked { reason: String },
    /// SMT returned sat — counterexample / obligation does not hold.
    Refuted { detail: String },
}

#[derive(Debug, Clone)]
pub struct Obligation {
    pub target: String,
    pub kind: String,
    pub status: Discharge,
    /// [P2B-DIAG] Source anchor: the fn declaration span for fn-level
    /// obligations, the call expression span for call-site obligations.
    pub span: Option<Span>,
    /// [P2D-ELIDE] Structured identity for fn-level obligations (`ensures` /
    /// `return_refine`): the declaring fn's name. `None` for call-site
    /// obligations — those are never elided in this slice.
    pub fn_name: Option<String>,
    /// [P2D-ELIDE] Which `ensures` clause (declaration order) a fn-level
    /// `ensures` obligation covers; `None` for every other kind.
    pub ensures_index: Option<usize>,
}

/// [P2D-ELIDE] Fn-level PROVED obligations, keyed for the interpreter's
/// proof-gated check elision (SPEC DP6 / INV-1). Built per process run from
/// `prove_program` output on the same `Program` value — never persisted, so
/// there is no stale-certificate path (INV-2 concern deferred with the
/// certificate store). Call-site obligations are deliberately excluded (the
/// interpreter has no call-site identity yet).
#[derive(Debug, Clone, Default)]
pub struct ProvedSet {
    ensures: HashSet<(String, usize)>,
    return_refines: HashSet<String>,
}

impl ProvedSet {
    /// Build from prove results. Functions whose name appears on more than one
    /// declaration are excluded wholesale: the interpreter resolves calls by
    /// name (last declaration wins), so a proof for one duplicate must never
    /// elide checks on the other.
    pub fn build(program: &Program, obligations: &[Obligation]) -> Self {
        let mut seen: HashSet<&str> = HashSet::new();
        let mut dup: HashSet<&str> = HashSet::new();
        for f in &program.functions {
            if !seen.insert(f.name.as_str()) {
                dup.insert(f.name.as_str());
            }
        }
        let mut set = ProvedSet::default();
        for o in obligations {
            if !matches!(o.status, Discharge::Proved) {
                continue;
            }
            let Some(name) = &o.fn_name else { continue };
            if dup.contains(name.as_str()) {
                continue;
            }
            match (o.kind.as_str(), o.ensures_index) {
                ("ensures", Some(i)) => {
                    set.ensures.insert((name.clone(), i));
                }
                ("return_refine", _) => {
                    set.return_refines.insert(name.clone());
                }
                _ => {}
            }
        }
        set
    }

    pub fn ensures_proved(&self, fn_name: &str, index: usize) -> bool {
        self.ensures.contains(&(fn_name.to_string(), index))
    }

    pub fn return_refine_proved(&self, fn_name: &str) -> bool {
        self.return_refines.contains(fn_name)
    }

    pub fn len(&self) -> usize {
        self.ensures.len() + self.return_refines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn encode_expr(expr: &Expr) -> Result<String, String> {
    match expr {
        Expr::LitInt { value, .. } => Ok(value.to_string()),
        Expr::LitBool { value, .. } => Ok(if *value {
            "true".into()
        } else {
            "false".into()
        }),
        Expr::Name { name, .. } => Ok(sanitize_sym(name)),
        Expr::UnaryOp { op, expr, .. } => {
            let e = encode_expr(expr)?;
            match op.as_str() {
                "!" | "not" => Ok(format!("(not {e})")),
                "-" => Ok(format!("(- {e})")),
                _ => Err(format!("unsupported unary op {op}")),
            }
        }
        Expr::BinOp {
            op,
            left,
            right,
            ..
        } => {
            let l = encode_expr(left)?;
            let r = encode_expr(right)?;
            match op.as_str() {
                "+" => Ok(format!("(+ {l} {r})")),
                "-" => Ok(format!("(- {l} {r})")),
                "*" => Ok(format!("(* {l} {r})")),
                // [P2-SOUND1] SMT-LIB div/mod are Euclidean; the interpreter's
                // checked_div/% truncate toward zero — encoding them lets Z3 prove
                // obligations the runtime then violates (e.g. x/2 at x = -7).
                "/" | "%" => Err(format!(
                    "binop {op} not in SMT slice (Euclidean div/mod vs truncating runtime)"
                )),
                "==" => Ok(format!("(= {l} {r})")),
                "!=" => Ok(format!("(not (= {l} {r}))")),
                "<" => Ok(format!("(< {l} {r})")),
                "<=" => Ok(format!("(<= {l} {r})")),
                ">" => Ok(format!("(> {l} {r})")),
                ">=" => Ok(format!("(>= {l} {r})")),
                "&&" => Ok(format!("(and {l} {r})")),
                "||" => Ok(format!("(or {l} {r})")),
                _ => Err(format!("unsupported binop {op}")),
            }
        }
        Expr::IfExpr {
            cond,
            then_body,
            else_body,
            ..
        } => {
            let c = encode_expr(cond)?;
            let t = encode_block(then_body)?;
            let e = encode_block(else_body)?;
            Ok(format!("(ite {c} {t} {e})"))
        }
        Expr::Block(b) => encode_block(b),
        // [GAPC2-SMT-LEN] The `len` measure as an OPAQUE Int constant: the
        // pred form `len(xs)` and the method form `xs.len()` of the same list
        // map to ONE symbol `vera_len_<xs>` (declared with the `>= 0` axiom by
        // the discharge paths via `collect_len_syms`). Receivers/arguments
        // must be plain `Name`s; every other Call shape stays unsupported.
        // Soundness both ways: PROVED quantifies over all lengths c >= 0
        // (over-approximation), and every model c >= 0 is realizable by an
        // actual list of length c — so no fake PROVED and no spurious REFUTED.
        Expr::Call { callee, args, .. } => match (callee.as_ref(), args.as_slice()) {
            (Expr::Name { name, .. }, [Expr::Name { name: xs, .. }]) if name == "len" => {
                Ok(len_sym(xs))
            }
            (Expr::FieldAccess { obj, field, .. }, []) if field == "len" => match obj.as_ref() {
                Expr::Name { name: xs, .. } => Ok(len_sym(xs)),
                _ => Err(".len() receiver must be a plain name in the SMT slice".into()),
            },
            _ => Err("unsupported expr kind for SMT slice".into()),
        },
        _ => Err("unsupported expr kind for SMT slice".into()),
    }
}

/// [GAPC2-SMT-LEN] SMT symbol for the opaque `len` measure of list `xs`.
fn len_sym(xs: &str) -> String {
    format!("vera_len_{}", sanitize_sym(xs))
}

/// [GAPC2-SMT-LEN] Collect the len-measure symbols an expression's encode may
/// reference (both `len(xs)` and `xs.len()` forms), so each discharge path
/// declares them exactly once with the `>= 0` axiom. Mirrors `encode_expr`
/// coverage; shapes the encoder rejects contribute nothing here (their encode
/// fails first and the obligation stays RUNTIME-CHECKED).
fn collect_len_syms(expr: &Expr, out: &mut Vec<String>) {
    match expr {
        Expr::Call { callee, args, .. } => {
            match (callee.as_ref(), args.as_slice()) {
                (Expr::Name { name, .. }, [Expr::Name { name: xs, .. }]) if name == "len" => {
                    let s = len_sym(xs);
                    if !out.contains(&s) {
                        out.push(s);
                    }
                }
                (Expr::FieldAccess { obj, field, .. }, []) if field == "len" => {
                    if let Expr::Name { name: xs, .. } = obj.as_ref() {
                        let s = len_sym(xs);
                        if !out.contains(&s) {
                            out.push(s);
                        }
                    }
                }
                _ => {}
            }
            for a in args {
                collect_len_syms(a, out);
            }
        }
        Expr::UnaryOp { expr, .. } => collect_len_syms(expr, out),
        Expr::BinOp { left, right, .. } => {
            collect_len_syms(left, out);
            collect_len_syms(right, out);
        }
        Expr::IfExpr {
            cond,
            then_body,
            else_body,
            ..
        } => {
            collect_len_syms(cond, out);
            collect_len_syms_block(then_body, out);
            collect_len_syms_block(else_body, out);
        }
        Expr::Block(b) => collect_len_syms_block(b, out),
        _ => {}
    }
}

fn collect_len_syms_block(block: &Block, out: &mut Vec<String>) {
    for stmt in &block.stmts {
        if let Stmt::Let { value, .. } = stmt {
            collect_len_syms(value, out);
        }
    }
    if let Some(r) = &block.result {
        collect_len_syms(r, out);
    }
}

fn encode_block(block: &Block) -> Result<String, String> {
    fn go(stmts: &[Stmt], result: &Option<Box<Expr>>) -> Result<String, String> {
        if stmts.is_empty() {
            let r = result
                .as_ref()
                .ok_or_else(|| "block has no result expression".to_string())?;
            return encode_expr(r);
        }
        match &stmts[0] {
            Stmt::Let { name, value, .. } => {
                let v = encode_expr(value)?;
                let body = go(&stmts[1..], result)?;
                Ok(format!("(let (({} {v})) {body})", sanitize_sym(name)))
            }
            Stmt::Expr { .. } => Err("statement expr in block not supported in SMT slice".into()),
        }
    }
    go(&block.stmts, &block.result)
}

fn sanitize_sym(name: &str) -> String {
    if name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        name.to_string()
    } else {
        format!("|{name}|")
    }
}

fn collect_int_params(fn_decl: &FnDecl) -> Vec<String> {
    let mut names = Vec::new();
    for p in &fn_decl.params {
        match &p.ty {
            Type::Int | Type::Refine { .. } => {
                let n = sanitize_sym(&p.name);
                if !names.contains(&n) {
                    names.push(n);
                }
            }
            _ => {}
        }
        if let Type::Refine { name, .. } = &p.ty {
            let bn = sanitize_sym(name);
            if !names.contains(&bn) {
                names.push(bn);
            }
        }
    }
    names
}

fn assert_param_refines(fn_decl: &FnDecl, lines: &mut Vec<String>) -> Result<(), String> {
    for p in &fn_decl.params {
        if let Type::Refine {
            name,
            pred: Some(pred),
        } = &p.ty
        {
            lines.push(format!(
                "(assert (= {} {}))",
                sanitize_sym(name),
                sanitize_sym(&p.name)
            ));
            let pe = encode_expr(pred)?;
            lines.push(format!("(assert {pe})"));
        }
    }
    Ok(())
}

fn assert_requires(fn_decl: &FnDecl, lines: &mut Vec<String>) -> Result<(), String> {
    for req in &fn_decl.requires {
        let e = encode_expr(req)?;
        lines.push(format!("(assert {e})"));
    }
    Ok(())
}

fn build_query(
    decls: &[String],
    assumptions: &[String],
    result_term: &str,
    binder_aliases: &[String],
    negated_goal: &str,
) -> String {
    let mut s = String::from("(set-logic QF_LIA)\n");
    for d in decls {
        s.push_str(&format!("(declare-const {d} Int)\n"));
    }
    for a in assumptions {
        s.push_str(a);
        s.push('\n');
    }
    s.push_str(&format!("(define-fun __result () Int {result_term})\n"));
    s.push_str("(assert (= result __result))\n");
    for b in binder_aliases {
        s.push_str(&format!("(assert (= {b} __result))\n"));
    }
    s.push_str(&format!("(assert (not {negated_goal}))\n"));
    s.push_str("(check-sat)\n");
    s
}

fn discharge_goal(
    fn_decl: &FnDecl,
    result_term: &str,
    binder_aliases: &[String],
    goal: &Expr,
    extra_decls: &[String],
) -> Result<Discharge, VcError> {
    let mut decls = collect_int_params(fn_decl);
    for e in extra_decls {
        if !decls.contains(e) {
            decls.push(e.clone());
        }
    }
    if !decls.iter().any(|d| d == "result") {
        decls.push("result".into());
    }
    for b in binder_aliases {
        if !decls.contains(b) {
            decls.push(b.clone());
        }
    }

    // [GAPC2-SMT-LEN] declare every len-measure symbol this query's exprs can
    // reference, once each, and assume the measure axiom `>= 0`. Len-free
    // programs collect nothing, so their scripts stay byte-identical.
    let mut len_syms = Vec::new();
    for p in &fn_decl.params {
        if let Type::Refine {
            pred: Some(pred), ..
        } = &p.ty
        {
            collect_len_syms(pred, &mut len_syms);
        }
    }
    for req in &fn_decl.requires {
        collect_len_syms(req, &mut len_syms);
    }
    collect_len_syms(goal, &mut len_syms);
    collect_len_syms_block(&fn_decl.body, &mut len_syms);

    let mut assumptions = Vec::new();
    for s in &len_syms {
        if !decls.contains(s) {
            decls.push(s.clone());
        }
        assumptions.push(format!("(assert (>= {s} 0))"));
    }
    if let Err(reason) = assert_param_refines(fn_decl, &mut assumptions) {
        return Ok(Discharge::RuntimeChecked { reason });
    }
    if let Err(reason) = assert_requires(fn_decl, &mut assumptions) {
        return Ok(Discharge::RuntimeChecked { reason });
    }
    let goal_smt = match encode_expr(goal) {
        Ok(g) => g,
        Err(reason) => return Ok(Discharge::RuntimeChecked { reason }),
    };
    let query = build_query(
        &decls,
        &assumptions,
        result_term,
        binder_aliases,
        &goal_smt,
    );
    match check_smtlib(&query)? {
        SatResult::Unsat => Ok(Discharge::Proved),
        SatResult::Sat => Ok(Discharge::Refuted {
            detail: "sat (counterexample exists)".into(),
        }),
        SatResult::Unknown => Ok(Discharge::RuntimeChecked {
            reason: "solver returned unknown".into(),
        }),
    }
}

fn prove_fn(fn_decl: &FnDecl, out: &mut Vec<Obligation>) {
    let result_term = match encode_block(&fn_decl.body) {
        Ok(t) => t,
        Err(reason) => {
            for (i, _) in fn_decl.ensures.iter().enumerate() {
                out.push(Obligation {
                    target: format!("{} ensures[{i}]", fn_decl.name),
                    kind: "ensures".into(),
                    status: Discharge::RuntimeChecked {
                        reason: reason.clone(),
                    },
                    span: Some(fn_decl.span),
                    fn_name: Some(fn_decl.name.clone()),
                    ensures_index: Some(i),
                });
            }
            if let Type::Refine { pred: Some(_), .. } = &fn_decl.ret {
                out.push(Obligation {
                    target: format!("{} return refine", fn_decl.name),
                    kind: "return_refine".into(),
                    status: Discharge::RuntimeChecked { reason },
                    span: Some(fn_decl.span),
                    fn_name: Some(fn_decl.name.clone()),
                    ensures_index: None,
                });
            }
            return;
        }
    };

    for (i, ens) in fn_decl.ensures.iter().enumerate() {
        let status = discharge_goal(fn_decl, &result_term, &[], ens, &[]).unwrap_or_else(|e| {
            Discharge::RuntimeChecked {
                reason: e.to_string(),
            }
        });
        out.push(Obligation {
            target: format!("{} ensures[{i}]", fn_decl.name),
            kind: "ensures".into(),
            status,
            span: Some(fn_decl.span),
            fn_name: Some(fn_decl.name.clone()),
            ensures_index: Some(i),
        });
    }

    if let Type::Refine {
        name,
        pred: Some(pred),
    } = &fn_decl.ret
    {
        let aliases = vec![sanitize_sym(name)];
        let status = discharge_goal(
            fn_decl,
            &result_term,
            &aliases,
            pred,
            &[sanitize_sym(name)],
        )
        .unwrap_or_else(|e| Discharge::RuntimeChecked {
            reason: e.to_string(),
        });
        out.push(Obligation {
            target: format!("{} return refine {{{name}: Int | …}}", fn_decl.name),
            kind: "return_refine".into(),
            status,
            span: Some(fn_decl.span),
            fn_name: Some(fn_decl.name.clone()),
            ensures_index: None,
        });
    }
}

fn prove_calls(program: &Program, out: &mut Vec<Obligation>) {
    let fns: HashMap<&str, &FnDecl> = program
        .functions
        .iter()
        .map(|f| (f.name.as_str(), f))
        .collect();
    for f in &program.functions {
        walk_block_calls(&f.body, &fns, &f.name, out);
    }
}

fn walk_block_calls(
    block: &Block,
    fns: &HashMap<&str, &FnDecl>,
    caller: &str,
    out: &mut Vec<Obligation>,
) {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Let { value, .. } | Stmt::Expr { expr: value, .. } => {
                walk_expr_calls(value, fns, caller, out);
            }
        }
    }
    if let Some(r) = &block.result {
        walk_expr_calls(r, fns, caller, out);
    }
}

fn walk_expr_calls(
    expr: &Expr,
    fns: &HashMap<&str, &FnDecl>,
    caller: &str,
    out: &mut Vec<Obligation>,
) {
    match expr {
        Expr::Call { callee, args, span } => {
            for a in args {
                walk_expr_calls(a, fns, caller, out);
            }
            if let Expr::Name { name, .. } = callee.as_ref() {
                if let Some(callee_fn) = fns.get(name.as_str()) {
                    prove_call_site(caller, callee_fn, args, *span, out);
                }
            }
            walk_expr_calls(callee, fns, caller, out);
        }
        Expr::BinOp { left, right, .. } => {
            walk_expr_calls(left, fns, caller, out);
            walk_expr_calls(right, fns, caller, out);
        }
        Expr::UnaryOp { expr, .. }
        | Expr::Propagate { expr, .. }
        | Expr::FieldAccess { obj: expr, .. } => {
            walk_expr_calls(expr, fns, caller, out);
        }
        Expr::IfExpr {
            cond,
            then_body,
            else_body,
            ..
        } => {
            walk_expr_calls(cond, fns, caller, out);
            walk_block_calls(then_body, fns, caller, out);
            walk_block_calls(else_body, fns, caller, out);
        }
        Expr::Block(b) => walk_block_calls(b, fns, caller, out),
        Expr::Ctor { args, .. } => {
            for a in args {
                walk_expr_calls(a, fns, caller, out);
            }
        }
        Expr::StructLit { fields, .. } => {
            for (_, e) in fields {
                walk_expr_calls(e, fns, caller, out);
            }
        }
        Expr::ListLit { elems, .. } => {
            for e in elems {
                walk_expr_calls(e, fns, caller, out);
            }
        }
        Expr::MatchExpr { scrutinee, arms, .. } => {
            walk_expr_calls(scrutinee, fns, caller, out);
            for arm in arms {
                walk_expr_calls(&arm.body, fns, caller, out);
            }
        }
        Expr::Lambda { body, .. } => walk_block_calls(body, fns, caller, out),
        _ => {}
    }
}

/// [P2-SOUND2] Call-site discharge is only sound for closed literal argument
/// terms: an open term (caller variable) reaches Z3 as an undeclared /
/// unconstrained symbol and yields a spurious REFUTED — caller-context WP is
/// not part of the Phase 2 slice.
fn expr_is_closed(expr: &Expr) -> bool {
    match expr {
        Expr::LitInt { .. } | Expr::LitBool { .. } => true,
        Expr::UnaryOp { expr, .. } => expr_is_closed(expr),
        Expr::BinOp { left, right, .. } => expr_is_closed(left) && expr_is_closed(right),
        _ => false,
    }
}

fn prove_call_site(
    caller: &str,
    callee: &FnDecl,
    args: &[Expr],
    call_span: Span,
    out: &mut Vec<Obligation>,
) {
    if args.len() != callee.params.len() {
        return;
    }
    if !args.iter().all(expr_is_closed) {
        if !callee.requires.is_empty()
            || callee
                .params
                .iter()
                .any(|p| matches!(p.ty, Type::Refine { pred: Some(_), .. }))
        {
            out.push(Obligation {
                target: format!("{caller} call {} (args)", callee.name),
                kind: "call_requires".into(),
                status: Discharge::RuntimeChecked {
                    reason: "argument is not a closed literal term (caller-context WP not in Phase 2 slice)"
                        .into(),
                },
                span: Some(call_span),
                fn_name: None,
                ensures_index: None,
            });
        }
        return;
    }
    let mut arg_smt = Vec::new();
    for a in args {
        match encode_expr(a) {
            Ok(s) => arg_smt.push(s),
            Err(reason) => {
                if !callee.requires.is_empty()
                    || callee
                        .params
                        .iter()
                        .any(|p| matches!(p.ty, Type::Refine { pred: Some(_), .. }))
                {
                    out.push(Obligation {
                        target: format!("{caller} call {} (args)", callee.name),
                        kind: "call_requires".into(),
                        status: Discharge::RuntimeChecked { reason },
                        span: Some(call_span),
                        fn_name: None,
                        ensures_index: None,
                    });
                }
                return;
            }
        }
    }

    for (i, req) in callee.requires.iter().enumerate() {
        let status = discharge_call_pred(callee, &arg_smt, req).unwrap_or_else(|e| {
            Discharge::RuntimeChecked {
                reason: e.to_string(),
            }
        });
        out.push(Obligation {
            target: format!("{caller} → {} requires[{i}]", callee.name),
            kind: "call_requires".into(),
            status,
            span: Some(call_span),
            fn_name: None,
            ensures_index: None,
        });
    }

    for (p, _) in callee.params.iter().zip(arg_smt.iter()) {
        if let Type::Refine {
            pred: Some(pred), ..
        } = &p.ty
        {
            let status = discharge_call_pred(callee, &arg_smt, pred).unwrap_or_else(|e| {
                Discharge::RuntimeChecked {
                    reason: e.to_string(),
                }
            });
            out.push(Obligation {
                target: format!("{caller} → {} arg `{}` refine", callee.name, p.name),
                kind: "call_arg_refine".into(),
                status,
                span: Some(call_span),
                fn_name: None,
                ensures_index: None,
            });
        }
    }
}

fn discharge_call_pred(
    callee: &FnDecl,
    arg_smt: &[String],
    pred: &Expr,
) -> Result<Discharge, VcError> {
    let pred_smt = match encode_expr(pred) {
        Ok(p) => p,
        Err(reason) => return Ok(Discharge::RuntimeChecked { reason }),
    };
    let mut s = String::from("(set-logic QF_LIA)\n");
    // [GAPC2-SMT-LEN] len-measure symbols in the pred: declare + axiom.
    // (Unreachable while [P2-SOUND2] keeps list args non-closed, but keeps
    // the encode fail-safe honest if that gate ever widens.)
    let mut len_syms = Vec::new();
    collect_len_syms(pred, &mut len_syms);
    for ls in &len_syms {
        s.push_str(&format!("(declare-const {ls} Int)\n(assert (>= {ls} 0))\n"));
    }
    for p in &callee.params {
        match &p.ty {
            Type::Int | Type::Refine { .. } => {
                s.push_str(&format!(
                    "(declare-const {} Int)\n",
                    sanitize_sym(&p.name)
                ));
            }
            _ => {}
        }
        if let Type::Refine { name, .. } = &p.ty {
            let bn = sanitize_sym(name);
            if bn != sanitize_sym(&p.name) {
                s.push_str(&format!("(declare-const {bn} Int)\n"));
            }
        }
    }
    for (p, a) in callee.params.iter().zip(arg_smt.iter()) {
        match &p.ty {
            Type::Int | Type::Refine { .. } => {
                s.push_str(&format!(
                    "(assert (= {} {a}))\n",
                    sanitize_sym(&p.name)
                ));
            }
            _ => {}
        }
        if let Type::Refine { name, .. } = &p.ty {
            s.push_str(&format!(
                "(assert (= {} {}))\n",
                sanitize_sym(name),
                sanitize_sym(&p.name)
            ));
        }
    }
    s.push_str(&format!("(assert (not {pred_smt}))\n(check-sat)\n"));
    match check_smtlib(&s)? {
        SatResult::Unsat => Ok(Discharge::Proved),
        SatResult::Sat => Ok(Discharge::Refuted {
            detail: "sat".into(),
        }),
        SatResult::Unknown => Ok(Discharge::RuntimeChecked {
            reason: "unknown".into(),
        }),
    }
}

/// [GAP5-INV2] Toolchain identity for INV-2 keying: any durable result must
/// be invalidated when the compiler or solver changes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ToolchainId {
    pub vera_version: String,
    /// Solver identity incl. version, e.g. "z3-4.16.0".
    pub solver_id: String,
}

impl ToolchainId {
    pub fn current(solver_id: impl Into<String>) -> Self {
        Self {
            vera_version: env!("CARGO_PKG_VERSION").into(),
            solver_id: solver_id.into(),
        }
    }
}

/// [GAP5-INV2] The INV-2 cache-key scheme (design pilot — NO durable store
/// yet): every future persisted result (proof certificate, FixPatch, memo)
/// must be keyed by content hash PLUS toolchain identity so an upgrade can
/// never serve a stale verdict (SPEC INV-2 / §6.4). D's in-process ProvedSet
/// intentionally carries no key: it dies with the process. Full scheme:
/// docs/pilot/GAP5_INV2_DESIGN_NOTE.md.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofCacheKey {
    /// Content hash of the definition the result is about (store hash).
    pub definition_hash: String,
    /// Query kind, e.g. "prove/ensures[0]", "typecheck", "fixpatch".
    pub query_kind: String,
    pub toolchain: ToolchainId,
}

/// Generate and discharge all Phase-2-slice obligations for a program.
pub fn prove_program(program: &Program) -> Result<Vec<Obligation>, VcError> {
    let mut out = Vec::new();
    for f in &program.functions {
        prove_fn(f, &mut out);
    }
    prove_calls(program, &mut out);
    Ok(out)
}

pub fn format_report(path: &str, obligations: &[Obligation]) -> String {
    let mut s = format!("prove: {path}\n");
    let mut proved = 0usize;
    let mut runtime = 0usize;
    let mut refuted = 0usize;
    for o in obligations {
        match &o.status {
            Discharge::Proved => {
                proved += 1;
                s.push_str(&format!("  [PROVED]          {} ({})\n", o.target, o.kind));
            }
            Discharge::RuntimeChecked { reason } => {
                runtime += 1;
                s.push_str(&format!(
                    "  [RUNTIME-CHECKED] {} ({}) — {reason}\n",
                    o.target, o.kind
                ));
            }
            Discharge::Refuted { detail } => {
                refuted += 1;
                s.push_str(&format!(
                    "  [REFUTED]         {} ({}) — {detail}\n",
                    o.target, o.kind
                ));
            }
        }
    }
    s.push_str(&format!(
        "summary: {proved} proved, {runtime} runtime-checked, {refuted} refuted\n"
    ));
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{check_program, parse};

    #[test]
    fn gap5_inv2_key_distinguishes_toolchains() {
        // [GAP5-INV2] the same definition + query under a bumped solver must
        // be a DIFFERENT key (no stale verdicts across upgrades).
        let a = ProofCacheKey {
            definition_hash: "abc123".into(),
            query_kind: "prove/ensures[0]".into(),
            toolchain: ToolchainId::current("z3-4.16.0"),
        };
        let same = ProofCacheKey {
            definition_hash: "abc123".into(),
            query_kind: "prove/ensures[0]".into(),
            toolchain: ToolchainId::current("z3-4.16.0"),
        };
        let bumped_solver = ProofCacheKey {
            toolchain: ToolchainId::current("z3-4.17.0"),
            ..a.clone()
        };
        let other_def = ProofCacheKey {
            definition_hash: "def456".into(),
            ..a.clone()
        };
        assert_eq!(a, same);
        assert_ne!(a, bumped_solver);
        assert_ne!(a, other_def);
    }

    #[test]
    fn clamp_return_refine_proved() {
        let src = r#"
fn clamp(x: Int, lo: Int, hi: Int) -> {r: Int | r >= lo && r <= hi}
    requires lo <= hi
{
    if x < lo { lo } else { if x > hi { hi } else { x } }
}
fn main(console: Console) -> Unit uses {console} {
    let _a: Int = clamp(5, 0, 10);
    console.print("ok");
}
"#;
        let prog = parse(src).unwrap();
        check_program(&prog).unwrap();
        let obs = prove_program(&prog).unwrap();
        let proved: Vec<_> = obs
            .iter()
            .filter(|o| matches!(o.status, Discharge::Proved))
            .collect();
        assert!(
            proved.iter().any(|o| o.kind == "return_refine"),
            "expected proved return_refine, got: {:?}",
            obs.iter()
                .map(|o| (&o.target, &o.kind, &o.status))
                .collect::<Vec<_>>()
        );
        assert!(
            proved.iter().any(|o| o.kind == "call_requires"),
            "expected proved call_requires for clamp(5,0,10)"
        );
    }

    #[test]
    fn div_stays_runtime_checked() {
        // [P2-SOUND1] guard: SMT div is Euclidean, runtime truncates — never prove through `/`.
        let src = r#"
fn half_leq(x: Int) -> {r: Int | r * 2 <= x} {
    x / 2
}
fn main(console: Console) -> Unit uses {console} {
    console.print("ok");
}
"#;
        let prog = parse(src).unwrap();
        check_program(&prog).unwrap();
        let obs = prove_program(&prog).unwrap();
        assert!(!obs.is_empty(), "expected a return-refine obligation");
        assert!(
            obs.iter()
                .all(|o| matches!(o.status, Discharge::RuntimeChecked { .. })),
            "{obs:?}"
        );
    }

    #[test]
    fn open_call_args_stay_runtime_checked() {
        // [P2-SOUND2] guard: variable args must not reach Z3 as free symbols (spurious REFUTED).
        let src = r#"
fn pos_id(x: {x: Int | x >= 1}) -> Int {
    x
}
fn main(console: Console) -> Unit uses {console} {
    let v: Int = 5;
    console.print(pos_id(v).show());
}
"#;
        let prog = parse(src).unwrap();
        check_program(&prog).unwrap();
        let obs = prove_program(&prog).unwrap();
        assert!(
            obs.iter().any(|o| o.kind == "call_requires"
                && matches!(o.status, Discharge::RuntimeChecked { .. })),
            "{obs:?}"
        );
        assert!(
            obs.iter()
                .all(|o| !matches!(o.status, Discharge::Refuted { .. })),
            "{obs:?}"
        );
    }

    #[test]
    fn gapc2_len_param_refine_assumption_enables_proved_ensures() {
        // [GAPC2-SMT-LEN] a len-refined param used to kill the WHOLE fn-level
        // obligation (the assumption's encode failed on `len` -> the ensures
        // came back RUNTIME-CHECKED even though the goal is len-free). With
        // len as an opaque measure constant the assumption is assertable:
        // k = i, 0 <= k && k < vera_len_xs |- result >= 0 is unsat-negated.
        let src = r#"
fn pick_nonneg(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int
    ensures result >= 0
{
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print("ok");
}
"#;
        let prog = parse(src).unwrap();
        check_program(&prog).unwrap();
        let obs = prove_program(&prog).unwrap();
        assert!(
            obs.iter()
                .any(|o| o.kind == "ensures" && matches!(o.status, Discharge::Proved)),
            "{obs:?}"
        );
    }

    #[test]
    fn gapc2_len_body_nonneg_ensures_proved_by_axiom() {
        // [GAPC2-SMT-LEN] the method form `xs.len()` in the BODY maps to the
        // same opaque constant; the `>= 0` measure axiom alone discharges
        // `ensures result >= 0`.
        let src = r#"
fn size_of(xs: List<Int>) -> Int
    ensures result >= 0
{
    xs.len()
}
fn main(console: Console) -> Unit uses {console} {
    console.print("ok");
}
"#;
        let prog = parse(src).unwrap();
        check_program(&prog).unwrap();
        let obs = prove_program(&prog).unwrap();
        assert!(
            obs.iter()
                .any(|o| o.kind == "ensures" && matches!(o.status, Discharge::Proved)),
            "{obs:?}"
        );
    }

    #[test]
    fn gapc2_refutable_len_ensures_stays_honest() {
        // [GAPC2-SMT-LEN] no fake PROVED: `result >= 1` over `xs.len()` is
        // genuinely violable — the c = 0 model IS the empty list — so the
        // obligation must come back REFUTED, never Proved.
        let src = r#"
fn size_pos(xs: List<Int>) -> Int
    ensures result >= 1
{
    xs.len()
}
fn main(console: Console) -> Unit uses {console} {
    console.print("ok");
}
"#;
        let prog = parse(src).unwrap();
        check_program(&prog).unwrap();
        let obs = prove_program(&prog).unwrap();
        assert!(
            obs.iter()
                .any(|o| o.kind == "ensures" && matches!(o.status, Discharge::Refuted { .. })),
            "{obs:?}"
        );
        assert!(
            obs.iter()
                .all(|o| !matches!(o.status, Discharge::Proved)),
            "{obs:?}"
        );
    }

    #[test]
    fn gapc2_call_site_len_args_stay_runtime_checked() {
        // [GAPC2-SMT-LEN] honest limit pinned: a call site passing a list
        // still fails the closed-literal gate ([P2-SOUND2]) — the measure
        // encode must not widen call-site discharge.
        let src = r#"
fn pick2(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    let data: List<Int> = [1, 2];
    console.print(pick2(data, 1).show());
}
"#;
        let prog = parse(src).unwrap();
        check_program(&prog).unwrap();
        let obs = prove_program(&prog).unwrap();
        assert!(
            obs.iter().any(|o| o.kind == "call_requires"
                && matches!(o.status, Discharge::RuntimeChecked { .. })),
            "{obs:?}"
        );
        assert!(
            obs.iter()
                .all(|o| !matches!(o.status, Discharge::Refuted { .. })),
            "{obs:?}"
        );
    }
}
