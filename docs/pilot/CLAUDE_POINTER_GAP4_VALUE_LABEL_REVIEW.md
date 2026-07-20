# Claude pointer -- GAP4-VALUE-LABEL review (post-land)

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Claude / Fable (review / close-out only). Madis is the operator.
GAP4-VALUE-LABEL landed and **committed** `28929dc` (publish merge `f4f3cc7`): marker `[GAP4-VALUE-LABEL]`.
Soft re-verify already **PASS** (2026-07-20): suite **68**, `gap4vl_` **5**, `soft_smoke` PASS, `prove_clamp` **6** proved.
Filled soft review: [`CLAUDE_REVIEW_GAP4_VALUE_LABEL.md`](CLAUDE_REVIEW_GAP4_VALUE_LABEL.md) -- **PASS**.

**Do not implement.** Do not invent next hard scope. Sibling GAP-C2 is a **separate** review (`CLAUDE_REVIEW_GAPC2_SMT_LEN.md`).

**Primary review brief (read first):**
`docs/pilot/CLAUDE_REVIEW_GAP4_VALUE_LABEL.md`

**Also read (SoT for what landed):**
- `docs/pilot/GAP4_VALUE_LABEL_SLICE.md` -- claimed / not-claimed
- `docs/pilot/CURSOR_SYNC_ACK_GAP4_VALUE_LABEL.md` -- soft ACK + baseline
- `docs/pilot/GAP4_R2_SURFACE_SLICE.md` -- surface pass this syntax feeds
- `docs/pilot/KNOWN_GAPS.md` -- GAP-4 row (value-label landed; R2 ergonomics still OPEN)
- `docs/spec/SPEC.md` §4.2 / CONF-P2 label wording

**Overclaim guards (do not weaken):**
1. Not full IFC; not taint propagation; not implicit flows.
2. R2 inference-ergonomics gate still **OPEN**.
3. One-hop rule: unannotated copy drops the label.
4. FixPatch stays ephemeral; GAP-D2 not opened.
5. Do not merge this review with GAP-C2.

Re-run smoke if Madis wants a second pair of eyes; cite real numbers. Soft baseline is **68**.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
