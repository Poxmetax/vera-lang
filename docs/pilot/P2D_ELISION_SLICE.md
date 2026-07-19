# P2D-ELIDE slice — proof-gated runtime check elision (INV-1 / DP6)

**Date:** 2026-07-20 · **Marker:** `[P2D-ELIDE]` · **Files:** `crates/vera/src/vc.rs`, `interp.rs`, `main.rs`, `lib.rs` (export)

## Plan alignment

- Handoff task **D**: `docs/pilot/FABLE5_CONF_P2D_HANDOFF_PROMPT.md`.
- SPEC: DP6 ("a proof, when it exists, replaces the runtime check"), INV-1
  (elision is proof-gated, never speculative), CONF-P2 (">=1 contract
  SMT-proved end-to-end with its runtime check elided").

## What landed

1. **`ProvedSet` (`vc.rs`)** — fn-level PROVED obligations keyed structurally:
   `Obligation` gained two additive fields (`fn_name`, `ensures_index`) set at
   all 8 construction sites; fn-level obligations carry `Some`, call-site ones
   `None`. No string parsing of `target`. **Duplicate fn names are excluded
   wholesale** from the set: the interpreter resolves calls by name (last
   declaration wins), so a proof for one duplicate must never elide checks on
   the other (guard + test).
2. **Interpreter elision gates (`interp.rs`)** — `Interpreter::with_proved`
   arms a set; `new()` keeps an empty set (default = check everything).
   In `call_fn`:
   - PROVED `return_refine` -> the pred **eval** is skipped; the env inserts
     (binder + `result`) are kept, so observable behavior — including the
     pre-existing binder-shadows-param corner in later ensures — is unchanged.
   - PROVED `ensures[i]` -> that clause's eval is skipped; other clauses of
     the same fn still check.
   - `requires` and **param-refine entry checks are never elided**: the proofs
     assume them (they are the assumption base), so eliding them would be
     unsound.
   - `pub elided_checks` counts skips (instrumentation for tests + CLI line).
3. **CLI `--prove-run` (`main.rs`)** — opt-in run-after-prove: prints the
   normal prove report, then (a) any REFUTED -> `[P2D-ELIDE] refuted
   obligation(s) -- not running`, **exit 3, program never runs**; (b) prove
   error -> exit 1; (c) otherwise arms the set (`[P2D-ELIDE] proof-gated
   elision armed: N fn-level obligation(s)`), runs, and reports
   `[P2D-ELIDE] elided M runtime check(s)` on stderr.
   **HR1 behavior:** default `vera file.vera` builds no set (zero elision,
   byte-identical); `--prove` alone still never runs and **takes precedence**
   over `--prove-run` when both are given.

## Soundness argument (why elision is correct)

- A fn-level proof (`ensures` / return refine) is discharged under the
  assumptions *param refines + requires* (`assert_param_refines` +
  `assert_requires` in `discharge_goal`). Those assumption checks stay active
  at runtime entry, so every elided check's premise is still established.
- Encoding faithfulness: `/` and `%` are outside the fragment ([P2-SOUND1]),
  calls/`?` fail encoding -> such fns are RuntimeChecked, never in the set.
  Overflow: SMT proves over unbounded Int; if the runtime body would overflow
  it traps *before* the elided return check is reached, so no elided check
  masks an overflow.
- INV-1: the set is built only from `Discharge::Proved` obligations of
  `prove_program` on the **same `Program` value in the same process**; nothing
  is persisted, so there is no stale-certificate path (a durable store is the
  deferred INV-2 surface).

## Demo (real output, 2026-07-20)

```
$ vera --prove-run examples/prove_clamp.vera
... 6 proved ...
[P2D-ELIDE] proof-gated elision armed: 3 fn-level obligation(s)
prove_clamp / 5 / 0 / 10
[P2D-ELIDE] elided 9 runtime check(s)        # 3 calls x (2 ensures + 1 refine)
$ vera --prove-run examples/prove_refuted.vera
... 1 refuted ...
[P2D-ELIDE] refuted obligation(s) -- not running   # exit 3, zero execution
```

Unit tests (4): `elide_skips_proved_fn_level_checks` (3 skips + default-zero),
`elide_never_skips_unproved_ensures` (RuntimeChecked `/` body still traps),
`elide_never_skips_refuted_ensures`, `elide_excludes_duplicate_fn_names`.
Suite **30 -> 34**.

## Honest limits (deferred)

| Item | Status |
|------|--------|
| Call-site obligations (`call_requires`, `call_arg_refine`) | **Never elided** — the interpreter has no call-site identity; callee-side `requires` / param-refine entry checks always run. Sound but conservative (a proved literal call still pays the entry check). |
| Persistent proof certificates (INV-2 keying: content hash + solver version) | Not this slice — the set lives only for one process run by construction. |
| `--prove` + `--prove-run` together | `--prove` wins (prove-only, no run) — documented in `usage()`. |
| Refuted programs under `--prove-run` | Refuse to run (exit 3). Plain `vera file.vera` still runs them with full runtime checks (unchanged pre-D behavior). |
| Duplicate fn declarations | Pre-existing language gap: the typechecker does not reject duplicate fn names (last wins at runtime). The proved set excludes them defensively; a typecheck-level reject is a future slice (typecheck.rs, out of D scope). |
| Elision reporting | `elided_checks` counts per-call skips (stderr line under `--prove-run` only). |

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib                                  # 34 passed (was 30)
cargo test -p vera --lib elide_                           # 4 passed
cargo run -p vera -- --prove examples/prove_clamp.vera    # unchanged: 6 proved, no run
cargo run -p vera -- --prove-run examples/prove_clamp.vera    # armed 3, elided 9, exit 0
cargo run -p vera -- --prove-run examples/prove_refuted.vera  # refuted -> exit 3, no run
powershell -File docs\pilot\soft_smoke.ps1                # SOFT-SMOKE PASS
```

Backups: `crates/vera/src/{vc,interp,main,lib}.rs.bak_20260720_000130_p2d_elide`.
`--diag-json` schema untouched (`diag.rs` not edited; new `Obligation` fields
are not serialized — `diagnostic_from_obligation` reads fields by name).
