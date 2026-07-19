"""Bucket 5 - Unhandled None / error paths. VERA-style proxy.

Emulates VERA's no-null + ``Result`` discipline (plan Section 2 "no null, ADTs,
Result/Option"). The lookup returns ``Result[int, str]`` - never a bare
``None`` - so the caller cannot use the value without discharging the error
variant. Using the ``Result`` as if it were an ``int`` is a ``mypy --strict``
type error (see ``bucket5_vera_violation.py``): the missing-key path is caught
before runtime.
"""
from __future__ import annotations

from vera_substrate import Err, Ok, Result


def get_timeout(config: dict[str, int], key: str) -> Result[int, str]:
    if key not in config:
        return Err(f"missing config key: {key}")
    return Ok(config[key] * 2)
