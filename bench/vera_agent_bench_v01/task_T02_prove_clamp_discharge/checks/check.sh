#!/usr/bin/env bash
# T02 — prove_clamp_discharge (POSIX mirror of check.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- --prove "$src" 2>&1)"; code=$?
if [ "$code" -ne 0 ]; then echo "FAIL exit $code"; echo "$out"; exit 1; fi
case "$out" in *"summary: 6 proved"*) : ;; *) echo "FAIL expected 6 proved"; echo "$out"; exit 1 ;; esac
echo "PASS T02"
exit 0
