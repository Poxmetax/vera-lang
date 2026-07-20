# Claude pointer -- GAPC1-SYM-LEN review (post-land)

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Claude / Fable (review / close-out only). Madis is the operator.
GAPC1-SYM-LEN landed and committed: marker `[GAPC1-SYM-LEN]`, TradingBot `4fbf7df`, publish merge `0bc3c22`.
Soft re-verify already **PASS** (2026-07-20): suite **59**, `soft_smoke` PASS, `prove_clamp` **6** proved.
Filled soft review: [`CLAUDE_REVIEW_GAPC1_SYM_LEN.md`](CLAUDE_REVIEW_GAPC1_SYM_LEN.md) -- **PASS**.

**Do not implement.** Do not invent next scope. Next hard task = **TBD Madis-gated** (GAP-C2 or value-label — soft does not pick).

**Primary review brief (read first):**
`docs/pilot/CLAUDE_REVIEW_GAPC1_SYM_LEN.md`

**Also read (SoT for what landed):**
- `docs/pilot/GAPC1_SYM_LEN_SLICE.md` -- claimed / not-claimed
- `docs/pilot/CURSOR_SYNC_ACK_GAPC1.md` -- soft ACK + baseline
- `docs/pilot/FABLE5_GAPC1_HANDOFF_PROMPT.md`
- `docs/pilot/KNOWN_GAPS.md` -- GAP-C1 CLOSED (same-term fragment); GAP-C2 OPEN
- `docs/pilot/P2C_LEN_SLICE.md` -- prior P2C deferral
- SPEC REQ-REFINE-2 wording (symbolic `len`-as-index case)

**Overclaim guards (do not weaken):**
1. Same-term fragment only — not full REQ-REFINE-2.
2. Soft cases are **design**: `xs.len()-1`, other-list `.len()`, aliases, Kleene `||` guard.
3. GAP-C2 SMT `len` encode still **OPEN**.
4. FixPatch / GAP4 surface / labels untouched.

Re-run smoke if Madis wants a second pair of eyes; cite real numbers. Soft baseline is **59**.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
