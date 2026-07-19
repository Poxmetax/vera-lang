//! Tree-walking interpreter with Console capability + runtime contracts (Phase 1).

use crate::ast::*;
use crate::vc::ProvedSet;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct Trap(pub String);

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
    Unit,
    Console,
    OptionSome(Box<Value>),
    OptionNone,
    ResultOk(Box<Value>),
    ResultErr(Box<Value>),
    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },
    Enum {
        type_name: String,
        variant: String,
        fields: Vec<Value>,
    },
    List(Vec<Value>),
    /// Captured lambda / first-class function.
    Closure {
        params: Vec<String>,
        body: Block,
        captured: HashMap<String, Value>,
    },
    /// Internal: `?` propagation bubbling `None` / `Err` out of the current call.
    EarlyReturn(Box<Value>),
}

#[derive(Debug, Default)]
pub struct Console {
    pub writes: Vec<String>,
}

impl Console {
    pub fn print_line(&mut self, s: &str) {
        self.writes.push(s.to_string());
        println!("{s}");
    }
}

struct Env {
    values: HashMap<String, Value>,
}

impl Env {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn get(&self, name: &str) -> Result<Value, Trap> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| Trap(format!("unbound name {name:?}")))
    }

    fn insert(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}

pub struct Interpreter<'a> {
    fns: HashMap<String, &'a FnDecl>,
    console: Console,
    /// [P2D-ELIDE] Fn-level obligations proved in THIS process run; empty by
    /// default, so `new()` keeps today's check-everything semantics.
    proved: ProvedSet,
    /// [P2D-ELIDE] How many runtime checks were skipped under proof
    /// (instrumentation for tests / the `--prove-run` report line).
    pub elided_checks: usize,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Program) -> Self {
        let fns = program
            .functions
            .iter()
            .map(|f| (f.name.clone(), f))
            .collect();
        Self {
            fns,
            console: Console::default(),
            proved: ProvedSet::default(),
            elided_checks: 0,
        }
    }

    /// [P2D-ELIDE] Interpreter with proof-gated check elision (SPEC DP6 /
    /// INV-1): fn-level `ensures` / return-refine checks whose obligations are
    /// PROVED in `proved` are skipped; everything else still checks. Never
    /// speculative — an empty set (the `new()` default) elides nothing.
    pub fn with_proved(program: &'a Program, proved: ProvedSet) -> Self {
        let mut interp = Self::new(program);
        interp.proved = proved;
        interp
    }

    pub fn into_console(self) -> Console {
        self.console
    }

    pub fn run_main(&mut self) -> Result<Value, Trap> {
        let main = self
            .fns
            .get("main")
            .copied()
            .ok_or_else(|| Trap("no main".into()))?;
        let mut args = Vec::new();
        for p in &main.params {
            if p.name == "console" || matches!(p.ty, Type::Console) {
                args.push(Value::Console);
            } else {
                return Err(Trap(format!(
                    "main cannot bind parameter {:?} in Phase 1 runner",
                    p.name
                )));
            }
        }
        self.call_fn(main, args)
    }

    fn call_fn(&mut self, fn_decl: &FnDecl, args: Vec<Value>) -> Result<Value, Trap> {
        if args.len() != fn_decl.params.len() {
            return Err(Trap(format!("{}: arity mismatch", fn_decl.name)));
        }
        let mut env = Env::new();
        for (p, a) in fn_decl.params.iter().zip(args.into_iter()) {
            env.insert(p.name.clone(), a.clone());
            if let Type::Refine {
                name,
                pred: Some(pred),
            } = &p.ty
            {
                env.insert(name.clone(), a);
                self.check_refine_pred(name, pred, &env)?;
            }
        }
        for req in &fn_decl.requires {
            match self.eval_expr(req, &env)? {
                Value::Bool(true) => {}
                Value::Bool(false) => {
                    return Err(Trap(format!("{}: requires violated", fn_decl.name)));
                }
                _ => return Err(Trap(format!("{}: requires not Bool", fn_decl.name))),
            }
        }
        let result = self.eval_block(&fn_decl.body, &mut env)?;
        let result = match result {
            Value::EarlyReturn(v) => *v,
            other => other,
        };
        if let Type::Refine {
            name,
            pred: Some(pred),
        } = &fn_decl.ret
        {
            env.insert(name.clone(), result.clone());
            env.insert("result".into(), result.clone());
            // [P2D-ELIDE] proof-gated elision (SPEC DP6 / INV-1): skip only the
            // pred EVAL when this fn's return refine is PROVED this run; the
            // env inserts above are kept so observable behavior is unchanged.
            if self.proved.return_refine_proved(&fn_decl.name) {
                self.elided_checks += 1;
            } else {
                self.check_refine_pred(name, pred, &env)?;
            }
        }
        env.insert("result".into(), result.clone());
        for (i, ens) in fn_decl.ensures.iter().enumerate() {
            // [P2D-ELIDE] a PROVED ensures[i] is skipped; unproved / refuted
            // clauses of the same fn still check (or trap) exactly as today.
            if self.proved.ensures_proved(&fn_decl.name, i) {
                self.elided_checks += 1;
                continue;
            }
            match self.eval_expr(ens, &env)? {
                Value::Bool(true) => {}
                Value::Bool(false) => {
                    return Err(Trap(format!("{}: ensures violated", fn_decl.name)));
                }
                _ => return Err(Trap(format!("{}: ensures not Bool", fn_decl.name))),
            }
        }
        Ok(result)
    }

    fn check_refine_pred(&mut self, name: &str, pred: &Expr, env: &Env) -> Result<(), Trap> {
        match self.eval_expr(pred, env)? {
            Value::Bool(true) => Ok(()),
            Value::Bool(false) => Err(Trap(format!("refinement {{{name}: Int | …}} violated"))),
            _ => Err(Trap(format!("refinement {{{name}}} predicate not Bool"))),
        }
    }

    fn call_closure(
        &mut self,
        params: &[String],
        body: &Block,
        captured: &HashMap<String, Value>,
        args: Vec<Value>,
    ) -> Result<Value, Trap> {
        if args.len() != params.len() {
            return Err(Trap("closure arity mismatch".into()));
        }
        let mut env = Env {
            values: captured.clone(),
        };
        for (p, a) in params.iter().zip(args.into_iter()) {
            env.insert(p.clone(), a);
        }
        match self.eval_block(body, &mut env)? {
            Value::EarlyReturn(v) => Ok(*v),
            other => Ok(other),
        }
    }

    fn eval_block(&mut self, block: &Block, env: &mut Env) -> Result<Value, Trap> {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Let { name, value, .. } => {
                    let v = self.eval_expr(value, env)?;
                    if matches!(v, Value::EarlyReturn(_)) {
                        return Ok(v);
                    }
                    env.insert(name.clone(), v);
                }
                Stmt::Expr { expr, .. } => {
                    let v = self.eval_expr(expr, env)?;
                    if matches!(v, Value::EarlyReturn(_)) {
                        return Ok(v);
                    }
                }
            }
        }
        if let Some(res) = &block.result {
            self.eval_expr(res, env)
        } else {
            Ok(Value::Unit)
        }
    }

    fn eval_expr(&mut self, expr: &Expr, env: &Env) -> Result<Value, Trap> {
        match expr {
            Expr::LitInt { value, .. } => Ok(Value::Int(*value)),
            Expr::LitBool { value, .. } => Ok(Value::Bool(*value)),
            Expr::LitStr { value, .. } => Ok(Value::Str(value.clone())),
            Expr::LitUnit { .. } => Ok(Value::Unit),
            Expr::Name { name, .. } => env.get(name),
            Expr::Ctor {
                type_name,
                name,
                args,
                ..
            } => {
                let mut argv = Vec::new();
                for a in args {
                    argv.push(self.eval_expr(a, env)?);
                }
                if let Some(tn) = type_name {
                    Ok(Value::Enum {
                        type_name: tn.clone(),
                        variant: name.clone(),
                        fields: argv,
                    })
                } else {
                    eval_prelude_ctor(name, argv)
                }
            }
            Expr::StructLit { name, fields, .. } => {
                let mut map = HashMap::new();
                for (fname, fexpr) in fields {
                    map.insert(fname.clone(), self.eval_expr(fexpr, env)?);
                }
                Ok(Value::Struct {
                    name: name.clone(),
                    fields: map,
                })
            }
            Expr::ListLit { elems, .. } => {
                let mut out = Vec::new();
                for e in elems {
                    out.push(self.eval_expr(e, env)?);
                }
                Ok(Value::List(out))
            }
            Expr::Lambda { params, body, .. } => Ok(Value::Closure {
                params: params.iter().map(|(n, _)| n.clone()).collect(),
                body: body.clone(),
                captured: env.values.clone(),
            }),
            Expr::UnaryOp { op, expr, .. } => {
                let v = self.eval_expr(expr, env)?;
                match (op.as_str(), v) {
                    ("-", Value::Int(a)) => Ok(Value::Int(checked_neg(a)?)),
                    ("!", Value::Bool(b)) => Ok(Value::Bool(!b)),
                    _ => Err(Trap(format!("bad unary {op}"))),
                }
            }
            Expr::BinOp {
                op, left, right, ..
            } => {
                let l = self.eval_expr(left, env)?;
                if op == "&&" {
                    let Value::Bool(lb) = l else {
                        return Err(Trap("&& needs Bool".into()));
                    };
                    if !lb {
                        return Ok(Value::Bool(false));
                    }
                    return match self.eval_expr(right, env)? {
                        Value::Bool(rb) => Ok(Value::Bool(rb)),
                        _ => Err(Trap("&& needs Bool".into())),
                    };
                }
                if op == "||" {
                    let Value::Bool(lb) = l else {
                        return Err(Trap("|| needs Bool".into()));
                    };
                    if lb {
                        return Ok(Value::Bool(true));
                    }
                    return match self.eval_expr(right, env)? {
                        Value::Bool(rb) => Ok(Value::Bool(rb)),
                        _ => Err(Trap("|| needs Bool".into())),
                    };
                }
                let r = self.eval_expr(right, env)?;
                match (op.as_str(), l, r) {
                    ("++", Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + &b)),
                    ("++", Value::List(a), Value::List(b)) => {
                        let mut out = a;
                        out.extend(b);
                        Ok(Value::List(out))
                    }
                    ("+", Value::Int(a), Value::Int(b)) => Ok(Value::Int(checked_add(a, b)?)),
                    ("-", Value::Int(a), Value::Int(b)) => Ok(Value::Int(checked_sub(a, b)?)),
                    ("*", Value::Int(a), Value::Int(b)) => Ok(Value::Int(checked_mul(a, b)?)),
                    ("/", Value::Int(a), Value::Int(b)) => Ok(Value::Int(checked_div(a, b)?)),
                    ("%", Value::Int(a), Value::Int(b)) => Ok(Value::Int(checked_mod(a, b)?)),
                    ("==", a, b) => Ok(Value::Bool(values_eq(&a, &b))),
                    ("!=", a, b) => Ok(Value::Bool(!values_eq(&a, &b))),
                    ("<", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                    ("<=", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                    (">", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                    (">=", Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                    _ => Err(Trap(format!("bad op {op}"))),
                }
            }
            Expr::FieldAccess { obj, field, .. } => {
                let o = self.eval_expr(obj, env)?;
                match o {
                    Value::Struct { fields, .. } => fields
                        .get(field)
                        .cloned()
                        .ok_or_else(|| Trap(format!("missing field {field}"))),
                    Value::Console => Err(Trap(
                        "bare Console field — call .print(...)".into(),
                    )),
                    Value::List(_) | Value::Int(_) => Err(Trap(format!(
                        "bare method {field} — call .{field}(...)"
                    ))),
                    _ => Err(Trap("field access on non-struct".into())),
                }
            }
            Expr::Call { callee, args, .. } => {
                if let Expr::FieldAccess { obj, field, .. } = callee.as_ref() {
                    let o = self.eval_expr(obj, env)?;
                    if matches!(o, Value::Console) && field == "print" {
                        let s = match self.eval_expr(&args[0], env)? {
                            Value::Str(s) => s,
                            other => format!("{other:?}"),
                        };
                        self.console.print_line(&s);
                        return Ok(Value::Unit);
                    }
                    if field == "show" {
                        return match o {
                            Value::Int(n) if args.is_empty() => Ok(Value::Str(n.to_string())),
                            _ => Err(Trap("show() expects Int receiver".into())),
                        };
                    }
                    if let Value::List(items) = o {
                        return eval_list_method(field, items, args, self, env);
                    }
                }
                if let Expr::Name { name, .. } = callee.as_ref() {
                    if is_prelude_ctor(name) {
                        let mut argv = Vec::new();
                        for a in args {
                            argv.push(self.eval_expr(a, env)?);
                        }
                        return eval_prelude_ctor(name, argv);
                    }
                    if let Some(fn_decl) = self.fns.get(name).copied() {
                        let mut argv = Vec::new();
                        for a in args {
                            argv.push(self.eval_expr(a, env)?);
                        }
                        return self.call_fn(fn_decl, argv);
                    }
                    // [P2-REFINE2] `len(e)` measure form (refinement predicates,
                    // SPEC §4.4 REQ-REFINE-2): only when `len` is neither a user
                    // fn (checked above) nor a local binding — those still win
                    // via their existing paths. The old path here was an
                    // unconditional unbound-name trap, so no legal program
                    // changes behavior.
                    if name == "len" && args.len() == 1 && env.get("len").is_err() {
                        return match self.eval_expr(&args[0], env)? {
                            Value::List(items) => Ok(Value::Int(items.len() as i64)),
                            other => Err(Trap(format!(
                                "len(...) measure expects a List, got {other:?}"
                            ))),
                        };
                    }
                }
                // First-class call: evaluate callee to a Closure.
                match self.eval_expr(callee, env)? {
                    Value::Closure {
                        params,
                        body,
                        captured,
                    } => {
                        let mut argv = Vec::new();
                        for a in args {
                            argv.push(self.eval_expr(a, env)?);
                        }
                        self.call_closure(&params, &body, &captured, argv)
                    }
                    _ => Err(Trap("bad call".into())),
                }
            }
            Expr::IfExpr {
                cond,
                then_body,
                else_body,
                ..
            } => match self.eval_expr(cond, env)? {
                Value::Bool(true) => {
                    let mut e = Env {
                        values: env.values.clone(),
                    };
                    self.eval_block(then_body, &mut e)
                }
                Value::Bool(false) => {
                    let mut e = Env {
                        values: env.values.clone(),
                    };
                    self.eval_block(else_body, &mut e)
                }
                _ => Err(Trap("if condition not Bool".into())),
            },
            Expr::MatchExpr {
                scrutinee, arms, ..
            } => {
                let v = self.eval_expr(scrutinee, env)?;
                if matches!(v, Value::EarlyReturn(_)) {
                    return Ok(v);
                }
                for arm in arms {
                    if let Some(binds) = match_pattern(&arm.pattern, &v)? {
                        let mut e = Env {
                            values: env.values.clone(),
                        };
                        for (n, bv) in binds {
                            e.insert(n, bv);
                        }
                        return self.eval_expr(&arm.body, &e);
                    }
                }
                Err(Trap(
                    "match: no arm matched (non-exhaustive at runtime)".into(),
                ))
            }
            Expr::Hole { name, .. } => Err(Trap(format!("unfilled hole ?{name}"))),
            Expr::Propagate { expr, .. } => match self.eval_expr(expr, env)? {
                Value::OptionSome(v) => Ok(*v),
                Value::OptionNone => Ok(Value::EarlyReturn(Box::new(Value::OptionNone))),
                Value::ResultOk(v) => Ok(*v),
                Value::ResultErr(e) => Ok(Value::EarlyReturn(Box::new(Value::ResultErr(e)))),
                Value::EarlyReturn(v) => Ok(Value::EarlyReturn(v)),
                _ => Err(Trap("`?` on non-Option/Result".into())),
            },
            Expr::Block(b) => {
                let mut e = Env {
                    values: env.values.clone(),
                };
                self.eval_block(b, &mut e)
            }
        }
    }
}

fn eval_prelude_ctor(name: &str, args: Vec<Value>) -> Result<Value, Trap> {
    match name {
        "None" => {
            if !args.is_empty() {
                return Err(Trap("None takes no args".into()));
            }
            Ok(Value::OptionNone)
        }
        "Some" => {
            if args.len() != 1 {
                return Err(Trap("Some takes 1 arg".into()));
            }
            Ok(Value::OptionSome(Box::new(args.into_iter().next().unwrap())))
        }
        "Ok" => {
            if args.len() != 1 {
                return Err(Trap("Ok takes 1 arg".into()));
            }
            Ok(Value::ResultOk(Box::new(args.into_iter().next().unwrap())))
        }
        "Err" => {
            if args.len() != 1 {
                return Err(Trap("Err takes 1 arg".into()));
            }
            Ok(Value::ResultErr(Box::new(args.into_iter().next().unwrap())))
        }
        _ => Err(Trap(format!("unknown ctor {name}"))),
    }
}

fn match_pattern(pat: &Pattern, value: &Value) -> Result<Option<Vec<(String, Value)>>, Trap> {
    match pat {
        Pattern::Wildcard { .. } => Ok(Some(vec![])),
        Pattern::Bind { name, .. } => Ok(Some(vec![(name.clone(), value.clone())])),
        Pattern::LitInt { value: n, .. } => match value {
            Value::Int(v) if v == n => Ok(Some(vec![])),
            _ => Ok(None),
        },
        Pattern::LitBool { value: b, .. } => match value {
            Value::Bool(v) if v == b => Ok(Some(vec![])),
            _ => Ok(None),
        },
        Pattern::LitStr { value: s, .. } => match value {
            Value::Str(v) if v == s => Ok(Some(vec![])),
            _ => Ok(None),
        },
        Pattern::LitUnit { .. } => match value {
            Value::Unit => Ok(Some(vec![])),
            _ => Ok(None),
        },
        Pattern::Ctor {
            type_name,
            name,
            args,
            ..
        } => {
            if let Some(tn) = type_name {
                match value {
                    Value::Enum {
                        type_name: etn,
                        variant,
                        fields,
                    } if etn == tn && variant == name => {
                        if args.len() != fields.len() {
                            return Err(Trap("enum pattern arity".into()));
                        }
                        let mut binds = Vec::new();
                        for (ap, fv) in args.iter().zip(fields.iter()) {
                            match match_pattern(ap, fv)? {
                                Some(b) => binds.extend(b),
                                None => return Ok(None),
                            }
                        }
                        Ok(Some(binds))
                    }
                    _ => Ok(None),
                }
            } else {
                match (name.as_str(), value) {
                    ("None", Value::OptionNone) => {
                        if args.is_empty() {
                            Ok(Some(vec![]))
                        } else {
                            Err(Trap("None pattern arity".into()))
                        }
                    }
                    ("Some", Value::OptionSome(inner)) => {
                        if args.len() != 1 {
                            return Err(Trap("Some pattern arity".into()));
                        }
                        match_pattern(&args[0], inner)
                    }
                    ("Ok", Value::ResultOk(inner)) => {
                        if args.len() != 1 {
                            return Err(Trap("Ok pattern arity".into()));
                        }
                        match_pattern(&args[0], inner)
                    }
                    ("Err", Value::ResultErr(inner)) => {
                        if args.len() != 1 {
                            return Err(Trap("Err pattern arity".into()));
                        }
                        match_pattern(&args[0], inner)
                    }
                    _ => Ok(None),
                }
            }
        }
    }
}

fn eval_list_method(
    field: &str,
    items: Vec<Value>,
    args: &[Expr],
    interp: &mut Interpreter<'_>,
    env: &Env,
) -> Result<Value, Trap> {
    match field {
        "len" => {
            if !args.is_empty() {
                return Err(Trap("len takes 0 args".into()));
            }
            Ok(Value::Int(items.len() as i64))
        }
        "get" => {
            if args.len() != 1 {
                return Err(Trap("get takes 1 arg".into()));
            }
            let idx = match interp.eval_expr(&args[0], env)? {
                Value::Int(i) => i,
                _ => return Err(Trap("get index must be Int".into())),
            };
            if idx < 0 {
                return Ok(Value::OptionNone);
            }
            let u = idx as usize;
            match items.get(u) {
                Some(v) => Ok(Value::OptionSome(Box::new(v.clone()))),
                None => Ok(Value::OptionNone),
            }
        }
        "head" => {
            if !args.is_empty() {
                return Err(Trap("head takes 0 args".into()));
            }
            match items.first() {
                Some(v) => Ok(Value::OptionSome(Box::new(v.clone()))),
                None => Ok(Value::OptionNone),
            }
        }
        "tail" => {
            if !args.is_empty() {
                return Err(Trap("tail takes 0 args".into()));
            }
            if items.is_empty() {
                Ok(Value::OptionNone)
            } else {
                Ok(Value::OptionSome(Box::new(Value::List(
                    items[1..].to_vec(),
                ))))
            }
        }
        "append" => {
            if args.len() != 1 {
                return Err(Trap("append takes 1 arg".into()));
            }
            let mut out = items;
            out.push(interp.eval_expr(&args[0], env)?);
            Ok(Value::List(out))
        }
        "map" => {
            if args.len() != 1 {
                return Err(Trap("map takes 1 function".into()));
            }
            let f = interp.eval_expr(&args[0], env)?;
            let mut out = Vec::new();
            for item in items {
                out.push(apply_unary(interp, &f, item)?);
            }
            Ok(Value::List(out))
        }
        "filter" => {
            if args.len() != 1 {
                return Err(Trap("filter takes 1 predicate".into()));
            }
            let f = interp.eval_expr(&args[0], env)?;
            let mut out = Vec::new();
            for item in items {
                match apply_unary(interp, &f, item.clone())? {
                    Value::Bool(true) => out.push(item),
                    Value::Bool(false) => {}
                    _ => return Err(Trap("filter predicate must return Bool".into())),
                }
            }
            Ok(Value::List(out))
        }
        "fold" => {
            if args.len() != 2 {
                return Err(Trap("fold takes init and function".into()));
            }
            let mut acc = interp.eval_expr(&args[0], env)?;
            let f = interp.eval_expr(&args[1], env)?;
            for item in items {
                acc = apply_binary(interp, &f, acc, item)?;
            }
            Ok(acc)
        }
        _ => Err(Trap(format!("unknown list method {field}"))),
    }
}

fn apply_unary(interp: &mut Interpreter<'_>, f: &Value, arg: Value) -> Result<Value, Trap> {
    match f {
        Value::Closure {
            params,
            body,
            captured,
        } => {
            if params.len() != 1 {
                return Err(Trap("unary apply expects 1-param closure".into()));
            }
            interp.call_closure(params, body, captured, vec![arg])
        }
        _ => Err(Trap("apply expected closure".into())),
    }
}

fn apply_binary(
    interp: &mut Interpreter<'_>,
    f: &Value,
    a: Value,
    b: Value,
) -> Result<Value, Trap> {
    match f {
        Value::Closure {
            params,
            body,
            captured,
        } => {
            if params.len() != 2 {
                return Err(Trap("binary apply expects 2-param closure".into()));
            }
            interp.call_closure(params, body, captured, vec![a, b])
        }
        _ => Err(Trap("apply expected closure".into())),
    }
}

fn values_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Unit, Value::Unit) => true,
        (Value::Console, Value::Console) => true,
        (Value::OptionNone, Value::OptionNone) => true,
        (Value::OptionSome(x), Value::OptionSome(y)) => values_eq(x, y),
        (Value::ResultOk(x), Value::ResultOk(y)) => values_eq(x, y),
        (Value::ResultErr(x), Value::ResultErr(y)) => values_eq(x, y),
        (
            Value::Struct {
                name: n1,
                fields: f1,
            },
            Value::Struct {
                name: n2,
                fields: f2,
            },
        ) => {
            n1 == n2
                && f1.len() == f2.len()
                && f1.iter().all(|(k, v)| f2.get(k).is_some_and(|w| values_eq(v, w)))
        }
        (
            Value::Enum {
                type_name: t1,
                variant: v1,
                fields: f1,
            },
            Value::Enum {
                type_name: t2,
                variant: v2,
                fields: f2,
            },
        ) => {
            t1 == t2
                && v1 == v2
                && f1.len() == f2.len()
                && f1.iter().zip(f2.iter()).all(|(a, b)| values_eq(a, b))
        }
        (Value::List(a), Value::List(b)) => {
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_eq(x, y))
        }
        // Closures: identity only (no structural eq).
        (Value::Closure { .. }, Value::Closure { .. }) => false,
        _ => false,
    }
}

fn checked_add(a: i64, b: i64) -> Result<i64, Trap> {
    a.checked_add(b).ok_or_else(|| Trap("Int overflow".into()))
}
fn checked_sub(a: i64, b: i64) -> Result<i64, Trap> {
    a.checked_sub(b).ok_or_else(|| Trap("Int overflow".into()))
}
fn checked_mul(a: i64, b: i64) -> Result<i64, Trap> {
    a.checked_mul(b).ok_or_else(|| Trap("Int overflow".into()))
}
fn checked_div(a: i64, b: i64) -> Result<i64, Trap> {
    if b == 0 {
        return Err(Trap("division by zero".into()));
    }
    a.checked_div(b).ok_or_else(|| Trap("Int overflow".into()))
}
fn checked_mod(a: i64, b: i64) -> Result<i64, Trap> {
    if b == 0 {
        return Err(Trap("division by zero".into()));
    }
    Ok(a % b)
}
fn checked_neg(a: i64) -> Result<i64, Trap> {
    a.checked_neg().ok_or_else(|| Trap("Int overflow".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::typecheck::check_program;

    fn run_checked(src: &str) -> Result<Console, Trap> {
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("typecheck");
        let mut interp = Interpreter::new(&prog);
        interp.run_main()?;
        Ok(interp.into_console())
    }

    #[test]
    fn len_measure_pred_runs_on_valid_call() {
        // [P2-REFINE2] runtime leg of REQ-REFINE-2: a valid in-range call
        // evaluates `len(xs)` in the param refinement and runs normally.
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
        let console = run_checked(src).expect("valid call must run");
        assert_eq!(console.writes, vec!["20".to_string()]);
    }

    #[test]
    fn len_measure_pred_traps_on_unbounded_oob_index() {
        // [P2-REFINE2] an index the checker could not bound stays soft at
        // compile time and is caught by the runtime refinement check.
        let src = r#"
fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    match xs.get(i) {
        Some(v) => v,
        None => -1,
    }
}
fn main(console: Console) -> Unit uses {console} {
    let j: Int = 9;
    console.print(nth([10, 20, 30], j).show());
}
"#;
        let err = run_checked(src).expect_err("OOB runtime index must trap");
        assert!(err.0.contains("refinement"), "{err}");
    }

    #[test]
    fn len_measure_does_not_shadow_local_len_binding() {
        // [P2-REFINE2] guard: a local `len` closure still wins over the measure.
        let src = r#"
fn main(console: Console) -> Unit uses {console} {
    let len = fn (n: Int) -> Int { n + 100 };
    console.print(len(1).show());
}
"#;
        let console = run_checked(src).expect("local len closure must run");
        assert_eq!(console.writes, vec!["101".to_string()]);
    }

    // ---- [P2D-ELIDE] proof-gated check elision (SPEC DP6 / INV-1) ----

    fn prove_and_build(src: &str) -> (crate::ast::Program, ProvedSet) {
        let prog = parse(src).expect("parse");
        check_program(&prog).expect("typecheck");
        let obs = crate::vc::prove_program(&prog).expect("prove");
        let proved = ProvedSet::build(&prog, &obs);
        (prog, proved)
    }

    #[test]
    fn elide_skips_proved_fn_level_checks() {
        // CONF-P2: a contract SMT-proved end-to-end has its runtime check
        // elided; the default interpreter (no proved-set) still checks all.
        let src = r#"
fn clamp(x: Int, lo: Int, hi: Int) -> {r: Int | r >= lo && r <= hi}
    requires lo <= hi
    ensures result >= lo
    ensures result <= hi
{
    if x < lo { lo } else { if x > hi { hi } else { x } }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(clamp(5, 0, 10).show());
}
"#;
        let (prog, proved) = prove_and_build(src);
        assert!(proved.return_refine_proved("clamp"), "clamp return refine must be proved");
        assert!(proved.ensures_proved("clamp", 0) && proved.ensures_proved("clamp", 1));
        let mut interp = Interpreter::with_proved(&prog, proved);
        interp.run_main().expect("elided run must succeed");
        assert_eq!(interp.console.writes, vec!["5".to_string()]);
        // 1 call x (return_refine + ensures[0] + ensures[1]) = 3 skips.
        assert_eq!(interp.elided_checks, 3, "expected 3 elided checks");
        // INV-1 gate closed by default: plain `new` elides nothing.
        let mut plain = Interpreter::new(&prog);
        plain.run_main().expect("plain run must succeed");
        assert_eq!(plain.elided_checks, 0);
    }

    #[test]
    fn elide_never_skips_unproved_ensures() {
        // INV-1 negative: `/` is outside the SMT fragment [P2-SOUND1], so the
        // ensures stays RuntimeChecked -> must still trap even with a
        // ProvedSet armed from the same prove run.
        let src = r#"
fn half(x: Int) -> Int
    ensures result >= 100
{
    x / 2
}
fn main(console: Console) -> Unit uses {console} {
    console.print(half(4).show());
}
"#;
        let (prog, proved) = prove_and_build(src);
        assert!(!proved.ensures_proved("half", 0), "unencodable ensures must not be proved");
        let mut interp = Interpreter::with_proved(&prog, proved);
        let err = interp.run_main().expect_err("violated unproved ensures must trap");
        assert!(err.0.contains("ensures violated"), "{err}");
        assert_eq!(interp.elided_checks, 0);
    }

    #[test]
    fn elide_never_skips_refuted_ensures() {
        // A REFUTED obligation is never in the proved set -> still checked.
        let src = r#"
fn bump(x: Int) -> Int
    ensures result >= 999
{
    x + 1
}
fn main(console: Console) -> Unit uses {console} {
    console.print(bump(1).show());
}
"#;
        let (prog, proved) = prove_and_build(src);
        assert!(!proved.ensures_proved("bump", 0), "refuted ensures must not be proved");
        let mut interp = Interpreter::with_proved(&prog, proved);
        let err = interp.run_main().expect_err("refuted ensures must still trap at runtime");
        assert!(err.0.contains("ensures violated"), "{err}");
        assert_eq!(interp.elided_checks, 0);
    }

    #[test]
    fn elide_excludes_duplicate_fn_names() {
        // The interpreter resolves calls by name (last decl wins), so a proof
        // for one duplicate must never elide checks on the other.
        let src = r#"
fn f() -> {r: Int | r >= 0} {
    1
}
fn f() -> {r: Int | r >= 0} {
    2
}
fn main(console: Console) -> Unit uses {console} {
    console.print(f().show());
}
"#;
        let (prog, proved) = prove_and_build(src);
        assert!(
            !proved.return_refine_proved("f"),
            "duplicated fn name must be excluded from the proved set"
        );
        let mut interp = Interpreter::with_proved(&prog, proved);
        interp.run_main().expect("run");
        assert_eq!(interp.console.writes, vec!["2".to_string()], "last decl wins");
        assert_eq!(interp.elided_checks, 0);
    }
}
