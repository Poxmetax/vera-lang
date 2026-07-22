"""Bucket 1 - Input validation (CWE-20). Idiomatic Python baseline.

Realistic task: apply a percentage discount to a price. This is fully type
annotated - exactly what a careful LLM emits - and it passes ``mypy --strict``.
Yet it does NO value validation, so out-of-range input silently produces a
nonsensical (negative) price. The point of this bucket: strict *types* alone do
not catch value-range bugs; you need contracts / refinement types.
"""
from __future__ import annotations


def apply_discount(price: float, pct: float) -> float:
    """Return ``price`` after a ``pct`` percent discount.

    BUG (CWE-20): ``pct`` is never checked to be within 0..100 and ``price`` is
    never checked to be non-negative, so ``apply_discount(100, 150)`` returns
    ``-50.0`` and ships silently.
    """
    return price * (1.0 - pct / 100.0)
