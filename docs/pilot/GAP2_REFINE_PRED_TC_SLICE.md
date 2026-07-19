# GAP2-REFINE-TC slice — refinement predicates typechecked at definition time

**Date:** 2026-07-20 · **Marker:** `[GAP2-REFINE-TC]` · **File:** `crates/vera/src/typecheck.rs`

## Plan alignment

- Campaign: `docs/pilot/FABLE5_GAPS_BEFORE_E_HANDOFF.md` (slice 1 of gaps-before-E).
- Detail brief: `docs/pilot/FABLE5_GAP2_HANDOFF_PROMPT.md`.
- Register: `docs/pilot/KNOWN_GAPS.md` GAP-2.

## What landed

1. **`check_refine_pred_ty`** — a dedicated fragment checker (deliberately NOT
   reusing `infer_expr`, keeping preds spec-strict per SPEC §3): a pred must be
   a **Bool** expression over the binder, in-scope names (refine-typed names
   read as Int), Int/Bool literals, unary `-`/`!`, Int arithmetic, Int
   comparisons, `&&`/`||`, and `len(<List-typed expr>) -> Int`. Every other
   form — `if`/`match`/lambdas/strings/ctors/**user fn calls** — is a
   compile-time `TypeError` with `[GAP2-REFINE-TC]`, zero execution.
2. **`check_type_refines`** — recursive type walker, so refines are found in
   nested positions too (`List<{k: Int | ...}>`, `Option<...>`, fn types).
3. **Checked positions + scoping:**
   - fn param refines — **prefix scoping matching the interpreter's binding
     order** (param i's pred sees params 0..=i; a forward reference is
     rejected exactly where runtime would trap unbound);
   - fn return refine — full parameter scope;
   - `let` annotations and lambda param/return refines — runtime-**inert**
     positions (interp never evaluates them) but now fragment-checked; scope =
     visible bindings (block locals / lambda params + captures). Combined into
     this slice per operator approval (safe: inert positions cannot contradict
     runtime; repo had zero such refines).

| Case | Before | After |
|------|--------|-------|
| `fn f(i: {k: Int | k < len(k)})` | typechecked, ran, runtime trap exit 2 | `error: 1:26: [GAP2-REFINE-TC] len(...) measure expects a List, got Int`, exit 1, zero exec |
| `{k: Int | k < zz}` (unknown name) | runtime trap | compile reject |
| `fn h(i: {k: Int | k < j}, j: Int)` (forward ref) | runtime trap (unbound) | compile reject (prefix scoping) |
| `{k: Int | k + 1}` (non-Bool) | runtime trap ("not Bool") | compile reject |
| pred with `if`/user-fn call | ran (unspecified behavior) | compile reject (spec fragment) |
| all 10 pre-existing pred shapes (incl. `r * 2 <= x`, `len(xs)`) | green | green (zero regressions) |

Unit tests: **9** `gap2_*`; suite **35 -> 44**.

## Behavior change (intentional, operator-approved)

Preds outside the spec fragment (e.g. calling a user fn) were previously
accepted whenever runtime-evaluable; they are now compile-rejected. SPEC §3
defines the pred fragment as binder + params + literals (+ `len` measure per
P2C); the old permissiveness was unspecified behavior. No committed program
used it (full inventory in this slice's review).

## Honest limits (deferred)

| Item | Status |
|------|--------|
| Struct/enum **field** refine types | Not checked (runtime-inert as well); future slice if fields grow refines. |
| Lambdas in HOF argument position (`map`/`filter`/`fold` literals) | Route through `check_hof_unary`/`check_hof_binary`, not `infer_lambda` — their param refines stay unchecked this slice. |
| `==`/`!=` on Bool operands in preds | Rejected (Int-only comparisons this slice); no known use case. |
| Lambda/let pred scope includes captures/locals | More permissive than fn-decl prefix rule — safe because those positions are runtime-inert; documented, not hidden. |
| requires/ensures | Unchanged — already typechecked via `infer_expr` (no `len` measure there; consistent with the SMT encoder which cannot encode calls). |
| GAP-C1 symbolic len-as-index / GAP-C2 SMT len encode | Untouched (separate register rows). |

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib          # 44 passed (was 35)
cargo test -p vera --lib gap2_    # 9 passed
powershell -File docs\pilot\soft_smoke.ps1   # SOFT-SMOKE PASS; prove_clamp 6 proved
```

Backup: `crates/vera/src/typecheck.rs.bak_20260720_011732_gap2_refine_tc`.
