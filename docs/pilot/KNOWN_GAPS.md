# VERA known gaps (debt register)

**Date:** 2026-07-20  
**Purpose:** Single-page register so documented deferrals do not become silent permanent holes.  
**Rule:** Factual only -- wording mirrors slice-note honest-limits tables. Do not soften or inflate.  
**Code fixes:** Fable handoffs only (soft track does not edit `vc.rs` / `smt.rs` / `typecheck.rs` / `interp.rs` / `diag.rs` / `main.rs` / `render.rs` / `label.rs`).

Source assessment: Claude/Fable project review sync 2026-07-20 (post A+B+C+D; suite then 34). Post GAP-1 close: suite **35** (`5c98c75`). Gaps-before-E campaign LANDED: GAP-2..5; suite then **50**. **[P2E-FIX]** LANDED, committed `ddc3d6a`: suite **53** (`cargo test -p vera --lib`; soft re-verify 2026-07-20: SOFT-SMOKE PASS, prove_clamp 6 proved, FixPatch additive omit-not-null). Soft ACK: [`CURSOR_SYNC_ACK_P2E.md`](CURSOR_SYNC_ACK_P2E.md); prior campaign [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md). **[GAP4-R2-SURFACE]** LANDED (working tree): seeded label checker pass; suite **56** (SOFT-SMOKE PASS, prove_clamp 6 proved) — [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md).

| id | gap | why it gets EXPENSIVE later | owner | trigger to revisit |
|----|-----|------------------------------|-------|--------------------|
| GAP-1 | ~~Duplicate fn names silently shadowed~~ **CLOSED** | Was: last decl wins via name-keyed maps; poisoned ProvedSet-style reasoning. | **CLOSED** -- `[P2-DUPFN]` / commit `5c98c75` / [`P2_DUPFN_SLICE.md`](P2_DUPFN_SLICE.md). Front-door typecheck reject at second decl span; ProvedSet dup-exclusion kept as defense-in-depth. | Reopen only if API paths need front-door too (honest limit: prove_program/Interpreter still allow unchecked Program) |
| GAP-2 | ~~Refinement predicates are not typechecked at definition time~~ **CLOSED** | Was: malformed pred (e.g. `len` over an Int) only caught fail-closed at runtime; latent-error pile grew with every refinement. | **CLOSED** -- `[GAP2-REFINE-TC]` / commit `c5222a8` / [`GAP2_REFINE_PRED_TC_SLICE.md`](GAP2_REFINE_PRED_TC_SLICE.md) / soft ACK [`CURSOR_SYNC_ACK_GAP2.md`](CURSOR_SYNC_ACK_GAP2.md). Spec-fragment checker at fn param (prefix scope) / return / let / lambda positions incl. nested types; suite 35 -> 44 (+9). OPEN limits: struct-field refines, HOF-lambda param refines, requires/ensures unchanged. | Reopen for struct-field refines or HOF-lambda param refines (honest limits in slice note) |
| GAP-3 | ~~Renderer emits `BinOp` chains without parentheses~~ **CLOSED** | Was: mixed-precedence shapes like `(1 + 2) * 3` did not round-trip AST-identically (PHASE12 F5). | **CLOSED** -- `[GAP3-RENDER-PAREN]` / commit `226e33c` / [`GAP3_RENDER_PAREN_SLICE.md`](GAP3_RENDER_PAREN_SLICE.md). Precedence-aware render (left-assoc + non-assoc cmp + unary/postfix operands); F5 probe now `round-trip OK`; no redundant parens (text-pinned); suite 44 -> 46 (+2). | Reopen for string Debug-escape story (F6) or `Expr::Block`-as-operand ASTs |
| GAP-4 | R2 label lattice: ~~zero implementation evidence~~ ~~no checker surface~~ **checker surface landed**; inference-ergonomics gate still OPEN | Was: riskiest unvalidated claim with no code at all. Pilot: lattice math + sink-bound mechanics executable (`[GAP4-R2-PILOT]` / commit `d4aebd3` / `label.rs` / [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md), 3 tests; suite 46 -> 49). Surface: seeded pass wires `Label::flows_to` into typecheck — E1/E6 explicit-flow rejects execute at call sites; front door runs it empty-seeded (inert by lattice laws); suite 53 -> 56 (`[GAP4-R2-SURFACE]`, working tree) — [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md). **NOT claimed:** value-label syntax, R2 inference ergonomics, CONF-P2 label gate, implicit flows, taint propagation, "labels/IFC implemented". | **Fable** -- remaining: language surface (value-label syntax, post-MVP) + ergonomics measurement (the actual R2 gate) | Before claiming CONF-P2 "label-inference ergonomics gate"; when value-label syntax lands |
| GAP-5 | INV-2 keying: ~~undesigned~~ **DESIGNED** | Was: persistent certificates / FixPatch / MCP risked unversioned blobs. Now: key tuple written down + typed (`[GAP5-INV2]` / commit `23f2e46` `ProofCacheKey`/`ToolchainId` in `vc.rs` + equality test) -- [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md): scheme, E-stays-ephemeral rule, bump rules. Durable store itself = GAP-D2 (unchanged). | **DESIGNED** -- implementation gated by Madis with first durable consumer | E must cite the note (FixPatch JSON stays EPHEMERAL until INV-2 keys wired / GAP-D2); GAP-D2 implements |
| GAP-C1 | Symbolic `nth(xs, xs.len())` / `len(xs)`-as-index not rejected at compile time (P2C decided-literal fragment only). | SPEC REQ-REFINE-2 cites this case; deferral is honest but incomplete vs full REQ-REFINE-2. | **Fable** | After GAP-2 or with symbolic measure reasoning slice; see `P2C_LEN_SLICE.md` |
| GAP-C2 | SMT encode of `len` as measure still open; `--prove` stays RUNTIME-CHECKED for `Call`/`len`. | Prove tier cannot discharge len-bounds statically; agents may over-trust typecheck-only. | **Fable** | When expanding VC encode fragment; see `PHASE2_VC_SLICE_REPORT.md` / `P2C_LEN_SLICE.md` |
| GAP-D1 | Call-site obligations (`call_requires` / `call_arg_refine`) are never elided -- interpreter has no call-site identity; callee entry checks always run (conservative, sound). | Fine for soundness; limits CONF-P2 "elided" surface; future elision needs call-site ids. | **Fable** | If elision completeness becomes a gate; see `P2D_ELISION_SLICE.md` |
| GAP-D2 | No persistent certificate / FixPatch store (D set + E patches live one process / one review cycle). | Same as GAP-5 when crossing process/CI/MCP; stale patch apply risk without INV-2 + content hash. | **Fable** | With INV-2 durable consumer; see `P2D_ELISION_SLICE.md` / `GAP5_INV2_DESIGN_NOTE.md` / `P2E_FIXPATCH_SLICE.md` |
| GAP-E1 | `TypeError` gained `Option<MatchFixInfo>` 2nd field (`[P2E-FIX]`); `store.rs` adapted arity-only (`TypeError(msg, None)` x4) -- store has no JSON FixPatch surface. | Downstream constructors / pattern matches on `TypeError` must keep the 2-tuple; easy to miss in future store/API paths. | **Fable** (documented) | When extending store diagnostics or SPEC §6.2 FixPatch; see `P2E_FIXPATCH_SLICE.md` |

## Strengths to preserve (do not regress)

1. Honest-limits tables + sync ACKs that say "do not overclaim".
2. Thin vertical slices with markers; green suite 17 -> 22 -> 30 -> 34 -> 35 (`[P2-DUPFN]`) -> 44 (`[GAP2-REFINE-TC]`, +9) -> 46 (`[GAP3-RENDER-PAREN]`, +2) -> 49 (`[GAP4-R2-PILOT]`, +3) -> 50 (`[GAP5-INV2]`, +1) -> 53 (`[P2E-FIX]`, +3) -> **56** (`[GAP4-R2-SURFACE]`, +3), zero regressions across A-E + gaps campaign + surface slice.
3. Store invariant: committed codebase always typechecks (`round_trip_all_examples`) -- never add examples that fail typecheck.
4. Soundness-first prove path: prefer honest RUNTIME-CHECKED over fake PROVED (`[P2-SOUND1/2/3]`).

## Soft-doc hygiene (Cursor)

Bump stale counts / flags in README and slice pointers when baselines change; keep this register linked from README status.

## Plan note

- **GAP-1:** **CLOSED** (`5c98c75` / `[P2-DUPFN]`).
- **GAP-2:** **CLOSED** (`c5222a8` / `[GAP2-REFINE-TC]`).
- **GAP-3:** **CLOSED** (`226e33c` / `[GAP3-RENDER-PAREN]`).
- **GAP-4:** lattice-math evidence LANDED (`d4aebd3` / `[GAP4-R2-PILOT]`); **checker-surface slice LANDED** (`[GAP4-R2-SURFACE]`, working tree; seeded pass, no syntax; suite 56) -- [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md); R2 inference-ergonomics / CONF-P2 label gate still OPEN -- see claimed/not-claimed table in [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md).
- **GAP-5:** **DESIGNED** (`23f2e46` / `[GAP5-INV2]` / [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md)); durable store = GAP-D2.
- **Campaign:** gaps-before-E complete (GAP-1..5). Soft ACK: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md).
- **E:** **LANDED** `[P2E-FIX]` (committed `ddc3d6a`, pushed; soft review PASS [`CLAUDE_REVIEW_P2E_FIXPATCH.md`](CLAUDE_REVIEW_P2E_FIXPATCH.md) / [`CURSOR_SYNC_ACK_P2E.md`](CURSOR_SYNC_ACK_P2E.md)). FixPatch JSON stays EPHEMERAL (`ephemeral: true`); durable store = GAP-D2; TypeError arity note = GAP-E1. Do not reopen GAP-2..5.
