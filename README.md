# VERA

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rustc-stable-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-research%20prototype-lightgrey.svg)](#status)

**VERA** (Verified, Effect-typed, Reproducible, Agent-native) is an **AI-native research programming language**: a low-ceremony surface over a machine-checked substrate -- static types (no null), refinement/contracts, typed holes, content-addressed edits, and a thin Z3 verification-condition (VC) path.

> **One-line goal:** the language an LLM writes with the fewest shipped bugs -- easy to write, hard to ship silent wrongness.

Public repo: [github.com/Poxmetax/vera-lang](https://github.com/Poxmetax/vera-lang)

## What makes VERA different

| Pillar | What exists today (honest) |
|--------|----------------------------|
| **Typed holes** | `?ident` parses; unfilled holes are type/runtime errors (synthesis later) |
| **Refinements + prove** | Runtime checks + `vera --prove` / `--prove-run` via Z3 SMT-LIB2 subprocess |
| **Content-addressed store** | Parse -> hash -> render -> parse round-trip; typed edit transactions |
| **FixPatch** | Live diagnostics stay `ephemeral: true`; durable copies exist only behind an integrity gate (content key + target hash); CLI reconcile via `--prove-cache` (re-prove-and-compare); opt-in `--prove-cache-skip` (skip Z3 on an exact cache hit -- store trusted under the flag; compare mode stays the tamper canary); opt-in `--prove-cache-prune` (explicit stale-toolchain prune only) -- **not** a general speed cache / **not** a production cert DB / **not** MCP |
| **Label lattice** | Lattice + thin surface + `T^{untrusted\|secret}` at param/let + intra-body inference -- **not** full IFC; **no** interprocedural inference; full CONF-P2 **not** closed |

This is a **research prototype**, not a production language. Do not read "pilot" as "information-flow control shipped."

## Status

| Phase | State |
|-------|--------|
| Thesis pilot (-1) | **PASS** -- [`bench/REPORT.md`](bench/REPORT.md) |
| Research + spec (0) | **Delivered** -- [`docs/research/RESEARCH_REPORT.md`](docs/research/RESEARCH_REPORT.md), [`docs/spec/SPEC.md`](docs/spec/SPEC.md) |
| Rust front-end + interpreter (1) | **CONF-P1 met** -- examples run; store round-trip; edit transactions |
| Thin VC + FixPatch (2) | **Toward CONF-P2** -- Z3 prove path + ephemeral FixPatch JSON; **not** durable certs by default; labels: intra-body inference landed, **not** full IFC; full CONF-P2 **not** closed |
| MCP compiler service (3) | **Docs stub only** -- [`mcp/README.md`](mcp/README.md) |

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
cargo run -p vera -- --prove --prove-cache .vera-cache examples/prove_clamp.vera  # reconcile durable INV-2 cert store (re-prove-and-compare; never changes prove results)
cargo run -p vera -- --prove --prove-cache .vera-cache --prove-cache-skip examples/prove_clamp.vera  # opt-in: skip Z3 on exact cache HIT (fail-closed; store trusted under flag; compare mode = tamper canary)
cargo run -p vera -- --prove examples/prove_runtime_hint.vera  # expect [RUNTIME-CHECKED]
cargo run -p vera -- --prove examples/prove_refuted.vera       # expect [REFUTED], exit 3
```

Useful flags: `--hash-only`, `--dump-ast`, `--prove`, `--prove-run`, `--prove-cache <dir>` (with `--prove`: reconcile durable cert store -- re-prove-and-compare; **not** a general speed cache; never changes prove results), `--prove-cache-skip` (with `--prove --prove-cache <dir>`: opt-in skip Z3 on an exact cert HIT -- fail-closed; store is trusted under this flag; compare mode stays the tamper canary), `--prove-cache-prune` (with `--prove --prove-cache <dir>`: opt-in explicit stale-toolchain prune -- **not** MCP), `--diag-json` (structured diagnostics; does not run the program).

More examples: [`examples/README.md`](examples/README.md)

## Docs

- Spec: [`docs/spec/SPEC.md`](docs/spec/SPEC.md)
- Pilot evidence: [`bench/REPORT.md`](bench/REPORT.md)
- SMT spike: [`bench/SMT_SPIKE_REPORT.md`](bench/SMT_SPIKE_REPORT.md)
- Trial pack: [`bench/README.md`](bench/README.md)

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). Security reports: [`SECURITY.md`](SECURITY.md).

Research-stage contributions welcome via [GitHub Issues](https://github.com/Poxmetax/vera-lang/issues) and PRs.

1. Prefer small, well-scoped changes with tests (`cargo test -p vera`).
2. Keep claims honest -- the triad (PROVED / REFUTED / RUNTIME-CHECKED) is load-bearing; never present a runtime-checked or refuted result as proved.
3. This repository is standalone research OSS; do not add mainnet trading dependencies or unrelated private paths.

## Repository layout

```
Cargo.toml              -- Rust workspace
crates/vera/            -- reference toolchain (primary)
compiler/               -- Python spike (reference only)
examples/               -- .vera programs (must typecheck)
bench/                  -- VeraAgentBench + claim-less evidence
docs/                   -- research, spec
mcp/                    -- Phase 3 stub (docs only)
.github/                -- CI, Dependabot, issue templates
```

**Test suite:** run `cargo test -p vera`. Claim-less evidence and the agent trial pack live under [`bench/`](bench/).

## License

Apache-2.0 -- see [`LICENSE`](LICENSE).
