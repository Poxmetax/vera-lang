# P2C-LEN slice — REQ-REFINE-2 `len` measure + provably-OOB call reject

**Date:** 2026-07-19 · **Marker:** `[P2-REFINE2]` · **Files:** `crates/vera/src/typecheck.rs`, `crates/vera/src/interp.rs`, `examples/refine_len_ok.vera` (new)

## Plan alignment

- Handoff task **C**: `docs/pilot/FABLE5_CONF_P2C_HANDOFF_PROMPT.md` (REQ-REFINE-2 + `len`).
- SPEC: `docs/spec/SPEC.md` §4.4 REQ-REFINE-2 and §2.3 E3 (`nth` / measure note).

## What landed

1. **`len(xs)` usable in refinements as a measure, on the check path.** The pred
   already parsed (refinement preds are full exprs); it is now meaningful at all
   three tiers:
   - **typecheck** — can reject through it (below);
   - **prove** — `--prove` stays honestly `RUNTIME-CHECKED` (`Call` is outside
     the QF_LIA encode slice; no spurious REFUTED — `[P2-SOUND2]` non-closed
     args guard also applies);
   - **runtime** — the interpreter now evaluates `len(e)` inside refinement
     predicates (see 3), so the runtime-check tier actually works.
2. **Compile-time reject of a provably OOB literal index — zero execution.**
   Mechanism: `pred_holds_for_lit` now uses **Kleene three-valued `&&` / `||`**:
   a decided operand decides the connective even when the other side is
   unevaluable (the `len(xs)` conjunct). `nth(xs, -1)` fails `0 <= k`, so the
   whole pred is decidably false → `TypeError` at the call span, interpreter
   never starts. Soundness vs the runtime: the evaluable fragment
   (literal/binder/neg comparisons) never traps, so every compile-decided case
   agrees with the interpreter's short-circuit `&&`/`||`; if the unknown side
   would trap at runtime, the call could never succeed anyway
   (trap-or-violation either way) — rejecting earlier is a strengthening.
3. **Interpreter `len(e)` measure evaluation.** *Scope note:* `interp.rs` was
   not on the expected-C file list, but without this every valid len-measure
   call trapped at runtime (`unbound name "len"`) — the runtime tier would have
   been fake. Guarded so that a user `fn len` and a local `len` binding still
   win; the measure branch fires only where the old code unconditionally
   trapped, so **no previously-legal program changes behavior**.
4. **Diagnostics:** reject message carries `[P2-REFINE2]` when the pred mentions
   the `len` measure (`pred_mentions_len` helper), `[P2-REFINE1]` otherwise.
   `--diag-json` shape is **unchanged** (existing `TYPE-ERROR` code + structured
   span; no new codes — `P2B_DIAG_SLICE.md` SoT untouched).

| Case | Result |
|------|--------|
| `nth([10,20,30], -1)` | `TypeError` + `[P2-REFINE2]`, zero execution; `--diag-json`: `TYPE-ERROR` with span |
| `nth([10,20,30], 1)` | typechecks + runs, prints `20` (`examples/refine_len_ok.vera`) |
| `nth([10,20,30], 5)` | soft at typecheck (honest limit below); runtime refinement check traps |
| `let j: Int = 9; nth(..., j)` | soft (non-literal); runtime trap `refinement {k: Int | …} violated` |
| pred `k < 0 \|\| k < len(xs)`, k = −1 | `true \|\| unknown = true` → no reject (Kleene-`\|\|` guard) |
| pred `0 <= k && k < len(xs) && k <= 100`, k = 200 | `unknown && false = false` → reject |

Unit tests: **5** typecheck (`refine2_*`) + **3** interp (`len_measure_*`); suite **22 → 30**.

## Honest limits (deferred)

| Item | Status |
|------|--------|
| `nth(xs, xs.len())` — SPEC's symbolic "`len(xs)` itself" case | **Not rejected this slice** — needs symbolic same-term reasoning (span-insensitive AST equality + param→arg substitution). Stays soft → runtime trap. Deferred per handoff option 4. |
| Unbounded / non-literal index | Soft by design: `--prove` emits `call_requires` RUNTIME-CHECKED; the runtime refinement check traps on violation; total API `xs.get(i) -> Option<Int>` remains the safe path. This is the SPEC's "explicit runtime-checked assertion" leg. |
| Upper-bound literal vs actual list length (`nth([...], 5)`) | Soft — call-site does not reason about the argument list's length (no list-literal length propagation). |
| SMT encoding of `len` as an uninterpreted/measure function | Not needed for this slice; `encode_expr` still rejects `Call` → RuntimeChecked. |
| Refinement preds not statically type-validated | Pre-existing Phase 1 behavior (preds bypass `infer_expr`); a malformed pred (e.g. `len` of an Int) traps at runtime with a clear message. |
| `get_unchecked` | Still absent from MVP stdlib (SPEC §9); example body uses total `get` + `match`. |
| Negative example on disk | Intentionally not shipped: `examples/` must typecheck (CONF-P1 round-trip invariant, enforced by `round_trip_all_examples`). Reject cases live in unit tests. |

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib                                 # 30 passed (was 22)
cargo run -p vera -- examples/refine_len_ok.vera         # prints 20, exit 0
cargo run -p vera -- --prove examples/prove_clamp.vera   # still: 6 proved
powershell -File docs\pilot\soft_smoke.ps1               # SOFT-SMOKE PASS
# Reject demo (zero execution): change the index in refine_len_ok.vera to -1,
# then run plain or with --diag-json (TYPE-ERROR + [P2-REFINE2] + span).
```

Backups: `crates/vera/src/typecheck.rs.bak_20260719_230030_p2c_len`,
`crates/vera/src/interp.rs.bak_20260719_230030_p2c_len`.
