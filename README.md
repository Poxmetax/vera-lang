# VERA

**VERA** (Verified · Effect-typed · Reproducible · Agent-native — working name) is a research prototype of an **AI-native programming language**: a familiar, low-ceremony surface over a strict, machine-verified substrate — static types with no null, a unified effect/capability/taint label lattice, contracts and refinement types checked at runtime and SMT-proved when possible, a content-addressed codebase edited through typed transactions, and agentic runtime primitives (typed LLM inference, confidence gating, actors, quarantine/policy security).

The goal, in one line: **the language an LLM writes with the fewest shipped bugs — easy to write, impossible to write wrong silently.**

## Status

- **Phase -1 (thesis pilot): PASS, 2026-07-19.** See [`docs/pilot/REPORT.md`](docs/pilot/REPORT.md).
- **Phase 0 (research + design): delivered, 2026-07-19.** [`docs/research/RESEARCH_REPORT.md`](docs/research/RESEARCH_REPORT.md), [`docs/spec/SPEC.md`](docs/spec/SPEC.md) v0.1.
- **SMT spike (pilot R1 partial): PASS.** [`docs/pilot/SMT_SPIKE_REPORT.md`](docs/pilot/SMT_SPIKE_REPORT.md).
- **Phase 1 (Rust front-end + interpreter): CONF-P1 gate met (plan §9).** Spec §3 MVP language surface + plan acceptance:
  - `.vera` programs run (9 examples; +3 Phase 2 prove demos — see [`examples/README.md`](examples/README.md); `prove_refuted.vera` fails by design)
  - content-addressed store round-trip: `parse → hash → render → parse` identity (`--round-trip`, `cargo test`)
  - typed edit transactions (U16): stale-base reject + typecheck-gated commit (`EditTransaction`)
  - typed holes `?ident` parse (unfilled = type/runtime error; synthesis later S1)
  - postfix `?` Option/Result propagation (enclosing return type checked, `[P2-SOUND3]`)
  - **Phase 2 (thin VC slice): in progress.** Z3 via SMT-LIB2 subprocess; `vera --prove` discharges Int/bool/`ite` requires·ensures·`{x:Int|pred}` (see [`docs/pilot/PHASE2_VC_SLICE_REPORT.md`](docs/pilot/PHASE2_VC_SLICE_REPORT.md)). REQ-REFINE-1 call-site + closed definition-time return refine landed ([P2-REFINE1] / [P2-REFINE1-DEF]); structured pipeline diagnostics landed ([P2B-DIAG], `--diag-json`, [`docs/pilot/P2B_DIAG_SLICE.md`](docs/pilot/P2B_DIAG_SLICE.md)); full CONF-P2 / REQ-REFINE-2 still open. See docs/pilot/P2_REFINE1_SLICE.md. Review: [`CLAUDE_REVIEW_P2_REFINE1_DEF.md`](docs/pilot/CLAUDE_REVIEW_P2_REFINE1_DEF.md) (template: [`CLAUDE_REVIEW_PROMPT_TEMPLATE.md`](docs/pilot/CLAUDE_REVIEW_PROMPT_TEMPLATE.md)).
- **Phase 3 MCP stub (docs only):** [mcp/README.md](mcp/README.md) — planned typecheck/prove compiler-service surface (CONF-P3 / DP8). No server code yet.

### Remaining → Fable 5 (CONF-P2 hard work)

Hard CONF-P2 tasks are deferred to Claude Code **Fable 5**. Paste-ready brief:

- [`docs/pilot/FABLE5_CONF_P2_HANDOFF_PROMPT.md`](docs/pilot/FABLE5_CONF_P2_HANDOFF_PROMPT.md)
- Soft parallel track (non-CONF-P2 polish): [`docs/pilot/SOFT_PARALLEL_QUEUE.md`](docs/pilot/SOFT_PARALLEL_QUEUE.md)
- Examples index: [`examples/README.md`](examples/README.md)

Do **not** implement here without Madis + that handoff: prove→typecheck diagnostics (B), REQ-REFINE-2/`len` (C), check-elision / INV-1 (D), FixPatch JSON (E), labels/IFC, z3 crate, Salsa, hole synthesis.

## Quick start

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo run -p vera -- examples/hello.vera
cargo run -p vera -- examples/propagate.vera
cargo run -p vera -- --round-trip examples/hello.vera
cargo test -p vera
cargo run -p vera -- --prove examples/prove_clamp.vera
cargo run -p vera -- --prove examples/prove_runtime_hint.vera  # expect [RUNTIME-CHECKED]
cargo run -p vera -- --prove examples/prove_refuted.vera       # expect [REFUTED], exit 3
cargo run -p vera -- examples/refine_call_ok.vera              # in-range refined calls run
cargo run -p vera -- --prove --diag-json examples/prove_refuted.vera  # machine-readable diagnostics JSON
```

Optional flags: `--hash-only`, `--dump-ast`, `--prove` (Phase 2 VC discharge).

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
└── mcp/                             — Phase 3 stub: mcp/README.md (docs only)
```

## Isolation note

This project is standalone and unrelated to the TradingBot mainnet runtime it happens to share a workspace with. It must never import or be imported by any TradingBot runtime file.

## License

Planned: Apache-2.0 (research prototype; final at first public release).
