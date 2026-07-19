"""Bucket 5 - Unhandled None / error paths. Idiomatic Python baseline.

Realistic task: look up a config value and use it. The natural code uses
``dict.get`` (returns ``None`` when missing) and then uses the result as if it
were always present. It passes ``mypy --strict`` ONLY because the code does not
annotate the hidden ``None`` path away - here we show the common form where the
``None`` reaches an operation and blows up / misbehaves at runtime.
"""
from __future__ import annotations


def get_timeout(config: dict[str, int], key: str) -> int:
    """BUG (null path): if ``key`` is missing, ``.get`` returns ``None`` and the
    arithmetic raises ``TypeError`` at runtime - an unhandled error path that a
    caller does not see in the signature (the ``int`` return type is a lie)."""
    value = config.get(key)  # Optional[int], silently
    return value * 2  # type: ignore[operator]  # explodes when value is None
