//! Lightweight Phase 1 type checker (annotated MVP; HM deferred).

use crate::ast::*;
use crate::label::{Atom, Label};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// [P2E-FIX] Structured payload for a machine-applicable fix on a
/// non-exhaustive `match` (SPEC §4.1): the match expression's span plus the
/// uncovered arms as valid, arity-aware pattern stubs (e.g. `Shape::Pt(_, _)`).
/// Plain data — serialization lives in the diag layer (`FixPatch`).
#[derive(Debug, Clone)]
pub struct MatchFixInfo {
    pub span: Span,
    pub missing: Vec<String>,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct TypeError(pub String, pub Option<MatchFixInfo>);

impl TypeError {
    fn at(span: Span, msg: impl Into<String>) -> Self {
        TypeError(format!("{}: {}", span, msg.into()), None)
    }

    /// [P2E-FIX] Error carrying a mechanical fix payload (diag serializes it).
    fn at_fix(span: Span, msg: impl Into<String>, missing: Vec<String>) -> Self {
        TypeError(
            format!("{}: {}", span, msg.into()),
            Some(MatchFixInfo { span, missing }),
        )
    }
}

struct AdtEnv {
    structs: HashMap<String, StructDecl>,
    enums: HashMap<String, EnumDecl>,
}

struct Env<'a> {
    vars: HashMap<String, Type>,
    fns: &'a HashMap<String, FnDecl>,
    adt: &'a AdtEnv,
    /// [P2-SOUND3] Declared return type of the enclosing fn / lambda — the type
    /// a `?` early-return actually escapes into (None = unannotated lambda).
    ret: Option<Type>,
}

impl<'a> Env<'a> {
    fn extend(&self, name: String, ty: Type) -> Env<'a> {
        let mut vars = self.vars.clone();
        vars.insert(name, ty);
        Env {
            vars,
            fns: self.fns,
            adt: self.adt,
            ret: self.ret.clone(),
        }
    }
}

fn types_equal(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Refine { .. }, Type::Int) | (Type::Int, Type::Refine { .. }) => true,
        (Type::Refine { .. }, Type::Refine { .. }) => true,
        _ => a.to_str() == b.to_str(),
    }
}

pub fn check_program(program: &Program) -> Result<(), TypeError> {
    let mut adt = AdtEnv {
        structs: HashMap::new(),
        enums: HashMap::new(),
    };
    for s in &program.structs {
        if adt.structs.contains_key(&s.name) || adt.enums.contains_key(&s.name) {
            return Err(TypeError::at(s.span, format!("duplicate type {}", s.name)));
        }
        adt.structs.insert(s.name.clone(), s.clone());
    }
    for e in &program.enums {
        if adt.structs.contains_key(&e.name) || adt.enums.contains_key(&e.name) {
            return Err(TypeError::at(e.span, format!("duplicate type {}", e.name)));
        }
        adt.enums.insert(e.name.clone(), e.clone());
    }

    // [P2-DUPFN] Reject duplicate function names (mirrors the duplicate-type
    // check above). Before this, a later `fn f` silently shadowed an earlier
    // one at runtime (last declaration wins in every name-keyed map), which
    // poisons name-keyed reasoning like the [P2D-ELIDE] proved set.
    let mut fns: HashMap<String, FnDecl> = HashMap::new();
    for f in &program.functions {
        if fns.contains_key(&f.name) {
            return Err(TypeError::at(
                f.span,
                format!("[P2-DUPFN] duplicate function {}", f.name),
            ));
        }
        fns.insert(f.name.clone(), f.clone());
    }
    if !fns.contains_key("main") {
        return Err(TypeError("program must define fn main".into(), None));
    }
    for fn_decl in &program.functions {
        check_fn(fn_decl, &fns, &adt)?;
    }
    // [GAP4-R2-SURFACE] Front-door label pass with EMPTY seeds — inert by the
    // lattice laws (every label is ⊥, and ⊥ ⊑ ⊥ always holds), so no
    // well-typed program changes verdict; the seeded entry point is the
    // demonstrable surface.
    check_program_labels(program, &HashMap::new())
}

fn check_fn(
    fn_decl: &FnDecl,
    fns: &HashMap<String, FnDecl>,
    adt: &AdtEnv,
) -> Result<(), TypeError> {
    for u in &fn_decl.uses {
        if u != "console" {
            return Err(TypeError::at(
                fn_decl.span,
                format!("unknown capability {u:?} (MVP allows only console)"),
            ));
        }
    }
    let mut vars = HashMap::new();
    for p in &fn_decl.params {
        // [GAP2-REFINE-TC] insert-then-check gives prefix scoping that matches
        // the interpreter's binding order: param i's pred sees params 0..=i
        // (a forward reference would be an unbound-name trap at runtime).
        vars.insert(p.name.clone(), p.ty.clone());
        check_type_refines(&p.ty, &vars)?;
    }
    // [GAP2-REFINE-TC] the return refine sees the full parameter scope.
    check_type_refines(&fn_decl.ret, &vars)?;
    let env = Env {
        vars,
        fns,
        adt,
        ret: Some(fn_decl.ret.clone()),
    };
    for req in &fn_decl.requires {
        let t = infer_expr(req, &env)?;
        if !matches!(t, Type::Bool) {
            return Err(TypeError::at(req.span(), "requires clause must be Bool"));
        }
    }
    let body_ty = check_block(&fn_decl.body, &env)?;
    if !types_equal(&body_ty, &fn_decl.ret) {
        return Err(TypeError::at(
            fn_decl.span,
            format!(
                "function {}: body type {} != declared {}",
                fn_decl.name,
                body_ty.to_str(),
                fn_decl.ret.to_str()
            ),
        ));
    }
    // [P2-REFINE1-DEF] hard reject when closed body falsifies return refine
    check_ret_refine_body(fn_decl)?;
    let ens_env = env.extend("result".into(), fn_decl.ret.clone());
    for ens in &fn_decl.ensures {
        let t = infer_expr(ens, &ens_env)?;
        if !matches!(t, Type::Bool) {
            return Err(TypeError::at(ens.span(), "ensures clause must be Bool"));
        }
    }
    Ok(())
}

fn check_block(block: &Block, env: &Env<'_>) -> Result<Type, TypeError> {
    let mut e_vars = env.vars.clone();
    for stmt in &block.stmts {
        match stmt {
            Stmt::Let {
                name, ty, value, span
            } => {
                let vty = infer_expr(
                    value,
                    &Env {
                        vars: e_vars.clone(),
                        fns: env.fns,
                        adt: env.adt,
                        ret: env.ret.clone(),
                    },
                )?;
                if let Some(annot) = ty {
                    // [GAP2-REFINE-TC] let-annotation refines are runtime-inert
                    // but still fragment-checked (visible bindings in scope).
                    check_type_refines(annot, &e_vars)?;
                    if !types_equal(&vty, annot) {
                        return Err(TypeError::at(
                            *span,
                            format!(
                                "let {name}: got {}, expected {}",
                                vty.to_str(),
                                annot.to_str()
                            ),
                        ));
                    }
                    e_vars.insert(name.clone(), annot.clone());
                } else {
                    e_vars.insert(name.clone(), vty);
                }
            }
            Stmt::Expr { expr, .. } => {
                infer_expr(
                    expr,
                    &Env {
                        vars: e_vars.clone(),
                        fns: env.fns,
                        adt: env.adt,
                        ret: env.ret.clone(),
                    },
                )?;
            }
        }
    }
    let env2 = Env {
        vars: e_vars,
        fns: env.fns,
        adt: env.adt,
        ret: env.ret.clone(),
    };
    if let Some(res) = &block.result {
        infer_expr(res, &env2)
    } else {
        Ok(Type::Unit)
    }
}

fn resolve_named(name: &str, adt: &AdtEnv) -> Option<Type> {
    if adt.structs.contains_key(name) || adt.enums.contains_key(name) {
        Some(Type::Named {
            name: name.to_string(),
        })
    } else {
        None
    }
}

fn infer_expr(expr: &Expr, env: &Env<'_>) -> Result<Type, TypeError> {
    match expr {
        Expr::LitInt { .. } => Ok(Type::Int),
        Expr::LitBool { .. } => Ok(Type::Bool),
        Expr::LitStr { .. } => Ok(Type::Str),
        Expr::LitUnit { .. } => Ok(Type::Unit),
        Expr::ListLit { elems, span } => {
            if elems.is_empty() {
                // Empty list defaults to List<Int> when unconstrained (Phase 1).
                return Ok(Type::List {
                    elem: Box::new(Type::Int),
                });
            }
            let first = infer_expr(&elems[0], env)?;
            for e in &elems[1..] {
                let t = infer_expr(e, env)?;
                if !types_equal(&first, &t) {
                    return Err(TypeError::at(
                        *span,
                        format!(
                            "list elements differ: {} vs {}",
                            first.to_str(),
                            t.to_str()
                        ),
                    ));
                }
            }
            Ok(Type::List {
                elem: Box::new(first),
            })
        }
        Expr::Lambda {
            params,
            ret,
            body,
            span,
        } => infer_lambda(params, ret.as_ref(), body, *span, env),
        Expr::Name { name, span } => {
            if let Some(t) = env.vars.get(name) {
                return Ok(t.clone());
            }
            if env.fns.contains_key(name) {
                return Err(TypeError::at(
                    *span,
                    format!("{name} is a function; call it with (...)"),
                ));
            }
            if is_prelude_ctor(name) {
                return Err(TypeError::at(
                    *span,
                    format!("{name} is a constructor; call it with (...)"),
                ));
            }
            Err(TypeError::at(*span, format!("unknown name {name:?}")))
        }
        Expr::Ctor {
            type_name,
            name,
            args,
            span,
        } => infer_ctor(type_name.as_deref(), name, args, *span, env),
        Expr::StructLit { name, fields, span } => {
            let Some(sd) = env.adt.structs.get(name) else {
                return Err(TypeError::at(*span, format!("unknown struct {name}")));
            };
            if fields.len() != sd.fields.len() {
                return Err(TypeError::at(
                    *span,
                    format!(
                        "struct {name} expects {} fields, got {}",
                        sd.fields.len(),
                        fields.len()
                    ),
                ));
            }
            let mut seen = HashSet::new();
            for (fname, fexpr) in fields {
                if !seen.insert(fname.clone()) {
                    return Err(TypeError::at(*span, format!("duplicate field {fname}")));
                }
                let Some(fd) = sd.fields.iter().find(|f| f.name == *fname) else {
                    return Err(TypeError::at(*span, format!("unknown field {fname}")));
                };
                let ft = infer_expr(fexpr, env)?;
                if !types_equal(&ft, &fd.ty) {
                    return Err(TypeError::at(
                        *span,
                        format!(
                            "field {fname}: got {}, expected {}",
                            ft.to_str(),
                            fd.ty.to_str()
                        ),
                    ));
                }
            }
            for fd in &sd.fields {
                if !seen.contains(&fd.name) {
                    return Err(TypeError::at(
                        *span,
                        format!("missing field {}", fd.name),
                    ));
                }
            }
            Ok(Type::Named {
                name: name.clone(),
            })
        }
        Expr::UnaryOp { op, expr, span } => {
            let t = infer_expr(expr, env)?;
            match (op.as_str(), &t) {
                ("-", Type::Int | Type::Refine { .. }) => Ok(Type::Int),
                ("!", Type::Bool) => Ok(Type::Bool),
                _ => Err(TypeError::at(
                    *span,
                    format!("unary {op} on {}", t.to_str()),
                )),
            }
        }
        Expr::BinOp {
            op,
            left,
            right,
            span,
        } => {
            let lt = infer_expr(left, env)?;
            let rt = infer_expr(right, env)?;
            match op.as_str() {
                "++" => match (&lt, &rt) {
                    (Type::Str, Type::Str) => Ok(Type::Str),
                    (Type::List { elem: a }, Type::List { elem: b }) if types_equal(a, b) => {
                        Ok(Type::List {
                            elem: a.clone(),
                        })
                    }
                    _ => Err(TypeError::at(*span, "++ expects Str++Str or List++List")),
                },
                "+" | "-" | "*" | "/" | "%" => {
                    if matches!(lt, Type::Int | Type::Refine { .. })
                        && matches!(rt, Type::Int | Type::Refine { .. })
                    {
                        Ok(Type::Int)
                    } else {
                        Err(TypeError::at(
                            *span,
                            format!("arithmetic on {} and {}", lt.to_str(), rt.to_str()),
                        ))
                    }
                }
                "==" | "!=" | "<" | "<=" | ">" | ">=" => Ok(Type::Bool),
                "&&" | "||" => {
                    if matches!(lt, Type::Bool) && matches!(rt, Type::Bool) {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeError::at(*span, "logical ops need Bool"))
                    }
                }
                _ => Err(TypeError::at(*span, format!("unknown operator {op}"))),
            }
        }
        Expr::FieldAccess { obj, field, span } => {
            let obj_t = infer_expr(obj, env)?;
            if matches!(obj_t, Type::Console) && field == "print" {
                return Ok(Type::Console);
            }
            // Method placeholders typed at Call site: len/get/head/tail/append/show
            if matches!(
                field.as_str(),
        "len" | "get" | "head" | "tail" | "append" | "show" | "map" | "filter" | "fold"
            ) {
                return Ok(obj_t);
            }
            if let Type::Named { name } = &obj_t {
                if let Some(sd) = env.adt.structs.get(name) {
                    if let Some(fd) = sd.fields.iter().find(|f| f.name == *field) {
                        return Ok(fd.ty.clone());
                    }
                    return Err(TypeError::at(
                        *span,
                        format!("unknown field {field} on {name}"),
                    ));
                }
            }
            Err(TypeError::at(
                *span,
                format!("unknown field {field} on {}", obj_t.to_str()),
            ))
        }
        Expr::Call {
            callee,
            args,
            span,
        } => {
            if let Expr::FieldAccess { obj, field, .. } = callee.as_ref() {
                let obj_t = infer_expr(obj, env)?;
                if matches!(obj_t, Type::Console) && field == "print" {
                    if args.len() != 1 {
                        return Err(TypeError::at(*span, "Console.print takes 1 argument"));
                    }
                    let at = infer_expr(&args[0], env)?;
                    if !matches!(at, Type::Str) {
                        return Err(TypeError::at(*span, "Console.print expects Str"));
                    }
                    return Ok(Type::Unit);
                }
                if field == "show" {
                    if args.is_empty() && matches!(obj_t, Type::Int | Type::Refine { .. }) {
                        return Ok(Type::Str);
                    }
                    return Err(TypeError::at(*span, "show() only on Int with 0 args"));
                }
                if let Type::List { elem } = &obj_t {
                    match field.as_str() {
                        "len" => {
                            if !args.is_empty() {
                                return Err(TypeError::at(*span, "len takes 0 args"));
                            }
                            return Ok(Type::Int);
                        }
                        "get" => {
                            if args.len() != 1 {
                                return Err(TypeError::at(*span, "get takes 1 Int index"));
                            }
                            let at = infer_expr(&args[0], env)?;
                            if !matches!(at, Type::Int | Type::Refine { .. }) {
                                return Err(TypeError::at(*span, "get index must be Int"));
                            }
                            return Ok(Type::Option {
                                inner: elem.clone(),
                            });
                        }
                        "head" => {
                            if !args.is_empty() {
                                return Err(TypeError::at(*span, "head takes 0 args"));
                            }
                            return Ok(Type::Option {
                                inner: elem.clone(),
                            });
                        }
                        "tail" => {
                            if !args.is_empty() {
                                return Err(TypeError::at(*span, "tail takes 0 args"));
                            }
                            return Ok(Type::Option {
                                inner: Box::new(Type::List {
                                    elem: elem.clone(),
                                }),
                            });
                        }
                        "append" => {
                            if args.len() != 1 {
                                return Err(TypeError::at(*span, "append takes 1 element"));
                            }
                            let at = infer_expr(&args[0], env)?;
                            if !types_equal(&at, elem) {
                                return Err(TypeError::at(
                                    *span,
                                    format!(
                                        "append elem {} != {}",
                                        at.to_str(),
                                        elem.to_str()
                                    ),
                                ));
                            }
                            return Ok(Type::List {
                                elem: elem.clone(),
                            });
                        }
                        "map" => {
                            if args.len() != 1 {
                                return Err(TypeError::at(*span, "map takes 1 function"));
                            }
                            let out_elem =
                                check_hof_unary(&args[0], elem, None, *span, env)?;
                            return Ok(Type::List {
                                elem: Box::new(out_elem),
                            });
                        }
                        "filter" => {
                            if args.len() != 1 {
                                return Err(TypeError::at(*span, "filter takes 1 predicate"));
                            }
                            let pred_ret =
                                check_hof_unary(&args[0], elem, Some(&Type::Bool), *span, env)?;
                            if !matches!(pred_ret, Type::Bool) {
                                return Err(TypeError::at(*span, "filter predicate must return Bool"));
                            }
                            return Ok(Type::List {
                                elem: elem.clone(),
                            });
                        }
                        "fold" => {
                            if args.len() != 2 {
                                return Err(TypeError::at(
                                    *span,
                                    "fold takes init and fn (acc, elem) -> acc",
                                ));
                            }
                            let init_ty = infer_expr(&args[0], env)?;
                            check_hof_binary(
                                &args[1],
                                &init_ty,
                                elem,
                                &init_ty,
                                *span,
                                env,
                            )?;
                            return Ok(init_ty);
                        }
                        _ => {}
                    }
                }
            }
            if let Expr::Name { name, .. } = callee.as_ref() {
                if let Some(fn_decl) = env.fns.get(name) {
                    if args.len() != fn_decl.params.len() {
                        return Err(TypeError::at(
                            *span,
                            format!(
                                "{} expects {} args, got {}",
                                fn_decl.name,
                                fn_decl.params.len(),
                                args.len()
                            ),
                        ));
                    }
                    for (a, p) in args.iter().zip(fn_decl.params.iter()) {
                        let at = infer_expr(a, env)?;
                        if !types_equal(&at, &p.ty) {
                            return Err(TypeError::at(
                                *span,
                                format!("arg type {} != {}", at.to_str(), p.ty.to_str()),
                            ));
                        }
                        // [P2-REFINE1] hard reject decidably-false refine on Int literals
                        check_lit_arg_refine(a, p, *span)?;
                        // [GAPC1-SYM-LEN] hard reject the symbolic same-term
                        // len-as-index case REQ-REFINE-2 names (P2C deferral)
                        check_sym_len_arg_refine(a, p, fn_decl, args, *span)?;
                    }
                    return Ok(erase_refine(&fn_decl.ret));
                }
                if is_prelude_ctor(name) {
                    return infer_ctor(None, name, args, *span, env);
                }
            }
            // Call a first-class function value (closure / lambda).
            let ft = infer_expr(callee, env)?;
            if let Type::Fn { params, ret } = ft {
                if args.len() != params.len() {
                    return Err(TypeError::at(
                        *span,
                        format!("function expects {} args, got {}", params.len(), args.len()),
                    ));
                }
                for (a, p) in args.iter().zip(params.iter()) {
                    let at = infer_expr(a, env)?;
                    if !types_equal(&at, p) {
                        return Err(TypeError::at(
                            *span,
                            format!("arg type {} != {}", at.to_str(), p.to_str()),
                        ));
                    }
                }
                return Ok(*ret);
            }
            Err(TypeError::at(*span, "unsupported call"))
        }
        Expr::IfExpr {
            cond,
            then_body,
            else_body,
            span,
        } => {
            let ct = infer_expr(cond, env)?;
            if !matches!(ct, Type::Bool) {
                return Err(TypeError::at(*span, "if condition must be Bool"));
            }
            let tt = check_block(then_body, env)?;
            let et = check_block(else_body, env)?;
            if !types_equal(&tt, &et) {
                return Err(TypeError::at(
                    *span,
                    format!("if branches differ: {} vs {}", tt.to_str(), et.to_str()),
                ));
            }
            Ok(tt)
        }
        Expr::MatchExpr {
            scrutinee,
            arms,
            span,
        } => check_match(scrutinee, arms, *span, env),
        Expr::Hole { name, span } => Err(TypeError::at(
            *span,
            format!("unfilled typed hole ?{name} (fill body or run synthesis)"),
        )),
        Expr::Propagate { expr, span } => {
            let t = infer_expr(expr, env)?;
            // [P2-SOUND3] `?` early-returns None/Err out of the enclosing fn/lambda,
            // so its declared return type must be able to carry that value
            // (interp unwraps EarlyReturn at exactly that boundary).
            match t {
                Type::Option { inner } => match &env.ret {
                    Some(Type::Option { .. }) => Ok(*inner),
                    Some(other) => Err(TypeError::at(
                        *span,
                        format!(
                            "`?` on Option needs the enclosing return type to be Option<_>, but it is {}",
                            other.to_str()
                        ),
                    )),
                    None => Err(TypeError::at(
                        *span,
                        "`?` needs an annotated enclosing return type (annotate the lambda return)",
                    )),
                },
                Type::Result { ok, err } => match &env.ret {
                    Some(Type::Result { err: renv, .. }) if types_equal(&err, renv) => Ok(*ok),
                    Some(Type::Result { err: renv, .. }) => Err(TypeError::at(
                        *span,
                        format!(
                            "`?` error type {} != enclosing error type {}",
                            err.to_str(),
                            renv.to_str()
                        ),
                    )),
                    Some(other) => Err(TypeError::at(
                        *span,
                        format!(
                            "`?` on Result needs the enclosing return type to be Result<_, _>, but it is {}",
                            other.to_str()
                        ),
                    )),
                    None => Err(TypeError::at(
                        *span,
                        "`?` needs an annotated enclosing return type (annotate the lambda return)",
                    )),
                },
                other => Err(TypeError::at(
                    *span,
                    format!("`?` propagation requires Option or Result, got {}", other.to_str()),
                )),
            }
        }
        Expr::Block(b) => check_block(b, env),
    }
}

/// [P2-REFINE1] REQ-REFINE-1 call-site slice: when an argument is an Int literal
/// and the parameter refine pred is a closed QF-LIA comparison/&& tree over
/// the binder + literals, evaluate it. Some(false) → type error (zero exec).
/// Unevaluable / non-literal args stay soft (prove / runtime). Definition-time
/// return-refine body reject: see [P2-REFINE1-DEF] `check_ret_refine_body`.
/// [P2-REFINE2] REQ-REFINE-2: Kleene && / || lets a decided conjunct reject even
/// when the other side is an unevaluable `len(xs)` measure — e.g. `nth(xs, -1)`
/// fails `0 <= k` and is rejected with zero execution (SPEC §4.4).
fn check_lit_arg_refine(arg: &Expr, param: &Param, span: Span) -> Result<(), TypeError> {
    let Type::Refine {
        name: binder,
        pred: Some(pred),
    } = &param.ty
    else {
        return Ok(());
    };
    // [P2-REFINE1] negative literals parse as unary minus over a literal — they
    // are literals too (the REQ-REFINE bounds cases are typically negative).
    let value = match arg {
        Expr::LitInt { value, .. } => *value,
        Expr::UnaryOp { op, expr, .. } if op == "-" => match expr.as_ref() {
            Expr::LitInt { value, .. } => match value.checked_neg() {
                Some(v) => v,
                None => return Ok(()),
            },
            _ => return Ok(()),
        },
        _ => return Ok(()),
    };
    if pred_holds_for_lit(pred, binder, value) == Some(false) {
        // [P2-REFINE2] len-measure preds carry the REQ-REFINE-2 marker so the
        // diagnostic names the conformance requirement that rejected the call.
        let marker = if pred_mentions_len(pred) {
            "[P2-REFINE2]"
        } else {
            "[P2-REFINE1]"
        };
        return Err(TypeError::at(
            span,
            format!(
                "{marker} arg {} = {value} violates parameter refinement",
                param.name
            ),
        ));
    }
    Ok(())
}

/// [GAPC1-SYM-LEN] REQ-REFINE-2's deferred symbolic case (P2C honest limit):
/// an argument of the shape `xs.len()` passed for a refined index parameter
/// whose predicate bounds the binder by `len(xs_param)` of the SAME list.
/// Substituting `k := len(xs_param)` makes both comparison sides the same
/// term, so the comparison decides by reflexivity alone (`<`/`>`/`!=` false,
/// `<=`/`>=`/`==` true) — no list value or length is needed. Scope: the
/// `.len()` receiver and the list argument must be plain `Name`s (immutable
/// bindings denote the same value at both positions — effect-free, so the
/// same-term claim is sound). Anything else stays soft -> prove / runtime
/// (P2C design).
fn check_sym_len_arg_refine(
    arg: &Expr,
    param: &Param,
    fn_decl: &FnDecl,
    args: &[Expr],
    span: Span,
) -> Result<(), TypeError> {
    let Type::Refine {
        name: binder,
        pred: Some(pred),
    } = &param.ty
    else {
        return Ok(());
    };
    // The argument must be exactly `<name>.len()` (the List measure method).
    let Expr::Call {
        callee,
        args: margs,
        ..
    } = arg
    else {
        return Ok(());
    };
    let Expr::FieldAccess { obj, field, .. } = callee.as_ref() else {
        return Ok(());
    };
    if field != "len" || !margs.is_empty() {
        return Ok(());
    }
    let Expr::Name { name: recv, .. } = obj.as_ref() else {
        return Ok(());
    };
    // param -> arg substitution: every callee parameter that receives the
    // SAME variable as the `.len()` receiver instantiates the predicate's
    // `len(<that param>)` to `len(recv)` — the argument's own value.
    for (q, qa) in fn_decl.params.iter().zip(args.iter()) {
        if !matches!(qa, Expr::Name { name, .. } if name == recv) {
            continue;
        }
        if pred_holds_for_sym_len(pred, binder, &q.name) == Some(false) {
            return Err(TypeError::at(
                span,
                format!(
                    "[GAPC1-SYM-LEN] arg {} = {recv}.len() violates parameter refinement (same-term len bound is decidably false)",
                    param.name
                ),
            ));
        }
    }
    Ok(())
}

/// [GAPC1-SYM-LEN] Kleene evaluation of a refinement predicate under the
/// symbolic substitution `binder := len(xs_param)`. A comparison decides only
/// when BOTH sides denote that same symbolic value (the binder itself or the
/// `len(xs_param)` measure call); mixed literal/symbolic sides stay unknown,
/// which keeps e.g. `k < 0 || k < len(xs)` soft (conservative by design —
/// this slice does not pretend to be a solver).
fn pred_holds_for_sym_len(pred: &Expr, binder: &str, xs_param: &str) -> Option<bool> {
    match pred {
        Expr::BinOp { op, left, right, .. } if op == "&&" => {
            match (
                pred_holds_for_sym_len(left, binder, xs_param),
                pred_holds_for_sym_len(right, binder, xs_param),
            ) {
                (Some(false), _) | (_, Some(false)) => Some(false),
                (Some(true), Some(true)) => Some(true),
                _ => None,
            }
        }
        Expr::BinOp { op, left, right, .. } if op == "||" => {
            match (
                pred_holds_for_sym_len(left, binder, xs_param),
                pred_holds_for_sym_len(right, binder, xs_param),
            ) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            }
        }
        Expr::BinOp { op, left, right, .. } => {
            if !(sym_len_term(left, binder, xs_param) && sym_len_term(right, binder, xs_param)) {
                return None;
            }
            // Both sides are the same symbolic value: reflexivity decides.
            match op.as_str() {
                "<" | ">" | "!=" => Some(false),
                "<=" | ">=" | "==" => Some(true),
                _ => None,
            }
        }
        Expr::UnaryOp { op, expr, .. } if op == "!" => {
            Some(!pred_holds_for_sym_len(expr, binder, xs_param)?)
        }
        Expr::LitBool { value, .. } => Some(*value),
        _ => None,
    }
}

/// Does this predicate expression denote the symbolic value `len(xs_param)` —
/// the refine binder itself (under the substitution) or the measure call?
fn sym_len_term(expr: &Expr, binder: &str, xs_param: &str) -> bool {
    match expr {
        Expr::Name { name, .. } => name == binder,
        Expr::Call { callee, args, .. } => {
            matches!(callee.as_ref(), Expr::Name { name, .. } if name == "len")
                && args.len() == 1
                && matches!(&args[0], Expr::Name { name, .. } if name == xs_param)
        }
        _ => false,
    }
}

/// [P2-REFINE2] Does a refinement predicate mention the `len(...)` measure?
/// Diagnostic labeling only — evaluation stays in `pred_holds_for_lit`.
fn pred_mentions_len(pred: &Expr) -> bool {
    match pred {
        Expr::Call { callee, args, .. } => {
            matches!(callee.as_ref(), Expr::Name { name, .. } if name == "len")
                || args.iter().any(pred_mentions_len)
        }
        Expr::BinOp { left, right, .. } => pred_mentions_len(left) || pred_mentions_len(right),
        Expr::UnaryOp { expr, .. } => pred_mentions_len(expr),
        _ => false,
    }
}

/// [P2-REFINE1-DEF] REQ-REFINE-1 definition-time: `{r: Int | pred}` return type
/// vs a *closed* body (Int literal / unary-minus / closed `if` tree). Decidable
/// false → type error (zero exec). Param-dependent bodies and requires-guided
/// binds stay soft (prove / runtime). See docs/pilot/P2_REFINE1_SLICE.md.
fn check_ret_refine_body(fn_decl: &FnDecl) -> Result<(), TypeError> {
    let Type::Refine {
        name: binder,
        pred: Some(pred),
    } = &fn_decl.ret
    else {
        return Ok(());
    };
    // Stmt-bearing bodies need dataflow; keep soft for this slice.
    if !fn_decl.body.stmts.is_empty() {
        return Ok(());
    }
    let Some(result) = &fn_decl.body.result else {
        return Ok(());
    };
    let Some(value) = eval_closed_int_expr(result) else {
        return Ok(());
    };
    if pred_holds_for_lit(pred, binder, value) == Some(false) {
        return Err(TypeError::at(
            fn_decl.span,
            format!(
                "[P2-REFINE1-DEF] body returns {value} which violates return refinement of {}",
                fn_decl.name
            ),
        ));
    }
    Ok(())
}

fn eval_closed_int_expr(expr: &Expr) -> Option<i64> {
    match expr {
        Expr::LitInt { value, .. } => Some(*value),
        Expr::UnaryOp { op, expr, .. } if op == "-" => eval_closed_int_expr(expr)?.checked_neg(),
        Expr::IfExpr {
            cond,
            then_body,
            else_body,
            ..
        } => {
            // Empty binder => Names in cond fail closedness (soft).
            let c = pred_holds_for_lit(cond, "", 0)?;
            let branch = if c { then_body } else { else_body };
            if !branch.stmts.is_empty() {
                return None;
            }
            eval_closed_int_expr(branch.result.as_ref()?)
        }
        _ => None,
    }
}

fn pred_holds_for_lit(pred: &Expr, binder: &str, val: i64) -> Option<bool> {
    match pred {
        // [P2-REFINE2] Kleene three-valued && / ||: a decided operand decides the
        // connective even when the other side is unevaluable (e.g. a `len(xs)`
        // measure call). Sound vs the interpreter: the evaluable fragment
        // (literal/binder/neg comparisons) never traps at runtime, so a compile
        // decision here agrees with every runtime path — `false && X` is false
        // whether X evaluates, and if X itself traps the call could never
        // succeed anyway (trap-or-violation either way).
        Expr::BinOp { op, left, right, .. } if op == "&&" => {
            match (
                pred_holds_for_lit(left, binder, val),
                pred_holds_for_lit(right, binder, val),
            ) {
                (Some(false), _) | (_, Some(false)) => Some(false),
                (Some(true), Some(true)) => Some(true),
                _ => None,
            }
        }
        Expr::BinOp { op, left, right, .. } if op == "||" => {
            match (
                pred_holds_for_lit(left, binder, val),
                pred_holds_for_lit(right, binder, val),
            ) {
                (Some(true), _) | (_, Some(true)) => Some(true),
                (Some(false), Some(false)) => Some(false),
                _ => None,
            }
        }
        Expr::BinOp { op, left, right, .. } => {
            let l = refine_as_int(left, binder, val)?;
            let r = refine_as_int(right, binder, val)?;
            match op.as_str() {
                "<" => Some(l < r),
                "<=" => Some(l <= r),
                ">" => Some(l > r),
                ">=" => Some(l >= r),
                "==" => Some(l == r),
                "!=" => Some(l != r),
                _ => None,
            }
        }
        Expr::UnaryOp { op, expr, .. } if op == "!" => {
            Some(!pred_holds_for_lit(expr, binder, val)?)
        }
        Expr::LitBool { value, .. } => Some(*value),
        _ => None,
    }
}

fn refine_as_int(expr: &Expr, binder: &str, val: i64) -> Option<i64> {
    match expr {
        Expr::LitInt { value, .. } => Some(*value),
        Expr::Name { name, .. } if name == binder => Some(val),
        Expr::UnaryOp { op, expr, .. } if op == "-" => Some(-refine_as_int(expr, binder, val)?),
        _ => None,
    }
}

fn erase_refine(t: &Type) -> Type {
    match t {
        Type::Refine { .. } => Type::Int,
        other => other.clone(),
    }
}

/// [GAP2-REFINE-TC] Definition-time typecheck of one refinement predicate
/// against the SPEC §3 fragment: a Bool expression over the binder, names in
/// `scope` (params / visible bindings; refine-typed names read as Int),
/// Int/Bool literals, unary `-`/`!`, Int arithmetic, Int comparisons,
/// `&&`/`||`, and the `len(<List-typed expr>)` measure. Anything else is a
/// compile-time error here instead of a runtime trap (or silent inert junk).
fn check_refine_pred_ty(
    pred: &Expr,
    binder: &str,
    scope: &HashMap<String, Type>,
) -> Result<Type, TypeError> {
    match pred {
        Expr::LitInt { .. } => Ok(Type::Int),
        Expr::LitBool { .. } => Ok(Type::Bool),
        Expr::Name { name, span } => {
            if name == binder {
                return Ok(Type::Int);
            }
            match scope.get(name) {
                Some(t) => Ok(erase_refine(t)),
                None => Err(TypeError::at(
                    *span,
                    format!(
                        "[GAP2-REFINE-TC] unknown name {name:?} in refinement predicate \
                         (in scope: binder + parameters declared before this one)"
                    ),
                )),
            }
        }
        Expr::UnaryOp { op, expr, span } => {
            let t = check_refine_pred_ty(expr, binder, scope)?;
            match (op.as_str(), &t) {
                ("-", Type::Int) => Ok(Type::Int),
                ("!", Type::Bool) => Ok(Type::Bool),
                _ => Err(TypeError::at(
                    *span,
                    format!("[GAP2-REFINE-TC] unary {op} on {} in refinement predicate", t.to_str()),
                )),
            }
        }
        Expr::BinOp { op, left, right, span } => {
            let lt = check_refine_pred_ty(left, binder, scope)?;
            let rt = check_refine_pred_ty(right, binder, scope)?;
            match op.as_str() {
                "&&" | "||" => {
                    if matches!(lt, Type::Bool) && matches!(rt, Type::Bool) {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeError::at(*span, "[GAP2-REFINE-TC] && / || need Bool operands in refinement predicate"))
                    }
                }
                "+" | "-" | "*" | "/" | "%" => {
                    if matches!(lt, Type::Int) && matches!(rt, Type::Int) {
                        Ok(Type::Int)
                    } else {
                        Err(TypeError::at(*span, "[GAP2-REFINE-TC] arithmetic needs Int operands in refinement predicate"))
                    }
                }
                "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                    if matches!(lt, Type::Int) && matches!(rt, Type::Int) {
                        Ok(Type::Bool)
                    } else {
                        Err(TypeError::at(*span, "[GAP2-REFINE-TC] comparisons need Int operands in refinement predicate"))
                    }
                }
                _ => Err(TypeError::at(
                    *span,
                    format!("[GAP2-REFINE-TC] operator {op} not allowed in refinement predicate"),
                )),
            }
        }
        Expr::Call { callee, args, span } => {
            let is_len = matches!(callee.as_ref(), Expr::Name { name, .. } if name == "len");
            if !is_len {
                return Err(TypeError::at(
                    *span,
                    "[GAP2-REFINE-TC] only the len(...) measure may be called in a refinement predicate",
                ));
            }
            if args.len() != 1 {
                return Err(TypeError::at(*span, "[GAP2-REFINE-TC] len(...) measure takes exactly 1 argument"));
            }
            let at = check_refine_pred_ty(&args[0], binder, scope)?;
            if matches!(at, Type::List { .. }) {
                Ok(Type::Int)
            } else {
                Err(TypeError::at(
                    *span,
                    format!("[GAP2-REFINE-TC] len(...) measure expects a List, got {}", at.to_str()),
                ))
            }
        }
        other => Err(TypeError::at(
            other.span(),
            "[GAP2-REFINE-TC] expression form not allowed in a refinement predicate \
             (fragment: binder/params, Int/Bool literals, - ! arithmetic, Int comparisons, && ||, len(List))",
        )),
    }
}

/// [GAP2-REFINE-TC] Walk a type and check every refinement predicate found in
/// it (including nested positions like `List<{k: Int | ...}>` and fn types).
/// The pred must check to Bool under `scope` with its binder overlaid.
fn check_type_refines(ty: &Type, scope: &HashMap<String, Type>) -> Result<(), TypeError> {
    match ty {
        Type::Refine { name, pred } => {
            if let Some(p) = pred {
                let t = check_refine_pred_ty(p, name, scope)?;
                if !matches!(t, Type::Bool) {
                    return Err(TypeError::at(
                        p.span(),
                        format!("[GAP2-REFINE-TC] refinement predicate of {{{name}: Int | ...}} must be Bool, got {}", t.to_str()),
                    ));
                }
            }
            Ok(())
        }
        Type::List { elem } => check_type_refines(elem, scope),
        Type::Option { inner } => check_type_refines(inner, scope),
        Type::Result { ok, err } => {
            check_type_refines(ok, scope)?;
            check_type_refines(err, scope)
        }
        Type::Fn { params, ret } => {
            for p in params {
                check_type_refines(p, scope)?;
            }
            check_type_refines(ret, scope)
        }
        _ => Ok(()),
    }
}

fn infer_lambda(
    params: &[(String, Option<Type>)],
    ret: Option<&Type>,
    body: &Block,
    span: Span,
    env: &Env<'_>,
) -> Result<Type, TypeError> {
    let mut e = env.vars.clone();
    let mut pts = Vec::new();
    for (name, ty) in params {
        let Some(t) = ty else {
            return Err(TypeError::at(
                span,
                format!(
                    "lambda param {name} needs a type annotation (or pass via map/filter/fold)"
                ),
            ));
        };
        pts.push(t.clone());
        e.insert(name.clone(), erase_refine(t));
        // [GAP2-REFINE-TC] lambda param refines are runtime-inert (call_closure
        // never evaluates preds) but still fragment-checked; scope = lambda
        // params so far + captured bindings.
        check_type_refines(t, &e)?;
    }
    if let Some(r) = ret {
        // [GAP2-REFINE-TC] lambda return refine, same inert-but-checked rule.
        check_type_refines(r, &e)?;
    }
    let body_ty = check_block(
        body,
        &Env {
            vars: e,
            fns: env.fns,
            adt: env.adt,
            ret: ret.cloned(),
        },
    )?;
    if let Some(r) = ret {
        if !types_equal(&body_ty, r) {
            return Err(TypeError::at(
                span,
                format!(
                    "lambda body {} != declared {}",
                    body_ty.to_str(),
                    r.to_str()
                ),
            ));
        }
        Ok(Type::Fn {
            params: pts,
            ret: Box::new(r.clone()),
        })
    } else {
        Ok(Type::Fn {
            params: pts,
            ret: Box::new(body_ty),
        })
    }
}

/// Unary HOF: `fn (x) { ... }` or `fn (x: T) -> R { ... }` against element type `elem`.
fn check_hof_unary(
    f: &Expr,
    elem: &Type,
    expected_ret: Option<&Type>,
    span: Span,
    env: &Env<'_>,
) -> Result<Type, TypeError> {
    match f {
        Expr::Lambda {
            params,
            ret,
            body,
            span: lsp,
        } => {
            if params.len() != 1 {
                return Err(TypeError::at(span, "unary HOF expects 1-param lambda"));
            }
            let (pname, pty) = &params[0];
            if let Some(t) = pty {
                if !types_equal(t, elem) {
                    return Err(TypeError::at(
                        *lsp,
                        format!("lambda param {} != list elem {}", t.to_str(), elem.to_str()),
                    ));
                }
            }
            let mut e = env.vars.clone();
            e.insert(pname.clone(), erase_refine(elem));
            let body_ty = check_block(
                body,
                &Env {
                    vars: e,
                    fns: env.fns,
                    adt: env.adt,
                    ret: ret.clone(),
                },
            )?;
            if let Some(r) = ret {
                if !types_equal(&body_ty, r) {
                    return Err(TypeError::at(*lsp, "lambda return mismatch"));
                }
            }
            if let Some(er) = expected_ret {
                if !types_equal(&body_ty, er) {
                    return Err(TypeError::at(
                        span,
                        format!("expected return {}, got {}", er.to_str(), body_ty.to_str()),
                    ));
                }
            }
            Ok(body_ty)
        }
        _ => {
            let ft = infer_expr(f, env)?;
            match ft {
                Type::Fn { params, ret } if params.len() == 1 && types_equal(&params[0], elem) => {
                    if let Some(er) = expected_ret {
                        if !types_equal(&ret, er) {
                            return Err(TypeError::at(span, "function return mismatch"));
                        }
                    }
                    Ok(*ret)
                }
                _ => Err(TypeError::at(span, "expected fn (elem) -> _")),
            }
        }
    }
}

fn check_hof_binary(
    f: &Expr,
    acc_ty: &Type,
    elem: &Type,
    expected_ret: &Type,
    span: Span,
    env: &Env<'_>,
) -> Result<(), TypeError> {
    match f {
        Expr::Lambda {
            params,
            ret,
            body,
            span: lsp,
        } => {
            if params.len() != 2 {
                return Err(TypeError::at(span, "fold fn expects 2 params (acc, elem)"));
            }
            let (a_name, a_ty) = &params[0];
            let (e_name, e_ty) = &params[1];
            if let Some(t) = a_ty {
                if !types_equal(t, acc_ty) {
                    return Err(TypeError::at(*lsp, "fold acc param type mismatch"));
                }
            }
            if let Some(t) = e_ty {
                if !types_equal(t, elem) {
                    return Err(TypeError::at(*lsp, "fold elem param type mismatch"));
                }
            }
            let mut e = env.vars.clone();
            e.insert(a_name.clone(), erase_refine(acc_ty));
            e.insert(e_name.clone(), erase_refine(elem));
            let body_ty = check_block(
                body,
                &Env {
                    vars: e,
                    fns: env.fns,
                    adt: env.adt,
                    ret: ret.clone(),
                },
            )?;
            if let Some(r) = ret {
                if !types_equal(&body_ty, r) {
                    return Err(TypeError::at(*lsp, "fold lambda return mismatch"));
                }
            }
            if !types_equal(&body_ty, expected_ret) {
                return Err(TypeError::at(
                    span,
                    format!(
                        "fold body {} != acc {}",
                        body_ty.to_str(),
                        expected_ret.to_str()
                    ),
                ));
            }
            Ok(())
        }
        _ => {
            let ft = infer_expr(f, env)?;
            match ft {
                Type::Fn { params, ret }
                    if params.len() == 2
                        && types_equal(&params[0], acc_ty)
                        && types_equal(&params[1], elem)
                        && types_equal(&ret, expected_ret) =>
                {
                    Ok(())
                }
                _ => Err(TypeError::at(span, "expected fn (acc, elem) -> acc")),
            }
        }
    }
}

fn infer_ctor(
    type_name: Option<&str>,
    name: &str,
    args: &[Expr],
    span: Span,
    env: &Env<'_>,
) -> Result<Type, TypeError> {
    if let Some(tn) = type_name {
        let Some(ed) = env.adt.enums.get(tn) else {
            return Err(TypeError::at(span, format!("unknown enum {tn}")));
        };
        let Some(vd) = ed.variants.iter().find(|v| v.name == name) else {
            return Err(TypeError::at(
                span,
                format!("unknown variant {tn}::{name}"),
            ));
        };
        if args.len() != vd.fields.len() {
            return Err(TypeError::at(
                span,
                format!(
                    "{tn}::{name} expects {} args, got {}",
                    vd.fields.len(),
                    args.len()
                ),
            ));
        }
        for (a, ft) in args.iter().zip(vd.fields.iter()) {
            let at = infer_expr(a, env)?;
            if !types_equal(&at, ft) {
                return Err(TypeError::at(
                    span,
                    format!("arg type {} != {}", at.to_str(), ft.to_str()),
                ));
            }
        }
        return Ok(Type::Named {
            name: tn.to_string(),
        });
    }

    match name {
        "None" => {
            if !args.is_empty() {
                return Err(TypeError::at(span, "None takes no arguments"));
            }
            Ok(Type::Option {
                inner: Box::new(Type::Int),
            })
        }
        "Some" => {
            if args.len() != 1 {
                return Err(TypeError::at(span, "Some takes 1 argument"));
            }
            let inner = infer_expr(&args[0], env)?;
            Ok(Type::Option {
                inner: Box::new(inner),
            })
        }
        "Ok" => {
            if args.len() != 1 {
                return Err(TypeError::at(span, "Ok takes 1 argument"));
            }
            let ok = infer_expr(&args[0], env)?;
            Ok(Type::Result {
                ok: Box::new(ok),
                err: Box::new(Type::Str),
            })
        }
        "Err" => {
            if args.len() != 1 {
                return Err(TypeError::at(span, "Err takes 1 argument"));
            }
            let err = infer_expr(&args[0], env)?;
            Ok(Type::Result {
                ok: Box::new(Type::Int),
                err: Box::new(err),
            })
        }
        _ => Err(TypeError::at(span, format!("unknown constructor {name}"))),
    }
}

/// [P2E-FIX] `Shape::Pt` + arity 2 -> `Shape::Pt(_, _)`; arity 0 -> bare name.
fn pattern_stub(name: &str, arity: usize) -> String {
    if arity == 0 {
        name.to_string()
    } else {
        format!("{name}({})", vec!["_"; arity].join(", "))
    }
}

fn check_match(
    scrutinee: &Expr,
    arms: &[MatchArm],
    span: Span,
    env: &Env<'_>,
) -> Result<Type, TypeError> {
    let st = infer_expr(scrutinee, env)?;
    let mut arm_tys: Vec<Type> = Vec::new();
    let mut covered: HashSet<String> = HashSet::new();
    let mut has_wildcard = false;

    for arm in arms {
        let (bindings, ctor_name) = check_pattern(&arm.pattern, &st, env.adt)?;
        if let Some(c) = ctor_name {
            covered.insert(c);
        } else if matches!(arm.pattern, Pattern::Wildcard { .. } | Pattern::Bind { .. }) {
            has_wildcard = true;
        }
        let mut e = env.vars.clone();
        for (n, t) in bindings {
            e.insert(n, t);
        }
        let arm_env = Env {
            vars: e,
            fns: env.fns,
            adt: env.adt,
            ret: env.ret.clone(),
        };
        arm_tys.push(infer_expr(&arm.body, &arm_env)?);
    }

    if !has_wildcard {
        match &st {
            Type::Option { .. } => {
                if !(covered.contains("Some") && covered.contains("None")) {
                    // [P2E-FIX] uncovered arms as pattern stubs for the FixPatch.
                    let mut missing: Vec<String> = Vec::new();
                    if !covered.contains("Some") {
                        missing.push("Some(_)".into());
                    }
                    if !covered.contains("None") {
                        missing.push("None".into());
                    }
                    return Err(TypeError::at_fix(
                        span,
                        "non-exhaustive match on Option (need Some and None, or _)",
                        missing,
                    ));
                }
            }
            Type::Result { .. } => {
                if !(covered.contains("Ok") && covered.contains("Err")) {
                    // [P2E-FIX] uncovered arms as pattern stubs for the FixPatch.
                    let mut missing: Vec<String> = Vec::new();
                    if !covered.contains("Ok") {
                        missing.push("Ok(_)".into());
                    }
                    if !covered.contains("Err") {
                        missing.push("Err(_)".into());
                    }
                    return Err(TypeError::at_fix(
                        span,
                        "non-exhaustive match on Result (need Ok and Err, or _)",
                        missing,
                    ));
                }
            }
            Type::Named { name } => {
                if let Some(ed) = env.adt.enums.get(name) {
                    // [P2E-FIX] collect ALL uncovered variants (SPEC §4.1 names
                    // the missing constructors) as arity-aware pattern stubs.
                    let mut names: Vec<String> = Vec::new();
                    let mut missing: Vec<String> = Vec::new();
                    for v in &ed.variants {
                        let key = format!("{}::{}", name, v.name);
                        if !covered.contains(&key) && !covered.contains(&v.name) {
                            names.push(v.name.clone());
                            missing.push(pattern_stub(&key, v.fields.len()));
                        }
                    }
                    if !missing.is_empty() {
                        return Err(TypeError::at_fix(
                            span,
                            format!(
                                "non-exhaustive match on {name}: missing {}",
                                names.join(", ")
                            ),
                            missing,
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    let first = arm_tys[0].clone();
    for t in &arm_tys[1..] {
        if !types_equal(&first, t) {
            return Err(TypeError::at(
                span,
                format!(
                    "match arms differ: {} vs {}",
                    first.to_str(),
                    t.to_str()
                ),
            ));
        }
    }
    let _ = resolve_named;
    Ok(first)
}

fn check_pattern(
    pat: &Pattern,
    expected: &Type,
    adt: &AdtEnv,
) -> Result<(Vec<(String, Type)>, Option<String>), TypeError> {
    match pat {
        Pattern::Wildcard { .. } => Ok((vec![], None)),
        Pattern::Bind { name, .. } => Ok((vec![(name.clone(), expected.clone())], None)),
        Pattern::LitInt { span, .. } => {
            if !matches!(expected, Type::Int | Type::Refine { .. }) {
                return Err(TypeError::at(*span, "int pattern on non-Int"));
            }
            Ok((vec![], None))
        }
        Pattern::LitBool { span, .. } => {
            if !matches!(expected, Type::Bool) {
                return Err(TypeError::at(*span, "bool pattern on non-Bool"));
            }
            Ok((vec![], None))
        }
        Pattern::LitStr { span, .. } => {
            if !matches!(expected, Type::Str) {
                return Err(TypeError::at(*span, "str pattern on non-Str"));
            }
            Ok((vec![], None))
        }
        Pattern::LitUnit { span } => {
            if !matches!(expected, Type::Unit) {
                return Err(TypeError::at(*span, "unit pattern on non-Unit"));
            }
            Ok((vec![], None))
        }
        Pattern::Ctor {
            type_name,
            name,
            args,
            span,
        } => {
            if let Some(tn) = type_name {
                let Type::Named { name: en } = expected else {
                    return Err(TypeError::at(
                        *span,
                        format!("pattern {tn}::{name} on non-enum"),
                    ));
                };
                if en != tn {
                    return Err(TypeError::at(
                        *span,
                        format!("pattern {tn}::{name} on type {en}"),
                    ));
                }
                let Some(ed) = adt.enums.get(tn) else {
                    return Err(TypeError::at(*span, format!("unknown enum {tn}")));
                };
                let Some(vd) = ed.variants.iter().find(|v| v.name == *name) else {
                    return Err(TypeError::at(
                        *span,
                        format!("unknown variant {tn}::{name}"),
                    ));
                };
                if args.len() != vd.fields.len() {
                    return Err(TypeError::at(*span, "variant pattern arity mismatch"));
                }
                let mut binds = Vec::new();
                for (a, ft) in args.iter().zip(vd.fields.iter()) {
                    let (b, _) = check_pattern(a, ft, adt)?;
                    binds.extend(b);
                }
                return Ok((binds, Some(format!("{tn}::{name}"))));
            }
            match (name.as_str(), expected) {
                ("None", Type::Option { .. }) => {
                    if !args.is_empty() {
                        return Err(TypeError::at(*span, "None pattern takes no args"));
                    }
                    Ok((vec![], Some("None".into())))
                }
                ("Some", Type::Option { inner }) => {
                    if args.len() != 1 {
                        return Err(TypeError::at(*span, "Some pattern takes 1 arg"));
                    }
                    let (binds, _) = check_pattern(&args[0], inner, adt)?;
                    Ok((binds, Some("Some".into())))
                }
                ("Ok", Type::Result { ok, .. }) => {
                    if args.len() != 1 {
                        return Err(TypeError::at(*span, "Ok pattern takes 1 arg"));
                    }
                    let (binds, _) = check_pattern(&args[0], ok, adt)?;
                    Ok((binds, Some("Ok".into())))
                }
                ("Err", Type::Result { err, .. }) => {
                    if args.len() != 1 {
                        return Err(TypeError::at(*span, "Err pattern takes 1 arg"));
                    }
                    let (binds, _) = check_pattern(&args[0], err, adt)?;
                    Ok((binds, Some("Err".into())))
                }
                _ => Err(TypeError::at(
                    *span,
                    format!(
                        "constructor {name} does not match scrutinee {}",
                        expected.to_str()
                    ),
                )),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// [GAP4-R2-SURFACE] Thin label typecheck surface over the [GAP4-R2-PILOT]
// lattice (SPEC §4.2 SUB-LABEL sink upper bound). Scope: EXPLICIT flows only,
// one hop — a call argument that is a plain `Name` carries its seeded label;
// every other expression shape is ⊥ (no taint propagation through
// computation this slice). Enforcement points:
//   1. named-fn call arguments against the callee parameter's seeded upper
//      bound (E1 injection shape: `db.insert`-style ∅-data params);
//   2. `Console.print` arguments against the ∅-data sink bound SPEC §4.2
//      names verbatim (E6 leak shape).
// Source labels and sink bounds BOTH come from `seeds`
// ((fn name, binding name) -> Label) — there is no value-label syntax yet
// (SPEC §3: `T^{...}` may be post-MVP; `uses` stays the only authority
// surface). Bounds compare on the DATA projection (SPEC wording: sinks bound
// "at ∅-data"), so authority atoms on capability handles never trip a data
// bound. NOT full IFC, NOT the R2 inference-ergonomics gate, NOT implicit
// flows.
// ---------------------------------------------------------------------------

/// [GAP4-R2-SURFACE] Seeded label pass. Run it after `check_program` (or on
/// an otherwise well-typed program): the walk assumes `.print` field-calls
/// are `Console.print` — the only field-call sink in the MVP surface — and
/// re-runs no other check. `check_program` itself calls this with empty
/// seeds, which is inert by the lattice laws.
pub fn check_program_labels(
    program: &Program,
    seeds: &HashMap<(String, String), Label>,
) -> Result<(), TypeError> {
    let mut fns: HashMap<String, &FnDecl> = HashMap::new();
    for f in &program.functions {
        fns.insert(f.name.clone(), f);
    }
    for f in &program.functions {
        label_walk_block(&f.body, &f.name, &fns, seeds)?;
    }
    Ok(())
}

/// Seeded label of a binding, ⊥ when unseeded.
fn seed_label(
    seeds: &HashMap<(String, String), Label>,
    fn_name: &str,
    binding: &str,
) -> Label {
    seeds
        .get(&(fn_name.to_string(), binding.to_string()))
        .cloned()
        .unwrap_or_else(Label::bottom)
}

/// One-hop source rule: a bare `Name` argument carries its seeded label;
/// every other expression shape is ⊥ this slice.
fn label_of_arg(
    arg: &Expr,
    fn_name: &str,
    seeds: &HashMap<(String, String), Label>,
) -> Label {
    match arg {
        Expr::Name { name, .. } => seed_label(seeds, fn_name, name),
        _ => Label::bottom(),
    }
}

fn label_data_str(l: &Label) -> String {
    let atoms: Vec<&str> = l
        .0
        .iter()
        .map(|a| match a {
            Atom::Auth(name) => name.as_str(),
            Atom::Untrusted => "untrusted",
            Atom::Secret => "secret",
        })
        .collect();
    format!("{{{}}}", atoms.join(", "))
}

/// (SUB-LABEL) data-projection flow check: `arg.data() ⊑ bound.data()`.
fn check_data_flow(
    arg: &Label,
    bound: &Label,
    what: &str,
    sink: &str,
    span: Span,
) -> Result<(), TypeError> {
    let (a, b) = (arg.data(), bound.data());
    if a.flows_to(&b) {
        Ok(())
    } else {
        Err(TypeError::at(
            span,
            format!(
                "[GAP4-R2-SURFACE] ill-labeled flow: {what} with data label {} does not flow to {sink} (bound {})",
                label_data_str(&a),
                label_data_str(&b),
            ),
        ))
    }
}

fn arg_desc(arg: &Expr) -> String {
    match arg {
        Expr::Name { name, .. } => format!("argument '{name}'"),
        _ => "argument".to_string(),
    }
}

fn label_walk_block(
    block: &Block,
    fn_name: &str,
    fns: &HashMap<String, &FnDecl>,
    seeds: &HashMap<(String, String), Label>,
) -> Result<(), TypeError> {
    for stmt in &block.stmts {
        match stmt {
            Stmt::Let { value, .. } => label_walk_expr(value, fn_name, fns, seeds)?,
            Stmt::Expr { expr, .. } => label_walk_expr(expr, fn_name, fns, seeds)?,
        }
    }
    if let Some(res) = &block.result {
        label_walk_expr(res, fn_name, fns, seeds)?;
    }
    Ok(())
}

fn label_walk_expr(
    expr: &Expr,
    fn_name: &str,
    fns: &HashMap<String, &FnDecl>,
    seeds: &HashMap<(String, String), Label>,
) -> Result<(), TypeError> {
    match expr {
        Expr::Call { callee, args, span } => {
            if let Expr::Name { name, .. } = callee.as_ref() {
                // (SUB-LABEL) E1 shape: each argument's data label must stay
                // within the callee parameter's seeded upper bound.
                if let Some(fd) = fns.get(name) {
                    for (a, p) in args.iter().zip(fd.params.iter()) {
                        let al = label_of_arg(a, fn_name, seeds);
                        let bound = seed_label(seeds, &fd.name, &p.name);
                        check_data_flow(
                            &al,
                            &bound,
                            &arg_desc(a),
                            &format!("parameter '{}' of {}", p.name, fd.name),
                            *span,
                        )?;
                    }
                }
            }
            if let Expr::FieldAccess { field, .. } = callee.as_ref() {
                // (SUB-LABEL) E6 shape: Console.print bounds its argument at
                // ∅-data (SPEC §4.2's verbatim leak example).
                if field == "print" {
                    for a in args {
                        let al = label_of_arg(a, fn_name, seeds);
                        check_data_flow(
                            &al,
                            &Label::bottom(),
                            &arg_desc(a),
                            "Console.print",
                            *span,
                        )?;
                    }
                }
            }
            label_walk_expr(callee, fn_name, fns, seeds)?;
            for a in args {
                label_walk_expr(a, fn_name, fns, seeds)?;
            }
            Ok(())
        }
        Expr::BinOp { left, right, .. } => {
            label_walk_expr(left, fn_name, fns, seeds)?;
            label_walk_expr(right, fn_name, fns, seeds)
        }
        Expr::UnaryOp { expr: e, .. } | Expr::Propagate { expr: e, .. } => {
            label_walk_expr(e, fn_name, fns, seeds)
        }
        Expr::FieldAccess { obj, .. } => label_walk_expr(obj, fn_name, fns, seeds),
        Expr::Ctor { args, .. } => {
            for a in args {
                label_walk_expr(a, fn_name, fns, seeds)?;
            }
            Ok(())
        }
        Expr::StructLit { fields, .. } => {
            for (_, e) in fields {
                label_walk_expr(e, fn_name, fns, seeds)?;
            }
            Ok(())
        }
        Expr::ListLit { elems, .. } => {
            for e in elems {
                label_walk_expr(e, fn_name, fns, seeds)?;
            }
            Ok(())
        }
        Expr::Lambda { body, .. } => label_walk_block(body, fn_name, fns, seeds),
        Expr::IfExpr {
            cond,
            then_body,
            else_body,
            ..
        } => {
            label_walk_expr(cond, fn_name, fns, seeds)?;
            label_walk_block(then_body, fn_name, fns, seeds)?;
            label_walk_block(else_body, fn_name, fns, seeds)
        }
        Expr::MatchExpr {
            scrutinee, arms, ..
        } => {
            label_walk_expr(scrutinee, fn_name, fns, seeds)?;
            for arm in arms {
                label_walk_expr(&arm.body, fn_name, fns, seeds)?;
            }
            Ok(())
        }
        Expr::Block(b) => label_walk_block(b, fn_name, fns, seeds),
        Expr::LitInt { .. }
        | Expr::LitStr { .. }
        | Expr::LitBool { .. }
        | Expr::LitUnit { .. }
        | Expr::Name { .. }
        | Expr::Hole { .. } => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn refine1_rejects_out_of_range_literal_call() {
        // SPEC §4.4 REQ-REFINE-1: apply_discount(100, 150) is a type error, zero exec.
        let src = r#"
fn apply_discount(price: {p: Int | p >= 0}, pct: {d: Int | 0 <= d && d <= 100}) -> Int {
    price
}
fn main(console: Console) -> Unit uses {console} {
    console.print(apply_discount(100, 150).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-REFINE1 reject");
        assert!(
            err.0.contains("[P2-REFINE1]"),
            "expected [P2-REFINE1] in {err}"
        );
    }

    #[test]
    fn refine1_accepts_in_range_literal_call() {
        let src = r#"
fn apply_discount(price: {p: Int | p >= 0}, pct: {d: Int | 0 <= d && d <= 100}) -> Int {
    price
}
fn main(console: Console) -> Unit uses {console} {
    console.print(apply_discount(100, 10).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("in-range call must typecheck");
    }

    #[test]
    fn refine1_rejects_negative_literal_call() {
        // [P2-REFINE1] `-5` (unary minus over a literal) is a literal for reject purposes.
        let src = r#"
fn pos(x: {x: Int | x >= 1}) -> Int {
    x
}
fn main(console: Console) -> Unit uses {console} {
    console.print(pos(-5).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-REFINE1 reject");
        assert!(err.0.contains("[P2-REFINE1]"), "{err}");
    }

    #[test]
    fn refine1_def_rejects_negative_literal_return() {
        // [P2-REFINE1-DEF] SPEC section 4.4 definition-time negative return.
        let src = r#"
fn bad() -> {r: Int | r >= 0} {
    -1
}
fn main(console: Console) -> Unit uses {console} {
    console.print(bad().show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-REFINE1-DEF reject");
        assert!(
            err.0.contains("[P2-REFINE1-DEF]"),
            "expected [P2-REFINE1-DEF] in {err}"
        );
    }

    #[test]
    fn refine1_def_accepts_nonneg_literal_return() {
        let src = r#"
fn good() -> {r: Int | r >= 0} {
    0
}
fn main(console: Console) -> Unit uses {console} {
    console.print(good().show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("nonneg literal return must typecheck");
    }

    #[test]
    fn refine1_def_rejects_closed_ite_false_branch() {
        // Closed ite: cond + branches are literals → decidable without SMT.
        let src = r#"
fn bad() -> {r: Int | r >= 0} {
    if 1 < 0 { 1 } else { -1 }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(bad().show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-REFINE1-DEF reject");
        assert!(err.0.contains("[P2-REFINE1-DEF]"), "{err}");
    }

    #[test]
    fn refine1_def_soft_on_param_dependent_body() {
        // Body mentions param - not closed; stay soft (prove/runtime).
        let src = r#"
fn id(x: Int) -> {r: Int | r >= 0} {
    x
}
fn main(console: Console) -> Unit uses {console} {
    console.print(id(1).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("param-dependent return refine stays soft");
    }

    #[test]
    fn refine2_rejects_negative_literal_index_with_len_measure() {
        // SPEC §4.4 REQ-REFINE-2: nth(xs, -1) is a type error, zero execution.
        // `0 <= k` decides false; the len(xs) conjunct stays unknown (Kleene).
        let src = r#"
fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    match xs.get(i) {
        Some(v) => v,
        None => -1,
    }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(nth([10, 20, 30], -1).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-REFINE2 reject");
        assert!(
            err.0.contains("[P2-REFINE2]"),
            "expected [P2-REFINE2] in {err}"
        );
    }

    #[test]
    fn refine2_accepts_in_range_literal_index() {
        // 0 <= 1 decides true; 1 < len(xs) stays soft (prove / runtime tier).
        let src = r#"
fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    match xs.get(i) {
        Some(v) => v,
        None => -1,
    }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(nth([10, 20, 30], 1).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("in-range literal index must typecheck");
    }

    #[test]
    fn refine2_unbounded_literal_index_stays_soft() {
        // Honest limit: 5 < len(xs) is undecidable here without list-length
        // reasoning — call stays soft; the runtime refinement check guards it.
        let src = r#"
fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    match xs.get(i) {
        Some(v) => v,
        None => -1,
    }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(nth([10, 20, 30], 5).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("unbounded literal index stays soft (runtime tier)");
    }

    #[test]
    fn refine2_kleene_or_true_short_circuits() {
        // k < 0 || k < len(xs) at k = -1: true || unknown = true → no reject,
        // matching the interpreter's short-circuit `||`.
        let src = r#"
fn f(xs: List<Int>, i: {k: Int | k < 0 || k < len(xs)}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f([1], -1).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("true || unknown must not reject");
    }

    #[test]
    fn refine2_kleene_second_conjunct_rejects() {
        // (0 <= k && k < len(xs)) && k <= 100 at k = 200:
        // left is unknown, right decides false → unknown && false = false → reject.
        let src = r#"
fn f(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs) && k <= 100}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f([1], 200).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-REFINE2 reject via second conjunct");
        assert!(
            err.0.contains("[P2-REFINE2]"),
            "expected [P2-REFINE2] in {err}"
        );
    }

    #[test]
    fn gap2_rejects_len_over_int_binder() {
        // [GAP2-REFINE-TC] this shape used to typecheck and trap at runtime
        // ("len(...) measure expects a List"); now rejected at define time.
        let src = r#"
fn f(i: {k: Int | k < len(k)}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f(1).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 reject");
        assert!(err.0.contains("[GAP2-REFINE-TC]"), "{err}");
        assert!(err.0.contains("expects a List"), "{err}");
    }

    #[test]
    fn gap2_rejects_unknown_name_in_pred() {
        let src = r#"
fn f(i: {k: Int | k < zz}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f(1).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 reject");
        assert!(err.0.contains("[GAP2-REFINE-TC]"), "{err}");
        assert!(err.0.contains("unknown name"), "{err}");
    }

    #[test]
    fn gap2_rejects_forward_param_reference() {
        // Prefix scoping matches the interpreter's binding order: param i's
        // pred may not reference a later param (runtime would trap unbound).
        let src = r#"
fn h(i: {k: Int | k < j}, j: Int) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(h(1, 2).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 forward-ref reject");
        assert!(err.0.contains("[GAP2-REFINE-TC]"), "{err}");
    }

    #[test]
    fn gap2_rejects_non_bool_pred() {
        let src = r#"
fn f(i: {k: Int | k + 1}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f(1).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 non-Bool reject");
        assert!(err.0.contains("must be Bool"), "{err}");
    }

    #[test]
    fn gap2_rejects_disallowed_form_in_pred() {
        // `if` is outside the spec pred fragment even when runtime-evaluable.
        let src = r#"
fn f(i: {k: Int | if true { true } else { false }}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f(1).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 form reject");
        assert!(err.0.contains("not allowed in a refinement predicate"), "{err}");
    }

    #[test]
    fn gap2_rejects_user_fn_call_in_pred() {
        // Preds are pure spec-fragment expressions: only the len(...) measure
        // may be called (runtime used to happily evaluate helper()).
        let src = r#"
fn helper(x: Int) -> Int {
    x
}
fn f(i: {k: Int | helper(k) > 0}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f(1).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 user-call reject");
        assert!(err.0.contains("only the len(...) measure"), "{err}");
    }

    #[test]
    fn gap2_checks_lambda_refine_params() {
        // Lambda param refines are runtime-inert but fragment-checked: valid
        // pred accepted, malformed pred rejected.
        let ok = r#"
fn main(console: Console) -> Unit uses {console} {
    let id = fn (x: {k: Int | k >= 0}) -> Int { x };
    console.print(id(1).show());
}
"#;
        let prog = parse(ok).expect("parse");
        check_program(&prog).expect("valid lambda refine pred must typecheck");

        let bad = r#"
fn main(console: Console) -> Unit uses {console} {
    let id = fn (x: {k: Int | k < len(k)}) -> Int { x };
    console.print(id(1).show());
}
"#;
        let prog = parse(bad).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 lambda pred reject");
        assert!(err.0.contains("[GAP2-REFINE-TC]"), "{err}");
    }

    #[test]
    fn gap2_checks_let_annotation_refines() {
        // Let-annotation refines are runtime-inert but fragment-checked,
        // including nested positions like List<{k: Int | ...}>.
        let ok = r#"
fn main(console: Console) -> Unit uses {console} {
    let x: {k: Int | k >= 0} = 5;
    console.print(x.show());
}
"#;
        let prog = parse(ok).expect("parse");
        check_program(&prog).expect("valid let refine pred must typecheck");

        let bad = r#"
fn main(console: Console) -> Unit uses {console} {
    let xs: List<{k: Int | k < len(k)}> = [1];
    console.print("n");
}
"#;
        let prog = parse(bad).expect("parse");
        let err = check_program(&prog).expect_err("expected GAP2 nested let pred reject");
        assert!(err.0.contains("[GAP2-REFINE-TC]"), "{err}");
    }

    #[test]
    fn gap2_accepts_full_valid_fragment() {
        // Arithmetic + parens + logic + len over a List param, backward refs.
        let src = r#"
fn f(lo: Int, hi: Int, xs: List<Int>, i: {k: Int | k * 2 <= hi && (k >= lo || k == 0) && k < len(xs)}) -> Int {
    i
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f(0, 10, [1, 2, 3], 1).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("full valid fragment must typecheck");
    }

    #[test]
    fn dupfn_rejects_duplicate_function_names() {
        // [P2-DUPFN] a later `fn f` used to silently shadow an earlier one at
        // runtime; now it is a compile-time error at the second declaration.
        let src = r#"
fn f() -> Int {
    1
}
fn f() -> Int {
    2
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f().show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-DUPFN reject");
        assert!(err.0.contains("[P2-DUPFN]"), "{err}");
        assert!(err.0.contains("duplicate function f"), "{err}");
    }

    #[test]
    fn p2e_non_exhaustive_enum_match_carries_full_fix_payload() {
        // [P2E-FIX] message names ALL missing variants; payload = arity-aware
        // arm pattern stubs anchored at the match expression's span.
        let src = r#"
enum Shape {
    Dot,
    Line(Int),
    Rect(Int, Int),
}
fn shape_label(s: Shape) -> Str {
    match s {
        Shape::Dot => "dot",
    }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(shape_label(Shape::Dot));
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected non-exhaustive reject");
        assert!(
            err.0.contains("non-exhaustive match on Shape: missing Line, Rect"),
            "{err}"
        );
        let fix = err.1.as_ref().expect("fix payload");
        assert_eq!(
            fix.missing,
            vec!["Shape::Line(_)".to_string(), "Shape::Rect(_, _)".to_string()]
        );
        assert_eq!(fix.span.line, 8, "match expr line, got {:?}", fix.span);
    }

    #[test]
    fn propagate_into_plain_int_ret_is_rejected() {
        // [P2-SOUND3] guard: `?` must not escape a fn whose return type cannot carry None.
        let src = r#"
fn first(x: Option<Int>) -> Int {
    let y: Int = x?;
    y
}
fn main(console: Console) -> Unit uses {console} {
    console.print(first(Some(1)).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected P2-SOUND3 reject");
        assert!(err.0.contains("`?` on Option"), "{err}");
    }

    #[test]
    fn propagate_into_option_ret_is_ok() {
        // [P2-SOUND3] the propagate.vera shape stays legal.
        let src = r#"
fn dig(xs: List<Int>) -> Option<Int> {
    let h: Int = xs.head()?;
    Some(h)
}
fn main(console: Console) -> Unit uses {console} {
    console.print(match dig([1]) {
        Some(n) => n.show(),
        None => "none",
    });
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("Option-into-Option `?` must typecheck");
    }

    #[test]
    fn propagate_result_err_mismatch_is_rejected() {
        // [P2-SOUND3] Err payload type must survive the early return unchanged.
        let src = r#"
fn conv(r: Result<Int, Int>) -> Result<Int, Str> {
    let x: Int = r?;
    Ok(x)
}
fn main(console: Console) -> Unit uses {console} {
    console.print("n");
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected err-type mismatch reject");
        assert!(err.0.contains("error type"), "{err}");
    }

    #[test]
    fn gap4_surface_rejects_untrusted_arg_into_bare_param_e1() {
        // [GAP4-R2-SURFACE] E1 injection shape (SPEC §4.2): store_row's `row`
        // is unlabeled (⊥-data bound), so an `untrusted`-seeded argument must
        // not flow into it. The same program stays green on the ordinary
        // front door (empty seeds — inertness half of the pair).
        let src = r#"
fn store_row(row: Str) -> Unit {
    row;
}
fn main(console: Console) -> Unit uses {console} {
    let user_input: Str = "row";
    store_row(user_input);
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("front door (empty seeds) must stay green");
        let mut seeds: HashMap<(String, String), Label> = HashMap::new();
        seeds.insert(
            ("main".into(), "user_input".into()),
            Label::of(&[Atom::Untrusted]),
        );
        let err = check_program_labels(&prog, &seeds).expect_err("expected E1 reject");
        assert!(err.0.contains("[GAP4-R2-SURFACE]"), "{err}");
        assert!(err.0.contains("argument 'user_input'"), "{err}");
        assert!(err.0.contains("{untrusted}"), "{err}");
        assert!(err.0.contains("parameter 'row' of store_row"), "{err}");
    }

    #[test]
    fn gap4_surface_rejects_secret_arg_into_console_print_e6() {
        // [GAP4-R2-SURFACE] E6 leak shape (SPEC §4.2's verbatim example):
        // Console.print bounds its argument at ∅-data, so a `secret`-seeded
        // value must not reach it.
        let src = r#"
fn main(console: Console) -> Unit uses {console} {
    let token: Str = "hunter2";
    console.print(token);
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("front door (empty seeds) must stay green");
        let mut seeds: HashMap<(String, String), Label> = HashMap::new();
        seeds.insert(("main".into(), "token".into()), Label::of(&[Atom::Secret]));
        let err = check_program_labels(&prog, &seeds).expect_err("expected E6 reject");
        assert!(err.0.contains("[GAP4-R2-SURFACE]"), "{err}");
        assert!(err.0.contains("{secret}"), "{err}");
        assert!(err.0.contains("Console.print"), "{err}");
    }

    #[test]
    fn gap4_surface_accepts_bounded_sink_and_auth_handle() {
        // [GAP4-R2-SURFACE] accept side of the pair: (a) a parameter seeded
        // at {secret} accepts a secret argument (`net.connect(auth:)` shape);
        // (b) an authority atom is not data — the ∅-data bound of an
        // unseeded parameter does not fire on a capability handle
        // (TAINT-PROP philosophy: authority rides handles, it does not
        // taint).
        let src = r#"
fn send_auth(a: Str) -> Unit {
    a;
}
fn main(console: Console) -> Unit uses {console} {
    let token: Str = "hunter2";
    send_auth(token);
}
"#;
        let prog = parse(src).expect("parse");
        let mut seeds: HashMap<(String, String), Label> = HashMap::new();
        seeds.insert(("main".into(), "token".into()), Label::of(&[Atom::Secret]));
        seeds.insert(
            ("send_auth".into(), "a".into()),
            Label::of(&[Atom::Secret]),
        );
        check_program_labels(&prog, &seeds)
            .expect("secret must flow into a secret-bounded sink");

        let mut auth_seeds: HashMap<(String, String), Label> = HashMap::new();
        auth_seeds.insert(
            ("main".into(), "token".into()),
            Label::of(&[Atom::Auth("console".into())]),
        );
        check_program_labels(&prog, &auth_seeds)
            .expect("authority atoms are not data; a ⊥-data bound must not fire");
    }

    #[test]
    fn gapc1_rejects_len_of_same_list_as_index() {
        // [GAPC1-SYM-LEN] SPEC REQ-REFINE-2's symbolic case (P2C honest
        // limit): nth_c1(data, data.len()) substitutes k := len(xs), so
        // `k < len(xs)` becomes `len(xs) < len(xs)` — decidably false with
        // zero execution and no knowledge of the list's actual length.
        let src = r#"
fn nth_c1(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    match xs.get(i) {
        Some(v) => v,
        None => 0,
    }
}
fn main(console: Console) -> Unit uses {console} {
    let data: List<Int> = [10, 20, 30];
    console.print(nth_c1(data, data.len()).show());
}
"#;
        let prog = parse(src).expect("parse");
        let err = check_program(&prog).expect_err("expected GAPC1 symbolic reject");
        assert!(err.0.contains("[GAPC1-SYM-LEN]"), "{err}");
        assert!(err.0.contains("data.len()"), "{err}");
    }

    #[test]
    fn gapc1_len_minus_one_and_other_list_stay_soft() {
        // Negative controls: `a.len() - 1` is not the bare same-term shape
        // (BinOp argument), and `b.len()` measures a DIFFERENT list — both
        // stay soft -> prove / runtime, exactly the P2C design.
        let src = r#"
fn nth_c2(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    match xs.get(i) {
        Some(v) => v,
        None => 0,
    }
}
fn main(console: Console) -> Unit uses {console} {
    let a: List<Int> = [1, 2, 3];
    let b: List<Int> = [1];
    console.print(nth_c2(a, a.len() - 1).show());
    console.print(nth_c2(a, b.len()).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("BinOp arg and other-list len must stay soft");
    }

    #[test]
    fn gapc1_kleene_or_guard_stays_soft() {
        // `k < 0 || k < len(xs)` under k := len(xs) is unknown || false ->
        // unknown: the Kleene-|| guard keeps it soft (a full solver would
        // reject; this slice honestly does not claim to be one).
        let src = r#"
fn pick_c3(xs: List<Int>, i: {k: Int | k < 0 || k < len(xs)}) -> Int {
    0
}
fn main(console: Console) -> Unit uses {console} {
    let data: List<Int> = [4, 5];
    console.print(pick_c3(data, data.len()).show());
}
"#;
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("Kleene-|| guard must stay soft");
    }
}
