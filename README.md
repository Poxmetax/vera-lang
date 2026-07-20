# VERA

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rustc-stable-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-research%20prototype-lightgrey.svg)](#status)

**VERA** (Verified · Effect-typed · Reproducible · Agent-native) is an **AI-native research programming language**: a low-ceremony surface over a machine-checked substrate — static types (no null), refinement/contracts, typed holes, content-addressed edits, and a thin Z3 verification-condition (VC) path.

> **One-line goal:** the language an LLM writes with the fewest shipped bugs — easy to write, hard to ship silent wrongness.

Public repo: [github.com/Poxmetax/vera-lang](https://github.com/Poxmetax/vera-lang)

## What makes VERA different

| Pillar | What exists today (honest) |
|--------|----------------------------|
| **Typed holes** | `?ident` parses; unfilled holes are type/runtime errors (synthesis later) |
| **Refinements + prove** | Runtime checks + `vera --prove` / `--prove-run` via Z3 SMT-LIB2 subprocess |
| **Content-addressed store** | Parse → hash → render → parse round-trip; typed edit transactions |
| **FixPatch** | Ephemeral diagnostic/fix patch path (CONF-P2); **not** a durable proof cache |
| **Label lattice** | Lattice + thin surface + minimal `T^{untrusted\|secret}` at param/let (feeds seeded pass) — **not** full IFC; no inference |

This is a **research prototype**, not a production language. Do not read “pilot” as “information-flow control shipped.”

## Status

| Phase | State |
|-------|--------|
| Thesis pilot (−1) | **PASS** — [`docs/pilot/REPORT.md`](docs/pilot/REPORT.md) |
| Research + spec (0) | **Delivered** — [`docs/research/RESEARCH_REPORT.md`](docs/research/RESEARCH_REPORT.md), [`docs/spec/SPEC.md`](docs/spec/SPEC.md) |
| Rust front-end + interpreter (1) | **CONF-P1 met** — examples run; store round-trip; edit transactions |
| Thin VC + FixPatch (2) | **Toward CONF-P2** — Z3 prove path + ephemeral FixPatch JSON (`[P2E-FIX]`); **not** durable certs; labels ≠ IFC |
| MCP compiler service (3) | **Docs stub only** — [`mcp/README.md`](mcp/README.md) |

Debt register: [`docs/pilot/KNOWN_GAPS.md`](docs/pilot/KNOWN_GAPS.md). FixPatch slice: [`docs/pilot/P2E_FIXPATCH_SLICE.md`](docs/pilot/P2E_FIXPATCH_SLICE.md).

## Quick start

**Requirements:** Rust stable (`cargo`), and [Z3](https://github.com/Z3Prover/z3/releases) on `PATH` for `--prove` / `--prove-run`.

```bash
git clone https://github.com/Poxmetax/vera-lang.git
cd vera-lang

cargo run -p vera -- examples/hello.vera
cargo run -p vera -- examples/propagate.vera
cargo run -p vera -- --round-trip examples/hello.vera
cargo test -p vera

# Phase 2 VC demos (need z3 on PATH)
cargo run -p vera -- --prove examples/prove_clamp.vera
cargo run -p vera -- --prove-run examples/prove_clamp.vera   # prove then run; elides proved checks
cargo run -p vera -- --prove examples/prove_runtime_hint.vera  # expect [RUNTIME-CHECKED]
cargo run -p vera -- --prove examples/prove_refuted.vera       # expect [REFUTED], exit 3
```

Useful flags: `--hash-only`, `--dump-ast`, `--prove`, `--prove-run`, `--diag-json` (structured diagnostics; does not run the program).

More examples: [`examples/README.md`](examples/README.md)

## Docs

- Spec: [`docs/spec/SPEC.md`](docs/spec/SPEC.md)
- Pilot evidence: [`docs/pilot/REPORT.md`](docs/pilot/REPORT.md)
- SMT spike: [`docs/pilot/SMT_SPIKE_REPORT.md`](docs/pilot/SMT_SPIKE_REPORT.md)
- Known gaps: [`docs/pilot/KNOWN_GAPS.md`](docs/pilot/KNOWN_GAPS.md)
- Promo / GitHub visibility materials: deferred (removed from tree; re-add later if needed).

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). Security reports: [`SECURITY.md`](SECURITY.md).

Research-stage contributions welcome via [GitHub Issues](https://github.com/Poxmetax/vera-lang/issues) and PRs.

1. Prefer small, well-scoped changes with tests (`cargo test -p vera`).
2. Keep claims honest — see [`KNOWN_GAPS.md`](docs/pilot/KNOWN_GAPS.md) before documenting features.
3. Do not add TradingBot / mainnet dependencies; this repo is standalone.

## Repository layout

```
vera-lang/
├── Cargo.toml              — Rust workspace
├── crates/vera/            — reference toolchain (primary)
├── compiler/               — Python spike (reference only)
├── examples/               — .vera programs (must typecheck)
├── bench/                  — VeraAgentBench v0.1 (claim-less CLI probe)
├── docs/                   — research, spec, pilot evidence
└── mcp/                    — Phase 3 stub (docs only)
```

**Test baseline (soft):** `cargo test -p vera --lib` → **68** passed; `soft_smoke` PASS; `prove_clamp` → **6** proved. Debt: [`KNOWN_GAPS.md`](docs/pilot/KNOWN_GAPS.md).

## License

Apache-2.0 — see [`LICENSE`](LICENSE).
