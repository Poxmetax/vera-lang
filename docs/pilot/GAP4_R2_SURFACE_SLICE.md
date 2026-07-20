# GAP4-R2-SURFACE slice — thin label typecheck surface (seeded checker pass)

**Date:** 2026-07-20 · **Marker:** `[GAP4-R2-SURFACE]` · **Files:** `crates/vera/src/typecheck.rs` only (`label.rs` untouched — lattice reused as-is)

## What landed (handoff option 1: checker-internal pass, no grammar change)

First CHECKER-integrated evidence for SPEC §4.2 (SUB-LABEL): explicit-flow
E1/E6-shaped rejects now fire from inside the typecheck walk, not only from
lattice unit math (`[GAP4-R2-PILOT]`).

- `check_program_labels(program, seeds)` — seeded label pass. Seeds map
  `(fn name, binding name) -> Label` and provide BOTH source labels and sink
  bounds (the language has no value-label syntax yet).
- Enforcement points (both compare the DATA projection, per SPEC's sink
  wording "bounds at ∅-data"):
  1. named-fn call arguments vs the callee parameter's seeded upper bound via
     `Label::flows_to` — **E1 injection shape** (`db.insert`-style ∅-data param);
  2. `Console.print` arguments vs the ∅-data bound — **E6 leak shape**
     (SPEC §4.2's verbatim example).
- One-hop source rule: a bare `Name` argument carries its seeded label; every
  other expression shape is ⊥ (no taint propagation through computation).
- Front door: `check_program` now ends with the pass on EMPTY seeds — inert
  by the lattice laws (every label ⊥, and ⊥ ⊑ ⊥), re-proven empirically by
  the full suite + `round_trip_all_examples` (every committed example still
  typechecks).
- The walk covers all expression positions (nested blocks, if/match arms,
  lambda bodies, ctor/struct/list arguments, `?`-propagate), so a call site
  inside nested control flow is still checked.

Diff: `typecheck.rs` +319/−1 (≈226 lines pass+docs, ≈90 lines tests, 3 lines
import/wiring). The >~30-line size and the option choice (A: checker-internal
pass over B: value-label syntax) were approved by Madis before writing.

## What this slice does and does NOT claim

| Claim | Status |
|-------|--------|
| GAP-4's "no checker surface" remainder | **Closed** — E1/E6-shaped explicit-flow rejects execute inside typecheck via `Label::flows_to`, on real parsed programs. |
| Value-label syntax (`T^{...}`) | **NOT claimed** — no lexer/parser/AST/render change; SPEC keeps `T^{...}` post-MVP and `uses` remains the only authority surface. Labels reach the checker via seeds (tests/API). |
| R2 label-**inference ergonomics** gate | **NOT claimed / still OPEN** — no inference, no corpus measurement. |
| CONF-P2 "ill-labeled flows rejected" on plain source text | **NOT claimed** — a `.vera` author cannot yet produce a non-⊥ label; E5 endorse path untouched. This slice = checker-reject evidence on seeded programs. |
| Taint propagation through computation | **NOT claimed** — one-hop Name-argument rule only; `taint_prop` stays pilot-tested math. |
| Implicit flows | **NOT claimed** — SPEC's own [UNVERIFIED/OPEN] item, untouched. |
| Policies / quarantine / `infer` / actors / endorse / declassify | **NOT touched.** |
| GAP-D2 durable store / FixPatch changes | **NOT touched** — FixPatch stays additive + `ephemeral: true` (P2E contract; suite pins it). |

## Honest limits

- **Seeds are a test/API surface, not a language surface.** Nothing a `.vera`
  author writes today produces a non-⊥ label; the demonstrable rejects live
  in the three unit tests. The natural next fragment (Madis gates it) is a
  minimal value-label annotation surface that feeds this same pass.
- `.print` field-calls are treated as `Console.print` without re-typing the
  receiver — in a front-door-typechecked MVP program only `Console` has
  `.print`; on standalone use the overmatch is conservative-sound.
- Copies do not propagate: `let y = x;` drops `x`'s seeded label (one-hop
  rule). Documented, not claimed otherwise.
- Authority atoms never trip data bounds (data-projection comparison) — the
  `uses` clause remains the authority mechanism; an `Auth`-labeled handle
  passes through ∅-data bounds (pinned by test).

## Tests (+3, suite 53 -> 56)

- `typecheck::tests::gap4_surface_rejects_untrusted_arg_into_bare_param_e1` —
  front door stays green (empty seeds), then a `{untrusted}`-seeded argument
  vs a ⊥-bound parameter rejects; message pins argument name, data atoms,
  parameter and callee.
- `typecheck::tests::gap4_surface_rejects_secret_arg_into_console_print_e6` —
  `{secret}`-seeded value vs `Console.print`'s ∅-data bound rejects.
- `typecheck::tests::gap4_surface_accepts_bounded_sink_and_auth_handle` —
  accept pair: `{secret}` flows into a `{secret}`-bounded parameter
  (`net.connect(auth:)` shape); an `Auth` atom passes a ⊥-data bound
  (data-only enforcement pinned).

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib                # 56 passed (was 53)
cargo test -p vera --lib gap4_surface   # 3 passed
powershell -File docs\pilot\soft_smoke.ps1                 # SOFT-SMOKE PASS
cargo run -p vera -- --prove examples/prove_clamp.vera     # 6 proved (unchanged)
```

Backup: `crates/vera/src/typecheck.rs.bak_20260720_040347_gap4_r2_surface`.
