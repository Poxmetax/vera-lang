//! Canonical surface renderer (Phase 1 / CONF-P1).
//! `parse → render → parse` must preserve content hashes (spans excluded).

use crate::ast::*;

pub fn render_program(program: &Program) -> String {
    let mut out = String::new();
    for s in &program.structs {
        out.push_str(&render_struct(s));
        out.push('\n');
    }
    for e in &program.enums {
        out.push_str(&render_enum(e));
        out.push('\n');
    }
    for f in &program.functions {
        out.push_str(&render_fn(f));
        out.push('\n');
    }
    out
}

fn render_struct(s: &StructDecl) -> String {
    let fields: Vec<String> = s
        .fields
        .iter()
        .map(|f| format!("{}: {}", f.name, render_type(&f.ty)))
        .collect();
    format!("struct {} {{\n    {}\n}}\n", s.name, fields.join(",\n    "))
}

fn render_enum(e: &EnumDecl) -> String {
    let vars: Vec<String> = e
        .variants
        .iter()
        .map(|v| {
            if v.fields.is_empty() {
                v.name.clone()
            } else {
                let ts: Vec<String> = v.fields.iter().map(render_type).collect();
                format!("{}({})", v.name, ts.join(", "))
            }
        })
        .collect();
    format!("enum {} {{\n    {}\n}}\n", e.name, vars.join(",\n    "))
}

fn render_fn(f: &FnDecl) -> String {
    let mut s = String::new();
    s.push_str("fn ");
    s.push_str(&f.name);
    s.push('(');
    let params: Vec<String> = f
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name, render_type(&p.ty)))
        .collect();
    s.push_str(&params.join(", "));
    s.push_str(") -> ");
    s.push_str(&render_type(&f.ret));
    s.push('\n');
    if !f.uses.is_empty() {
        s.push_str("    uses {");
        s.push_str(&f.uses.join(", "));
        s.push_str("}\n");
    }
    for r in &f.requires {
        s.push_str("    requires ");
        s.push_str(&render_expr(r));
        s.push('\n');
    }
    for e in &f.ensures {
        s.push_str("    ensures ");
        s.push_str(&render_expr(e));
        s.push('\n');
    }
    s.push_str(&render_block(&f.body, 0));
    s.push('\n');
    s
}

pub fn render_type(t: &Type) -> String {
    match t {
        Type::Int => "Int".into(),
        Type::Bool => "Bool".into(),
        Type::Str => "Str".into(),
        Type::Unit => "Unit".into(),
        Type::Console => "Console".into(),
        Type::Named { name } => name.clone(),
        Type::List { elem } => format!("List<{}>", render_type(elem)),
        Type::Option { inner } => format!("Option<{}>", render_type(inner)),
        Type::Result { ok, err } => format!("Result<{}, {}>", render_type(ok), render_type(err)),
        Type::Fn { params, ret } => {
            let ps: Vec<String> = params.iter().map(render_type).collect();
            format!("fn({}) -> {}", ps.join(", "), render_type(ret))
        }
        Type::Refine { name, pred } => {
            let p = pred
                .as_ref()
                .map(|e| render_expr(e))
                .unwrap_or_else(|| "true".into());
            format!("{{{name}: Int | {p}}}")
        }
    }
}

fn indent(n: usize) -> String {
    "    ".repeat(n)
}

fn render_block(b: &Block, depth: usize) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    for stmt in &b.stmts {
        s.push_str(&indent(depth + 1));
        s.push_str(&render_stmt(stmt, depth + 1));
        s.push('\n');
    }
    if let Some(res) = &b.result {
        s.push_str(&indent(depth + 1));
        s.push_str(&render_expr(res));
        s.push('\n');
    }
    s.push_str(&indent(depth));
    s.push('}');
    s
}

fn render_stmt(stmt: &Stmt, depth: usize) -> String {
    match stmt {
        Stmt::Let { name, ty, value, .. } => {
            let mut s = format!("let {name}");
            if let Some(t) = ty {
                s.push_str(": ");
                s.push_str(&render_type(t));
            }
            s.push_str(" = ");
            s.push_str(&render_expr_prec(value, 0, depth));
            s.push(';');
            s
        }
        Stmt::Expr { expr, .. } => {
            let mut s = render_expr_prec(expr, 0, depth);
            s.push(';');
            s
        }
    }
}

pub fn render_expr(e: &Expr) -> String {
    render_expr_prec(e, 0, 0)
}

fn render_expr_prec(e: &Expr, _prec: i32, depth: usize) -> String {
    match e {
        Expr::LitInt { value, .. } => value.to_string(),
        Expr::LitBool { value, .. } => if *value { "true" } else { "false" }.into(),
        Expr::LitStr { value, .. } => format!("{:?}", value), // Rust Debug = quoted escapes
        Expr::LitUnit { .. } => "unit".into(),
        Expr::Name { name, .. } => name.clone(),
        Expr::Ctor {
            type_name,
            name,
            args,
            ..
        } => {
            let head = match type_name {
                Some(t) => format!("{t}::{name}"),
                None => name.clone(),
            };
            if args.is_empty() && name == "None" {
                head
            } else if args.is_empty() {
                head
            } else {
                let a: Vec<String> = args.iter().map(|x| render_expr_prec(x, 0, depth)).collect();
                format!("{head}({})", a.join(", "))
            }
        }
        Expr::StructLit { name, fields, .. } => {
            let fs: Vec<String> = fields
                .iter()
                .map(|(n, v)| format!("{n}: {}", render_expr_prec(v, 0, depth)))
                .collect();
            format!("{name}({})", fs.join(", "))
        }
        Expr::ListLit { elems, .. } => {
            let es: Vec<String> = elems
                .iter()
                .map(|x| render_expr_prec(x, 0, depth))
                .collect();
            format!("[{}]", es.join(", "))
        }
        Expr::Lambda {
            params, ret, body, ..
        } => {
            let ps: Vec<String> = params
                .iter()
                .map(|(n, t)| match t {
                    Some(ty) => format!("{n}: {}", render_type(ty)),
                    None => n.clone(),
                })
                .collect();
            let mut s = format!("fn ({})", ps.join(", "));
            if let Some(r) = ret {
                s.push_str(" -> ");
                s.push_str(&render_type(r));
            }
            s.push(' ');
            s.push_str(&render_block(body, depth));
            s
        }
        Expr::BinOp {
            op, left, right, ..
        } => format!(
            "{} {} {}",
            render_expr_prec(left, 0, depth),
            op,
            render_expr_prec(right, 0, depth)
        ),
        Expr::UnaryOp { op, expr, .. } => {
            format!("{op}{}", render_expr_prec(expr, 0, depth))
        }
        Expr::Call { callee, args, .. } => {
            let a: Vec<String> = args
                .iter()
                .map(|x| render_expr_prec(x, 0, depth))
                .collect();
            format!("{}({})", render_expr_prec(callee, 0, depth), a.join(", "))
        }
        Expr::FieldAccess { obj, field, .. } => {
            format!("{}.{}", render_expr_prec(obj, 0, depth), field)
        }
        Expr::IfExpr {
            cond,
            then_body,
            else_body,
            ..
        } => {
            // else-if sugar: else { if ... } rendered as else if when possible
            if then_body.stmts.is_empty()
                && else_body.stmts.is_empty()
                && else_body.result.as_ref().is_some_and(|e| matches!(e.as_ref(), Expr::IfExpr { .. }))
            {
                // keep simple form
            }
            format!(
                "if {} {} else {}",
                render_expr_prec(cond, 0, depth),
                render_block(then_body, depth),
                render_block(else_body, depth)
            )
        }
        Expr::MatchExpr {
            scrutinee, arms, ..
        } => {
            let mut s = format!("match {} {{\n", render_expr_prec(scrutinee, 0, depth));
            for (i, arm) in arms.iter().enumerate() {
                s.push_str(&indent(depth + 1));
                s.push_str(&render_pattern(&arm.pattern));
                s.push_str(" => ");
                s.push_str(&render_expr_prec(&arm.body, 0, depth + 1));
                if i + 1 < arms.len() {
                    s.push(',');
                }
                s.push('\n');
            }
            s.push_str(&indent(depth));
            s.push('}');
            s
        }
        Expr::Block(b) => render_block(b, depth),
        Expr::Hole { name, .. } => format!("?{name}"),
        Expr::Propagate { expr, .. } => {
            format!("{}?", render_expr_prec(expr, 0, depth))
        },
    }
}

fn render_pattern(p: &Pattern) -> String {
    match p {
        Pattern::Wildcard { .. } => "_".into(),
        Pattern::LitInt { value, .. } => value.to_string(),
        Pattern::LitBool { value, .. } => if *value { "true" } else { "false" }.into(),
        Pattern::LitStr { value, .. } => format!("{:?}", value),
        Pattern::LitUnit { .. } => "unit".into(),
        Pattern::Bind { name, .. } => name.clone(),
        Pattern::Ctor {
            type_name,
            name,
            args,
            ..
        } => {
            let head = match type_name {
                Some(t) => format!("{t}::{name}"),
                None => name.clone(),
            };
            if args.is_empty() {
                head
            } else {
                let a: Vec<String> = args.iter().map(render_pattern).collect();
                format!("{head}({})", a.join(", "))
            }
        }
    }
}
