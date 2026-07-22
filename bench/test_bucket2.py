"""Bucket 2 tests - SQL injection (CWE-89).

The pre-runtime catch for this bucket is a ``mypy --strict`` error on
``bucket2_vera_violation.py`` (tainted data -> privileged sink). These runtime
tests document the behavioural difference: the baseline is injectable; the
VERA-style sink + sanitizer is not.
"""
from __future__ import annotations

import bucket2_baseline as base
import bucket2_vera as vera
from vera_substrate import Some, Tainted


def test_baseline_is_injectable() -> None:
    conn = base.make_db()
    # Classic tautology injection returns EVERY row, including admin.
    rows = base.find_user(conn, "zzz' OR '1'='1")
    assert ("admin", "admin") in rows
    assert len(rows) == 2


def test_vera_sanitizer_blocks_injection() -> None:
    conn = base.make_db()
    tainted = Tainted("zzz' OR '1'='1")
    promoted = vera.sanitize(tainted)
    # The injection payload fails the allowlist -> never becomes Trusted.
    assert not isinstance(promoted, Some)


def test_vera_allows_benign_input() -> None:
    conn = base.make_db()
    promoted = vera.sanitize(Tainted("admin"))
    assert isinstance(promoted, Some)
    rows = vera.query_by_name(conn, promoted.value)
    assert rows == [("admin", "admin")]
