# Claude pointer -- GAP4-R2-SURFACE review (post-land)

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Claude / Fable (review / close-out only). Madis is the operator.
GAP4-R2-SURFACE landed (working tree, **not committed**): marker `[GAP4-R2-SURFACE]`.
Soft re-verify already **PASS** (2026-07-20): suite **56**, `soft_smoke` PASS, `prove_clamp` **6** proved.
Filled soft review: [`CLAUDE_REVIEW_GAP4_SURFACE.md`](CLAUDE_REVIEW_GAP4_SURFACE.md) -- **PASS**.

**Do not implement.** Do not invent next scope. Stop until Madis commits GAP4 surface (**exclude** `*.bak_*`).

**Primary review brief (read first):**
`docs/pilot/CLAUDE_REVIEW_GAP4_SURFACE.md`

**Also read (SoT for what landed):**
- `docs/pilot/GAP4_R2_SURFACE_SLICE.md` -- claimed / not-claimed
- `docs/pilot/CURSOR_SYNC_ACK_GAP4_SURFACE.md` -- soft ACK + baseline
- `docs/pilot/FABLE5_GAP4_R2_SURFACE_HANDOFF_PROMPT.md`
- `docs/pilot/KNOWN_GAPS.md` -- GAP-4 row (surface landed; R2 ergonomics still OPEN)
- `docs/pilot/GAP4_R2_PILOT_SLICE.md` -- lattice-math only (prior)
- `docs/spec/SPEC.md` §4.2 / DP4 / CONF-P2 label wording

**Overclaim guards (do not weaken):**
1. Seeds ≠ value-label syntax (`T^{...}` not claimed).
2. Not full IFC; not taint propagation; not implicit flows.
3. R2 inference-ergonomics gate still **OPEN**.
4. FixPatch stays ephemeral; GAP-D2 not opened.

Re-run smoke if Madis wants a second pair of eyes; cite real numbers. Soft baseline is **56**.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
