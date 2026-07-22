"""Lightweight Phase 1 type checker (HM inference deferred; annotated MVP)."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from .ast_nodes import (
    BinOp,
    Block,
    Call,
    Expr,
    ExprStmt,
    FieldAccess,
    FnDecl,
    IfExpr,
    LetStmt,
    LitBool,
    LitInt,
    LitStr,
    LitUnit,
    Name,
    Program,
    Span,
    TyBool,
    TyConsole,
    TyInt,
    TyRefine,
    TyStr,
    TyUnit,
    Type,
    UnaryOp,
    type_to_str,
)


class TypeError(Exception):
    def __init__(self, message: str, span: Optional[Span] = None) -> None:
        loc = f"{span.line}:{span.col}: " if span else ""
        super().__init__(f"{loc}{message}")


@dataclass
class Env:
    vars: dict[str, Type]
    fns: dict[str, FnDecl]

    def extend(self, name: str, ty: Type) -> "Env":
        n = dict(self.vars)
        n[name] = ty
        return Env(n, self.fns)


def types_equal(a: Type, b: Type) -> bool:
    if type(a) is not type(b):
        # Int and {x:Int|...} are compatible for Phase 1 assignment into refined params at runtime
        if isinstance(a, TyRefine) and isinstance(b, TyInt):
            return True
        if isinstance(b, TyRefine) and isinstance(a, TyInt):
            return True
        return False
    if isinstance(a, TyRefine) and isinstance(b, TyRefine):
        return True
    return type_to_str(a) == type_to_str(b)


def check_program(program: Program) -> None:
    fns = {f.name: f for f in program.functions}
    if "main" not in fns:
        raise TypeError("program must define fn main")
    for fn in program.functions:
        check_fn(fn, fns)


def check_fn(fn: FnDecl, fns: dict[str, FnDecl]) -> None:
    for u in fn.uses:
        if u != "console":
            raise TypeError(f"unknown capability {u!r} (MVP allows only console)", fn.span)
    env = Env({p.name: p.ty for p in fn.params}, fns)
    for req in fn.requires:
        t = infer_expr(req, env)
        if not isinstance(t, TyBool):
            raise TypeError("requires clause must be Bool", getattr(req, "span", fn.span))
    body_ty = check_block(fn.body, env)
    if not types_equal(body_ty, fn.ret) and not (isinstance(fn.ret, TyUnit) and isinstance(body_ty, TyUnit)):
        # allow Unit body when last stmt is exprstmt-only
        if not (isinstance(fn.ret, TyUnit) and isinstance(body_ty, TyUnit)):
            raise TypeError(
                f"function {fn.name}: body type {type_to_str(body_ty)} != declared {type_to_str(fn.ret)}",
                fn.span,
            )
    # ensures refer to `result`
    ens_env = env.extend("result", fn.ret)
    for ens in fn.ensures:
        t = infer_expr(ens, ens_env)
        if not isinstance(t, TyBool):
            raise TypeError("ensures clause must be Bool", getattr(ens, "span", fn.span))


def check_block(block: Block, env: Env) -> Type:
    e = env
    for stmt in block.stmts:
        if isinstance(stmt, LetStmt):
            vty = infer_expr(stmt.value, e)
            if stmt.ty is not None and not types_equal(vty, stmt.ty):
                raise TypeError(
                    f"let {stmt.name}: got {type_to_str(vty)}, expected {type_to_str(stmt.ty)}",
                    stmt.span,
                )
            e = e.extend(stmt.name, stmt.ty or vty)
        elif isinstance(stmt, ExprStmt):
            infer_expr(stmt.expr, e)
    if block.result is not None:
        return infer_expr(block.result, e)
    return TyUnit()


def infer_expr(expr: Expr, env: Env) -> Type:
    if isinstance(expr, LitInt):
        return TyInt()
    if isinstance(expr, LitBool):
        return TyBool()
    if isinstance(expr, LitStr):
        return TyStr()
    if isinstance(expr, LitUnit):
        return TyUnit()
    if isinstance(expr, Name):
        if expr.name in env.vars:
            return env.vars[expr.name]
        if expr.name in env.fns:
            raise TypeError(f"{expr.name} is a function; call it with (...)", expr.span)
        raise TypeError(f"unknown name {expr.name!r}", expr.span)
    if isinstance(expr, UnaryOp):
        t = infer_expr(expr.expr, env)
        if expr.op == "-" and isinstance(t, (TyInt, TyRefine)):
            return TyInt()
        if expr.op == "!" and isinstance(t, TyBool):
            return TyBool()
        raise TypeError(f"unary {expr.op} on {type_to_str(t)}", expr.span)
    if isinstance(expr, BinOp):
        lt = infer_expr(expr.left, env)
        rt = infer_expr(expr.right, env)
        if expr.op == "++":
            if isinstance(lt, TyStr) and isinstance(rt, TyStr):
                return TyStr()
            raise TypeError("++ expects Str ++ Str", expr.span)
        if expr.op in {"+", "-", "*", "/", "%"}:
            if isinstance(lt, (TyInt, TyRefine)) and isinstance(rt, (TyInt, TyRefine)):
                return TyInt()
            raise TypeError(f"arithmetic on {type_to_str(lt)} and {type_to_str(rt)}", expr.span)
        if expr.op in {"==", "!=", "<", "<=", ">", ">="}:
            return TyBool()
        if expr.op in {"&&", "||"}:
            if isinstance(lt, TyBool) and isinstance(rt, TyBool):
                return TyBool()
            raise TypeError("logical ops need Bool", expr.span)
        raise TypeError(f"unknown operator {expr.op}", expr.span)
    if isinstance(expr, FieldAccess):
        obj_t = infer_expr(expr.obj, env)
        if isinstance(obj_t, TyConsole) and expr.field == "print":
            # method type is special-cased in Call
            return TyConsole()  # placeholder; Call handles print
        raise TypeError(f"unknown field {expr.field} on {type_to_str(obj_t)}", expr.span)
    if isinstance(expr, Call):
        if isinstance(expr.callee, FieldAccess):
            obj_t = infer_expr(expr.callee.obj, env)
            if isinstance(obj_t, TyConsole) and expr.callee.field == "print":
                if len(expr.args) != 1:
                    raise TypeError("Console.print takes 1 argument", expr.span)
                at = infer_expr(expr.args[0], env)
                if not isinstance(at, TyStr):
                    raise TypeError("Console.print expects Str", expr.span)
                return TyUnit()
        if isinstance(expr.callee, Name) and expr.callee.name in env.fns:
            fn = env.fns[expr.callee.name]
            if len(expr.args) != len(fn.params):
                raise TypeError(
                    f"{fn.name} expects {len(fn.params)} args, got {len(expr.args)}",
                    expr.span,
                )
            for a, p in zip(expr.args, fn.params):
                at = infer_expr(a, env)
                if not types_equal(at, p.ty):
                    raise TypeError(
                        f"arg type {type_to_str(at)} != {type_to_str(p.ty)}",
                        expr.span,
                    )
            return fn.ret
        raise TypeError("unsupported call", expr.span)
    if isinstance(expr, IfExpr):
        ct = infer_expr(expr.cond, env)
        if not isinstance(ct, TyBool):
            raise TypeError("if condition must be Bool", expr.span)
        tt = check_block(expr.then_body, env)
        et = check_block(expr.else_body, env)
        if not types_equal(tt, et):
            raise TypeError(
                f"if branches differ: {type_to_str(tt)} vs {type_to_str(et)}",
                expr.span,
            )
        return tt
    if isinstance(expr, Block):
        return check_block(expr, env)
    raise TypeError(f"unhandled expression {type(expr).__name__}")
