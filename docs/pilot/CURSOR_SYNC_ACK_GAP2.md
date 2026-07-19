# Cursor sync ACK -- GAP-2 CLOSED (awaiting Madis review; commit pending)

> **Superseded for live status** by [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md) (campaign complete; GAP-2 commit **`c5222a8`**; baseline **50**). Keep this file for GAP-2 soft-author rules archaeology.

**Date:** 2026-07-20  
**Commit:** `c5222a8` (was pending at first soft sync; now on local main)  
**Marker:** `[GAP2-REFINE-TC]`  
**SoT:** `docs/pilot/GAP2_REFINE_PRED_TC_SLICE.md`  
**KNOWN_GAPS:** GAP-2 -> **CLOSED**

## What landed (Fable)

| Path | Note |
|------|------|
| `crates/vera/src/typecheck.rs` only | `[GAP2-REFINE-TC]` -- soft track must not edit |
| `check_refine_pred_ty` | Spec-strict pred fragment (not `infer_expr`); junk -> compile `TypeError`, zero exec |
| `check_type_refines` | Nested `List` / `Option` / fn types |
| Positions | Fn param (prefix scope), fn return (full params), let + lambda refine annotations |
| Demo | `fn f(i: {k: Int \| k < len(k)})` was typecheck+runtime trap exit 2; now TypeError exit 1, zero exec |

## Baseline (Fable-reported; Cursor soft sync -- cargo not re-run this ACK)

- `cargo test -p vera --lib` -> **44** passed (was 35; +9 `gap2_*`)
- soft_smoke: prove_clamp **6** proved; **SOFT-SMOKE PASS** -- expectations unchanged (no hardcoded count in `soft_smoke.ps1`)

## Soft author rules (real behavior change)

1. Refinement preds must stay in the fragment -- no user fn calls, no `if`/`match`, no strings inside `{x: Int | ...}`.
2. No forward references in param preds (only earlier params + binder).
3. `let` / lambda refine annotations are checked too, including nested `List<{...}>` -- junk preds fail typecheck; `round_trip_all_examples` catches loudly.
4. Still hold: unique `fn` names (`[P2-DUPFN]`); every committed example typechecks.

## Honest OPEN limits (do **not** mark done)

- Struct-field refines still unchecked
- HOF-position lambda param refines (`map`/`filter`/`fold` literals) still unchecked
- `requires` / `ensures` unchanged
- GAP-C1 / GAP-C2 untouched

## Campaign / E gate

- GAP-2 touched **only** `typecheck.rs` -- GAP-3 render-parens material remains **valid**.
- **Do not** paste GAP-3 implement to Claude until Madis green-lights after GAP-2 review.
- GAP-4 / GAP-5 pointers prepared; optional order after GAP-3.
- **Do not start E** until Madis explicitly green-lights post-gaps.

## Soft TODOs this sync

- [x] Write this ACK (no fake commit hash)
- [x] Fold / confirm KNOWN_GAPS GAP-2 CLOSED (commit pending)
- [x] Bump live soft counts 35 -> **44** (checklist / queue / README / campaign notes)
- [x] Confirm GAP-3/4/5 pointers + tiny ACK notes
- [x] English-only Claude paste blocks (no Estonian paste phrasing)
- [ ] Madis review of GAP-2 code; then commit; then paste GAP-3 pointer

## Claude paste blocks (English only -- Madis → Claude)

**GAP-3 — WAIT FOR GO** (do not paste for implement until green-light after GAP-2 review):

```text
Read and follow: docs/pilot/CLAUDE_POINTER_GAP3_IMPLEMENT.md
Context: GAP-2 CLOSED ([GAP2-REFINE-TC], commit pending). Implement only after Madis green-light. Do not start E.
```

**GAP-4:**

```text
Read and follow: docs/pilot/CLAUDE_POINTER_GAP4_IMPLEMENT.md
Context: Prefer after GAP-3 unless Madis reorders. R2 thin pilot only. Do not claim full IFC. Do not start E.
```

**GAP-5:**

```text
Read and follow: docs/pilot/CLAUDE_POINTER_GAP5_IMPLEMENT.md
Context: INV-2 design note (optional tiny key stub). No durable cert DB. Do not start E.
```
