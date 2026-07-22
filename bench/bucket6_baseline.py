"""Bucket 6 - Out-of-bounds indexing / integer bounds (CWE-787). Baseline.

Realistic task: read the element at a computed offset in a buffer. The natural
code indexes directly. It is fully typed and passes ``mypy --strict`` (types
don't track list length), yet:
  * a NEGATIVE offset silently returns the wrong element (Python wrap-around) -
    a silent logic corruption, the CWE-787 analogue; and
  * an offset >= len raises ``IndexError`` at runtime.
Either way the bug is not caught before the code runs.
"""
from __future__ import annotations


def read_at(buffer: list[int], offset: int) -> int:
    """BUG (CWE-787 analogue): no bounds check. ``read_at([1,2,3], -1)`` returns
    ``3`` (silent wrong value); ``read_at([1,2,3], 9)`` raises at runtime."""
    return buffer[offset]
