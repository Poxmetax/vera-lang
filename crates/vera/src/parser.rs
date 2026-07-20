//! VERA MVP recursive-descent parser (Phase 1).

use crate::ast::*;
use crate::lexer::{lex, LexError, TokKind, Token};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{span}: {message}")]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        ParseError {
            message: e.message,
            span: e.span,
        }
    }
}

/// [R2-DEPTH] Ceiling on nested recursive descent (expr / type / unary),
/// chosen well below the native-stack overflow threshold yet far above any
/// realistic program's nesting.
const MAX_PARSE_DEPTH: usize = 256;

struct Parser {
    tokens: Vec<Token>,
    i: usize,
    /// [R2-DEPTH] current recursion depth, guarded by `enter_depth`.
    depth: usize,
}

impl Parser {
    fn cur(&self) -> &Token {
        &self.tokens[self.i]
    }

    fn at(&self, texts: &[&str]) -> bool {
        texts.iter().any(|t| self.cur().text == *t)
    }

    fn at_kind(&self, kind: TokKind) -> bool {
        self.cur().kind == kind
    }

    fn advance(&mut self) -> Token {
        let t = self.cur().clone();
        if t.kind != TokKind::Eof {
            self.i += 1;
        }
        t
    }

    fn expect(&mut self, text: &str) -> Result<Token, ParseError> {
        let t = self.cur().clone();
        if t.text != text {
            return Err(ParseError {
                message: format!("expected {text:?}, got {:?}", t.text),
                span: t.span,
            });
        }
        Ok(self.advance())
    }

    /// [R1-SMT-INJECT] Binder names (param / let / refine / lambda param) must be
    /// charset-safe identifiers so they cannot smuggle SMT-LIB metacharacters
    /// into the `--prove` encoder. Accept `Ident` / `TypeIdent` (both lexed from
    /// `[A-Za-z_][A-Za-z0-9_]*`); reject `Str` and every other token kind. This
    /// is the parser-layer half of the fail-closed fix; `vc::sanitize_sym` is the
    /// encoder-layer backstop.
    fn expect_binder_name(&mut self) -> Result<Token, ParseError> {
        let t = self.cur().clone();
        if !matches!(t.kind, TokKind::Ident | TokKind::TypeIdent) {
            return Err(ParseError {
                message: format!("expected an identifier binder name, got {:?}", t.text),
                span: t.span,
            });
        }
        Ok(self.advance())
    }

    /// [R2-DEPTH] Bounded-recursion guard shared by `parse_expr` / `parse_unary`
    /// / `parse_type`. A crafted deeply-nested input would otherwise overflow the
    /// native stack (an uncatchable abort); above the ceiling we return a clean
    /// `ParseError`. Each guarded entry decrements on the way out.
    fn enter_depth(&mut self) -> Result<(), ParseError> {
        self.depth += 1;
        if self.depth > MAX_PARSE_DEPTH {
            self.depth -= 1;
            return Err(ParseError {
                message: "nesting depth exceeds parser limit".into(),
                span: self.cur().span,
            });
        }
        Ok(())
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let span = self.cur().span;
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        while !self.at_kind(TokKind::Eof) {
            if self.at(&["struct"]) {
                structs.push(self.parse_struct()?);
            } else if self.at(&["enum"]) {
                enums.push(self.parse_enum()?);
            } else if self.at(&["fn"]) {
                functions.push(self.parse_fn()?);
            } else {
                return Err(ParseError {
                    message: format!("expected struct, enum, or fn, got {:?}", self.cur().text),
                    span: self.cur().span,
                });
            }
        }
        Ok(Program {
            structs,
            enums,
            functions,
            span,
        })
    }

    fn parse_struct(&mut self) -> Result<StructDecl, ParseError> {
        let start = self.expect("struct")?.span;
        let name_tok = self.advance();
        if name_tok.kind != TokKind::TypeIdent {
            return Err(ParseError {
                message: "expected type name after struct".into(),
                span: name_tok.span,
            });
        }
        self.expect("{")?;
        let mut fields = Vec::new();
        if !self.at(&["}"]) {
            loop {
                let fname = self.advance().text;
                self.expect(":")?;
                let ty = self.parse_type()?;
                fields.push(FieldDecl { name: fname, ty });
                if self.at(&[","]) {
                    self.advance();
                    if self.at(&["}"]) {
                        break;
                    }
                    continue;
                }
                break;
            }
        }
        self.expect("}")?;
        Ok(StructDecl {
            name: name_tok.text,
            fields,
            span: start,
        })
    }

    fn parse_enum(&mut self) -> Result<EnumDecl, ParseError> {
        let start = self.expect("enum")?.span;
        let name_tok = self.advance();
        if name_tok.kind != TokKind::TypeIdent {
            return Err(ParseError {
                message: "expected type name after enum".into(),
                span: name_tok.span,
            });
        }
        self.expect("{")?;
        let mut variants = Vec::new();
        if !self.at(&["}"]) {
            loop {
                let vname_tok = self.advance();
                if vname_tok.kind != TokKind::TypeIdent {
                    return Err(ParseError {
                        message: "expected variant name".into(),
                        span: vname_tok.span,
                    });
                }
                let mut fields = Vec::new();
                if self.at(&["("]) {
                    self.advance();
                    if !self.at(&[")"]) {
                        fields.push(self.parse_type()?);
                        while self.at(&[","]) {
                            self.advance();
                            fields.push(self.parse_type()?);
                        }
                    }
                    self.expect(")")?;
                }
                variants.push(VariantDecl {
                    name: vname_tok.text,
                    fields,
                });
                if self.at(&[","]) {
                    self.advance();
                    if self.at(&["}"]) {
                        break;
                    }
                    continue;
                }
                break;
            }
        }
        self.expect("}")?;
        Ok(EnumDecl {
            name: name_tok.text,
            variants,
            span: start,
        })
    }

    fn parse_fn(&mut self) -> Result<FnDecl, ParseError> {
        let start = self.expect("fn")?.span;
        let name_tok = self.advance();
        if !matches!(name_tok.kind, TokKind::Ident | TokKind::Kw) {
            return Err(ParseError {
                message: "expected function name".into(),
                span: name_tok.span,
            });
        }
        let name = name_tok.text;
        self.expect("(")?;
        let mut params = Vec::new();
        if !self.at(&[")"]) {
            params.push(self.parse_param()?);
            while self.at(&[","]) {
                self.advance();
                params.push(self.parse_param()?);
            }
        }
        self.expect(")")?;
        self.expect("->")?;
        let ret = self.parse_type()?;
        let mut uses = Vec::new();
        if self.at(&["uses"]) {
            self.advance();
            self.expect("{")?;
            if !self.at(&["}"]) {
                uses.push(self.advance().text);
                while self.at(&[","]) {
                    self.advance();
                    uses.push(self.advance().text);
                }
            }
            self.expect("}")?;
        }
        let mut requires = Vec::new();
        let mut ensures = Vec::new();
        while self.at(&["requires", "ensures"]) {
            let which = self.advance().text;
            let expr = self.parse_expr()?;
            if which == "requires" {
                requires.push(expr);
            } else {
                ensures.push(expr);
            }
        }
        let body = self.parse_block()?;
        Ok(FnDecl {
            name,
            params,
            ret,
            uses,
            requires,
            ensures,
            body,
            span: start,
        })
    }

    fn parse_param(&mut self) -> Result<Param, ParseError> {
        let name = self.expect_binder_name()?.text;
        self.expect(":")?;
        let ty = self.parse_type()?;
        // [GAP4-VALUE-LABEL] optional value-label postfix on the param type.
        let label = self.parse_opt_label()?;
        Ok(Param { name, ty, label })
    }

    /// [GAP4-VALUE-LABEL] Optional `^{atom, ...}` postfix after an annotation
    /// type — param and let positions only (return / nested type positions
    /// stay label-free this slice). Atoms: the two DATA atoms `untrusted` /
    /// `secret`; authority stays on the `uses` clause. Canonicalized
    /// (sorted + deduped) at parse so render round-trips byte-identically.
    fn parse_opt_label(&mut self) -> Result<Vec<String>, ParseError> {
        if !self.at(&["^"]) {
            return Ok(Vec::new());
        }
        self.advance();
        self.expect("{")?;
        if self.at(&["}"]) {
            let t = self.cur().clone();
            return Err(ParseError {
                message: "empty label set (write no `^{}` instead)".into(),
                span: t.span,
            });
        }
        let mut atoms: Vec<String> = Vec::new();
        loop {
            let t = self.advance();
            match t.text.as_str() {
                "untrusted" | "secret" => {
                    if !atoms.contains(&t.text) {
                        atoms.push(t.text.clone());
                    }
                }
                other => {
                    return Err(ParseError {
                        message: format!(
                            "unknown label atom {other:?} (this slice: untrusted, secret)"
                        ),
                        span: t.span,
                    });
                }
            }
            if self.at(&[","]) {
                self.advance();
                continue;
            }
            break;
        }
        self.expect("}")?;
        atoms.sort();
        Ok(atoms)
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        self.enter_depth()?;
        let out = self.parse_type_inner();
        self.depth -= 1;
        out
    }

    fn parse_type_inner(&mut self) -> Result<Type, ParseError> {
        let t = self.cur().clone();
        match t.text.as_str() {
            "Int" => {
                self.advance();
                Ok(Type::Int)
            }
            "Bool" => {
                self.advance();
                Ok(Type::Bool)
            }
            "Str" => {
                self.advance();
                Ok(Type::Str)
            }
            "Unit" => {
                self.advance();
                Ok(Type::Unit)
            }
            "Console" => {
                self.advance();
                Ok(Type::Console)
            }
            "fn" => {
                // fn (T, U) -> R
                self.advance();
                self.expect("(")?;
                let mut params = Vec::new();
                if !self.at(&[")"]) {
                    params.push(self.parse_type()?);
                    while self.at(&[","]) {
                        self.advance();
                        params.push(self.parse_type()?);
                    }
                }
                self.expect(")")?;
                self.expect("->")?;
                let ret = self.parse_type()?;
                Ok(Type::Fn {
                    params,
                    ret: Box::new(ret),
                })
            }
            "List" => {
                self.advance();
                self.expect("<")?;
                let elem = self.parse_type()?;
                self.expect(">")?;
                Ok(Type::List {
                    elem: Box::new(elem),
                })
            }
            "Option" => {
                self.advance();
                self.expect("<")?;
                let inner = self.parse_type()?;
                self.expect(">")?;
                Ok(Type::Option {
                    inner: Box::new(inner),
                })
            }
            "Result" => {
                self.advance();
                self.expect("<")?;
                let ok = self.parse_type()?;
                self.expect(",")?;
                let err = self.parse_type()?;
                self.expect(">")?;
                Ok(Type::Result {
                    ok: Box::new(ok),
                    err: Box::new(err),
                })
            }
            "{" => {
                self.advance();
                let name = self.expect_binder_name()?.text;
                self.expect(":")?;
                self.expect("Int")?;
                self.expect("|")?;
                let pred = self.parse_expr()?;
                self.expect("}")?;
                Ok(Type::Refine {
                    name,
                    pred: Some(Box::new(pred)),
                })
            }
            _ if t.kind == TokKind::TypeIdent => {
                self.advance();
                Ok(Type::Named { name: t.text })
            }
            _ => Err(ParseError {
                message: format!("expected type, got {:?}", t.text),
                span: t.span,
            }),
        }
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let start = self.expect("{")?.span;
        let mut stmts = Vec::new();
        let mut result = None;
        while !self.at(&["}"]) {
            if self.at(&["let"]) {
                stmts.push(self.parse_let()?);
            } else {
                let expr = self.parse_expr()?;
                if self.at(&[";"]) {
                    self.advance();
                    let span = expr.span();
                    stmts.push(Stmt::Expr { expr, span });
                } else {
                    if !self.at(&["}"]) {
                        return Err(ParseError {
                            message: "expected ';' or '}' after expression".into(),
                            span: self.cur().span,
                        });
                    }
                    result = Some(Box::new(expr));
                    break;
                }
            }
        }
        self.expect("}")?;
        Ok(Block {
            stmts,
            result,
            span: start,
        })
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        let start = self.expect("let")?.span;
        let name = self.expect_binder_name()?.text;
        let ty = if self.at(&[":"]) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        // [GAP4-VALUE-LABEL] optional value-label postfix, only after an
        // explicit type annotation (`let x: Str^{secret} = ...`).
        let label = if ty.is_some() {
            self.parse_opt_label()?
        } else {
            Vec::new()
        };
        self.expect("=")?;
        let value = self.parse_expr()?;
        self.expect(";")?;
        Ok(Stmt::Let {
            name,
            ty,
            value,
            span: start,
            label,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.enter_depth()?;
        let out = self.parse_expr_inner();
        self.depth -= 1;
        out
    }

    fn parse_expr_inner(&mut self) -> Result<Expr, ParseError> {
        if self.at(&["match"]) {
            return self.parse_match();
        }
        if self.at(&["if"]) {
            return self.parse_if();
        }
        // Lambda: `fn (` … — not a top-level fn_decl (those start at program level).
        if self.at(&["fn"]) {
            return self.parse_lambda();
        }
        self.parse_or()
    }

    fn parse_lambda(&mut self) -> Result<Expr, ParseError> {
        let start = self.expect("fn")?.span;
        self.expect("(")?;
        let mut params = Vec::new();
        if !self.at(&[")"]) {
            loop {
                let name = self.expect_binder_name()?.text;
                let ty = if self.at(&[":"]) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };
                params.push((name, ty));
                if self.at(&[","]) {
                    self.advance();
                    continue;
                }
                break;
            }
        }
        self.expect(")")?;
        let ret = if self.at(&["->"]) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(Expr::Lambda {
            params,
            ret,
            body,
            span: start,
        })
    }

    fn parse_match(&mut self) -> Result<Expr, ParseError> {
        let start = self.expect("match")?.span;
        let scrutinee = self.parse_expr()?;
        self.expect("{")?;
        let mut arms = Vec::new();
        while !self.at(&["}"]) {
            let pattern = self.parse_pattern()?;
            self.expect("=>")?;
            let body = self.parse_expr()?;
            arms.push(MatchArm { pattern, body });
            if self.at(&[","]) {
                self.advance();
            } else if !self.at(&["}"]) {
                return Err(ParseError {
                    message: "expected ',' or '}' after match arm".into(),
                    span: self.cur().span,
                });
            }
        }
        self.expect("}")?;
        if arms.is_empty() {
            return Err(ParseError {
                message: "match needs at least one arm".into(),
                span: start,
            });
        }
        Ok(Expr::MatchExpr {
            scrutinee: Box::new(scrutinee),
            arms,
            span: start,
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        let t = self.cur().clone();
        if t.text == "_" {
            self.advance();
            return Ok(Pattern::Wildcard { span: t.span });
        }
        if t.kind == TokKind::Int {
            self.advance();
            let value: i64 = t.text.parse().map_err(|_| ParseError {
                message: format!("bad int in pattern {}", t.text),
                span: t.span,
            })?;
            return Ok(Pattern::LitInt {
                value,
                span: t.span,
            });
        }
        if t.kind == TokKind::Str {
            self.advance();
            return Ok(Pattern::LitStr {
                value: t.text,
                span: t.span,
            });
        }
        if t.text == "true" {
            self.advance();
            return Ok(Pattern::LitBool {
                value: true,
                span: t.span,
            });
        }
        if t.text == "false" {
            self.advance();
            return Ok(Pattern::LitBool {
                value: false,
                span: t.span,
            });
        }
        if t.text == "unit" {
            self.advance();
            return Ok(Pattern::LitUnit { span: t.span });
        }
        // Uppercase ctor: prelude (Some/None/Ok/Err) or Enum::Variant
        if t.kind == TokKind::TypeIdent || is_prelude_ctor(&t.text) {
            self.advance();
            // Enum path Type::Variant
            if self.at(&["::"]) {
                self.advance();
                let v = self.advance();
                let mut args = Vec::new();
                if self.at(&["("]) {
                    self.advance();
                    if !self.at(&[")"]) {
                        args.push(self.parse_pattern()?);
                        while self.at(&[","]) {
                            self.advance();
                            args.push(self.parse_pattern()?);
                        }
                    }
                    self.expect(")")?;
                }
                return Ok(Pattern::Ctor {
                    type_name: Some(t.text),
                    name: v.text,
                    args,
                    span: t.span,
                });
            }
            let name = t.text;
            let mut args = Vec::new();
            if self.at(&["("]) {
                self.advance();
                if !self.at(&[")"]) {
                    args.push(self.parse_pattern()?);
                    while self.at(&[","]) {
                        self.advance();
                        args.push(self.parse_pattern()?);
                    }
                }
                self.expect(")")?;
            } else if name != "None" && is_prelude_ctor(&name) {
                return Err(ParseError {
                    message: format!("{name} pattern requires arguments in (...)"),
                    span: t.span,
                });
            } else if !is_prelude_ctor(&name) {
                // Bare TypeIdent without :: — unit enum variant only if we know it; reject for now
                // unless zero-arg allowed as variant shorthand — SPEC requires Type::Variant.
                return Err(ParseError {
                    message: format!("use {name}::Variant in patterns"),
                    span: t.span,
                });
            }
            return Ok(Pattern::Ctor {
                type_name: None,
                name,
                args,
                span: t.span,
            });
        }
        // Lowercase binder
        if t.kind == TokKind::Ident {
            self.advance();
            return Ok(Pattern::Bind {
                name: t.text,
                span: t.span,
            });
        }
        Err(ParseError {
            message: format!("unexpected pattern token {:?}", t.text),
            span: t.span,
        })
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while self.at(&["||"]) {
            let op = self.advance().text;
            let right = self.parse_and()?;
            let span = left.span();
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_cmp()?;
        while self.at(&["&&"]) {
            let op = self.advance().text;
            let right = self.parse_cmp()?;
            let span = left.span();
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_cmp(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_add()?;
        if self.at(&["==", "!=", "<", "<=", ">", ">="]) {
            let op = self.advance().text;
            let right = self.parse_add()?;
            let span = left.span();
            return Ok(Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            });
        }
        Ok(left)
    }

    fn parse_add(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_mul()?;
        while self.at(&["+", "-", "++"]) {
            let op = self.advance().text;
            let right = self.parse_mul()?;
            let span = left.span();
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_mul(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        while self.at(&["*", "/", "%"]) {
            let op = self.advance().text;
            let right = self.parse_unary()?;
            let span = left.span();
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        self.enter_depth()?;
        let out = self.parse_unary_inner();
        self.depth -= 1;
        out
    }

    fn parse_unary_inner(&mut self) -> Result<Expr, ParseError> {
        if self.at(&["-", "!"]) {
            let op_tok = self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: op_tok.text,
                expr: Box::new(expr),
                span: op_tok.span,
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.at(&["::"]) {
                let (type_name, span) = match &expr {
                    Expr::Name { name, span } => (name.clone(), *span),
                    _ => {
                        return Err(ParseError {
                            message: "`::` expects a type name on the left".into(),
                            span: self.cur().span,
                        });
                    }
                };
                self.advance();
                let v = self.advance();
                let mut args = Vec::new();
                if self.at(&["("]) {
                    self.advance();
                    if !self.at(&[")"]) {
                        args.push(self.parse_expr()?);
                        while self.at(&[","]) {
                            self.advance();
                            args.push(self.parse_expr()?);
                        }
                    }
                    self.expect(")")?;
                }
                expr = Expr::Ctor {
                    type_name: Some(type_name),
                    name: v.text,
                    args,
                    span,
                };
            } else if self.at(&["("]) {
                let span = self.advance().span;
                // Named args → struct lit; positional → call / prelude ctor.
                let named = self.at_kind(TokKind::Ident)
                    && self.tokens.get(self.i + 1).map(|t| t.text.as_str()) == Some(":");
                if named {
                    let mut fields = Vec::new();
                    loop {
                        let fname = self.advance().text;
                        self.expect(":")?;
                        let val = self.parse_expr()?;
                        fields.push((fname, val));
                        if self.at(&[","]) {
                            self.advance();
                            if self.at(&[")"]) {
                                break;
                            }
                            continue;
                        }
                        break;
                    }
                    self.expect(")")?;
                    let name = match &expr {
                        Expr::Name { name, .. } => name.clone(),
                        _ => {
                            return Err(ParseError {
                                message: "named args only allowed for struct construction"
                                    .into(),
                                span,
                            });
                        }
                    };
                    expr = Expr::StructLit {
                        name,
                        fields,
                        span,
                    };
                } else {
                    let mut args = Vec::new();
                    if !self.at(&[")"]) {
                        args.push(self.parse_expr()?);
                        while self.at(&[","]) {
                            self.advance();
                            args.push(self.parse_expr()?);
                        }
                    }
                    self.expect(")")?;
                    if let Expr::Name { name, span: nspan } = &expr {
                        if is_prelude_ctor(name) {
                            expr = Expr::Ctor {
                                type_name: None,
                                name: name.clone(),
                                args,
                                span: *nspan,
                            };
                            continue;
                        }
                    }
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                        span,
                    };
                }
            } else if self.at(&["."]) {
                self.advance();
                let field = self.advance().text;
                let span = expr.span();
                expr = Expr::FieldAccess {
                    obj: Box::new(expr),
                    field,
                    span,
                };
            } else if self.at(&["?"]) {
                let span = self.advance().span;
                expr = Expr::Propagate {
                    expr: Box::new(expr),
                    span,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let t = self.cur().clone();
        if t.text == "if" {
            return self.parse_if();
        }
        if t.text == "match" {
            return self.parse_match();
        }
        if t.kind == TokKind::Int {
            self.advance();
            let value: i64 = t.text.parse().map_err(|_| ParseError {
                message: format!("bad int literal {}", t.text),
                span: t.span,
            })?;
            return Ok(Expr::LitInt {
                value,
                span: t.span,
            });
        }
        if t.kind == TokKind::Str {
            self.advance();
            return Ok(Expr::LitStr {
                value: t.text,
                span: t.span,
            });
        }
        if t.text == "true" {
            self.advance();
            return Ok(Expr::LitBool {
                value: true,
                span: t.span,
            });
        }
        if t.text == "false" {
            self.advance();
            return Ok(Expr::LitBool {
                value: false,
                span: t.span,
            });
        }
        if t.text == "unit" {
            self.advance();
            return Ok(Expr::LitUnit { span: t.span });
        }
        if t.text == "(" {
            self.advance();
            let e = self.parse_expr()?;
            self.expect(")")?;
            return Ok(e);
        }
        if t.text == "{" {
            return Ok(Expr::Block(self.parse_block()?));
        }
        if t.text == "[" {
            self.advance();
            let mut elems = Vec::new();
            if !self.at(&["]"]) {
                elems.push(self.parse_expr()?);
                while self.at(&[","]) {
                    self.advance();
                    if self.at(&["]"]) {
                        break;
                    }
                    elems.push(self.parse_expr()?);
                }
            }
            self.expect("]")?;
            return Ok(Expr::ListLit {
                elems,
                span: t.span,
            });
        }
        // Typed hole: `?body` lexed as a single "?body" token.
        if t.text.starts_with('?') && t.text.len() > 1 {
            self.advance();
            return Ok(Expr::Hole {
                name: t.text[1..].to_string(),
                span: t.span,
            });
        }
        // Prelude ctors as TypeIdent or bare None
        if t.kind == TokKind::TypeIdent || is_prelude_ctor(&t.text) {
            self.advance();
            if t.text == "None" || (is_prelude_ctor(&t.text) && !self.at(&["("])) {
                if t.text != "None" && is_prelude_ctor(&t.text) {
                    return Err(ParseError {
                        message: format!("{} requires (...)", t.text),
                        span: t.span,
                    });
                }
                return Ok(Expr::Ctor {
                    type_name: None,
                    name: t.text,
                    args: vec![],
                    span: t.span,
                });
            }
            // Some/Ok/Err without call yet — Name; postfix upgrades to Ctor.
            // TypeIdent may also be struct name or enum prefix for `::`.
            return Ok(Expr::Name {
                name: t.text,
                span: t.span,
            });
        }
        let reserved = [
            "fn", "let", "if", "else", "match", "uses", "requires", "ensures",
        ];
        if matches!(t.kind, TokKind::Ident | TokKind::Kw) && !reserved.contains(&t.text.as_str()) {
            if t.kind == TokKind::Ident || t.text == "console" {
                self.advance();
                return Ok(Expr::Name {
                    name: t.text,
                    span: t.span,
                });
            }
        }
        if t.kind == TokKind::Ident {
            self.advance();
            return Ok(Expr::Name {
                name: t.text,
                span: t.span,
            });
        }
        Err(ParseError {
            message: format!("unexpected token {:?}", t.text),
            span: t.span,
        })
    }

    fn parse_if(&mut self) -> Result<Expr, ParseError> {
        let start = self.expect("if")?.span;
        let cond = self.parse_or()?; // avoid nested if/match ambiguity at top of parse_expr
        let then_body = self.parse_block()?;
        self.expect("else")?;
        let else_body = if self.at(&["if"]) {
            let else_inner = self.parse_if()?;
            let span = else_inner.span();
            Block {
                stmts: vec![],
                result: Some(Box::new(else_inner)),
                span,
            }
        } else {
            self.parse_block()?
        };
        Ok(Expr::IfExpr {
            cond: Box::new(cond),
            then_body,
            else_body,
            span: start,
        })
    }
}

pub fn parse(source: &str) -> Result<Program, ParseError> {
    let tokens = lex(source)?;
    let mut p = Parser {
        tokens,
        i: 0,
        depth: 0,
    };
    let prog = p.parse_program()?;
    if !p.at_kind(TokKind::Eof) {
        return Err(ParseError {
            message: format!("trailing token {:?}", p.cur().text),
            span: p.cur().span,
        });
    }
    Ok(prog)
}
