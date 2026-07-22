#!/usr/bin/env bash
# T05 — nonexhaustive_match_fixpatch, diag phase (POSIX mirror of check_diag.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- --diag-json "$src" 2>&1)"
case "$out" in *"add-match-arms"*) : ;; *) echo "FAIL missing add-match-arms"; echo "$out"; exit 1 ;; esac
if ! printf '%s' "$out" | grep -Eq '"ephemeral"[[:space:]]*:[[:space:]]*true'; then
  echo "FAIL ephemeral not true"; echo "$out"; exit 1
fi
echo "PASS T05 diag"
exit 0
