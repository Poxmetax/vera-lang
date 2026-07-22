"""Tree-walking interpreter with Console capability + runtime contracts (Phase 1)."""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Callable, Optional

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
    UnaryOp,
)


class RuntimeError_(Exception):
    """VERA deterministic trap."""


@dataclass
class Console:
    writes: list[str] = field(default_factory=list)

    def print(self, s: str) -> None:
        self.writes.append(s)
        print(s, flush=True)


@dataclass
class Env:
    values: dict[str, Any]
    parent: Optional["Env"] = None

    def get(self, name: str) -> Any:
        if name in self.values:
            return self.values[name]
        if self.parent:
            return self.parent.get(name)
        raise RuntimeError_(f"unbound name {name!r}")

    def extend(self, name: str, value: Any) -> "Env":
        return Env({**self.values, name: value}, self.parent)


class Interpreter:
    def __init__(self, program: Program, console: Optional[Console] = None) -> None:
        self.fns = {f.name: f for f in program.functions}
        self.console = console or Console()

    def run_main(self) -> Any:
        main = self.fns.get("main")
        if main is None:
            raise RuntimeError_("no main")
        # Inject Console capability — no ambient authority elsewhere.
        args = []
        for p in main.params:
            if p.name == "console" or getattr(p.ty, "kind", None) == "Console":
                args.append(self.console)
            else:
                raise RuntimeError_(f"main cannot bind parameter {p.name!r} in Phase 1 runner")
        return self.call_fn(main, args)

    def call_fn(self, fn: FnDecl, args: list[Any]) -> Any:
        if len(args) != len(fn.params):
            raise RuntimeError_(f"{fn.name}: arity mismatch")
        env = Env({p.name: a for p, a in zip(fn.params, args)})
        for req in fn.requires:
            if not self.eval_expr(req, env):
                raise RuntimeError_(f"{fn.name}: requires violated")
        result = self.eval_block(fn.body, env)
        ens_env = env.extend("result", result)
        for ens in fn.ensures:
            if not self.eval_expr(ens, ens_env):
                raise RuntimeError_(f"{fn.name}: ensures violated")
        return result

    def eval_block(self, block: Block, env: Env) -> Any:
        e = env
        for stmt in block.stmts:
            if isinstance(stmt, LetStmt):
                e = e.extend(stmt.name, self.eval_expr(stmt.value, e))
            elif isinstance(stmt, ExprStmt):
                self.eval_expr(stmt.expr, e)
        if block.result is not None:
            return self.eval_expr(block.result, e)
        return None

    def eval_expr(self, expr: Expr, env: Env) -> Any:
        if isinstance(expr, LitInt):
            return expr.value
        if isinstance(expr, LitBool):
            return expr.value
        if isinstance(expr, LitStr):
            return expr.value
        if isinstance(expr, LitUnit):
            return None
        if isinstance(expr, Name):
            return env.get(expr.name)
        if isinstance(expr, UnaryOp):
            v = self.eval_expr(expr.expr, env)
            if expr.op == "-":
                return checked_neg(v)
            if expr.op == "!":
                return not v
            raise RuntimeError_(f"bad unary {expr.op}")
        if isinstance(expr, BinOp):
            l = self.eval_expr(expr.left, env)
            if expr.op == "&&":
                return bool(l) and bool(self.eval_expr(expr.right, env))
            if expr.op == "||":
                return bool(l) or bool(self.eval_expr(expr.right, env))
            r = self.eval_expr(expr.right, env)
            if expr.op == "++":
                return str(l) + str(r)
            if expr.op == "+":
                return checked_add(l, r)
            if expr.op == "-":
                return checked_sub(l, r)
            if expr.op == "*":
                return checked_mul(l, r)
            if expr.op == "/":
                return checked_div(l, r)
            if expr.op == "%":
                return checked_mod(l, r)
            ops: dict[str, Callable[[Any, Any], bool]] = {
                "==": lambda a, b: a == b,
                "!=": lambda a, b: a != b,
                "<": lambda a, b: a < b,
                "<=": lambda a, b: a <= b,
                ">": lambda a, b: a > b,
                ">=": lambda a, b: a >= b,
            }
            if expr.op in ops:
                return ops[expr.op](l, r)
            raise RuntimeError_(f"bad op {expr.op}")
        if isinstance(expr, FieldAccess):
            # bare method reference unsupported; only Call uses FieldAccess
            return (self.eval_expr(expr.obj, env), expr.field)
        if isinstance(expr, Call):
            if isinstance(expr.callee, FieldAccess):
                obj = self.eval_expr(expr.callee.obj, env)
                if isinstance(obj, Console) and expr.callee.field == "print":
                    s = self.eval_expr(expr.args[0], env)
                    obj.print(str(s))
                    return None
            if isinstance(expr.callee, Name) and expr.callee.name in self.fns:
                fn = self.fns[expr.callee.name]
                args = [self.eval_expr(a, env) for a in expr.args]
                return self.call_fn(fn, args)
            raise RuntimeError_("bad call")
        if isinstance(expr, IfExpr):
            if self.eval_expr(expr.cond, env):
                return self.eval_block(expr.then_body, env)
            return self.eval_block(expr.else_body, env)
        if isinstance(expr, Block):
            return self.eval_block(expr, env)
        raise RuntimeError_(f"unhandled {type(expr).__name__}")


def checked_add(a: int, b: int) -> int:
    r = a + b
    if r.bit_length() > 63 or r < -(1 << 63):
        raise RuntimeError_("Int overflow")
    # keep Python int; overflow check for Phase 1 demo
    if not (-(1 << 63) <= r < (1 << 63)):
        raise RuntimeError_("Int overflow")
    return r


def checked_sub(a: int, b: int) -> int:
    r = a - b
    if not (-(1 << 63) <= r < (1 << 63)):
        raise RuntimeError_("Int overflow")
    return r


def checked_mul(a: int, b: int) -> int:
    r = a * b
    if not (-(1 << 63) <= r < (1 << 63)):
        raise RuntimeError_("Int overflow")
    return r


def checked_div(a: int, b: int) -> int:
    if b == 0:
        raise RuntimeError_("division by zero")
    return a // b


def checked_mod(a: int, b: int) -> int:
    if b == 0:
        raise RuntimeError_("division by zero")
    return a % b


def checked_neg(a: int) -> int:
    r = -a
    if not (-(1 << 63) <= r < (1 << 63)):
        raise RuntimeError_("Int overflow")
    return r
