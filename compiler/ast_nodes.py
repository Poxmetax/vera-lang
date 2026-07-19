"""VERA MVP AST nodes (Phase 1)."""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Optional, Union


@dataclass(frozen=True)
class Span:
    line: int
    col: int


# ---- types ----

@dataclass(frozen=True)
class TyInt:
    kind: str = "Int"


@dataclass(frozen=True)
class TyBool:
    kind: str = "Bool"


@dataclass(frozen=True)
class TyStr:
    kind: str = "Str"


@dataclass(frozen=True)
class TyUnit:
    kind: str = "Unit"


@dataclass(frozen=True)
class TyConsole:
    kind: str = "Console"


@dataclass(frozen=True)
class TyNamed:
    name: str


@dataclass(frozen=True)
class TyList:
    elem: "Type"


@dataclass(frozen=True)
class TyOption:
    inner: "Type"


@dataclass(frozen=True)
class TyResult:
    ok: "Type"
    err: "Type"


@dataclass(frozen=True)
class TyRefine:
    name: str
    base: "Type"  # Int in MVP
    pred: "Expr"


Type = Union[TyInt, TyBool, TyStr, TyUnit, TyConsole, TyNamed, TyList, TyOption, TyResult, TyRefine]


# ---- expressions / statements ----

@dataclass(frozen=True)
class LitInt:
    value: int
    span: Span


@dataclass(frozen=True)
class LitStr:
    value: str
    span: Span


@dataclass(frozen=True)
class LitBool:
    value: bool
    span: Span


@dataclass(frozen=True)
class LitUnit:
    span: Span


@dataclass(frozen=True)
class Name:
    name: str
    span: Span


@dataclass(frozen=True)
class BinOp:
    op: str
    left: "Expr"
    right: "Expr"
    span: Span


@dataclass(frozen=True)
class UnaryOp:
    op: str
    expr: "Expr"
    span: Span


@dataclass(frozen=True)
class Call:
    callee: "Expr"
    args: tuple["Expr", ...]
    span: Span


@dataclass(frozen=True)
class FieldAccess:
    obj: "Expr"
    field: str
    span: Span


@dataclass(frozen=True)
class IfExpr:
    cond: "Expr"
    then_body: "Block"
    else_body: "Block"
    span: Span


@dataclass(frozen=True)
class Block:
    stmts: tuple["Stmt", ...]
    result: Optional["Expr"]
    span: Span


@dataclass(frozen=True)
class LetStmt:
    name: str
    ty: Optional[Type]
    value: "Expr"
    span: Span


@dataclass(frozen=True)
class ExprStmt:
    expr: "Expr"
    span: Span


Stmt = Union[LetStmt, ExprStmt]
Expr = Union[
    LitInt, LitStr, LitBool, LitUnit, Name, BinOp, UnaryOp, Call, FieldAccess, IfExpr, Block
]


@dataclass(frozen=True)
class Param:
    name: str
    ty: Type


@dataclass(frozen=True)
class FnDecl:
    name: str
    params: tuple[Param, ...]
    ret: Type
    uses: tuple[str, ...]
    requires: tuple[Expr, ...]
    ensures: tuple[Expr, ...]
    body: Block
    span: Span


@dataclass(frozen=True)
class Program:
    functions: tuple[FnDecl, ...]
    span: Span


def type_to_str(t: Type) -> str:
    if isinstance(t, (TyInt, TyBool, TyStr, TyUnit, TyConsole)):
        return t.kind
    if isinstance(t, TyNamed):
        return t.name
    if isinstance(t, TyList):
        return f"List<{type_to_str(t.elem)}>"
    if isinstance(t, TyOption):
        return f"Option<{type_to_str(t.inner)}>"
    if isinstance(t, TyResult):
        return f"Result<{type_to_str(t.ok)},{type_to_str(t.err)}>"
    if isinstance(t, TyRefine):
        return f"{{{t.name}: Int | ...}}"
    return str(t)


def ast_to_canonical(obj: Any) -> Any:
    """Deterministic JSON-serializable shape for content hashing."""
    if obj is None or isinstance(obj, (str, int, bool)):
        return obj
    if isinstance(obj, tuple):
        return [ast_to_canonical(x) for x in obj]
    if isinstance(obj, Span):
        return None  # spans excluded from content hash
    if hasattr(obj, "__dataclass_fields__"):
        d: dict[str, Any] = {"__node__": type(obj).__name__}
        for k in obj.__dataclass_fields__:
            if k == "span":
                continue
            d[k] = ast_to_canonical(getattr(obj, k))
        return d
    return repr(obj)
