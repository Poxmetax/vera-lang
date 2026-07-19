# VERA

**VERA** (Verified · Effect-typed · Reproducible · Agent-native — working name) is a research prototype of an **AI-native programming language**: a familiar, low-ceremony surface over a strict, machine-verified substrate — static types with no null, a unified effect/capability/taint label lattice, contracts and refinement types checked at runtime and SMT-proved when possible, a content-addressed codebase edited through typed transactions, and agentic runtime primitives (typed LLM inference, confidence gating, actors, quarantine/policy security).

The goal, in one line: **the language an LLM writes with the fewest shipped bugs — easy to write, impossible to write wrong silently.**

## Status

- **Phase -1 (thesis pilot): PASS, 2026-07-19.** See [`docs/pilot/REPORT.md`](docs/pilot/REPORT.md).
- **Phase 0 (research + design): delivered, 2026-07-19.** [`docs/research/RESEARCH_REPORT.md`](docs/research/RESEARCH_REPORT.md), [`docs/spec/SPEC.md`](docs/spec/SPEC.md) v0.1.
- **SMT spike (pilot R1 partial): PASS.** [`docs/pilot/SMT_SPIKE_REPORT.md`](docs/pilot/SMT_SPIKE_REPORT.md).
- **Phase 1 (Rust front-end + interpreter): CONF-P1 gate met (plan §9).** Spec §3 MVP language surface + plan acceptance:
  - `.vera` programs run (10 examples; +3 Phase 2 prove demos — see [`examples/README.md`](examples/README.md); `prove_refuted.vera` fails by design)
  - content-addressed store round-trip: `parse → hash → render → parse` identity (`--round-trip`, `cargo test`)
  - typed edit transactions (U16): stale-base reject + typecheck-gated commit (`EditTransaction`)
  - typed holes `?ident` parse (unfilled = type/runtime error; synthesis later S1)
  - postfix `?` Option/Result propagation (enclosing return type checked, `[P2-SOUND3]`)
  - **Phase 2 (thin VC slice): in progress.** Z3 via SMT-LIB2 subprocess; `vera --prove` / `--prove-run` ([P2D-ELIDE]). REQ-REFINE-1/2 + diagnostics + INV-1 elision landed; CONF-P2 still needs E FixPatch (+ labels / R2 later). Debt register: [`docs/pilot/KNOWN_GAPS.md`](docs/pilot/KNOWN_GAPS.md). SoT elision: [`P2D_ELISION_SLICE.md`](docs/pilot/P2D_ELISION_SLICE.md).
- **Phase 3 MCP stub (docs only):** [mcp/README.md](mcp/README.md) — planned typecheck/prove compiler-service surface (CONF-P3 / DP8). No server code yet.
- **Known gaps (debt register):** [`docs/pilot/KNOWN_GAPS.md`](docs/pilot/KNOWN_GAPS.md) — GAP-1..5 + deferred C/D limits; do not overclaim.

### Remaining -> Fable 5 (CONF-P2 hard work)

A+B+C+D + **gaps-before-E campaign complete (GAP-1..5)**. Soft smoke expects **50** tests. Sync: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](docs/pilot/CURSOR_SYNC_ACK_GAPS_BEFORE_E.md). Hashes: GAP-1 `5c98c75` · GAP-2 `c5222a8` · GAP-3 `226e33c` · GAP-4 `d4aebd3` · GAP-5 `23f2e46`. **Next:** Task E **GREEN-LIT by Madis (2026-07-20)** -- FixPatch implement.

- **Gaps before E:** campaign complete -- [`CLAUDE_POINTER_GAPS_BEFORE_E.md`](docs/pilot/CLAUDE_POINTER_GAPS_BEFORE_E.md) · [GAP-2](docs/pilot/CLAUDE_POINTER_GAP2_IMPLEMENT.md) · [GAP-3](docs/pilot/CLAUDE_POINTER_GAP3_IMPLEMENT.md) · [GAP-4](docs/pilot/CLAUDE_POINTER_GAP4_IMPLEMENT.md) · [GAP-5](docs/pilot/CLAUDE_POINTER_GAP5_IMPLEMENT.md) (all LANDED; GAP-4 = lattice-math evidence only, not IFC)
- **E (GREEN-LIT):** [`CLAUDE_POINTER_P2E_IMPLEMENT.md`](docs/pilot/CLAUDE_POINTER_P2E_IMPLEMENT.md) -- FixPatch stays EPHEMERAL until INV-2 keys wired ([`GAP5_INV2_DESIGN_NOTE.md`](docs/pilot/GAP5_INV2_DESIGN_NOTE.md)); do not claim IFC or durable proof cache

Do **not** soft-steal: FixPatch JSON / `diag.rs` while Fable owns E; labels/IFC overclaims; z3 crate; Salsa; hole synthesis. A-D + gaps already landed -- do not re-scaffold.

## Quick start

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo run -p vera -- examples/hello.vera
cargo run -p vera -- examples/propagate.vera
cargo run -p vera -- --round-trip examples/hello.vera
cargo test -p vera
cargo run -p vera -- --prove examples/prove_clamp.vera
cargo run -p vera -- --prove-run examples/prove_clamp.vera  # prove then run; elides proved checks
cargo run -p vera -- --prove examples/prove_runtime_hint.vera  # expect [RUNTIME-CHECKED]
cargo run -p vera -- --prove examples/prove_refuted.vera       # expect [REFUTED], exit 3
cargo run -p vera -- examples/refine_call_ok.vera              # in-range refined calls run
cargo run -p vera -- examples/refine_len_ok.vera               # [P2-REFINE2] len-measure nth
cargo run -p vera -- --prove --diag-json examples/prove_refuted.vera  # machine-readable diagnostics JSON
```

Optional flags: `--hash-only`, `--dump-ast`, `--prove` (VC discharge), `--prove-run` (prove then run with `[P2D-ELIDE]`), `--diag-json` (structured diagnostics; never runs the program -- wins over `--prove-run`).

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
