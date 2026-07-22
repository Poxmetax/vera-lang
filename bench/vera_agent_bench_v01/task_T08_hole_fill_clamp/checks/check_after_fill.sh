#!/usr/bin/env bash
# T08 — run after agent fills the typed hole by editing source
# (POSIX mirror of check_after_fill.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- "$src" 2>&1)"; code=$?
if [ "$code" -ne 0 ]; then echo "FAIL still errors"; echo "$out"; exit 1; fi
echo "PASS T08 after-fill"
exit 0
