# Cursor sync ACK -- Fable landed CONF-P2 E ([P2E-FIX], awaiting Madis commit)

**Date:** 2026-07-20  
**Commit:** *(none yet -- working tree only; Madis commits after review)*  
**Soft review:** PASS -- [`CLAUDE_REVIEW_P2E_FIXPATCH.md`](CLAUDE_REVIEW_P2E_FIXPATCH.md) / pointer [`CLAUDE_POINTER_P2E_REVIEW.md`](CLAUDE_POINTER_P2E_REVIEW.md)  
**SoT:** [`P2E_FIXPATCH_SLICE.md`](P2E_FIXPATCH_SLICE.md)

## What landed (do not overclaim)

| Path | Note |
|------|------|
| `typecheck.rs` | `MatchFixInfo {span, missing}`; three non-exhaustive sites emit arity-aware stubs; enum message names **all** missing variants; `TypeError` 2nd field |
| `diag.rs` | `FixPatch {kind, ephemeral, span, missing}` + additive `Diagnostic.fix` (omit-not-null) |
| `store.rs` | `TypeError(msg, None)` arity only -- **no** store-level FixPatch JSON claimed |
| `main.rs` | unchanged -- fix rides under `--diag-json` |
| `P2E_FIXPATCH_SLICE.md` / `P2B_DIAG_SLICE.md` | SoT + additive fix pointer |

One fix kind only: `"add-match-arms"`. Always `ephemeral: true` (GAP5 / GAP-D2 durable store **not** implemented).

## Baseline (soft re-ran 2026-07-20)

- `cargo test -p vera --lib` -> **53** passed (was 50; +3, 0 regression)
- `soft_smoke.ps1` -> SOFT-SMOKE PASS; prove_clamp **6** proved
- `--diag-json` Signal demo -> TYPE-ERROR + `fix.missing: ["Signal::Sell(_)", "Signal::Hold"]`; exit **1**

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`
- Every `examples/*.vera` must typecheck
- Soft docs expect **53** tests
- Do **not** open GAP-2..5; do **not** claim durable FixPatch store (GAP-D2)

## Next

1. **Madis:** re-run `cargo test -p vera --lib` (and optionally `soft_smoke.ps1`) then commit P2E code+docs; **exclude** `*.bak_*` backups.
2. Optional: soft push to `vera-github` after commit (Madis decides).
3. Optional: Claude second-look via [`CLAUDE_POINTER_P2E_REVIEW.md`](CLAUDE_POINTER_P2E_REVIEW.md).
4. After commit: CONF-P2 E closed; **recommended next hard** = GAP4-R2-SURFACE (thin label typecheck; not full IFC) -- paste [`CLAUDE_POINTER_GAP4_R2_SURFACE_IMPLEMENT.md`](CLAUDE_POINTER_GAP4_R2_SURFACE_IMPLEMENT.md); full [`FABLE5_GAP4_R2_SURFACE_HANDOFF_PROMPT.md`](FABLE5_GAP4_R2_SURFACE_HANDOFF_PROMPT.md). Remaining CONF-P2 surface = labels/R2 ergonomics (GAP-4 OPEN gate), not FixPatch.
