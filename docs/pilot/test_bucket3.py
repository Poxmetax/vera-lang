"""Bucket 3 tests - crypto misuse (CWE-327).

Pre-runtime catch is the ``mypy --strict`` error on
``bucket3_vera_violation.py`` (weak algorithm rejected by the ``ApprovedHash``
type). These runtime tests document that the baseline emits a raw unsalted MD5
digest while the VERA-style version emits a salted PBKDF2 digest.
"""
from __future__ import annotations

import hashlib

import bucket3_baseline as base
import bucket3_vera as vera


def test_baseline_uses_weak_md5() -> None:
    # The baseline output is exactly a bare MD5 hex digest (32 chars, unsalted).
    out = base.hash_password("hunter2")
    assert out == hashlib.md5(b"hunter2").hexdigest()
    assert len(out) == 32


def test_vera_uses_salted_kdf() -> None:
    out = vera.hash_password("hunter2")
    # salt:digest form, and it is NOT the raw md5.
    assert ":" in out
    assert out != hashlib.md5(b"hunter2").hexdigest()


def test_vera_salt_is_random() -> None:
    # Two calls must differ (random salt) - a property MD5 baseline can't have.
    assert vera.hash_password("hunter2") != vera.hash_password("hunter2")
