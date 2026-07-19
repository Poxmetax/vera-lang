# VERA known gaps (debt register)

**Date:** 2026-07-20  
**Purpose:** Single-page register so documented deferrals do not become silent permanent holes.  
**Rule:** Factual only -- wording mirrors slice-note honest-limits tables. Do not soften or inflate.  
**Code fixes:** Fable handoffs only (soft track does not edit `vc.rs` / `smt.rs` / `typecheck.rs` / `interp.rs` / `diag.rs` / `main.rs` / `render.rs` / `label.rs`).

Source assessment: Claude/Fable project review sync 2026-07-20 (post A+B+C+D; suite then 34). Post GAP-1 close: suite **35** (`5c98c75`). Gaps-before-E campaign LANDED: GAP-2..5 commits on local main; suite **50** (`cargo test -p vera --lib`; Fable re-verify: 0 build warnings, SOFT-SMOKE PASS, prove_clamp 6 proved, diag-json schema unchanged 8 keys). Soft ACK: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md).

| id | gap | why it gets EXPENSIVE later | owner | trigger to revisit |
|----|-----|------------------------------|-------|--------------------|
| GAP-1 | ~~Duplicate fn names silently shadowed~~ **CLOSED** | Was: last decl wins via name-keyed maps; poisoned ProvedSet-style reasoning. | **CLOSED** -- `[P2-DUPFN]` / commit `5c98c75` / [`P2_DUPFN_SLICE.md`](P2_DUPFN_SLICE.md). Front-door typecheck reject at second decl span; ProvedSet dup-exclusion kept as defense-in-depth. | Reopen only if API paths need front-door too (honest limit: prove_program/Interpreter still allow unchecked Program) |
| GAP-2 | ~~Refinement predicates are not typechecked at definition time~~ **CLOSED** | Was: malformed pred (e.g. `len` over an Int) only caught fail-closed at runtime; latent-error pile grew with every refinement. | **CLOSED** -- `[GAP2-REFINE-TC]` / commit `c5222a8` / [`GAP2_REFINE_PRED_TC_SLICE.md`](GAP2_REFINE_PRED_TC_SLICE.md) / soft ACK [`CURSOR_SYNC_ACK_GAP2.md`](CURSOR_SYNC_ACK_GAP2.md). Spec-fragment checker at fn param (prefix scope) / return / let / lambda positions incl. nested types; suite 35 -> 44 (+9). OPEN limits: struct-field refines, HOF-lambda param refines, requires/ensures unchanged. | Reopen for struct-field refines or HOF-lambda param refines (honest limits in slice note) |
| GAP-3 | ~~Renderer emits `BinOp` chains without parentheses~~ **CLOSED** | Was: mixed-precedence shapes like `(1 + 2) * 3` did not round-trip AST-identically (PHASE12 F5). | **CLOSED** -- `[GAP3-RENDER-PAREN]` / commit `226e33c` / [`GAP3_RENDER_PAREN_SLICE.md`](GAP3_RENDER_PAREN_SLICE.md). Precedence-aware render (left-assoc + non-assoc cmp + unary/postfix operands); F5 probe now `round-trip OK`; no redundant parens (text-pinned); suite 44 -> 46 (+2). | Reopen for string Debug-escape story (F6) or `Expr::Block`-as-operand ASTs |
| GAP-4 | R2 label lattice: ~~zero implementation evidence~~ **evidence landed**; inference-ergonomics gate still OPEN | Was: riskiest unvalidated claim with no code at all. Now: lattice math + sink-bound mechanics executable (`[GAP4-R2-PILOT]` / commit `d4aebd3` / `label.rs` / [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md), 3 tests: laws, E1/E6 sink rejects, data-only taint-prop; suite 46 -> 49). **NOT claimed:** R2 inference ergonomics, CONF-P2 label gate, implicit flows, "labels/IFC implemented". | **Fable** -- remaining: surface integration + ergonomics measurement (the actual R2 gate) | Before claiming CONF-P2 "label-inference ergonomics gate"; when label surface lands |
| GAP-5 | INV-2 keying: ~~undesigned~~ **DESIGNED** | Was: persistent certificates / FixPatch / MCP risked unversioned blobs. Now: key tuple written down + typed (`[GAP5-INV2]` / commit `23f2e46` `ProofCacheKey`/`ToolchainId` in `vc.rs` + equality test) -- [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md): scheme, E-stays-ephemeral rule, bump rules. Durable store itself = GAP-D2 (unchanged). | **DESIGNED** -- implementation gated by Madis with first durable consumer | E must cite the note (FixPatch JSON stays EPHEMERAL until INV-2 keys wired / GAP-D2); GAP-D2 implements |
| GAP-C1 | Symbolic `nth(xs, xs.len())` / `len(xs)`-as-index not rejected at compile time (P2C decided-literal fragment only). | SPEC REQ-REFINE-2 cites this case; deferral is honest but incomplete vs full REQ-REFINE-2. | **Fable** | After GAP-2 or with symbolic measure reasoning slice; see `P2C_LEN_SLICE.md` |
| GAP-C2 | SMT encode of `len` as measure still open; `--prove` stays RUNTIME-CHECKED for `Call`/`len`. | Prove tier cannot discharge len-bounds statically; agents may over-trust typecheck-only. | **Fable** | When expanding VC encode fragment; see `PHASE2_VC_SLICE_REPORT.md` / `P2C_LEN_SLICE.md` |
| GAP-D1 | Call-site obligations (`call_requires` / `call_arg_refine`) are never elided -- interpreter has no call-site identity; callee entry checks always run (conservative, sound). | Fine for soundness; limits CONF-P2 "elided" surface; future elision needs call-site ids. | **Fable** | If elision completeness becomes a gate; see `P2D_ELISION_SLICE.md` |
| GAP-D2 | No persistent certificate store (D set lives one process). | Same as GAP-5 when crossing process/CI/MCP. | **Fable** | With INV-2 design; see `P2D_ELISION_SLICE.md` / `GAP5_INV2_DESIGN_NOTE.md` |

## Strengths to preserve (do not regress)

1. Honest-limits tables + sync ACKs that say "do not overclaim".
2. Thin vertical slices with markers; green suite 17 -> 22 -> 30 -> 34 -> 35 (`[P2-DUPFN]`) -> 44 (`[GAP2-REFINE-TC]`, +9) -> 46 (`[GAP3-RENDER-PAREN]`, +2) -> 49 (`[GAP4-R2-PILOT]`, +3) -> **50** (`[GAP5-INV2]`, +1), zero regressions across A-D + gaps campaign.
3. Store invariant: committed codebase always typechecks (`round_trip_all_examples`) -- never add examples that fail typecheck.
4. Soundness-first prove path: prefer honest RUNTIME-CHECKED over fake PROVED (`[P2-SOUND1/2/3]`).

## Soft-doc hygiene (Cursor)

Bump stale counts / flags in README and slice pointers when baselines change; keep this register linked from README status.

## Plan note

- **GAP-1:** **CLOSED** (`5c98c75` / `[P2-DUPFN]`).
- **GAP-2:** **CLOSED** (`c5222a8` / `[GAP2-REFINE-TC]`).
- **GAP-3:** **CLOSED** (`226e33c` / `[GAP3-RENDER-PAREN]`).
- **GAP-4:** lattice-math evidence LANDED (`d4aebd3` / `[GAP4-R2-PILOT]`); R2 inference-ergonomics / CONF-P2 label gate still OPEN -- see claimed/not-claimed table in [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md).
- **GAP-5:** **DESIGNED** (`23f2e46` / `[GAP5-INV2]` / [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md)); durable store = GAP-D2.
- **Campaign:** gaps-before-E complete (GAP-1..5). Soft ACK: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md).
- **E:** **GREEN-LIT by Madis (2026-07-20)** -- [`CLAUDE_POINTER_P2E_IMPLEMENT.md`](CLAUDE_POINTER_P2E_IMPLEMENT.md). FixPatch JSON stays EPHEMERAL until INV-2 keys wired (cite GAP-5 design note / GAP-D2); GAP-4 lattice-only; no durable cert store in E.
