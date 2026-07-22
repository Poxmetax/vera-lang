#!/usr/bin/env bash
# T04 — prove_runtime_checked_str (POSIX mirror of check.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- --prove "$src" 2>&1)"; code=$?
if [ "$code" -ne 0 ]; then echo "FAIL exit $code"; echo "$out"; exit 1; fi
case "$out" in *"[RUNTIME-CHECKED]"*) : ;; *) echo "FAIL missing RUNTIME-CHECKED"; echo "$out"; exit 1 ;; esac
echo "PASS T04"
exit 0
