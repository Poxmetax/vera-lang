"""CLI: python -m compiler <file.vera> [--hash-only]

Run from the vera-lang/ directory so `compiler` is importable.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

from .interpreter import Interpreter, RuntimeError_
from .parser import ParseError, parse
from .store import CodebaseStore
from .typecheck import TypeError, check_program


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(prog="vera", description="VERA Phase 1 reference runner")
    ap.add_argument("file", type=Path, help=".vera source file")
    ap.add_argument("--hash-only", action="store_true", help="parse + content-hash, do not run")
    ap.add_argument("--dump-ast", action="store_true", help="print parsed function names")
    args = ap.parse_args(argv)

    source = args.file.read_text(encoding="utf-8")
    try:
        program = parse(source)
        check_program(program)
    except (ParseError, TypeError) as e:
        print(f"error: {e}", file=sys.stderr)
        return 1

    store = CodebaseStore()
    entries = store.load_program(program)
    print("content-addressed definitions:")
    for e in entries:
        print(f"  {e.name}  #{e.content_hash}")

    if args.dump_ast:
        print(json.dumps(store.summary(), indent=2))

    if args.hash_only:
        return 0

    try:
        Interpreter(program).run_main()
    except RuntimeError_ as e:
        print(f"trap: {e}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
