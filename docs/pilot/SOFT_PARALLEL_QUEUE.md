# Soft parallel queue (vs Fable 5 CONF-P2)

**Date:** 2026-07-20
**Purpose:** Split ownership so soft polish does not race Fable 5 on hard CONF-P2.

## Fable 5 owns (hard — do not soft-steal)

| ID | Task | Notes |
|----|------|-------|
| A | REQ-REFINE-1 hard typecheck reject | **done (closed fragment)** — call-site `[P2-REFINE1]` + def-time `[P2-REFINE1-DEF]` (2026-07-19); requires-guided/param bodies still soft |
| B | prove ↔ typecheck diagnostics | **done** — `[P2B-DIAG]` `--diag-json` + `diagnose_source`/`diagnose_program` (2026-07-19); FixPatch stays task E |
| C | REQ-REFINE-2 + `len` measures | **done + review PASS** `976231b` `[P2-REFINE2]` -- SoT [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md) |
| D | INV-1 check-elision | **done + review PASS** `77f7077` `[P2D-ELIDE]` -- SoT [`P2D_ELISION_SLICE.md`](P2D_ELISION_SLICE.md) |
| E | FixPatch JSON diagnostics | **CLOSED** `[P2E-FIX]` commit `ddc3d6a` (pushed; publish merge `3c72ce4`) -- SoT [`P2E_FIXPATCH_SLICE.md`](P2E_FIXPATCH_SLICE.md); soft review PASS [`CLAUDE_REVIEW_P2E_FIXPATCH.md`](CLAUDE_REVIEW_P2E_FIXPATCH.md) / ACK [`CURSOR_SYNC_ACK_P2E.md`](CURSOR_SYNC_ACK_P2E.md); FixPatch stays EPHEMERAL (GAP-D2 durable store not claimed) |
| GAP-1 | dup-fn typecheck reject | **CLOSED** `5c98c75` `[P2-DUPFN]` |
| GAP-2 | refine-pred def-time TC | **CLOSED** `c5222a8` `[GAP2-REFINE-TC]` / [`GAP2_REFINE_PRED_TC_SLICE.md`](GAP2_REFINE_PRED_TC_SLICE.md) |
| GAP-3 | render parens / round-trip | **CLOSED** `226e33c` `[GAP3-RENDER-PAREN]` / [`GAP3_RENDER_PAREN_SLICE.md`](GAP3_RENDER_PAREN_SLICE.md) |
| GAP-4 | R2 thin pilot | **LANDED** `d4aebd3` `[GAP4-R2-PILOT]` lattice-math evidence ONLY -- [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md); R2 ergonomics / CONF-P2 label gate still OPEN |
| GAP-5 | INV-2 design | **DESIGNED** `23f2e46` `[GAP5-INV2]` / [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md); no durable store (GAP-D2) |
| **GAP4-R2-SURFACE** | Thin label typecheck surface (post-E) | **CLOSED** `[GAP4-R2-SURFACE]` commit `658e14b` (publish merge `34d7459`) -- SoT [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md); soft ACK [`CURSOR_SYNC_ACK_GAP4_SURFACE.md`](CURSOR_SYNC_ACK_GAP4_SURFACE.md); seeded E1/E6 rejects in typecheck; **not** full IFC / no label syntax or inference |
| **GAP-C1** | Symbolic same-term `len`-as-index reject | **CLOSED** `[GAPC1-SYM-LEN]` commit `4fbf7df` (publish merge `0bc3c22`) -- SoT [`GAPC1_SYM_LEN_SLICE.md`](GAPC1_SYM_LEN_SLICE.md); soft ACK [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md); soft review PASS [`CLAUDE_REVIEW_GAPC1_SYM_LEN.md`](CLAUDE_REVIEW_GAPC1_SYM_LEN.md); same-term fragment only; soft cases = design |
| **GAP-C2** | SMT `len` measure encode (prove-tier) | **PREPARED** Madis-gated -- pointer [`CLAUDE_POINTER_GAPC2_IMPLEMENT.md`](CLAUDE_POINTER_GAPC2_IMPLEMENT.md); brief [`FABLE5_GAPC2_HANDOFF_PROMPT.md`](FABLE5_GAPC2_HANDOFF_PROMPT.md); marker `[GAPC2-SMT-LEN]`; do not auto-start |
| **GAP4-VALUE-LABEL** | Minimal value-label syntax → GAP4 surface | **PREPARED** Madis-gated -- pointer [`CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md`](CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md); brief [`FABLE5_GAP4_VALUE_LABEL_HANDOFF_PROMPT.md`](FABLE5_GAP4_VALUE_LABEL_HANDOFF_PROMPT.md); marker `[GAP4-VALUE-LABEL]`; surface ≠ IFC; R2 gate stays OPEN; do not auto-start |

Handoff (A-E overview): [FABLE5_CONF_P2_HANDOFF_PROMPT.md](FABLE5_CONF_P2_HANDOFF_PROMPT.md).
**E / GAP4 surface / GAP-C1 closed.** Remaining OPEN = R2 ergonomics / value-label syntax / GAP-C2 SMT `len` encode (all Madis-gated). **Both implement briefs prepared** — soft does **not** auto-pick which Fable runs first. **Paste POINTER files to Claude, not full handoffs.**

### Next recommended (Madis-gated — both prepared)

| Field | Value |
|-------|-------|
| **Default next hard task** | **TBD Madis-gated** — soft does **not** pick which runs first |
| **Candidate A** | **GAP-C2** — SMT `len` measure encode (prove-tier) — pointer [`CLAUDE_POINTER_GAPC2_IMPLEMENT.md`](CLAUDE_POINTER_GAPC2_IMPLEMENT.md); brief [`FABLE5_GAPC2_HANDOFF_PROMPT.md`](FABLE5_GAPC2_HANDOFF_PROMPT.md); marker `[GAPC2-SMT-LEN]` |
| **Candidate B** | **GAP4-VALUE-LABEL** — minimal value-label syntax feeding GAP4-R2-SURFACE (not R2 ergonomics gate; not full IFC) — pointer [`CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md`](CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md); brief [`FABLE5_GAP4_VALUE_LABEL_HANDOFF_PROMPT.md`](FABLE5_GAP4_VALUE_LABEL_HANDOFF_PROMPT.md); marker `[GAP4-VALUE-LABEL]` |
| **Not these by default** | GAP4-R2-ERGO alone (measurement probe later); F6 polish; GAP-D2 (only if durable wanted) |
| **Gate** | Madis chooses **one** and pastes that implement pointer |

**Task C implement (historical):** paste [CLAUDE_POINTER_P2C_IMPLEMENT.md](CLAUDE_POINTER_P2C_IMPLEMENT.md). Full brief: [FABLE5_CONF_P2C_HANDOFF_PROMPT.md](FABLE5_CONF_P2C_HANDOFF_PROMPT.md).
**Sync ACK (Cursor):** [CURSOR_SYNC_ACK_GAPC1.md](CURSOR_SYNC_ACK_GAPC1.md) (GAP-C1 CLOSED soft re-verify PASS, baseline **59**, `4fbf7df` / `0bc3c22`); prior [CURSOR_SYNC_ACK_GAP4_SURFACE.md](CURSOR_SYNC_ACK_GAP4_SURFACE.md) (`658e14b` / `34d7459`); [CURSOR_SYNC_ACK_P2E.md](CURSOR_SYNC_ACK_P2E.md) (`ddc3d6a` / `3c72ce4`); [CURSOR_SYNC_ACK_GAPS_BEFORE_E.md](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md) / [CURSOR_SYNC_ACK_GAP2.md](CURSOR_SYNC_ACK_GAP2.md). Soft frozen on Fable files.

**Claude review prompts:** pointer template [CLAUDE_POINTER_PROMPT_TEMPLATE.md](CLAUDE_POINTER_PROMPT_TEMPLATE.md); full review template [CLAUDE_REVIEW_PROMPT_TEMPLATE.md](CLAUDE_REVIEW_PROMPT_TEMPLATE.md); **GAP-C1 soft review (filled PASS)** [CLAUDE_POINTER_GAPC1_REVIEW.md](CLAUDE_POINTER_GAPC1_REVIEW.md) (full: [CLAUDE_REVIEW_GAPC1_SYM_LEN.md](CLAUDE_REVIEW_GAPC1_SYM_LEN.md)); GAP4 surface [CLAUDE_POINTER_GAP4_SURFACE_REVIEW.md](CLAUDE_POINTER_GAP4_SURFACE_REVIEW.md); post-E [CLAUDE_POINTER_P2E_REVIEW.md](CLAUDE_POINTER_P2E_REVIEW.md); prior C/D review pointers still archaeology.

**Do not edit while Fable owns:** `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, **`diag.rs`**, **`store.rs`**, **`render.rs`**, **`label.rs`**, **`main.rs`** (prefer leave `parser.rs` / `ast.rs` alone). Soft track: E landed -- soft docs/baselines only until Madis commits; do not soft-steal FixPatch code.

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
# expect: 59 tests pass (post [GAPC1-SYM-LEN]); prove_clamp -> 6 proved
cargo run -p vera -- --prove examples/prove_runtime_hint.vera
# expect: ≥1 [RUNTIME-CHECKED]
cargo run -p vera -- --prove examples/prove_refuted.vera
# expect: [REFUTED], exit 3
# or: powershell -File docs/pilot/soft_smoke.ps1
```

## Soft track status (post-freeze)

**ACTIVE (post-freeze, no-rename rule).** Expect **59** tests. Task E **CLOSED** `[P2E-FIX]` (`ddc3d6a` / `3c72ce4`). **GAP4-R2-SURFACE CLOSED** (`658e14b` / `34d7459`). **GAP-C1 CLOSED** `[GAPC1-SYM-LEN]` (`4fbf7df` / `0bc3c22`) — SoT [GAPC1_SYM_LEN_SLICE.md](GAPC1_SYM_LEN_SLICE.md); ACK [CURSOR_SYNC_ACK_GAPC1.md](CURSOR_SYNC_ACK_GAPC1.md); soft review PASS [CLAUDE_REVIEW_GAPC1_SYM_LEN.md](CLAUDE_REVIEW_GAPC1_SYM_LEN.md). **Next = TBD Madis-gated** — both briefs prepared: GAP-C2 (`CLAUDE_POINTER_GAPC2_IMPLEMENT.md`) **or** GAP4-VALUE-LABEL (`CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md`); soft does **not** pick which Fable runs first. Do not soft-steal label/typecheck/diag/vc/smt; do not open GAP-D2 unless Madis switches. Debt: [KNOWN_GAPS.md](KNOWN_GAPS.md). Commit gate: [COMMIT_CHECKLIST.md](COMMIT_CHECKLIST.md).
