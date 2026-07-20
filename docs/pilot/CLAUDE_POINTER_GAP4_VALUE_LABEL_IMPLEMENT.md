# Claude pointer -- GAP4-VALUE-LABEL implement

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Fable 5 / Claude Code (implement). Madis is the operator.

**Gate:** Madis-gated candidate (soft does **not** auto-pick vs GAP-C2). Prefer baseline **59** -- see [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md); surface SoT [`CURSOR_SYNC_ACK_GAP4_SURFACE.md`](CURSOR_SYNC_ACK_GAP4_SURFACE.md).

Implement **GAP4-VALUE-LABEL only** (minimal value-label / annotation syntax that feeds the existing GAP4-R2-SURFACE seeds pass). Marker `[GAP4-VALUE-LABEL]`. **Not** full IFC. **Not** R2 ergonomics gate closed. **Not** GAP-C2. **Not** GAP-D2. Ask before >~30 lines (especially parser/AST).

**Primary brief (read first, follow exactly):**
`docs/pilot/FABLE5_GAP4_VALUE_LABEL_HANDOFF_PROMPT.md`

**Also read these already-written files:**
- `docs/pilot/KNOWN_GAPS.md` -- GAP-4: surface landed; value-label + R2 gate still OPEN
- `docs/pilot/GAP4_R2_SURFACE_SLICE.md` -- seeded pass SoT; natural next = value-label feeding this pass
- `docs/pilot/GAP4_R2_PILOT_SLICE.md` -- lattice-math; reuse `label.rs`
- `docs/pilot/CURSOR_SYNC_ACK_GAP4_SURFACE.md`
- `docs/pilot/SOFT_PARALLEL_QUEUE.md` -- both candidates prepared; Madis picks
- `docs/spec/SPEC.md` §4.2 / value-label `T^{...}` / R2 inference stance
- `crates/vera/src/label.rs`
- `README.md`

**Overclaim / safety guards (do not weaken):**
1. Surface ≠ full IFC; R2 inference-ergonomics gate stays **OPEN** until Madis accepts a separate probe.
2. No policies, quarantine, `infer`, actors, endorse/declassify, **implicit flows**.
3. Surgical diffs; ask Madis before any change **>~30 lines**.
4. Unique `fn` names; every committed example typechecks; soft_smoke PASS; prove_clamp still **6** proved.
5. Do not soft-claim CONF-P2 labels complete.

Do not invent scope from memory. Stop for review when done.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
