<!--
Operator: chat paste SHORT POINTER -- CLAUDE_POINTER_GAP2_IMPLEMENT.md
Prepared follow-on after GAP-1 / E (Madis chooses order). Not E.
-->

# Fable 5 -- VERA GAP-2 handoff (refine-pred definition-time typecheck)

Canonical brief for **GAP-2 only**. Pointer: [`CLAUDE_POINTER_GAP2_IMPLEMENT.md`](CLAUDE_POINTER_GAP2_IMPLEMENT.md).

---

You are continuing **VERA**. Madis is the operator. Implement **GAP-2**: typecheck refinement predicates at **definition time** (fail closed at compile time, not only at runtime).

## Why (from KNOWN_GAPS)

Malformed preds (e.g. `len` over an Int) currently slip past fn typecheck and trap later. Latent-error pile grows with every new refinement. Soft track does not edit typecheck -- this is Fable.

## Hard constraints

Same as other Fable handoffs: `vera-lang/` only; no mainnet/`.env`; no commit/push unless asked; surgical; ask before >~30 lines; no renames; UTF-8; preserve A-E markers/smoke; never add failing examples; honest-limits table in slice note.

## Preconditions

- Prefer GAP-1 (dup-fn reject) already landed or composed -- do not invent tests that depend on silent shadowing.
- Read `KNOWN_GAPS.md`, `P2_REFINE1_SLICE.md`, `P2C_LEN_SLICE.md`, `typecheck.rs` refine paths.

## What YOU must do (smallest closed fragment)

1. Typecheck refinement predicate expressions when the refine type / contract pred is bound (param refine, return refine, and/or requires/ensures preds -- pick the smallest set that demonstrably catches a malformed pred at compile time).
2. Demo: a program that today typechecks but traps at runtime on bad pred -> **TypeError** with marker `[GAP2-REFINE-TC]` (or similar; grep uniqueness), zero execution.
3. Valid preds still typecheck + run (e.g. existing refine examples stay green).
4. Slice note: `docs/pilot/GAP2_REFINE_PRED_TC_SLICE.md` with HONEST LIMITS (what pred fragment is checked; what stays deferred).
5. Smoke: `cargo test -p vera --lib`; `soft_smoke.ps1`; prove_clamp 6 proved; no typecheck-failing examples.

## Out of scope

GAP-3 render parens; GAP-4 labels; GAP-C1 symbolic len-as-index; FixPatch E (unless already landed -- do not rewrite); SMT len encode.

## Return (English short)

VERDICT DONE-GAP2 | BLOCKED | PARTIAL; files; smoke; honest limits; next suggestion only.

End of GAP-2 handoff.
