# GAPC1-SYM-LEN slice — symbolic same-term len-as-index compile-time reject

**Date:** 2026-07-20 · **Marker:** `[GAPC1-SYM-LEN]` · **Files:** `crates/vera/src/typecheck.rs` only (+191/−0)

## What landed (P2C's deferred symbolic case, REQ-REFINE-2)

`nth(xs, xs.len())` — SPEC's own "`len(xs)` itself as the index" case — is now
a compile-time `TypeError` with zero execution. P2C could only decide literal
indices; this slice adds the one symbolic fragment the SPEC names:

1. **Shape gate** (`check_sym_len_arg_refine`, hooked beside
   `check_lit_arg_refine` in the named-fn call path): the argument must be
   exactly `<name>.len()` and the refined parameter's predicate is evaluated
   under the substitution `k := len(xs_param)`.
2. **param→arg substitution:** every callee parameter that receives the SAME
   variable as the `.len()` receiver instantiates the predicate's
   `len(<that param>)` to the argument's own value. Receiver and list
   argument must both be plain `Name`s — immutable bindings denote the same
   value at both positions (effect-free), which is what makes the same-term
   claim sound. Anything else stays soft.
3. **Reflexivity decision** (`pred_holds_for_sym_len` + `sym_len_term`): a
   comparison decides only when BOTH sides denote the same symbolic value
   (the binder under substitution, or the `len(xs_param)` measure call):
   `<` / `>` / `!=` → false, `<=` / `>=` / `==` → true. Mixed
   literal/symbolic sides stay unknown. Kleene `&&` / `||` / `!` combine
   exactly as in P2C's `pred_holds_for_lit`, so `0 <= k && k < len(xs)`
   becomes `unknown && false = false` → reject.

| Case | Result |
|------|--------|
| `nth_c1(data, data.len())`, pred `0 <= k && k < len(xs)` | `TypeError` `[GAPC1-SYM-LEN]`, zero execution |
| `nth_c2(a, a.len() - 1)` | soft (BinOp argument — not the bare same-term shape) |
| `nth_c2(a, b.len())` (different list) | soft (no param receives `b`) |
| pred `k < 0 \|\| k < len(xs)`, arg `data.len()` | soft — `unknown \|\| false = unknown` (Kleene guard; a full solver would reject, this slice does not pretend to be one) |

## What this slice does and does NOT claim

| Claim | Status |
|-------|--------|
| SPEC REQ-REFINE-2's symbolic `nth(xs, xs.len())` case rejected statically | **Closed** — same-term fragment, zero execution. |
| Full REQ-REFINE-2 / general symbolic arithmetic (`xs.len() - 1`, cross-term bounds) | **NOT claimed** — stays soft → prove / runtime (P2C design). |
| SMT `len` measure encode (`--prove` discharging len bounds) | **NOT claimed** — GAP-C2, untouched; `Call` stays outside the QF_LIA slice. |
| List-literal length propagation (`nth([1,2], 5)`) | **NOT claimed** — pre-existing P2C soft case, unchanged. |
| Non-`Name` receivers (`f().len()`, field access) | **NOT claimed** — excluded on purpose (effectful/composite receivers void the same-term argument). |
| Labels / IFC / value-label syntax / R2 ergonomics / GAP-D2 | **NOT touched** — GAP4 surface contracts preserved (suite pins them). |

## Honest limits

- Decision base is reflexivity on ONE shared term only; there is no linear
  arithmetic, no interval reasoning, no cross-conjunct inference. Mixed
  literal/symbolic comparisons are unknown by construction — so the Kleene
  `||` guard case above stays soft even though a real solver would reject it.
- The substitution is name-identity per call site; aliasing via re-binding
  (`let ys = xs; nth(xs, ys.len())`) is NOT recognized (stays soft → runtime
  trap). Documented, not claimed otherwise.
- Diagnostics reuse the existing `TYPE-ERROR` code + span (P2B contract); the
  message carries `[GAPC1-SYM-LEN]`. No new diag codes, no FixPatch for this
  error class.

## Tests (+3, suite 56 -> 59)

- `typecheck::tests::gapc1_rejects_len_of_same_list_as_index` — the SPEC
  case end-to-end from source text; message pins marker + `data.len()`.
- `typecheck::tests::gapc1_len_minus_one_and_other_list_stay_soft` —
  negative controls: BinOp argument and a different list's `len` both accept.
- `typecheck::tests::gapc1_kleene_or_guard_stays_soft` — pins the
  conservative `unknown || false = unknown` choice.

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib            # 59 passed (was 56)
cargo test -p vera --lib gapc1_     # 3 passed
powershell -File docs\pilot\soft_smoke.ps1                 # SOFT-SMOKE PASS
cargo run -p vera -- --prove examples/prove_clamp.vera     # 6 proved (unchanged)
```

Backup: `crates/vera/src/typecheck.rs.bak_20260720_044832_gapc1_sym_len`.
