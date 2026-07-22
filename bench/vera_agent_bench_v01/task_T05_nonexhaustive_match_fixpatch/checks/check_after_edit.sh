#!/usr/bin/env bash
# T05 — run after agent edits initial/main.vera to be exhaustive
# (POSIX mirror of check_after_edit.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- "$src" 2>&1)"; code=$?
if [ "$code" -ne 0 ]; then echo "FAIL still errors"; echo "$out"; exit 1; fi
echo "PASS T05 after-edit"
exit 0
