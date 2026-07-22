#!/usr/bin/env bash
# T09 — fail_closed_binder (POSIX mirror of check.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- --prove "$src" 2>&1)"; code=$?
if [ "$code" -ne 1 ]; then echo "FAIL expected exit 1 got $code"; echo "$out"; exit 1; fi
case "$out" in *"error:"*) : ;; *) echo "FAIL missing error line"; echo "$out"; exit 1 ;; esac
case "$out" in *"[PROVED]"*) echo "FAIL forged PROVED present"; echo "$out"; exit 1 ;; *) : ;; esac
echo "PASS T09"
exit 0
