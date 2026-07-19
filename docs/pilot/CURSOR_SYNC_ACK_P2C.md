# Cursor sync ACK -- Fable landed CONF-P2 C (commit 976231b)

**Date:** 2026-07-19  
**Commit:** `976231b` -- Add VERA task C len-measure refinements (P2-REFINE2).  
**Review:** PASS (Fable self-review + Cursor soft follow-ups). Optional: fresh-session independent re-review via `CLAUDE_POINTER_P2C_REVIEW.md`.

## What landed (do not overclaim)

| Path | Marker / note |
|------|----------------|
| `crates/vera/src/typecheck.rs` | `[P2-REFINE2]` Kleene three-valued `&&`/`||`; literal OOB + `len` measure -> TypeError, zero exec; +5 `refine2_*` |
| `crates/vera/src/interp.rs` | `len(e)` in refine preds at runtime (guarded: user/local `len` wins); +3 `len_measure_*` |
| `examples/refine_len_ok.vera` | in-range demo, prints 20 |
| `docs/pilot/P2C_LEN_SLICE.md` | **SoT** for honest limits (symbolic `len(xs)`-as-index DEFERRED; unbounded = runtime + `get -> Option`) |

`--diag-json` schema **unchanged** (TYPE-ERROR + span; message may carry `[P2-REFINE2]`).

## Baseline

- `cargo test -p vera --lib` -> **30** passed
- `soft_smoke.ps1`: prove_clamp **6** proved; runtime-hint RUNTIME-CHECKED; refuted exit **3**; SOFT-SMOKE PASS

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`
- Every `examples/*.vera` must typecheck (`round_trip_all_examples`)
- Soft docs: expect **30** tests; list `refine_len_ok.vera`; README example count **10** + 3 prove demos

## Soft follow-ups done after review

- `README.md`: example count 9 -> 10
- `PHASE2_VC_SLICE_REPORT.md`: REQ-REFINE-2 line nuanced (decided-literal landed; SMT len-encode open)

## Future slice (NOT now)

Refine-pred definition-time typecheck (today fail-closed at runtime) -- backlog only.

## Next (plan)

1. **D implement now:** paste `CLAUDE_POINTER_P2D_IMPLEMENT.md` (full: `FABLE5_CONF_P2D_HANDOFF_PROMPT.md`).
2. After D lands: mint/review via pointer pattern; then E FixPatch.
