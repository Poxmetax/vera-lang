"""Bucket 2 - Injection, SQL (CWE-89). VERA-style proxy.

Emulates VERA's unified taint label + "untrusted data can't reach a privileged
sink" (CaMeL, plan Section 8). The DB sink ``query_by_name`` REQUIRES a
``Trusted[str]``; raw user input is ``Tainted[str]``. The only way to obtain a
``Trusted`` value is to pass through the explicit ``sanitize`` allowlist. Handing
tainted data straight to the sink is a ``mypy --strict`` type error - see
``bucket2_vera_violation.py``.

The sink also uses a parameterized query (defense in depth), matching what VERA
would compile the trusted-string interpolation into.
"""
from __future__ import annotations

import sqlite3

from vera_substrate import Nothing, Option, Some, Tainted, Trusted


def sanitize(raw: Tainted[str]) -> Option[Trusted[str]]:
    """Promote untrusted input to Trusted only if it passes an allowlist.
    Returns ``Nothing`` for anything that is not a plain identifier."""
    value = raw.reveal()
    if value.isalnum():
        return Some(Trusted(value))
    return Nothing()


def query_by_name(conn: sqlite3.Connection, name: Trusted[str]) -> list[tuple[str, str]]:
    """Privileged sink: only accepts Trusted data, and parameterizes anyway."""
    return list(
        conn.execute(
            "SELECT name, role FROM users WHERE name = ?",
            (name.get(),),
        )
    )
