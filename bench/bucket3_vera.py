"""Bucket 3 - Crypto misuse (CWE-327). VERA-style proxy.

Emulates VERA's "reuse vetted crypto, never roll your own" (plan Section 8,
SEC5) by making the ALGORITHM CHOICE a type. Only members of the approved set
``ApprovedHash`` are assignable to the ``algo`` parameter; a weak algorithm like
``"md5"`` is a ``mypy --strict`` type error (see ``bucket3_vera_violation.py``),
so the misuse is caught before runtime.

The implementation also uses a salted PBKDF2-HMAC KDF (strong + salted),
contract-guaranteeing a non-empty digest - the correct behaviour VERA's stdlib
would provide.
"""
from __future__ import annotations

import hashlib
import os
from typing import Literal

import icontract

# The unified capability/label idea, narrowed to "which primitives are vetted".
ApprovedHash = Literal["sha256", "sha512"]


@icontract.ensure(lambda result: len(result) > 0)
def hash_password(password: str, algo: ApprovedHash = "sha256") -> str:
    """Salted PBKDF2-HMAC over an APPROVED hash only.

    ``algo`` is typed as ``ApprovedHash``; passing a broken algorithm does not
    type-check under ``mypy --strict``."""
    salt = os.urandom(16)
    digest = hashlib.pbkdf2_hmac(algo, password.encode(), salt, 200_000)
    return salt.hex() + ":" + digest.hex()
