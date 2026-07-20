# Claude pointer -- GAPC2-SMT-LEN review (post-land)

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Claude / Fable (review / close-out only). Madis is the operator.
GAPC2-SMT-LEN landed and **committed** `f8b67cc` (publish merge `f4f3cc7` with sibling VL): marker `[GAPC2-SMT-LEN]`.
Soft re-verify already **PASS** (2026-07-20): suite **68** (after sibling VALUE-LABEL; C2 alone was 59→63), `gapc2_` **4**, `soft_smoke` PASS, `prove_clamp` **6** proved.
Filled soft review: [`CLAUDE_REVIEW_GAPC2_SMT_LEN.md`](CLAUDE_REVIEW_GAPC2_SMT_LEN.md) -- **PASS**.

**Do not implement.** Do not invent next hard scope. Sibling VALUE-LABEL is a **separate** review (`CLAUDE_REVIEW_GAP4_VALUE_LABEL.md`).

**Primary review brief (read first):**
`docs/pilot/CLAUDE_REVIEW_GAPC2_SMT_LEN.md`

**Also read (SoT for what landed):**
- `docs/pilot/GAPC2_SMT_LEN_SLICE.md` -- claimed / not-claimed
- `docs/pilot/CURSOR_SYNC_ACK_GAPC2.md` -- soft ACK + baseline
- `docs/pilot/GAPC1_SYM_LEN_SLICE.md` -- prior typecheck leg
- `docs/pilot/P2C_LEN_SLICE.md` -- deferred SMT context
- `docs/pilot/KNOWN_GAPS.md` -- GAP-C2 row (opaque-constant fragment CLOSED)
- `docs/spec/SPEC.md` §4.4

**Overclaim guards (do not weaken):**
1. Opaque measure only — no list theory / literal-length propagation.
2. Call-site list args stay RUNTIME-CHECKED ([P2-SOUND2]).
3. Not full REQ-REFINE-2.
4. Labels / IFC / GAP-D2 not touched.
5. Do not merge this review with GAP4-VALUE-LABEL.

Re-run smoke if Madis wants a second pair of eyes; cite real numbers. Soft baseline is **68**.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
