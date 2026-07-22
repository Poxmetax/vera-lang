"""Bucket 6 tests - out-of-bounds indexing (CWE-787).

The pre-runtime catch is the ``icontract`` precondition + a ``hypothesis``
property. The baseline both silently returns a wrong value on a negative offset
AND raises on a too-large offset; the VERA-style version rejects both at the
contract boundary.
"""
from __future__ import annotations

import icontract
import pytest
from hypothesis import given, settings
from hypothesis import strategies as st

import bucket6_baseline as base
import bucket6_vera as vera


def test_baseline_negative_offset_silently_wrong() -> None:
    # No error - returns the LAST element instead of failing. Silent corruption.
    assert base.read_at([10, 20, 30], -1) == 30


def test_baseline_large_offset_raises_at_runtime() -> None:
    with pytest.raises(IndexError):
        base.read_at([10, 20, 30], 9)


def test_vera_rejects_negative_offset() -> None:
    with pytest.raises(icontract.errors.ViolationError):
        vera.read_at([10, 20, 30], -1)


def test_vera_rejects_out_of_range_offset() -> None:
    with pytest.raises(icontract.errors.ViolationError):
        vera.read_at([10, 20, 30], 9)


@settings(max_examples=100)
@given(data=st.data())
def test_vera_property_returns_correct_element(data: st.DataObject) -> None:
    buffer = data.draw(st.lists(st.integers(), min_size=1, max_size=50))
    offset = data.draw(st.integers(min_value=0, max_value=len(buffer) - 1))
    assert vera.read_at(buffer, offset) == buffer[offset]


@pytest.mark.xfail(strict=True, reason="baseline accepts negative offset; property catches it")
@settings(max_examples=200)
@given(data=st.data())
def test_baseline_property_fails_on_negative(data: st.DataObject) -> None:
    buffer = data.draw(st.lists(st.integers(), min_size=1, max_size=10))
    offset = data.draw(st.integers(min_value=-len(buffer), max_value=-1))
    # A bounds-checked read_at MUST reject a negative (out-of-bounds) offset.
    # EXPECTED to fail: the baseline silently wraps around instead of raising.
    with pytest.raises(IndexError):
        base.read_at(buffer, offset)
