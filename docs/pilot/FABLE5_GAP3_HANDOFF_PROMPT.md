<!--
Operator: chat paste SHORT POINTER -- CLAUDE_POINTER_GAP3_IMPLEMENT.md
Part of gaps-before-E campaign. Not FixPatch E.
STATUS: LANDED 226e33c [GAP3-RENDER-PAREN] -- do not re-implement; pointer marked LANDED.
-->

# Fable 5 -- VERA GAP-3 handoff (renderer parentheses / precedence round-trip)

**LANDED** `226e33c` `[GAP3-RENDER-PAREN]` · [`GAP3_RENDER_PAREN_SLICE.md`](GAP3_RENDER_PAREN_SLICE.md). Campaign ACK: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md). Baseline now **50**. This file is historical implement brief.

Canonical brief for **GAP-3 only**. Pointer: [`CLAUDE_POINTER_GAP3_IMPLEMENT.md`](CLAUDE_POINTER_GAP3_IMPLEMENT.md).

---

You are continuing **VERA**. Madis is the operator. Implement **GAP-3**: precedence-aware **canonical render** so mixed-precedence ASTs survive `render -> parse -> hash` identity.

## Why (KNOWN_GAPS + PHASE12 F5)

`render.rs` emits BinOp chains without parentheses (`_prec` unused). Source like `(1 + 2) * 3` re-renders as `1 + 2 * 3` -> different AST -> `--round-trip` FAIL. Examples suite catches loudly; store-API users can hit silently. Gets worse as corpus grows.

## Hard constraints

`vera-lang/` only; no mainnet/`.env`; no commit/push unless asked; surgical; **ask before >~30 lines** (PHASE12 estimated ~60 lines -- get Madis y if over budget); no renames; UTF-8; unique `fn` names; never add typecheck-failing examples; honest-limits table in slice note; soft_smoke PASS; prove_clamp 6 proved. Baseline tests **44+**. Soft authors: refine preds stay in GAP-2 fragment; no forward param refs.

## Preconditions

- GAP-1 CLOSED (`P2_DUPFN_SLICE.md`). GAP-2 CLOSED awaiting Madis review (`GAP2_REFINE_PRED_TC_SLICE.md` / `CURSOR_SYNC_ACK_GAP2.md`) -- `typecheck.rs` only; this slice is `render.rs`.
- Read `render.rs`, `parser` precedence, `PHASE12_REVIEW_FINDINGS.md` F5, `KNOWN_GAPS.md` GAP-3.
- **Implement only after Madis green-lights** post GAP-2 review.

## What YOU must do (smallest closed fragment)

1. Implement precedence-aware parenthesization on render (parenthesize child when binding weaker; handle equal-precedence / associativity so AST shape is preserved).
2. Marker e.g. `[GAP3-RENDER-PAREN]` (grep uniqueness).
3. Unit / round-trip tests for mixed-precedence shapes that **failed before** and **pass after** (e.g. `(1 + 2) * 3`, `(a || b) && c` if in grammar).
4. Existing `round_trip_all_examples` stays green -- do not weaken the invariant.
5. Slice note: `docs/pilot/GAP3_RENDER_PAREN_SLICE.md` with HONEST LIMITS (what ops covered; string Debug escapes etc. if still open -- cite F6 if deferred).
6. Update `KNOWN_GAPS.md` GAP-3 -> CLOSED with commit/marker/slice when done.

## Out of scope

GAP-4 labels; GAP-5 cert store; FixPatch E; full string-escape lexer story unless needed for your tests; GAP-2 refine TC (separate slice).

## Return (English short)

VERDICT DONE-GAP3 | BLOCKED | PARTIAL; files; smoke; honest limits; next.

End of GAP-3 handoff.
