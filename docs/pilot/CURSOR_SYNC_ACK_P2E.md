# Cursor sync ACK -- Fable landed CONF-P2 E ([P2E-FIX], committed)

**Date:** 2026-07-20  
**Commit:** `ddc3d6a` (pushed; publish merge `3c72ce4` on GitHub main / vera-lang subtree)  
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

## Baseline (soft re-ran 2026-07-20, pre-surface)

- `cargo test -p vera --lib` -> **53** passed (was 50; +3, 0 regression) at P2E land
- `soft_smoke.ps1` -> SOFT-SMOKE PASS; prove_clamp **6** proved
- `--diag-json` Signal demo -> TYPE-ERROR + `fix.missing: ["Signal::Sell(_)", "Signal::Hold"]`; exit **1**

**Current soft baseline (post GAPC1):** **59** — see [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md).

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`
- Every `examples/*.vera` must typecheck
- Soft docs expect **59** tests (post GAP-C1; was 53 at P2E commit)
- Do **not** claim durable FixPatch store (GAP-D2)

## Next

1. ~~**Madis:** re-run smoke then commit P2E~~ **DONE** — commit `ddc3d6a`.
2. ~~Optional soft push to `vera-github`~~ **DONE** — publish merge `3c72ce4`.
3. Optional: Claude second-look via [`CLAUDE_POINTER_P2E_REVIEW.md`](CLAUDE_POINTER_P2E_REVIEW.md) (historical).
4. CONF-P2 E closed. **GAP4-R2-SURFACE** CLOSED (`658e14b` / `34d7459`). **GAP-C1** CLOSED (`4fbf7df` / `0bc3c22`) — soft ACK [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md). Remaining OPEN = R2 inference ergonomics / value-label / GAP-C2 — Madis-gated; not FixPatch.
