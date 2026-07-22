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
        .map(|p| {
            format!(
                "{}: {}{}",
                p.name,
                render_type(&p.ty),
                render_label(&p.label)
            )
        })
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

/// [GAP4-VALUE-LABEL] `^{a, b}` postfix for a non-empty binding label
/// (atoms are parser-canonical, so this render reparses AST-identically).
fn render_label(label: &[String]) -> String {
    if label.is_empty() {
        String::new()
    } else {
        format!("^{{{}}}", label.join(", "))
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
        Stmt::Let {
            name,
            ty,
            value,
            label,
            ..
        } => {
            let mut s = format!("let {name}");
            if let Some(t) = ty {
                s.push_str(": ");
                s.push_str(&render_type(t));
                // [GAP4-VALUE-LABEL] label rides the explicit annotation only.
                s.push_str(&render_label(label));
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

/// [GAP3-RENDER-PAREN] Binding strength of an expression form as an operand.
/// Mirrors the parser's ladder (SPEC §3.1 / parser.rs): if/match/lambda (0)
/// < `||` (1) < `&&` (2) < comparisons (3, NON-associative: one optional
/// rel_op) < `+ - ++` (4) < `* / %` (5) < unary (6) < postfix (7) < atoms (8).
fn prec_of(e: &Expr) -> i32 {
    match e {
        Expr::IfExpr { .. } | Expr::MatchExpr { .. } | Expr::Lambda { .. } => 0,
        Expr::BinOp { op, .. } => bin_prec(op),
        Expr::UnaryOp { .. } => 6,
        Expr::Call { .. } | Expr::FieldAccess { .. } | Expr::Propagate { .. } => 7,
        _ => 8,
    }
}

fn bin_prec(op: &str) -> i32 {
    match op {
        "||" => 1,
        "&&" => 2,
        "==" | "!=" | "<" | "<=" | ">" | ">=" => 3,
        "+" | "-" | "++" => 4,
        _ => 5, // * / %
    }
}

/// [GAP3-RENDER-PAREN] `min_prec` is the weakest binding allowed unwrapped in
/// this position; a child that binds weaker is parenthesized so the rendered
/// text re-parses to the identical AST (CONF-P1 round-trip, PHASE12 F5).
fn render_expr_prec(e: &Expr, min_prec: i32, depth: usize) -> String {
    let s = render_expr_raw(e, depth);
    if prec_of(e) < min_prec {
        format!("({s})")
    } else {
        s
    }
}

/// [F6-STR-ESCAPE] Render a string literal using ONLY escapes the lexer decodes
/// (`\n \t \r \" \\`), emitting every other character raw. This is the exact
/// inverse of the lexer's escape table (`lexer.rs`), so `parse -> render ->
/// parse` is identity for any string the lexer can represent. Matching on
/// individual chars makes escaping fusion-safe: a literal backslash becomes
/// `\\` and a following `r` stays raw, never fusing into a spurious `\r`.
fn escape_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn render_expr_raw(e: &Expr, depth: usize) -> String {
    match e {
        Expr::LitInt { value, .. } => value.to_string(),
        Expr::LitBool { value, .. } => if *value { "true" } else { "false" }.into(),
        Expr::LitStr { value, .. } => escape_str(value), // [F6-STR-ESCAPE] lexer-parity escapes
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
            if args.is_empty() {
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
        } => {
            // [GAP3-RENDER-PAREN] left-assoc: equal-prec LEFT child stays bare,
            // equal-prec RIGHT child needs parens (`a - (b - c)`); comparisons
            // are non-associative, so BOTH cmp children need parens.
            let p = bin_prec(op);
            let lmin = if p == 3 { p + 1 } else { p };
            format!(
                "{} {} {}",
                render_expr_prec(left, lmin, depth),
                op,
                render_expr_prec(right, p + 1, depth)
            )
        }
        Expr::UnaryOp { op, expr, .. } => {
            // [GAP3-RENDER-PAREN] operand must bind at postfix strength:
            // `-(a + b)`, `-(-x)` (the grammar's unary prefix is single).
            format!("{op}{}", render_expr_prec(expr, 7, depth))
        }
        Expr::Call { callee, args, .. } => {
            let a: Vec<String> = args.iter().map(|x| render_expr_prec(x, 0, depth)).collect();
            format!("{}({})", render_expr_prec(callee, 7, depth), a.join(", "))
        }
        Expr::FieldAccess { obj, field, .. } => {
            format!("{}.{}", render_expr_prec(obj, 7, depth), field)
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
                && else_body
                    .result
                    .as_ref()
                    .is_some_and(|e| matches!(e.as_ref(), Expr::IfExpr { .. }))
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
            format!("{}?", render_expr_prec(expr, 7, depth))
        }
    }
}

fn render_pattern(p: &Pattern) -> String {
    match p {
        Pattern::Wildcard { .. } => "_".into(),
        Pattern::LitInt { value, .. } => value.to_string(),
        Pattern::LitBool { value, .. } => if *value { "true" } else { "false" }.into(),
        Pattern::LitStr { value, .. } => escape_str(value),
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

#[cfg(test)]
mod tests {
    use crate::store::CodebaseStore;

    // [GAP3-RENDER-PAREN] shapes that FAILED round-trip before this slice
    // (PHASE12 F5 probe class) and must survive render -> parse -> hash now.
    #[test]
    fn gap3_mixed_precedence_shapes_round_trip() {
        let src = r#"
fn main(console: Console) -> Unit uses {console} {
    let _a: Int = (1 + 2) * 3;
    let _b: Int = 1 - (2 - 3);
    let _c: Bool = (true || false) && true;
    let _d: Bool = (1 < 2) == true;
    let _e: Int = -(1 + 2);
    let _f: Str = (1 + 2).show();
    console.print(_f);
}
"#;
        CodebaseStore::round_trip_ok(src).expect("mixed-precedence round trip");
    }

    // [F6-STR-ESCAPE] strings with CR/LF/tab/backslash/quote must survive
    // parse -> render -> parse identically (previously `{:?}` emitted `\r`,
    // which the lexer rejected as an unknown escape).
    #[test]
    fn f6_string_escapes_round_trip() {
        let src = r#"
fn main(console: Console) -> Unit uses {console} {
    let _a: Str = "tab\there";
    let _b: Str = "crlf\r\nend";
    let _c: Str = "back\\slash";
    let _d: Str = "quote\"inside";
    let _e: Str = "back\\r stays two chars";
    console.print(_a);
}
"#;
        CodebaseStore::round_trip_ok(src).expect("F6 string escapes round trip");
    }

    #[test]
    fn f6_render_emits_only_lexer_accepted_escapes() {
        // Pin the escape table: render must emit `\t \r \n` (not `\u{..}` or a
        // raw control char), and a literal backslash-r must render as `\\r`.
        let src = r#"
fn main(console: Console) -> Unit uses {console} {
    let _a: Str = "a\tb\r\nc";
    let _b: Str = "back\\r";
    console.print(_a);
}
"#;
        let prog = crate::parse(src).expect("parse");
        let out = crate::render_program(&prog);
        assert!(out.contains(r#""a\tb\r\nc""#), "escape-table drift: {out}");
        assert!(out.contains(r#""back\\r""#), "backslash-r fused: {out}");
    }

    #[test]
    fn f6_string_escape_edge_cases_round_trip() {
        // Empty, raw non-ASCII, and a value ending in a backslash all round-trip.
        // (The five individual escapes are covered by f6_string_escapes_round_trip.)
        let src = "\
fn main(console: Console) -> Unit uses {console} {\n\
    let _empty: Str = \"\";\n\
    let _uni: Str = \"cafe \u{2603} ok\";\n\
    let _trail: Str = \"ends with a backslash \\\\\";\n\
    console.print(_trail);\n\
}\n";
        CodebaseStore::round_trip_ok(src).expect("F6 edge cases round trip");
    }

    #[test]
    fn gap3_no_redundant_parens_on_natural_precedence() {
        // Hash identity alone cannot catch over-parenthesization (redundant
        // parens re-parse to the same AST), so pin the rendered text too.
        let src = r#"
fn main(console: Console) -> Unit uses {console} {
    let _a: Int = 1 + 2 * 3;
    let _b: Int = (1 + 2) * 3;
    console.print("k");
}
"#;
        let prog = crate::parse(src).expect("parse");
        let out = crate::render_program(&prog);
        assert!(
            out.contains("let _a: Int = 1 + 2 * 3;"),
            "over-parenthesized: {out}"
        );
        assert!(
            out.contains("let _b: Int = (1 + 2) * 3;"),
            "lost parens: {out}"
        );
    }
}
