"""Bucket 4 - Hard-coded credentials (CWE-259). VERA-style proxy.

Emulates VERA's ``Secret<T>`` (plan Section 8, SEC5: non-loggable,
non-serializable, debug-redacted) plus loading credentials via a capability
(env var) instead of hard-coding.

Two layers of protection:
  * PRE-RUNTIME (mypy --strict): the ``connect`` sink REQUIRES ``Secret[str]``.
    Passing a bare plaintext literal - the hard-coded-credential pattern - is a
    type error (see ``bucket4_vera_violation.py``).
  * RUNTIME/TEST: even once wrapped, a ``Secret`` refuses to leak - ``str`` is
    redacted and ``json.dumps`` raises, so it can't reach logs or serialized
    output.
"""
from __future__ import annotations

import os

from vera_substrate import Nothing, Option, Secret, Some


def load_secret(env_var: str) -> Option[Secret[str]]:
    """Load a credential from the environment (a capability), not from source."""
    raw = os.environ.get(env_var)
    if raw is None:
        return Nothing()
    return Some(Secret(raw))


def connect(password: Secret[str]) -> str:
    """Privileged sink: requires a managed Secret. Uses .expose() internally
    only; the returned status never contains the raw secret."""
    _ = password.expose()  # used to authenticate; never logged
    return f"connected to db with password={password}"  # -> 'Secret(***)'
