"""Content-addressed definition store (Phase 1)."""
from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass
from typing import Any

from .ast_nodes import FnDecl, Program, ast_to_canonical


@dataclass(frozen=True)
class DefEntry:
    name: str
    content_hash: str
    node: FnDecl


class CodebaseStore:
    """Maps names → content-hashed definitions. Codebase is never 'broken':
    only fully parsed FnDecls are inserted."""

    def __init__(self) -> None:
        self._by_hash: dict[str, DefEntry] = {}
        self._by_name: dict[str, str] = {}  # name -> hash

    @staticmethod
    def hash_def(fn: FnDecl) -> str:
        payload = json.dumps(ast_to_canonical(fn), sort_keys=True, separators=(",", ":"))
        return hashlib.sha256(payload.encode("utf-8")).hexdigest()[:16]

    def insert(self, fn: FnDecl) -> DefEntry:
        h = self.hash_def(fn)
        entry = DefEntry(fn.name, h, fn)
        self._by_hash[h] = entry
        self._by_name[fn.name] = h
        return entry

    def load_program(self, program: Program) -> list[DefEntry]:
        return [self.insert(fn) for fn in program.functions]

    def get(self, name: str) -> DefEntry | None:
        h = self._by_name.get(name)
        return self._by_hash[h] if h else None

    def summary(self) -> list[dict[str, Any]]:
        return [
            {"name": e.name, "hash": e.content_hash}
            for e in (self._by_hash[h] for h in self._by_name.values())
        ]
