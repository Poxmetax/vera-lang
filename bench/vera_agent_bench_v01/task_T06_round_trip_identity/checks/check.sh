#!/usr/bin/env bash
# T06' — round_trip_paren_identity (POSIX mirror of check.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- --round-trip "$src" 2>&1)"; code=$?
if [ "$code" -ne 0 ]; then echo "FAIL exit $code"; echo "$out"; exit 1; fi
case "$out" in *"round-trip OK"*) : ;; *) echo "FAIL missing round-trip OK"; echo "$out"; exit 1 ;; esac
echo "PASS T06"
exit 0
