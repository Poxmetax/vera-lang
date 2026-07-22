"""Bucket 5 tests - unhandled None / error paths.

Pre-runtime catch is the ``mypy --strict`` error on
``bucket5_vera_violation.py`` (Result used as int). These runtime tests document
that the baseline explodes at runtime on the missing key while the VERA-style
version returns a handled ``Err``.
"""
from __future__ import annotations

import pytest

import bucket5_baseline as base
import bucket5_vera as vera
from vera_substrate import Err, Ok


def test_baseline_happy_path_runs() -> None:
    assert base.get_timeout({"timeout": 5}, "timeout") == 10


def test_baseline_explodes_on_missing_key() -> None:
    # The 'int' return type was a lie; missing key -> runtime TypeError.
    with pytest.raises(TypeError):
        base.get_timeout({}, "timeout")


def test_vera_returns_err_on_missing_key() -> None:
    result = vera.get_timeout({}, "timeout")
    assert isinstance(result, Err)


def test_vera_returns_ok_on_present_key() -> None:
    result = vera.get_timeout({"timeout": 5}, "timeout")
    assert isinstance(result, Ok)
    assert result.value == 10
