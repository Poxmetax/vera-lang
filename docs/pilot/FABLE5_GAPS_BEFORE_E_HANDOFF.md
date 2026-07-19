<!--
Operator: chat paste SHORT POINTER -- CLAUDE_POINTER_GAPS_BEFORE_E.md
Historical campaign brief. E is GREEN-LIT 2026-07-20 -- paste CLAUDE_POINTER_P2E_IMPLEMENT.md for FixPatch.
-->

# Fable 5 -- VERA gaps campaign (before CONF-P2 E)

Madis: **gaps-before-E campaign COMPLETE** (GAP-1..5 on local main). Soft ACK: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md). **E GREEN-LIT 2026-07-20** -- paste [`CLAUDE_POINTER_P2E_IMPLEMENT.md`](CLAUDE_POINTER_P2E_IMPLEMENT.md).

Debt register SoT: [`KNOWN_GAPS.md`](KNOWN_GAPS.md).

| Gap | Status | Commit | Marker |
|-----|--------|--------|--------|
| GAP-1 | **CLOSED** | `5c98c75` | `[P2-DUPFN]` |
| GAP-2 | **CLOSED** | `c5222a8` | `[GAP2-REFINE-TC]` |
| GAP-3 | **CLOSED** | `226e33c` | `[GAP3-RENDER-PAREN]` |
| GAP-4 | **LANDED** (lattice-math evidence; R2 ergonomics / CONF-P2 label gate still OPEN) | `d4aebd3` | `[GAP4-R2-PILOT]` |
| GAP-5 | **DESIGNED** (typed key; no durable store) | `23f2e46` | `[GAP5-INV2]` |

Suite baseline **50** tests. Soft authors: refine-pred fragment rules; unique `fn` names; mixed-precedence render OK; F6 string Debug-escape still open.

## Hard constraints (still apply for any follow-on)

1. Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. No TradingBot mainnet / `.env` / live state; no commit/push unless Madis asks.
3. Surgical slices; ask before >~30 lines; phase markers; UTF-8; no renames.
4. **Never** add examples that fail typecheck; keep `fn` names unique (GAP-1).
5. Honest-limits table in every new slice note; do not overclaim.
6. Preserve A-D + gaps campaign; soft_smoke PASS; prove_clamp **6** proved.
7. Suite baseline **50** tests (bump as you add).
8. Soft author rules post GAP-2: refine preds stay in fragment (no user fn / if / match / strings in `{x: Int | ...}`); no forward refs in param preds; let/lambda refine annotations checked; unique `fn` names.
9. **E GREEN-LIT (2026-07-20):** FixPatch via [`CLAUDE_POINTER_P2E_IMPLEMENT.md`](CLAUDE_POINTER_P2E_IMPLEMENT.md). FixPatch JSON stays **EPHEMERAL** until INV-2 keys wired -- cite [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md). Still out of E: MCP server, z3 crate, Salsa, durable cert store (GAP-D2).

## Ordered work (historical; all LANDED)

### 1) GAP-2 -- refine-pred definition-time typecheck -- **CLOSED** `c5222a8`

[`GAP2_REFINE_PRED_TC_SLICE.md`](GAP2_REFINE_PRED_TC_SLICE.md) · `[GAP2-REFINE-TC]` · soft ACK [`CURSOR_SYNC_ACK_GAP2.md`](CURSOR_SYNC_ACK_GAP2.md). OPEN limits: struct-field / HOF-lambda param refines; requires/ensures unchanged.

### 2) GAP-3 -- renderer parentheses / precedence round-trip -- **CLOSED** `226e33c`

[`GAP3_RENDER_PAREN_SLICE.md`](GAP3_RENDER_PAREN_SLICE.md) · `[GAP3-RENDER-PAREN]`. Mixed-precedence round-trip OK; F6 string Debug-escape still open.

### 3) GAP-4 -- R2 label-lattice thin pilot -- **LANDED** `d4aebd3`

[`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md) · `[GAP4-R2-PILOT]`. **Lattice-math evidence ONLY** -- not full IFC / CONF-P2 labels gate / R2 inference ergonomics.

### 4) GAP-5 -- INV-2 design note -- **DESIGNED** `23f2e46`

[`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md) · `[GAP5-INV2]` typed `ProofCacheKey`/`ToolchainId`. **No** durable cert DB (GAP-D2).

### Deferred (not required to unblock E; leave open unless Madis asks)

- GAP-C1 symbolic `len(xs)`-as-index
- GAP-C2 SMT `len` encode
- GAP-D1 call-site elision
- GAP-D2 persistent cert store (ties to GAP-5)

## After each slice (historical checklist)

Update `KNOWN_GAPS.md` row to CLOSED with commit/marker/slice path. Re-run:

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
```

## Return (English)

```text
## VERDICT
CAMPAIGN-COMPLETE | E-GREEN-LIT-2026-07-20

## Smoke
50 tests; soft_smoke; prove_clamp 6

## Next
Paste CLAUDE_POINTER_P2E_IMPLEMENT.md; FixPatch ephemeral + cite GAP5_INV2_DESIGN_NOTE.md; GAP-4 lattice-only
```

End of gaps-before-E handoff.
