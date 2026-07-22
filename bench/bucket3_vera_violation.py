"""Bucket 3 - the mypy-caught violation (pre-runtime catch evidence).

INTENTIONALLY wrong: asks for the broken ``"md5"`` algorithm. Because ``algo``
is typed ``ApprovedHash = Literal["sha256", "sha512"]``, ``mypy --strict``
rejects this before it can run. Running mypy on this file is the recorded
evidence for bucket 3.
"""
from __future__ import annotations

from bucket3_vera import hash_password


def store(password: str) -> str:
    # mypy --strict error: Literal["md5"] is not assignable to ApprovedHash.
    return hash_password(password, "md5")
