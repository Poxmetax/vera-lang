"""Bucket 4 - Hard-coded credentials (CWE-259). Idiomatic Python baseline.

Realistic task: build a DB connection string. The natural code hard-codes the
password as a module constant and then leaks it into a human-readable status
message / serialized config. Fully typed, passes ``mypy --strict``, runs fine -
and the secret is now in source control AND in logs.
"""
from __future__ import annotations

DB_PASSWORD = "admin123"  # BUG (CWE-259): hard-coded secret in source.


def connection_status() -> str:
    """Leaks the credential into a plaintext status string."""
    return f"connected to db with password={DB_PASSWORD}"


def config_dict() -> dict[str, str]:
    """Leaks the credential into a serializable structure (-> logs / JSON)."""
    return {"host": "db.internal", "password": DB_PASSWORD}
