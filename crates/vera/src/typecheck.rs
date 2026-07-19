//! Lightweight Phase 1 type checker (annotated MVP; HM deferred).

use crate::ast::*;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct TypeError(pub String);

impl TypeError {
    fn at(span: Span, msg: impl Into<String>) -> Self {
        TypeError(format!("{}: {}", span, msg.into()))
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
}

impl<'a> Env<'a> {
    fn extend(&self, name: String, ty: Type) -> Env<'a> {
        let mut vars = self.vars.clone();
        vars.insert(name, ty);
        Env {
            vars,
            fns: self.fns,
            adt: self.adt,
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

    let fns: HashMap<String, FnDecl> = program
        .functions
        .iter()
        .map(|f| (f.name.clone(), f.clone()))
        .collect();
    if !fns.contains_key("main") {
        return Err(TypeError("program must define fn main".into()));
    }
    for fn_decl in &program.functions {
        check_fn(fn_decl, &fns, &adt)?;
    }
    Ok(())
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
        vars.insert(p.name.clone(), p.ty.clone());
    }
    let env = Env { vars, fns, adt };
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
                    },
                )?;
                if let Some(annot) = ty {
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
                    },
                )?;
            }
        }
    }
    let env2 = Env {
        vars: e_vars,
        fns: env.fns,
        adt: env.adt,
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
            match t {
                Type::Option { inner } => Ok(*inner),
                Type::Result { ok, .. } => Ok(*ok),
                other => Err(TypeError::at(
                    *span,
                    format!("`?` propagation requires Option or Result, got {}", other.to_str()),
                )),
            }
        }
        Expr::Block(b) => check_block(b, env),
    }
}

fn erase_refine(t: &Type) -> Type {
    match t {
        Type::Refine { .. } => Type::Int,
        other => other.clone(),
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
    }
    let body_ty = check_block(
        body,
        &Env {
            vars: e,
            fns: env.fns,
            adt: env.adt,
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
        };
        arm_tys.push(infer_expr(&arm.body, &arm_env)?);
    }

    if !has_wildcard {
        match &st {
            Type::Option { .. } => {
                if !(covered.contains("Some") && covered.contains("None")) {
                    return Err(TypeError::at(
                        span,
                        "non-exhaustive match on Option (need Some and None, or _)",
                    ));
                }
            }
            Type::Result { .. } => {
                if !(covered.contains("Ok") && covered.contains("Err")) {
                    return Err(TypeError::at(
                        span,
                        "non-exhaustive match on Result (need Ok and Err, or _)",
                    ));
                }
            }
            Type::Named { name } => {
                if let Some(ed) = env.adt.enums.get(name) {
                    for v in &ed.variants {
                        let key = format!("{}::{}", name, v.name);
                        if !covered.contains(&key) && !covered.contains(&v.name) {
                            return Err(TypeError::at(
                                span,
                                format!("non-exhaustive match on {name}: missing {}", v.name),
                            ));
                        }
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
