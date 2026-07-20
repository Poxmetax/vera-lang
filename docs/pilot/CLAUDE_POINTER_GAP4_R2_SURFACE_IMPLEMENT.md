# Claude pointer -- GAP4-R2-SURFACE implement

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Fable 5 / Claude Code (implement). Madis is the operator.

**Gate:** awaiting Madis paste / green-light after CONF-P2 E. Prefer E committed first (baseline **53**, soft review PASS). Sync: [`CURSOR_SYNC_ACK_P2E.md`](CURSOR_SYNC_ACK_P2E.md).

Implement **GAP4-R2-SURFACE only** (thin label typecheck surface on existing lattice). **Not** full IFC. **Not** GAP-D2 durable store unless Madis re-scopes.

**Primary brief (read first, follow exactly):**
`docs/pilot/FABLE5_GAP4_R2_SURFACE_HANDOFF_PROMPT.md`

**Also read these already-written files:**
- `docs/pilot/KNOWN_GAPS.md` -- GAP-4 OPEN gate = R2 ergonomics / CONF-P2 labels; A--E closed for implement
- `docs/pilot/GAP4_R2_PILOT_SLICE.md` -- lattice-math SoT; reuse `label.rs`; do not re-pilot
- `docs/pilot/GAP5_INV2_DESIGN_NOTE.md` -- FixPatch / certs stay ephemeral; GAP-D2 out of scope
- `docs/pilot/P2E_FIXPATCH_SLICE.md` -- preserve additive FixPatch / `ephemeral: true`
- `docs/pilot/CURSOR_SYNC_ACK_P2E.md`
- `docs/spec/SPEC.md` §4.2 / DP4 / CONF-P2 label wording
- `crates/vera/src/label.rs`
- `README.md`

**Overclaim / safety guards (do not weaken):**
1. GAP-4 surface ≠ full IFC / ≠ R2 ergonomics gate closed (leave gate OPEN unless Madis accepts a probe).
2. No policies, quarantine, `infer`, actors, endorse/declassify, implicit flows.
3. Surgical diffs; ask Madis before any change **>~30 lines**.
4. Unique `fn` names; every committed example typechecks; soft_smoke PASS; prove_clamp still **6** proved.

Do not invent scope from memory. Stop for review when done.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
