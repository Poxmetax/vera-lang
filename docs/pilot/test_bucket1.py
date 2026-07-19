"""Bucket 1 tests - input validation (CWE-20).

Green-suite strategy (used across all buckets):
  * ``test_baseline_ships_bug``     documents that (a) runs and returns the bad
                                    value with no error.
  * ``test_vera_contract_catches``  documents that (b) rejects the bad input at
                                    the contract boundary *before* the buggy
                                    arithmetic runs.
  * ``test_vera_property_holds``    hypothesis proof that (b) upholds the
                                    invariant across the whole valid input space.
  * an ``xfail(strict=True)`` hypothesis test proves the property FAILS on the
                                    baseline logic (i.e. the property catches the
                                    bug) while keeping the suite green.
"""
from __future__ import annotations

import icontract
import pytest
from hypothesis import given, settings
from hypothesis import strategies as st

import bucket1_baseline as base
import bucket1_vera as vera


def test_baseline_ships_bug() -> None:
    # (a) silently produces a negative price for an out-of-range discount.
    assert base.apply_discount(100.0, 150.0) == -50.0


def test_vera_contract_catches() -> None:
    # (b) rejects the same input at the @require boundary, before runtime logic.
    with pytest.raises(icontract.errors.ViolationError):
        vera.apply_discount(100.0, 150.0)


@settings(max_examples=100)
@given(
    price=st.floats(min_value=0.0, max_value=1e9),
    pct=st.floats(min_value=0.0, max_value=100.0),
)
def test_vera_property_holds(price: float, pct: float) -> None:
    # For every valid input, the discounted price is in [0, price].
    result = vera.apply_discount(price, pct)
    assert 0.0 <= result <= price + 1e-6


@pytest.mark.xfail(strict=True, reason="baseline violates 'result >= 0'; property catches it")
@settings(max_examples=200)
@given(
    price=st.floats(min_value=0.0, max_value=1e9),
    pct=st.floats(min_value=0.0, max_value=1000.0),
)
def test_baseline_property_fails(price: float, pct: float) -> None:
    # This is EXPECTED to fail: hypothesis finds pct>100 -> negative result.
    assert base.apply_discount(price, pct) >= 0.0
