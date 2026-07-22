"""Bucket 6 - Out-of-bounds indexing / integer bounds (CWE-787). VERA proxy.

Emulates VERA's refinement-type / contract bounds checking (plan U8 liquid types
"bounds, overflow", and ``requires`` contracts). The precondition
``0 <= offset < len(buffer)`` is declared with ``icontract``; any out-of-range
access - including the silently-wrong negative index - is rejected at the
contract boundary before the indexing runs. A ``hypothesis`` property proves the
returned element is genuinely the one at ``offset`` across the input space.
"""
from __future__ import annotations

import icontract


@icontract.require(lambda buffer, offset: 0 <= offset < len(buffer))
def read_at(buffer: list[int], offset: int) -> int:
    return buffer[offset]
