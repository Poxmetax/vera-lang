"""Bucket 2 - the mypy-caught violation (pre-runtime catch evidence).

This file is INTENTIONALLY wrong: it hands untrusted ``Tainted[str]`` straight to
the privileged sink, which is exactly the SQL-injection bug. ``mypy --strict``
rejects it before it can ever run. Running mypy on this file is the recorded
evidence for bucket 2.
"""
from __future__ import annotations

import sqlite3

from bucket2_vera import query_by_name
from vera_substrate import Tainted


def handle_request(conn: sqlite3.Connection, user_input: Tainted[str]) -> list[tuple[str, str]]:
    # mypy --strict error: Tainted[str] is not assignable to Trusted[str].
    return query_by_name(conn, user_input)
