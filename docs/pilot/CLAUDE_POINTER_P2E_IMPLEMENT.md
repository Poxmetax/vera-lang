# Claude pointer -- P2E implement

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Fable 5 / Claude Code (implement). Madis is the operator.

**Gate:** **GREEN-LIT by Madis (2026-07-20).** Gaps-before-E campaign complete (GAP-1..5). Baseline **50** (`cargo test -p vera --lib`). Sync: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md).

Implement **task E only** (FixPatch JSON).

**Primary brief (read first, follow exactly):**
`docs/pilot/FABLE5_CONF_P2E_HANDOFF_PROMPT.md`

**Also read these already-written files:**
- `docs/pilot/KNOWN_GAPS.md` -- GAP-1 CLOSED (`5c98c75`); GAP-5 DESIGNED (`23f2e46`); GAP-D2 = durable store (out of E)
- `docs/pilot/GAP5_INV2_DESIGN_NOTE.md` -- **required:** FixPatch JSON stays **EPHEMERAL** until INV-2 keys wired (durable store = GAP-D2); no durable cert/proof cache in E
- `docs/pilot/GAP4_R2_PILOT_SLICE.md` -- do **not** claim labels/IFC done (lattice-math evidence only)
- `docs/pilot/GAP2_REFINE_PRED_TC_SLICE.md` -- refine preds stay in GAP-2 fragment; do not widen pred surface in E
- `docs/pilot/P2_DUPFN_SLICE.md` -- keep example / demo `fn` names **unique**
- `docs/pilot/P2B_DIAG_SLICE.md` -- diag schema SoT (extend additively; 8 keys at campaign end; FixPatch optional/additive)
- `docs/pilot/CURSOR_SYNC_ACK_P2D.md`
- `docs/pilot/P2D_ELISION_SLICE.md`
- `docs/spec/SPEC.md` (DP8, CONF-P2 FixPatch, INV-2)
- `crates/vera/src/diag.rs`
- `README.md`

**Overclaim / safety guards (do not weaken):**
1. GAP-4 = lattice-math only -- never claim full IFC / label inference gate.
2. FixPatch = ephemeral suggest-edit -- never claim durable INV-2 proof/patch store (GAP-D2 later).
3. Surgical diffs; ask Madis before any change **>~30 lines**.
4. Unique `fn` names; every committed example typechecks; soft_smoke PASS; prove_clamp still **6** proved.

Do not invent scope from memory. Stop for review when done.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
