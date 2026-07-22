"""VERA-style substrate proxy (Phase -1 pilot).

Hand-rolled, dependency-light emulation of three VERA substrate features that
the plan's Section 1.1 / Section 8 describe as ONE unified label + explicit
error discipline:

  * Option[T] / Result[T, E]  -> no bare ``None`` leaks; error paths are an
                                 explicit variant the caller MUST handle.
  * Tainted[T] / Trusted[T]   -> IFC taint label: untrusted data cannot reach a
                                 privileged sink unless it passes through an
                                 explicit sanitizer (CaMeL "untrusted data can't
                                 reach a privileged sink").
  * Secret[T]                 -> confidential value that refuses to be logged or
                                 serialized; the only read path is .expose().

This module is deliberately tiny and MUST itself pass ``mypy --strict``. It is
self-contained and imports no external runtime.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Generic, NoReturn, TypeVar, Union

T = TypeVar("T")
D = TypeVar("D")
E = TypeVar("E")


# --------------------------------------------------------------- Option[T] ---
@dataclass(frozen=True)
class Some(Generic[T]):
    """The "value is present" variant of Option."""

    value: T

    def is_some(self) -> bool:
        return True

    def unwrap(self) -> T:
        return self.value

    def unwrap_or(self, default: T) -> T:
        return self.value


@dataclass(frozen=True)
class Nothing:
    """The "value is absent" variant of Option. Distinct type, not ``None``."""

    def is_some(self) -> bool:
        return False

    def unwrap(self) -> NoReturn:
        raise ValueError("called unwrap() on Nothing")

    def unwrap_or(self, default: D) -> D:
        return default


# Generic alias: an Option[T] is either Some[T] or the (untyped) Nothing.
Option = Union[Some[T], Nothing]


# --------------------------------------------------------------- Result[T,E] -
@dataclass(frozen=True)
class Ok(Generic[T]):
    value: T

    def is_ok(self) -> bool:
        return True

    def unwrap(self) -> T:
        return self.value


@dataclass(frozen=True)
class Err(Generic[E]):
    error: E

    def is_ok(self) -> bool:
        return False

    def unwrap(self) -> NoReturn:
        raise ValueError(f"called unwrap() on Err({self.error!r})")


Result = Union[Ok[T], Err[E]]


# --------------------------------------------------- Tainted[T] / Trusted[T] -
@dataclass(frozen=True)
class Tainted(Generic[T]):
    """Untrusted data (network / user / LLM output).

    It carries the payload but the type system forbids handing it directly to a
    privileged sink. The only escape is .reveal(), which is intentionally ugly
    so an auditor can grep for every raw-untrusted read.
    """

    _value: T

    def reveal(self) -> T:
        return self._value


@dataclass(frozen=True)
class Trusted(Generic[T]):
    """Data that has passed an explicit validation/sanitization step and is
    therefore authorized to reach a privileged sink."""

    _value: T

    def get(self) -> T:
        return self._value


# ---------------------------------------------------------------- Secret[T] --
@dataclass(frozen=True)
class Secret(Generic[T]):
    """A confidential value that refuses to leak.

    * ``str``/``repr`` are redacted, so it cannot leak via f-strings or logs.
    * It is not JSON-serializable (json.dumps raises TypeError by default).
    * The only read path is the explicit, audited .expose().
    """

    _value: T

    def __str__(self) -> str:
        return "Secret(***)"

    def __repr__(self) -> str:
        return "Secret(***)"

    def expose(self) -> T:
        return self._value
