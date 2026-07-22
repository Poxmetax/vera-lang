#!/usr/bin/env bash
# T03 — prove_refuted_false_ensures (POSIX mirror of check.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- --prove "$src" 2>&1)"; code=$?
if [ "$code" -ne 3 ]; then echo "FAIL expected exit 3 got $code"; echo "$out"; exit 1; fi
case "$out" in *"[REFUTED]"*) : ;; *) echo "FAIL missing REFUTED"; echo "$out"; exit 1 ;; esac
echo "PASS T03"
exit 0
