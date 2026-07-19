//! Tree-walking interpreter with Console capability + runtime contracts (Phase 1).

use crate::ast::*;
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
        }
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
            env.insert(p.name.clone(), a);
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
        env.insert("result".into(), result.clone());
        for ens in &fn_decl.ensures {
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

    fn eval_block(&mut self, block: &Block, env: &mut Env) -> Result<Value, Trap> {
        for stmt in &block.stmts {
            match stmt {
                Stmt::Let { name, value, .. } => {
                    let v = self.eval_expr(value, env)?;
                    env.insert(name.clone(), v);
                }
                Stmt::Expr { expr, .. } => {
                    self.eval_expr(expr, env)?;
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
                }
                Err(Trap("bad call".into()))
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
