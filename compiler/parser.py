"""VERA MVP recursive-descent parser (Phase 1)."""
from __future__ import annotations

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
    Param,
    Program,
    Span,
    TyBool,
    TyConsole,
    TyInt,
    TyList,
    TyNamed,
    TyOption,
    TyRefine,
    TyResult,
    TyStr,
    TyUnit,
    Type,
    UnaryOp,
)
from .lexer import LexError, TokKind, Token, lex


class ParseError(Exception):
    def __init__(self, message: str, span: Span) -> None:
        super().__init__(f"{span.line}:{span.col}: {message}")
        self.span = span


class Parser:
    def __init__(self, tokens: list[Token]) -> None:
        self.tokens = tokens
        self.i = 0

    def cur(self) -> Token:
        return self.tokens[self.i]

    def at(self, *texts: str) -> bool:
        return self.cur().text in texts

    def at_kind(self, kind: TokKind) -> bool:
        return self.cur().kind == kind

    def advance(self) -> Token:
        t = self.cur()
        if t.kind != TokKind.EOF:
            self.i += 1
        return t

    def expect(self, text: str) -> Token:
        t = self.cur()
        if t.text != text:
            raise ParseError(f"expected {text!r}, got {t.text!r}", t.span)
        return self.advance()

    def parse_program(self) -> Program:
        span = self.cur().span
        fns: list[FnDecl] = []
        while not self.at_kind(TokKind.EOF):
            fns.append(self.parse_fn())
        return Program(tuple(fns), span)

    def parse_fn(self) -> FnDecl:
        start = self.expect("fn").span
        name_tok = self.advance()
        if name_tok.kind not in (TokKind.IDENT, TokKind.KW):
            raise ParseError("expected function name", name_tok.span)
        name = name_tok.text
        self.expect("(")
        params: list[Param] = []
        if not self.at(")"):
            params.append(self.parse_param())
            while self.at(","):
                self.advance()
                params.append(self.parse_param())
        self.expect(")")
        self.expect("->")
        ret = self.parse_type()
        uses: list[str] = []
        if self.at("uses"):
            self.advance()
            self.expect("{")
            if not self.at("}"):
                uses.append(self.advance().text)
                while self.at(","):
                    self.advance()
                    uses.append(self.advance().text)
            self.expect("}")
        requires: list[Expr] = []
        ensures: list[Expr] = []
        while self.at("requires", "ensures"):
            which = self.advance().text
            expr = self.parse_expr()
            if which == "requires":
                requires.append(expr)
            else:
                ensures.append(expr)
        body = self.parse_block()
        return FnDecl(
            name,
            tuple(params),
            ret,
            tuple(uses),
            tuple(requires),
            tuple(ensures),
            body,
            start,
        )

    def parse_param(self) -> Param:
        name = self.advance().text
        self.expect(":")
        return Param(name, self.parse_type())

    def parse_type(self) -> Type:
        t = self.cur()
        if t.text == "Int":
            self.advance()
            return TyInt()
        if t.text == "Bool":
            self.advance()
            return TyBool()
        if t.text == "Str":
            self.advance()
            return TyStr()
        if t.text == "Unit":
            self.advance()
            return TyUnit()
        if t.text == "Console":
            self.advance()
            return TyConsole()
        if t.text == "List":
            self.advance()
            self.expect("<")
            elem = self.parse_type()
            self.expect(">")
            return TyList(elem)
        if t.text == "Option":
            self.advance()
            self.expect("<")
            inner = self.parse_type()
            self.expect(">")
            return TyOption(inner)
        if t.text == "Result":
            self.advance()
            self.expect("<")
            ok = self.parse_type()
            self.expect(",")
            err = self.parse_type()
            self.expect(">")
            return TyResult(ok, err)
        if t.text == "{":
            self.advance()
            name = self.advance().text
            self.expect(":")
            self.expect("Int")
            self.expect("|")
            pred = self.parse_expr()
            self.expect("}")
            return TyRefine(name, TyInt(), pred)
        if t.kind == TokKind.TYPE_IDENT:
            return TyNamed(self.advance().text)
        raise ParseError(f"expected type, got {t.text!r}", t.span)

    def parse_block(self) -> Block:
        start = self.expect("{").span
        stmts: list = []
        result: Expr | None = None
        while not self.at("}"):
            if self.at("let"):
                stmts.append(self.parse_let())
            else:
                expr = self.parse_expr()
                if self.at(";"):
                    self.advance()
                    stmts.append(ExprStmt(expr, expr.span if hasattr(expr, "span") else start))
                else:
                    # last expression without semicolon is the block value
                    if not self.at("}"):
                        raise ParseError("expected ';' or '}' after expression", self.cur().span)
                    result = expr
                    break
        self.expect("}")
        return Block(tuple(stmts), result, start)

    def parse_let(self) -> LetStmt:
        start = self.expect("let").span
        name = self.advance().text
        ty = None
        if self.at(":"):
            self.advance()
            ty = self.parse_type()
        self.expect("=")
        value = self.parse_expr()
        self.expect(";")
        return LetStmt(name, ty, value, start)

    def parse_expr(self) -> Expr:
        return self.parse_or()

    def parse_or(self) -> Expr:
        left = self.parse_and()
        while self.at("||"):
            op = self.advance().text
            right = self.parse_and()
            left = BinOp(op, left, right, left.span)  # type: ignore[attr-defined]
        return left

    def parse_and(self) -> Expr:
        left = self.parse_cmp()
        while self.at("&&"):
            op = self.advance().text
            right = self.parse_cmp()
            left = BinOp(op, left, right, left.span)  # type: ignore[attr-defined]
        return left

    def parse_cmp(self) -> Expr:
        left = self.parse_add()
        if self.at("==", "!=", "<", "<=", ">", ">="):
            op = self.advance().text
            right = self.parse_add()
            return BinOp(op, left, right, left.span)  # type: ignore[attr-defined]
        return left

    def parse_add(self) -> Expr:
        left = self.parse_mul()
        while self.at("+", "-", "++"):
            op = self.advance().text
            right = self.parse_mul()
            left = BinOp(op, left, right, left.span)  # type: ignore[attr-defined]
        return left

    def parse_mul(self) -> Expr:
        left = self.parse_unary()
        while self.at("*", "/", "%"):
            op = self.advance().text
            right = self.parse_unary()
            left = BinOp(op, left, right, left.span)  # type: ignore[attr-defined]
        return left

    def parse_unary(self) -> Expr:
        if self.at("-", "!"):
            op_tok = self.advance()
            expr = self.parse_unary()
            return UnaryOp(op_tok.text, expr, op_tok.span)
        return self.parse_postfix()

    def parse_postfix(self) -> Expr:
        expr = self.parse_primary()
        while True:
            if self.at("("):
                span = self.advance().span
                args: list[Expr] = []
                if not self.at(")"):
                    args.append(self.parse_expr())
                    while self.at(","):
                        self.advance()
                        args.append(self.parse_expr())
                self.expect(")")
                expr = Call(expr, tuple(args), span)
            elif self.at("."):
                self.advance()
                field = self.advance().text
                expr = FieldAccess(expr, field, expr.span)  # type: ignore[attr-defined]
            else:
                break
        return expr

    def parse_primary(self) -> Expr:
        t = self.cur()
        if t.text == "if":
            return self.parse_if()
        if t.kind == TokKind.INT:
            self.advance()
            return LitInt(int(t.text), t.span)
        if t.kind == TokKind.STR:
            self.advance()
            return LitStr(t.text, t.span)
        if t.text == "true":
            self.advance()
            return LitBool(True, t.span)
        if t.text == "false":
            self.advance()
            return LitBool(False, t.span)
        if t.text == "unit":
            self.advance()
            return LitUnit(t.span)
        if t.text == "(":
            self.advance()
            e = self.parse_expr()
            self.expect(")")
            return e
        if t.text == "{":
            return self.parse_block()
        if t.kind in (TokKind.IDENT, TokKind.TYPE_IDENT, TokKind.KW) and t.text not in {
            "fn",
            "let",
            "if",
            "else",
            "uses",
            "requires",
            "ensures",
        }:
            # allow Console etc. as names in expressions rarely; idents only for values
            if t.kind == TokKind.IDENT or t.text in {"console"}:
                self.advance()
                return Name(t.text, t.span)
        if t.kind == TokKind.IDENT:
            self.advance()
            return Name(t.text, t.span)
        raise ParseError(f"unexpected token {t.text!r}", t.span)

    def parse_if(self) -> IfExpr:
        start = self.expect("if").span
        cond = self.parse_expr()
        then_b = self.parse_block()
        self.expect("else")
        if self.at("if"):
            else_inner = self.parse_if()
            else_b = Block((), else_inner, else_inner.span)
        else:
            else_b = self.parse_block()
        return IfExpr(cond, then_b, else_b, start)


def parse(source: str) -> Program:
    try:
        tokens = lex(source)
    except LexError as e:
        raise ParseError(str(e), e.span) from e
    p = Parser(tokens)
    prog = p.parse_program()
    if not p.at_kind(TokKind.EOF):
        raise ParseError(f"trailing token {p.cur().text!r}", p.cur().span)
    return prog
