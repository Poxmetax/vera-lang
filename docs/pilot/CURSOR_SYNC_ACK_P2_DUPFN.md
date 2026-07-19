# Cursor sync ACK -- P2-DUPFN (GAP-1 closed, commit 5c98c75)

**Date:** 2026-07-20  
**Commit:** `5c98c75` -- Add VERA P2-DUPFN duplicate-function reject (micro-slice).  
**SoT:** `docs/pilot/P2_DUPFN_SLICE.md`  
**KNOWN_GAPS:** GAP-1 -> **CLOSED**

## What landed

| Path | Note |
|------|------|
| `typecheck.rs` | `[P2-DUPFN]` reject at second `fn` decl span |
| `interp.rs` | `elide_excludes_duplicate_fn_names` asserts front-door + ProvedSet defense-in-depth |
| `P2_DUPFN_SLICE.md` | honest limits (fn vs type namespaces disjoint; API paths unchanged) |

## Baseline

- `cargo test -p vera --lib` -> **35** passed (was 34) -- Cursor verified
- soft_smoke expectations unchanged (prove_clamp 6 proved)

## Soft rules

- Example / demo `fn` names must be **unique** (round_trip fails otherwise)
- Do **not** edit Fable-owned Rust
- Soft docs expect **35** tests
- **Do not start E until Madis explicitly green-lights**

## Soft TODOs this sync

- [x] KNOWN_GAPS GAP-1 CLOSED
- [x] Bump live soft counts 34 -> 35 (checklist / queue / README)
- Soft working-tree docs (flags, examples index, PHASE2 nuance) already aligned earlier -- fold into next soft commit with this ACK
