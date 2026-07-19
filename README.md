# VERA

**VERA** (Verified · Effect-typed · Reproducible · Agent-native — working name) is a research prototype of an **AI-native programming language**: a familiar, low-ceremony surface over a strict, machine-verified substrate — static types with no null, a unified effect/capability/taint label lattice, contracts and refinement types checked at runtime and SMT-proved when possible, a content-addressed codebase edited through typed transactions, and agentic runtime primitives (typed LLM inference, confidence gating, actors, quarantine/policy security).

The goal, in one line: **the language an LLM writes with the fewest shipped bugs — easy to write, impossible to write wrong silently.**

## Status

- **Phase -1 (thesis pilot): PASS, 2026-07-19.** See [`docs/pilot/REPORT.md`](docs/pilot/REPORT.md).
- **Phase 0 (research + design): delivered, 2026-07-19.** [`docs/research/RESEARCH_REPORT.md`](docs/research/RESEARCH_REPORT.md), [`docs/spec/SPEC.md`](docs/spec/SPEC.md) v0.1.
- **SMT spike (pilot R1 partial): PASS.** [`docs/pilot/SMT_SPIKE_REPORT.md`](docs/pilot/SMT_SPIKE_REPORT.md) (Python `z3-solver`; native Z3 CLI also present for Phase 2).
- **Phase 1 (Rust front-end + interpreter): in progress.** Workspace `crates/vera` — lexer, parser, BLAKE3 store, typecheck, interpreter. Supported MVP slice: Int/Bool/Str/Unit/Console, `Option`/`Result`, user `struct`/`enum`, `match` (exhaustive for Option/Result/enums), `requires`/`ensures` (runtime), content-addressed defs.

## Quick start

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo run -p vera -- examples/hello.vera
cargo run -p vera -- examples/clamp.vera
cargo run -p vera -- examples/option_result.vera
cargo run -p vera -- examples/adt.vera
```

Optional flags: `--hash-only`, `--dump-ast`.

### Native Z3 (Phase 2; already unpacked)

Binary: `C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin\z3.exe`

Add to User PATH (new terminal after):

```powershell
[Environment]::SetEnvironmentVariable(
  "Path",
  $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin",
  "User"
)
z3 --version   # expect: Z3 version 4.16.0
```

## Repository layout

```
vera-lang/
├── Cargo.toml                       — Rust workspace
├── crates/vera/                     — Phase 1 reference toolchain (primary)
├── compiler/                        — Python spike (reference only; not primary)
├── examples/                        — .vera programs
├── docs/
│   ├── research/RESEARCH_REPORT.md
│   ├── spec/SPEC.md
│   └── pilot/                       — Phase -1 + SMT spike evidence
└── mcp/                             — (future, Phase 3)
```

## Isolation note

This project is standalone and unrelated to the TradingBot mainnet runtime it happens to share a workspace with. It must never import or be imported by any TradingBot runtime file.

## License

Planned: Apache-2.0 (research prototype; final at first public release).
