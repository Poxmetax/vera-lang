# Claude pointer -- GAP-C2 implement

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Fable 5 / Claude Code (implement). Madis is the operator.

**Gate:** Madis-gated candidate (soft does **not** auto-pick vs GAP4-VALUE-LABEL). Prefer baseline **59** + GAP-C1 committed -- see [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md).

Implement **GAP-C2 only** (thin SMT/`len` measure encode in VC path). Marker `[GAPC2-SMT-LEN]`. **Not** full REQ-REFINE-2. **Not** labels/IFC / value-label / R2. **Not** GAP-D2. Ask before >~30 lines.

**Primary brief (read first, follow exactly):**
`docs/pilot/FABLE5_GAPC2_HANDOFF_PROMPT.md`

**Also read these already-written files:**
- `docs/pilot/KNOWN_GAPS.md` -- GAP-C2 row OPEN; GAP-C1 CLOSED
- `docs/pilot/P2C_LEN_SLICE.md` -- deferred SMT encode honest limit (SoT)
- `docs/pilot/GAPC1_SYM_LEN_SLICE.md` -- typecheck fragment closed; leave alone
- `docs/pilot/CURSOR_SYNC_ACK_GAPC1.md`
- `docs/pilot/PHASE2_VC_SLICE_REPORT.md` -- prove tiers / QF_LIA limits
- `docs/pilot/SOFT_PARALLEL_QUEUE.md` -- both candidates prepared; Madis picks
- `docs/spec/SPEC.md` §4.4 measures / REQ-REFINE-2
- `README.md`

**Overclaim / safety guards (do not weaken):**
1. Thin `len` encode fragment -- not every Call proved; soundness-first RUNTIME-CHECKED over fake PROVED.
2. Preserve soft_smoke PASS; prove_clamp still **6** proved unless Madis expands.
3. No value-label syntax; R2 ergonomics stays OPEN; FixPatch stays ephemeral.
4. Surgical diffs; ask Madis before any change **>~30 lines**.
5. Unique `fn` names; every committed example typechecks.

Do not invent scope from memory. Stop for review when done.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
