# Cursor sync ACK -- gaps-before-E campaign COMPLETE

**Date:** 2026-07-20  
**Baseline:** `cargo test -p vera --lib` -> **50** passed (Fable re-verify; soft cites without re-run)  
**Soft smoke:** SOFT-SMOKE PASS; prove_clamp **6** proved; diag-json schema unchanged (**8** keys)  
**Campaign ACK supersedes** interim [`CURSOR_SYNC_ACK_GAP2.md`](CURSOR_SYNC_ACK_GAP2.md) for live status (GAP-2 historical detail still valid).

## Landed commits (local main)

| Gap | Marker | Commit | Slice / note |
|-----|--------|--------|--------------|
| GAP-1 | `[P2-DUPFN]` | `5c98c75` | [`P2_DUPFN_SLICE.md`](P2_DUPFN_SLICE.md) |
| GAP-2 | `[GAP2-REFINE-TC]` | `c5222a8` | [`GAP2_REFINE_PRED_TC_SLICE.md`](GAP2_REFINE_PRED_TC_SLICE.md) |
| GAP-3 | `[GAP3-RENDER-PAREN]` | `226e33c` | [`GAP3_RENDER_PAREN_SLICE.md`](GAP3_RENDER_PAREN_SLICE.md) |
| GAP-4 | `[GAP4-R2-PILOT]` | `d4aebd3` | [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md) |
| GAP-5 | `[GAP5-INV2]` | `23f2e46` | [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md) |

Suite path: 35 (GAP-1) -> 44 (GAP-2, +9) -> 46 (GAP-3, +2) -> 49 (GAP-4, +3) -> **50** (GAP-5, +1).

## Overclaim guards (mandatory)

1. **GAP-4** = lattice-math evidence ONLY. Do **not** write "labels/IFC implemented". Do **not** claim R2 inference-ergonomics gate or CONF-P2 label gate. SoT: claimed/not-claimed table in [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md).
2. **GAP-5** = DESIGN + typed key (`ProofCacheKey` / `ToolchainId`). **No** persistent cache. Any E-prep doc must cite [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md): FixPatch JSON stays **EPHEMERAL** until INV-2 keys wired (durable store = GAP-D2).
3. **Renderer:** mixed-precedence examples OK in docs now (round-trip). String Debug-escape (F6) still open -- avoid exotic string literals in demos.
4. Soft authors: refine preds stay in GAP-2 fragment; unique `fn` names; every committed example typechecks.

## E gate

- Gaps-before-E campaign **complete** (GAP-1..5).
- Task **E (FixPatch)** pointer: [`CLAUDE_POINTER_P2E_IMPLEMENT.md`](CLAUDE_POINTER_P2E_IMPLEMENT.md).
- **GREEN-LIT by Madis (2026-07-20)** -- paste/implement E now.
- Overclaim guards still bind: GAP-4 lattice-only; FixPatch JSON **EPHEMERAL** until INV-2 keys wired (cite [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md) / GAP-D2); baseline **50**; unique `fn` names; refine preds stay in GAP-2 fragment; diag-json additive only; no durable cert store in E.

## Soft TODOs this sync

- [x] Write this campaign ACK with real hashes
- [x] KNOWN_GAPS hashes + CLOSED / DESIGNED rows
- [x] Bump live soft counts -> **50**
- [x] Queue / README / pointers: campaign complete; next = E waiting green light
- [x] Madis green-lights E (2026-07-20) -- queue/README/ACK/pointer flipped; overclaim guards retained
- [ ] Madis soft commit of dirty soft docs (Cursor prepares; Madis commits -- no auto commit/push)

## Claude paste (English only -- GREEN LIGHT)

**E — MADIS GREEN LIGHT (2026-07-20)** -- paste now:

```text
Read and follow: docs/pilot/CLAUDE_POINTER_P2E_IMPLEMENT.md
Context: Gaps-before-E complete (GAP-1..5). Madis green-lights Phase E (FixPatch) now. Baseline cargo test -p vera --lib = 50. Obey overclaim guards: GAP-4 lattice-only; FixPatch ephemeral per GAP5_INV2_DESIGN_NOTE / GAP-D2; unique fn names; refine preds stay in GAP-2 fragment. Do not claim full IFC or durable proof cache. Surgical diffs; ask before >~30 lines.
```
