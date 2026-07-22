"""VERA MVP lexer (Phase 1)."""
from __future__ import annotations

from dataclasses import dataclass
from enum import Enum, auto
from typing import Iterator

from .ast_nodes import Span


class TokKind(Enum):
    IDENT = auto()
    TYPE_IDENT = auto()
    INT = auto()
    STR = auto()
    KW = auto()
    OP = auto()
    PUNCT = auto()
    EOF = auto()


KEYWORDS = {
    "fn",
    "let",
    "if",
    "else",
    "uses",
    "requires",
    "ensures",
    "true",
    "false",
    "unit",
    "Int",
    "Bool",
    "Str",
    "Unit",
    "Console",
    "List",
    "Option",
    "Result",
}


@dataclass(frozen=True)
class Token:
    kind: TokKind
    text: str
    span: Span


class LexError(Exception):
    def __init__(self, message: str, span: Span) -> None:
        super().__init__(f"{span.line}:{span.col}: {message}")
        self.span = span


def lex(source: str) -> list[Token]:
    i = 0
    line = 1
    col = 1
    n = len(source)
    out: list[Token] = []

    def peek(k: int = 0) -> str:
        j = i + k
        return source[j] if j < n else ""

    def advance() -> str:
        nonlocal i, line, col
        ch = source[i]
        i += 1
        if ch == "\n":
            line += 1
            col = 1
        else:
            col += 1
        return ch

    def span_here() -> Span:
        return Span(line, col)

    while i < n:
        ch = peek()
        if ch in " \t\r\n":
            advance()
            continue
        if ch == "/" and peek(1) == "/":
            while i < n and peek() != "\n":
                advance()
            continue

        start = span_here()

        if ch.isalpha() or ch == "_":
            text = ""
            while i < n and (peek().isalnum() or peek() == "_"):
                text += advance()
            if text in KEYWORDS:
                # Built-in type names are keywords; value keywords too.
                if text[0].isupper() and text not in {"true", "false", "unit"}:
                    out.append(Token(TokKind.KW, text, start))
                elif text in {"true", "false", "unit", "fn", "let", "if", "else", "uses", "requires", "ensures"}:
                    out.append(Token(TokKind.KW, text, start))
                else:
                    out.append(Token(TokKind.KW, text, start))
            elif text[0].isupper():
                out.append(Token(TokKind.TYPE_IDENT, text, start))
            else:
                out.append(Token(TokKind.IDENT, text, start))
            continue

        if ch.isdigit():
            text = ""
            while i < n and (peek().isdigit() or peek() == "_"):
                text += advance()
            out.append(Token(TokKind.INT, text.replace("_", ""), start))
            continue

        if ch == '"':
            advance()
            text = ""
            while i < n and peek() != '"':
                if peek() == "\\":
                    advance()
                    esc = advance()
                    mapping = {"n": "\n", "t": "\t", '"': '"', "\\": "\\"}
                    if esc not in mapping:
                        raise LexError(f"unknown escape \\{esc}", start)
                    text += mapping[esc]
                else:
                    text += advance()
            if peek() != '"':
                raise LexError("unterminated string", start)
            advance()
            out.append(Token(TokKind.STR, text, start))
            continue

        two = ch + peek(1)
        if two in {"==", "!=", "<=", ">=", "||", "&&", "->", "++"}:
            advance()
            advance()
            out.append(Token(TokKind.OP, two, start))
            continue

        if ch in "+-*/%<>=!":
            out.append(Token(TokKind.OP, advance(), start))
            continue

        if ch in "(){},;:|.":
            out.append(Token(TokKind.PUNCT, advance(), start))
            continue

        raise LexError(f"unexpected character {ch!r}", start)

    out.append(Token(TokKind.EOF, "", Span(line, col)))
    return out
