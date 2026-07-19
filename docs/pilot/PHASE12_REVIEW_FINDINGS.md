# VERA Phase 1/2 — independent review findings

**Date:** 2026-07-19 · **Reviewer:** Claude (Fable 5, fresh session) · **Scope:** commits `f9e7db4` (Phase 1) + `5f3692c` (Phase 2 thin VC slice), per `FABLE5_CONF_P2_HANDOFF_PROMPT.md` ("independent review first, fold soundness findings before A–E").

**Method:** full read of all 11 `crates/vera/src/*.rs` modules, all 9 examples, pilot/spec docs; `cargo test -p vera`; all 9 examples executed; `--prove` / `--round-trip` exercised; 6 adversarial probe programs written to a session scratchpad (not committed) and run against the toolchain.

---

## Verdict

**Phase 1 core: solid.** All 9 examples run (exit 0, correct outputs), round-trip test covers all examples, edit transactions (stale-base reject + typecheck-gated commit) verified by tests, typed holes parse and are rejected at typecheck (probe), statement-position `?` works. Pilot/spec documentation is honest and cross-checks against its own logs (`pytest.txt`: `21 passed, 2 xfailed` — matches REPORT.md).

**Phase 2 slice: two soundness-class defects found, both empirically confirmed and both FIXED in this session** ([P2-SOUND1], [P2-SOUND2], each a ≤30-line surgical change + regression guards). Three medium defects and assorted minors remain **open** for operator decision.

Post-fix smoke: `cargo test -p vera` → **7 passed / 0 failed**; `--prove examples/prove_clamp.vera` → **6 proved, 0 runtime-checked, 0 refuted, exit 0** (handoff expectation preserved).

---

## Confirmed findings

### F1 — CRITICAL (FIXED): `--prove` returned PROVED for an obligation the runtime then violates

SMT-LIB `div`/`mod` are Euclidean; the interpreter truncates toward zero (`checked_div`, `%` — `interp.rs:772-783`). `vc.rs` encoded `/`→`(div …)`, `%`→`(mod …)`, so Z3 proved properties that fail at runtime for negative operands.

Probe (`demoA`): `fn half_leq(x: Int) -> {r: Int | r * 2 <= x} { x / 2 }` called with `-7`:
- pre-fix `--prove`: `[PROVED] half_leq return refine` — exit 0
- plain run: `trap: refinement {r: Int | …} violated` — exit 2

This inverts DP6/INV-1 ("a proof replaces the runtime check"): had proof-gated check elision (task D) landed first, this would have produced silently wrong values.

**Fix [P2-SOUND1]** (`vc.rs` `encode_expr`): `/` and `%` removed from the encodable fragment → such obligations stay `[RUNTIME-CHECKED]` (tier-4 per SPEC §4.4). Guard test: `div_stays_runtime_checked`. Post-fix probe: `[RUNTIME-CHECKED] … binop / not in SMT slice`, exit 0; runtime trap still catches the violation (sound).

*Proper re-inclusion later:* encode truncating division explicitly (`ite`-based sign correction) with differential tests, or switch runtime to Euclidean semantics (language design decision).

### F2 — HIGH (FIXED): spurious REFUTED (exit 3) for any open (variable) argument at a contracted call site

`prove_call_site` encoded `Expr::Name` args as bare symbols. `discharge_call_pred` declares only callee params, so a caller variable reached Z3 undeclared (Z3 prints `(error …)` for that assert and continues) — or, if the caller variable shadows the callee param name, as an unconstrained tautology. Either way the refinement query answered `sat` → `[REFUTED]`, exit 3, on perfectly valid programs.

Probes (`demoB`, `demoB2`): `pos_id(v)` / `pos_id(x)` with value 5 against `{x: Int | x >= 1}`:
- pre-fix `--prove`: `[REFUTED] main → pos_id arg `x` refine — sat`, exit 3
- plain run: prints `5`, exit 0

The slice report's phrase "when arguments encode as **closed** SMT terms" described intent, not code — no closedness check existed.

**Fix [P2-SOUND2]** (`vc.rs`): `expr_is_closed` gate (literals + unary/binop over literals); open args → `[RUNTIME-CHECKED] … argument is not a closed literal term`. Literal call sites (the REQ-REFINE-1 shape, e.g. future `apply_discount(100, 150)`) still refute correctly; `prove_clamp` still proves 6/6. Guard test: `open_call_args_stay_runtime_checked`.

### F3 — MEDIUM (FIXED 2026-07-19, `[P2-SOUND3]`): typechecker admits `?` in functions whose return type cannot carry the propagation

`Expr::Propagate` unwraps Option/Result but never checks the **enclosing** function's return type. Probe (`demoC2`): `fn first_or_trap(x: Option<Int>) -> Int { let y: Int = x?; y }` typechecks; at runtime `first_or_trap(None)` returns `None` bound as an `Int`, and the program traps later with the misleading `show() expects Int receiver`. Static type soundness hole (value of wrong type escapes through a checked signature).

*Proposed fix (~25 lines, typecheck.rs):* thread the enclosing `ret` type into expression checking; `?` on `Option<T>` requires ret `Option<_>`, on `Result<T,E>` requires ret `Result<_, E'>` with matching `E` (Rust rule). Examples stay green (`propagate.vera` conforms).

**FIXED (same day):** implemented as `[P2-SOUND3]` — `Env.ret` threaded through all environment constructions; `Expr::Propagate` now requires a compatible enclosing return type (Option→Option, Result→Result with equal error type; `?` in an unannotated lambda is rejected with a fix hint). Guard tests: `propagate_into_plain_int_ret_is_rejected`, `propagate_into_option_ret_is_ok`, `propagate_result_err_mismatch_is_rejected`. The demoC2 probe now fails typecheck instead of trapping at runtime. Related: REQ-REFINE-1 call-site literal half landed as `[P2-REFINE1]` (Cursor soft session, reviewed + extended by Fable with negative-literal support and a guard test). The definition-time half landed same day as `[P2-REFINE1-DEF]` (closed literal / unary-minus / closed-`if` bodies; Cursor patcher `apply_p2_refine1_def.py`, independently reviewed by Fable — conservative `Some(false)`-only reject, no arithmetic in the closed fragment, param-dependent bodies stay soft). Handoff task A is complete for the decidable closed fragment; suite at 17 tests.

### F4 — MEDIUM (OPEN): nested `?` mis-evaluates at runtime

`Value::EarlyReturn` is only intercepted at let/statement boundaries, match scrutinees, and function return. `?` nested in a larger expression breaks: probe (`demoC`) `Some(xs.head()? + 1)` with `xs = []` traps `bad op +` (exit 2) instead of returning `None`. Same class: `f(x?)` passes an `EarlyReturn` value *into* the callee as a parameter; `console.print(s?)` would Debug-print the internal `EarlyReturn` representation.

*Fix options:* (a) propagate `EarlyReturn` at every operand/argument evaluation site in `interp.rs` (mechanical, ~15 sites, >30 lines → needs approval); (b) restrict `?` syntactically to statement/let/scrutinee position (smaller, but narrows the documented surface). Recommend (a).

### F5 — MEDIUM (OPEN): canonical renderer is precedence-blind → round-trip fails on parenthesized source

`render.rs` never parenthesizes (`_prec` is unused), and the parser does not preserve parens as AST nodes. Probe (`demoD`): `let a: Int = (1 + 2) * 3;` renders as `1 + 2 * 3` → reparse yields a different AST → `--round-trip` **FAIL** (hash mismatch, exit 1). Failure is *loud* (the hash check works as designed — nothing silent), but CONF-P1 "round-trip identity" currently holds only for programs whose AST shape already matches precedence, which the 9 examples happen to satisfy.

*Proposed fix (~60 lines, render.rs):* precedence-aware rendering (parenthesize child when its binding is weaker; parenthesize equal-precedence right children of left-associative operators to preserve AST shape) + round-trip tests over nasty expressions. Needs approval (>30 lines).

### F6 — LOW (OPEN): minors

- `==`/`!=` typecheck as `Bool` for **any** operand type pair (`typecheck.rs:339`) — `1 == "a"` is well-typed, always-false.
- Refinement predicates (`{x: Int | pred}`) are never statically typechecked (only requires/ensures are); an ill-typed pred is a runtime/VC-encode failure.
- Bare method access without call (`xs.len`) typechecks as the receiver type (`typecheck.rs:355-360`) but traps at runtime ("bare method").
- A parameter literally named `result` collides with the VC result binder (`build_query` asserts `(= result __result)`).
- `match` exhaustiveness is enforced for Option/Result/enums but not for Int/Str/Bool scrutinees (runtime `no arm matched` trap possible without `_`).
- Renderer emits strings via Rust `Debug` escapes; a string containing e.g. `\r` renders as an escape the lexer does not accept → loud round-trip fail.
- Z3 `-T:5` timeout output ("timeout" line) surfaces as `BadOutput` → mapped to RUNTIME-CHECKED via the error path (conservative, acceptable; slightly opaque reason text).

### F7 — DOC (OPEN): missing evidence file `smt_refine_spike.py`

Referenced three times (`SMT_SPIKE_REPORT.md:4,46`, `FABLE5_CONF_P2_HANDOFF_PROMPT.md:33`, `PHASE2_VC_SLICE_REPORT.md:41`) but **absent from the tree** — the spike's "re-run" instruction is not executable. Options: recreate the script (clearly labeled as a recreation) or amend the reports to state the script was not committed. Operator decision; do **not** silently backfill it as if original.

---

## Changes applied in this session (uncommitted; operator reviews & commits)

| File | Change | Size |
|---|---|---|
| `crates/vera/src/vc.rs` | [P2-SOUND1] `/`,`%` excluded from SMT encoding | ~6 lines |
| `crates/vera/src/vc.rs` | [P2-SOUND2] `expr_is_closed` gate at call sites | ~29 lines |
| `crates/vera/src/vc.rs` | 2 regression guard tests (no Z3 dependency) | ~52 lines (test-only) |
| `docs/pilot/PHASE2_VC_SLICE_REPORT.md` | honest-limit bullets for both fixes | ~6 lines |
| `docs/pilot/PHASE12_REVIEW_FINDINGS.md` | this document | new file |

Each code change is an independent ≤30-line surgical diff per the handoff rule; the two test guards are additive. Rollback: `git checkout -- vera-lang/crates/vera/src/vc.rs vera-lang/docs/pilot/PHASE2_VC_SLICE_REPORT.md` (restores `5f3692c`).

**Post-fix smoke evidence (2026-07-19):**

```
cargo test -p vera                       → 7 passed; 0 failed
--prove examples/prove_clamp.vera        → 6 proved, 0 runtime-checked, 0 refuted (exit 0)
--prove demoA (div probe)                → 1 runtime-checked (was: falsely PROVED)
--prove demoB/B2 (open-arg probes)       → 1 runtime-checked (was: falsely REFUTED, exit 3)
all 9 examples                           → exit 0, outputs correct
```

---

## Impact on CONF-P2 task order (handoff A–E)

- **A (REQ-REFINE-1 hard reject):** unblocked — with F2 fixed, a REFUTED verdict is trustworthy for literal call sites, which is exactly the REQ-REFINE-1 shape. F3 should land with/before A (both touch typecheck's relationship to refinements).
- **B (prove → diagnostics):** unchanged.
- **C (REQ-REFINE-2 / `len` measures):** unchanged; note QF_LIA fragment limits.
- **D (proof-gated check elision):** **was the dangerous one** — with F1 unfixed, elision would have deleted the runtime check that was masking the unsoundness. Safe to attempt only after F1-class differential tests exist (OLD-checks vs NEW-elides comparison on negative-operand arithmetic).
- **E (FixPatch JSON):** unchanged.
