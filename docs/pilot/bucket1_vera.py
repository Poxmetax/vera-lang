"""Bucket 1 - Input validation (CWE-20). VERA-style proxy.

Same task, but the precondition (0 <= pct <= 100, price >= 0) and the
postcondition (result >= 0) are declared as ``icontract`` contracts. This is the
proxy for VERA's ``requires``/``ensures`` (Section 1.1). Out-of-range input is
rejected at the contract boundary *before* the buggy arithmetic runs, and the
``hypothesis`` property test exercises the whole input space at test time -
i.e. before the code would ever ship.

Real VERA would additionally catch this statically via an inferred refinement
type ``{x: Int | 0 <= x <= 100}`` (plan U8, liquid types); the proxy has no
refinement-type checker, so the pre-runtime catch here is contract + property.
"""
from __future__ import annotations

import icontract


@icontract.require(lambda price: price >= 0.0)
@icontract.require(lambda pct: 0.0 <= pct <= 100.0)
@icontract.ensure(lambda result: result >= 0.0)
def apply_discount(price: float, pct: float) -> float:
    """Return ``price`` after a ``pct`` percent discount, with the bounds and
    the non-negative-result guarantee enforced by contracts."""
    return price * (1.0 - pct / 100.0)
