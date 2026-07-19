# P2-REFINE1 slice — REQ-REFINE-1 call-site hard reject

**Date:** 2026-07-19 · **Marker:** `[P2-REFINE1]` · **File:** `crates/vera/src/typecheck.rs`

## Plan alignment

- Plan: `.cursor/plans/vera_ai-native_language_5ea95226.plan.md` — Phase 2 / CONF-P2 next after thin VC slice.
- Handoff task **A**: `docs/pilot/FABLE5_CONF_P2_HANDOFF_PROMPT.md` (REQ-REFINE-1).
- SPEC: `docs/spec/SPEC.md` §4.4 REQ-REFINE-1.

## What landed

Call-site **compile-time** reject when:

1. Parameter type is `{binder: Int | pred}`, and
2. Argument is an `Int` literal (including negative literals via unary minus — Fable extension, 2026-07-19), and
3. `pred` is a closed comparison / `&&` / `||` / `!` tree over the binder + literals, and
4. Evaluation yields `false`.

Example shape (unit test): `apply_discount(100, 150)` → `TypeError` containing `[P2-REFINE1]` (zero interpreter execution).

In-range literals still typecheck (`apply_discount(100, 10)`). Positive runnable example: `examples/refine_call_ok.vera`.

## Honest limits (deferred)

| Item | Status |
|------|--------|
| Non-literal / unevaluable preds | Soft — prove/runtime (unchanged) |
| Definition-time body vs return refine | **Done (closed lit/ite)** — `[P2-REFINE1-DEF]` 2026-07-19; requires-guided/param still soft |
| Full CONF-P2 / REQ-REFINE-2 / check-elision / FixPatch | Not this slice |
| `vc.rs` / Z3 path | **Not touched** (Fable dirty tree owns P2-SOUND on `vc.rs`) |

## Collision note

`git diff` at start showed dirty `vc.rs` (+`main.rs` soft help) only — Fable mid-flight soundness (`[P2-SOUND1]`/`[P2-SOUND2]`). This slice edits **`typecheck.rs` only** so it composes without rewriting VC encoder work.

## Verify

```powershell
cargo test -p vera --lib
# expect: refine1_rejects_out_of_range_literal_call + refine1_accepts_in_range_literal_call
powershell -File docs/pilot/soft_smoke.ps1
```

---

## [P2-REFINE1-DEF] definition-time return refine (2026-07-19)

**Marker:** `[P2-REFINE1-DEF]` · **File:** `crates/vera/src/typecheck.rs` · **Patcher:** `apply_p2_refine1_def.py`

### What landed

When return type is `{binder: Int | pred}` and the function body is a **closed** Int literal / unary-minus / closed `if` tree (no free names, no stmts), evaluate the body and **hard-reject** if `pred` is decidably false.

| Case | Result |
|------|--------|
| `fn bad() -> {r: Int | r >= 0} { -1 }` | `TypeError` + `[P2-REFINE1-DEF]` |
| `fn good() -> {r: Int | r >= 0} { 0 }` | Ok |
| closed `if 1 < 0 { 1 } else { -1 }` vs nonneg | reject |
| `fn id(x: Int) -> {r: Int | r >= 0} { x }` | soft (param-dependent) |

Unit tests: `refine1_def_rejects_negative_literal_return`, `refine1_def_accepts_nonneg_literal_return`, `refine1_def_rejects_closed_ite_false_branch`, `refine1_def_soft_on_param_dependent_body`.

### Honest limits (still deferred)

| Item | Status |
|------|--------|
| Requires-guided binds (e.g. `requires x == 5` then body `x`) | Soft — prove/runtime |
| Stmt / let dataflow bodies | Soft |
| Param-dependent arithmetic bodies | Soft (handoff B+ / SMT) |
| `vc.rs` | **Not touched** |

### Verify

```powershell
cargo test -p vera --lib -- refine1_def_
# expect: 4 passed
powershell -File docs/pilot/soft_smoke.ps1
# expect: 17 passed; prove_clamp 6 proved; SOFT-SMOKE PASS
```

Handoff **A** (REQ-REFINE-1 call-site + definition-time closed body) is now complete for the decidable closed fragment. Next logical plan step: handoff **B** (prove tiers in typecheck/CLI diagnostics).
