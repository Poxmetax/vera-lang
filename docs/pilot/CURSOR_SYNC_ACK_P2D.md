# Cursor sync ACK -- Fable landed CONF-P2 D ([P2D-ELIDE], commit 77f7077)

**Date:** 2026-07-20  
**Commit:** `77f7077` -- Add VERA task D proof-gated check elision (P2D-ELIDE).  
**Claude post-land review:** PASS (2 LOW soft TODOs -- addressed below).  
**SoT:** `docs/pilot/P2D_ELISION_SLICE.md`

## What landed (do not overclaim)

| Path | Note |
|------|------|
| `vc.rs` | `Obligation.fn_name` / `ensures_index`; `ProvedSet` (fn-level PROVED; dup-fn excluded) |
| `interp.rs` | `with_proved`; elide PROVED return_refine + ensures[i]; never requires/param-refine; +4 `elide_*` |
| `main.rs` | `--prove-run` opt-in; default/`--prove` unchanged; `--prove` precedence over `--prove-run` |
| `lib.rs` | exports `ProvedSet` |
| `P2D_ELISION_SLICE.md` | SoT |

`--diag-json` schema **unchanged**. Call-site elision deferred. No persistent cert store.

## Baseline

- `cargo test -p vera --lib` -> **34** passed (was 30)
- `soft_smoke.ps1` -> SOFT-SMOKE PASS; prove_clamp **6** proved
- `--prove-run examples/prove_clamp.vera` -> armed 3; 5/0/10; elided 9; exit 0
- `--prove-run examples/prove_refuted.vera` -> not running; exit 3

## Soft TODOs from review (done)

1. README Optional flags + Quick start: `--prove-run`, `--diag-json`
2. README note: `--diag-json` wins over `--prove-run` (diag never runs)
3. `examples/README.md` includes `refine_len_ok.vera` (fold into next soft commit with other soft docs)

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`
- Every `examples/*.vera` must typecheck
- Soft docs expect **34** tests
- **Do not start task E (FixPatch) until Madis green-lights** after this sync

## Next

1. Madis reviews [`KNOWN_GAPS.md`](KNOWN_GAPS.md) -- especially **GAP-1** (decision) and **GAP-5** (INV-2 before durable proofs).
2. Resume E when Madis confirms: paste `CLAUDE_POINTER_P2E_IMPLEMENT.md`.
3. Optional: Fable thin pilot for GAP-4 (R2) interleaved vs after E.
