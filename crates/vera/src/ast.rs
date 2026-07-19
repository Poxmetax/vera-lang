//! VERA MVP AST nodes (Phase 1). Spans are excluded from content hashes.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: u32,
    pub col: u32,
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub enum Type {
    Int,
    Bool,
    Str,
    Unit,
    Console,
    Named { name: String },
    List { elem: Box<Type> },
    Option { inner: Box<Type> },
    Result { ok: Box<Type>, err: Box<Type> },
    Refine {
        name: String,
        pred: Option<Box<Expr>>,
    },
}

impl Type {
    pub fn to_str(&self) -> String {
        match self {
            Type::Int => "Int".into(),
            Type::Bool => "Bool".into(),
            Type::Str => "Str".into(),
            Type::Unit => "Unit".into(),
            Type::Console => "Console".into(),
            Type::Named { name } => name.clone(),
            Type::List { elem } => format!("List<{}>", elem.to_str()),
            Type::Option { inner } => format!("Option<{}>", inner.to_str()),
            Type::Result { ok, err } => format!("Result<{},{}>", ok.to_str(), err.to_str()),
            Type::Refine { name, .. } => format!("{{{name}: Int | ...}}"),
        }
    }
}

/// Patterns for `match` (case is load-bearing: lowercase = binder, uppercase = ctor).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub enum Pattern {
    Wildcard {
        #[serde(skip)]
        span: Span,
    },
    LitInt {
        value: i64,
        #[serde(skip)]
        span: Span,
    },
    LitStr {
        value: String,
        #[serde(skip)]
        span: Span,
    },
    LitBool {
        value: bool,
        #[serde(skip)]
        span: Span,
    },
    LitUnit {
        #[serde(skip)]
        span: Span,
    },
    Bind {
        name: String,
        #[serde(skip)]
        span: Span,
    },
    Ctor {
        /// Optional enum type prefix.
        type_name: Option<String>,
        name: String,
        args: Vec<Pattern>,
        #[serde(skip)]
        span: Span,
    },
}

impl Pattern {
    pub fn span(&self) -> Span {
        match self {
            Pattern::Wildcard { span }
            | Pattern::LitInt { span, .. }
            | Pattern::LitStr { span, .. }
            | Pattern::LitBool { span, .. }
            | Pattern::LitUnit { span }
            | Pattern::Bind { span, .. }
            | Pattern::Ctor { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub enum Expr {
    LitInt {
        value: i64,
        #[serde(skip)]
        span: Span,
    },
    LitStr {
        value: String,
        #[serde(skip)]
        span: Span,
    },
    LitBool {
        value: bool,
        #[serde(skip)]
        span: Span,
    },
    LitUnit {
        #[serde(skip)]
        span: Span,
    },
    Name {
        name: String,
        #[serde(skip)]
        span: Span,
    },
    /// Prelude / enum constructor: `Some(x)`, `None`, `Ok(v)`, `Err(e)`, `Verdict::Rejected(s)`.
    Ctor {
        /// Optional enum type prefix (`Verdict` in `Verdict::Rejected`).
        type_name: Option<String>,
        name: String,
        args: Vec<Expr>,
        #[serde(skip)]
        span: Span,
    },
    /// Struct construction with named fields: `Point(x: 1, y: 2)`.
    StructLit {
        name: String,
        fields: Vec<(String, Expr)>,
        #[serde(skip)]
        span: Span,
    },
    BinOp {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
        #[serde(skip)]
        span: Span,
    },
    UnaryOp {
        op: String,
        expr: Box<Expr>,
        #[serde(skip)]
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        #[serde(skip)]
        span: Span,
    },
    FieldAccess {
        obj: Box<Expr>,
        field: String,
        #[serde(skip)]
        span: Span,
    },
    IfExpr {
        cond: Box<Expr>,
        then_body: Block,
        else_body: Block,
        #[serde(skip)]
        span: Span,
    },
    MatchExpr {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
        #[serde(skip)]
        span: Span,
    },
    Block(Block),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::LitInt { span, .. }
            | Expr::LitStr { span, .. }
            | Expr::LitBool { span, .. }
            | Expr::LitUnit { span }
            | Expr::Name { span, .. }
            | Expr::Ctor { span, .. }
            | Expr::StructLit { span, .. }
            | Expr::BinOp { span, .. }
            | Expr::UnaryOp { span, .. }
            | Expr::Call { span, .. }
            | Expr::FieldAccess { span, .. }
            | Expr::IfExpr { span, .. }
            | Expr::MatchExpr { span, .. } => *span,
            Expr::Block(b) => b.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub result: Option<Box<Expr>>,
    #[serde(skip)]
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Expr,
        #[serde(skip)]
        span: Span,
    },
    Expr {
        expr: Expr,
        #[serde(skip)]
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct FieldDecl {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
    #[serde(skip)]
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct VariantDecl {
    pub name: String,
    pub fields: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<VariantDecl>,
    #[serde(skip)]
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Type,
    pub uses: Vec<String>,
    pub requires: Vec<Expr>,
    pub ensures: Vec<Expr>,
    pub body: Block,
    #[serde(skip)]
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "__node__")]
pub struct Program {
    pub structs: Vec<StructDecl>,
    pub enums: Vec<EnumDecl>,
    pub functions: Vec<FnDecl>,
    #[serde(skip)]
    pub span: Span,
}

/// Built-in ADT constructors (prelude).
pub fn is_prelude_ctor(name: &str) -> bool {
    matches!(name, "Some" | "None" | "Ok" | "Err")
}
