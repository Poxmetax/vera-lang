"""Bucket 4 tests - hard-coded credentials (CWE-259).

Pre-runtime catch is the ``mypy --strict`` error on
``bucket4_vera_violation.py`` (bare literal -> Secret[str] sink). These runtime
tests document that the baseline leaks the secret into strings/JSON while the
VERA-style ``Secret`` refuses to.
"""
from __future__ import annotations

import json

import pytest

import bucket4_baseline as base
import bucket4_vera as vera
from vera_substrate import Secret


def test_baseline_leaks_into_status_and_json() -> None:
    assert "admin123" in base.connection_status()
    assert "admin123" in json.dumps(base.config_dict())


def test_vera_secret_redacts_in_str() -> None:
    s: Secret[str] = Secret("admin123")
    assert "admin123" not in str(s)
    assert "admin123" not in repr(s)
    assert "admin123" not in vera.connect(s)


def test_vera_secret_refuses_serialization() -> None:
    s: Secret[str] = Secret("admin123")
    with pytest.raises(TypeError):
        json.dumps({"password": s})  # type: ignore[dict-item]
