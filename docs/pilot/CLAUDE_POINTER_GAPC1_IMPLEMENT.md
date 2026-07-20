# Claude pointer -- GAP-C1 implement

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Fable 5 / Claude Code (implement). Madis is the operator.

**GATE -- WAIT FOR GAP4 COMMIT:** Do **not** start until Madis has committed `[GAP4-R2-SURFACE]` (exclude `*.bak_*`). Soft review PASS / baseline **56** -- see [`CURSOR_SYNC_ACK_GAP4_SURFACE.md`](CURSOR_SYNC_ACK_GAP4_SURFACE.md). If GAP4 is still uncommitted, STOP and ask Madis.

Implement **GAP-C1 only** (symbolic `len`-as-index compile-time reject fragment). **Not** GAP-C2 SMT encode. **Not** labels/IFC / value-label / R2 ergonomics. **Not** GAP-D2.

**Primary brief (read first, follow exactly):**
`docs/pilot/FABLE5_GAPC1_HANDOFF_PROMPT.md`

**Also read these already-written files:**
- `docs/pilot/KNOWN_GAPS.md` -- GAP-C1 row
- `docs/pilot/P2C_LEN_SLICE.md` -- deferred symbolic honest limit (SoT for prior fragment)
- `docs/pilot/GAP4_R2_SURFACE_SLICE.md` -- leave surface alone; do not reopen labels
- `docs/pilot/CURSOR_SYNC_ACK_GAP4_SURFACE.md`
- `docs/pilot/SOFT_PARALLEL_QUEUE.md` -- next-recommended = GAP-C1
- `docs/spec/SPEC.md` REQ-REFINE-2 / `nth`+`len` wording
- `README.md`

**Overclaim / safety guards (do not weaken):**
1. One thin symbolic same-term / len-as-index reject -- not full REQ-REFINE-2.
2. Non-literal / unbounded indices stay soft → runtime (P2C design).
3. No value-label syntax; R2 ergonomics stays OPEN; FixPatch stays ephemeral.
4. Surgical diffs; ask Madis before any change **>~30 lines**.
5. Unique `fn` names; every committed example typechecks; soft_smoke PASS; prove_clamp still **6** proved.

Do not invent scope from memory. Stop for review when done.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
