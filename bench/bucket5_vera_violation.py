"""Bucket 5 - the mypy-caught violation (pre-runtime catch evidence).

INTENTIONALLY wrong: uses the ``Result[int, str]`` as if it were an ``int``,
skipping the error variant. ``mypy --strict`` rejects the arithmetic (the Union
has no ``*`` operator) before runtime. Running mypy on this file is the recorded
evidence for bucket 5.
"""
from __future__ import annotations

from bucket5_vera import get_timeout


def double_timeout(config: dict[str, int]) -> int:
    # mypy --strict error: unsupported operand type(s) for * (Result has no __mul__).
    return get_timeout(config, "timeout") * 2
