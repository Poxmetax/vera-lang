#!/usr/bin/env bash
# T08 — unfilled hole must reject (POSIX mirror of check_initial_rejects.ps1)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
src="$(cd "$here/.." && pwd)/initial/main.vera"
cd "$repo"
out="$(cargo run -p vera -- "$src" 2>&1)"; code=$?
if [ "$code" -eq 0 ]; then echo "FAIL expected unfilled hole to reject"; echo "$out"; exit 1; fi
echo "PASS T08 initial-reject"
exit 0
