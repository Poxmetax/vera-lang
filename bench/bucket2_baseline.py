"""Bucket 2 - Injection, SQL (CWE-89). Idiomatic Python baseline.

Realistic task: look up a user row by name in a SQLite DB. The natural code an
LLM emits interpolates the name straight into the SQL string. It is fully typed
and passes ``mypy --strict``, runs fine on benign input, and is trivially
injectable - the bug ships silently.
"""
from __future__ import annotations

import sqlite3


def make_db() -> sqlite3.Connection:
    conn = sqlite3.connect(":memory:")
    conn.execute("CREATE TABLE users (name TEXT, role TEXT)")
    conn.execute("INSERT INTO users VALUES ('alice', 'user')")
    conn.execute("INSERT INTO users VALUES ('admin', 'admin')")
    conn.commit()
    return conn


def find_user(conn: sqlite3.Connection, name: str) -> list[tuple[str, str]]:
    """BUG (CWE-89): ``name`` is interpolated into the query, so
    ``find_user(conn, "zzz' OR '1'='1")`` returns every row."""
    query = f"SELECT name, role FROM users WHERE name = '{name}'"
    return list(conn.execute(query))
