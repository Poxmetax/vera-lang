# GAPC2-SMT-LEN slice — `len` as an opaque measure constant in the VC path

**Date:** 2026-07-20 · **Marker:** `[GAPC2-SMT-LEN]` · **Files:** `crates/vera/src/vc.rs` only (+226/−0; `smt.rs` untouched — still the plain subprocess runner)

## What landed (P2C's deferred SMT leg, SPEC §4.4 "QF_LIA plus measures like `len`")

`len` now participates in the VC/SMT encode as an **opaque Int constant** with
the one measure axiom that is unconditionally true: `len(xs) >= 0`.

1. **Encode** (`encode_expr` + `len_sym`): the pred form `len(xs)` and the
   method form `xs.len()` of the SAME list map to ONE symbol
   `vera_len_<xs>`. Receivers/arguments must be plain `Name`s; every other
   `Call` shape stays unsupported (→ RUNTIME-CHECKED, as before).
2. **Declaration + axiom** (`collect_len_syms` / `collect_len_syms_block`):
   both discharge paths collect the symbols their exprs can reference and
   declare each once with `(assert (>= vera_len_<xs> 0))`. Len-free programs
   collect nothing — their SMT scripts stay **byte-identical** (prove_clamp
   still 6 proved, empirically re-run).
3. **Effect on fn-level obligations** (`discharge_goal`): a len-refined
   parameter used to kill the WHOLE obligation (its pred failed to encode as
   an assumption → ensures/return_refine RUNTIME-CHECKED even when the goal
   was len-free). Now the assumption is assertable, so e.g.
   `fn pick_nonneg(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int
   ensures result >= 0 { i }` is honestly **[PROVED]** (k = i, 0 <= k ⊢
   result >= 0), and `ensures result >= 0` over a body `xs.len()` is proved
   by the axiom alone.

**Soundness, both directions** (the [P2-SOUND1/2/3] habit):
- PROVED quantifies over ALL c >= 0 for the opaque length — a strict
  over-approximation of the one real length, so no fake PROVED.
- Every countermodel assigns some c >= 0, and a list of length c exists — so
  REFUTED remains genuine (pinned by test: `ensures result >= 1` over
  `xs.len()` comes back REFUTED, because the empty list IS the c = 0 model).

## What this slice does and does NOT claim

| Claim | Status |
|-------|--------|
| "SMT encode of `len` still open" (GAP-C2 register debt) | **Closed** — `len` encodes as an opaque measure constant with the `>= 0` axiom; at least one relevant prove path is honestly PROVED instead of a silent catch-all RUNTIME-CHECKED. |
| Relating `len(xs)` to list CONTENT or literal lists (`[1,2].len() == 2`) | **NOT claimed** — the constant is opaque; no list theory, no length-of-literal propagation. |
| Call-site discharge with list arguments | **NOT claimed** — [P2-SOUND2] closed-literal gate unchanged; such call sites stay RUNTIME-CHECKED (pinned by test). The call-site pred path still got the decl+axiom plumbing so the encode stays fail-safe honest if that gate ever widens. |
| Full REQ-REFINE-2 / general symbolic arithmetic | **NOT claimed** — GAP-C1's typecheck fragment and this encode are the two thin legs; everything else stays soft. |
| `/` `%` encode, list theory, quantifiers, `z3` crate linking | **NOT touched.** |
| Labels / IFC / value-label syntax / R2 ergonomics / GAP-D2 | **NOT touched** — GAP4 surface + FixPatch contracts preserved (suite pins them). |

## Honest limits

- The measure is per-NAME within one query: `vera_len_<xs>` identifies
  occurrences by variable name, which is exactly right inside a single fn
  scope (immutable bindings) and is never shared across obligations.
- A user binding shadowing patterns or exotic identifiers that sanitize to a
  colliding symbol would produce an invalid/duplicate declaration — Z3 then
  errors and the obligation falls back to RUNTIME-CHECKED (fail-safe, never
  fake-PROVED). Not reachable from parsed idents today.
- `prove_runtime_hint.vera` keeps its RUNTIME-CHECKED demo (Str body — no
  len involved); soft_smoke expectations unchanged.
- The GAP-C1 typecheck reject is untouched; GAP-C1's soft cases
  (`xs.len() - 1`, aliasing) did NOT silently become proved.

## Tests (+4, suite 59 -> 63)

- `vc::tests::gapc2_len_param_refine_assumption_enables_proved_ensures` —
  the headline: len-refined param no longer kills a len-free ensures; PROVED.
- `vc::tests::gapc2_len_body_nonneg_ensures_proved_by_axiom` — method form
  in the body; the `>= 0` axiom discharges `result >= 0`.
- `vc::tests::gapc2_refutable_len_ensures_stays_honest` — `result >= 1` is
  REFUTED (c = 0 realizable), and nothing in the program is Proved.
- `vc::tests::gapc2_call_site_len_args_stay_runtime_checked` — list-arg call
  sites keep the [P2-SOUND2] RUNTIME-CHECKED verdict; no Refuted anywhere.

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib            # 63 passed (was 59)
cargo test -p vera --lib gapc2_     # 4 passed
powershell -File docs\pilot\soft_smoke.ps1                 # SOFT-SMOKE PASS
cargo run -p vera -- --prove examples/prove_clamp.vera     # 6 proved (unchanged)
```

Backup: `crates/vera/src/vc.rs.bak_20260720_052207_gapc2_smt_len`.
