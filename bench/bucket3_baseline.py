"""Bucket 3 - Crypto misuse (CWE-327). Idiomatic Python baseline.

Realistic task: hash a password for storage. The natural, familiar code reaches
for ``hashlib.md5`` (fast, ubiquitous in examples). It is fully typed, passes
``mypy --strict``, runs fine - and ships a broken-by-design weak hash with no
salt. CWE-327 (use of a broken/risky cryptographic algorithm).
"""
from __future__ import annotations

import hashlib


def hash_password(password: str) -> str:
    """BUG (CWE-327): MD5 is cryptographically broken and unsalted here."""
    return hashlib.md5(password.encode()).hexdigest()
