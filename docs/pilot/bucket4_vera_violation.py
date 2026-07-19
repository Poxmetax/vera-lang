"""Bucket 4 - the mypy-caught violation (pre-runtime catch evidence).

INTENTIONALLY wrong: passes a hard-coded plaintext literal to ``connect``, which
requires ``Secret[str]``. ``mypy --strict`` rejects it before runtime. Running
mypy on this file is the recorded evidence for bucket 4.
"""
from __future__ import annotations

from bucket4_vera import connect


def open_conn() -> str:
    # mypy --strict error: str is not assignable to Secret[str].
    return connect("admin123")
