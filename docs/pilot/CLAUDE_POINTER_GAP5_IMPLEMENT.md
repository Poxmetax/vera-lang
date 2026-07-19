# Claude pointer -- GAP-5 implement

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Fable 5 / Claude Code (implement / design). Madis is the operator.

**Status: DESIGNED / LANDED (typed key)** -- do not re-implement. Commit `23f2e46` · marker `[GAP5-INV2]` · SoT [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md). Campaign ACK: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md). Baseline **50**.

**Overclaim guard:** DESIGN + typed `ProofCacheKey`/`ToolchainId` only. **No** durable store (GAP-D2). Any E-prep must cite the design note: FixPatch JSON stays **EPHEMERAL** until INV-2 keys wired.

Historical brief (read-only archaeology): `docs/pilot/FABLE5_GAP5_HANDOFF_PROMPT.md`

**Do not start E** until Madis green light.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
