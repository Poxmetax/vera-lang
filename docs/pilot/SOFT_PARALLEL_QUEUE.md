# Soft parallel queue (vs Fable 5 CONF-P2)

**Date:** 2026-07-19
**Purpose:** Split ownership so soft polish does not race Fable 5 on hard CONF-P2.

## Fable 5 owns (hard — do not soft-steal)

| ID | Task | Notes |
|----|------|-------|
| A | REQ-REFINE-1 hard typecheck reject | **done (closed fragment)** — call-site `[P2-REFINE1]` + def-time `[P2-REFINE1-DEF]` (2026-07-19); requires-guided/param bodies still soft |
| B | prove ↔ typecheck diagnostics | **done** — `[P2B-DIAG]` `--diag-json` + `diagnose_source`/`diagnose_program` (2026-07-19); FixPatch stays task E |
| C | REQ-REFINE-2 + `len` measures | List bounds |
| D | INV-1 check-elision | interpreter skips only when proved |
| E | FixPatch JSON diagnostics | machine-readable patches |

Handoff: [`FABLE5_CONF_P2_HANDOFF_PROMPT.md`](FABLE5_CONF_P2_HANDOFF_PROMPT.md).

**Claude review prompts:** template [CLAUDE_REVIEW_PROMPT_TEMPLATE.md](CLAUDE_REVIEW_PROMPT_TEMPLATE.md); P2-REFINE1 call-site [CLAUDE_REVIEW_P2_REFINE1.md](CLAUDE_REVIEW_P2_REFINE1.md); **current** def-time [CLAUDE_REVIEW_P2_REFINE1_DEF.md](CLAUDE_REVIEW_P2_REFINE1_DEF.md) (paste for review — not implementation).

**Do not edit while Fable owns:** `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs` (prefer leave `parser.rs` / `ast.rs` alone).

## Soft track (parallel-safe)

| Status | Item | Where |
|--------|------|-------|
| done | `--prove` help text | `crates/vera/src/main.rs` `[SOFT-PROVE-HELP]` |
| done | Z3 path print on `--prove` | `main.rs` `[SOFT-Z3-PATH]` |
| done | exit-code summary in usage | `main.rs` `[SOFT-EXIT-HELP]` |
| done | RUNTIME-CHECKED demo | `examples/prove_runtime_hint.vera` |
| done | this ownership queue | `docs/pilot/SOFT_PARALLEL_QUEUE.md` |
| done | examples index | `examples/README.md` |
| done | REFUTED soft demo (Int ensures) | `examples/prove_refuted.vera` — exit 3, no VC edit |
| done | soft smoke script | `docs/pilot/soft_smoke.ps1` |
| skip | anything needing VC encoder change | defer to Fable / Madis |
| done | Phase 3 MCP stub README | `mcp/README.md` `[SOFT-MCP-STUB]` — docs only, no server |

## Explicitly out of soft scope

REQ-REFINE-1/2, prove↔typecheck wiring, len measures, check-elision, FixPatch, labels/IFC, `z3` crate, Salsa, hole synthesis.

## Smoke (must stay green after soft edits)

```powershell
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib
cargo run -p vera -- --prove examples/prove_clamp.vera
# expect: ≥17 tests pass; prove_clamp → 6 proved
cargo run -p vera -- --prove examples/prove_runtime_hint.vera
# expect: ≥1 [RUNTIME-CHECKED]
cargo run -p vera -- --prove examples/prove_refuted.vera
# expect: [REFUTED], exit 3
# or: powershell -File docs/pilot/soft_smoke.ps1
```

## Soft track status (post-freeze)

**ACTIVE (post-freeze, no-rename rule).** Soft feature code queue exhausted; docs-only safe lane may continue (MCP stub done). Further soft *code* still needs Madis/Fable CONF-P2 clearance. Operator commit gate: [COMMIT_CHECKLIST.md](COMMIT_CHECKLIST.md) (run `soft_smoke.ps1` immediately before commit; never rename files during soft review).
