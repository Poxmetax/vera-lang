# Cursor sync ACK -- Fable landed GAPC1-SYM-LEN ([GAPC1-SYM-LEN], committed)

**Date:** 2026-07-20  
**Commit:** `4fbf7df` (TradingBot main; code + slice + KNOWN_GAPS + README promo defer)  
**Publish:** code merge `0bc3c22` (`9fee15d` snapshot); soft-docs merge `4181371` (`a90d7f7` snapshot) on GitHub main  
**Soft review docs:** `7cf813f` (TradingBot main; review/ACK/queue/checklist); hash-fill `3550dad`  
**Soft review:** PASS -- [`CLAUDE_REVIEW_GAPC1_SYM_LEN.md`](CLAUDE_REVIEW_GAPC1_SYM_LEN.md) / pointer [`CLAUDE_POINTER_GAPC1_REVIEW.md`](CLAUDE_POINTER_GAPC1_REVIEW.md)  
**SoT:** [`GAPC1_SYM_LEN_SLICE.md`](GAPC1_SYM_LEN_SLICE.md)

## What landed (do not overclaim)

| Claim | Status |
|-------|--------|
| Same-term `nth(xs, xs.len())` compile-time reject | **Yes** — `check_sym_len_arg_refine` + param→arg subst + reflexivity under Kleene `&&`/`||` |
| Marker `[GAPC1-SYM-LEN]` on TypeError | **Yes** — zero execution on reject |
| Soft: `xs.len()-1`, other-list `.len()`, aliases, `k < 0 \|\| k < len(xs)` | **Design** — stay soft → prove / runtime (not bugs) |
| Full REQ-REFINE-2 / general symbolic arithmetic | **NOT claimed** |
| SMT `len` measure encode (`--prove` discharges len bounds) | **NOT claimed** — GAP-C2 OPEN |
| List-literal length propagation | **NOT claimed** |
| Non-`Name` receivers (`f().len()`, fields) | **NOT claimed** — excluded on purpose |
| FixPatch / GAP-D2 / GAP4 surface / labels | **NOT touched** |

One file: `crates/vera/src/typecheck.rs` (+191/−0). Backup on disk: `typecheck.rs.bak_20260720_044832_gapc1_sym_len` — **exclude** from commit.

## Baseline (soft re-ran 2026-07-20)

- `cargo test -p vera --lib` -> **59** passed; 0 failed (was 56; +3 `gapc1_*`; 0 regression)
- `cargo test -p vera --lib gapc1_` -> **3** passed
- `powershell -File docs\pilot\soft_smoke.ps1` -> **SOFT-SMOKE PASS** (exit 0)
- `cargo run -p vera -- --prove examples/prove_clamp.vera` -> **6** proved; 0 runtime-checked; 0 refuted (exit 0)

Prior committed baseline: **[GAP4-R2-SURFACE]** `658e14b` (suite 56); publish merge `34d7459`.

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`
- Every `examples/*.vera` must typecheck
- Soft docs expect **59** tests
- Do **not** claim full REQ-REFINE-2, GAP-C2, or that soft cases are defects

## Next

1. Soft docs commit + push (this ACK + review/queue/checklist) — Madis asked commit+push.
2. **Next hard task = TBD Madis-gated** (GAP-C2 or value-label / R2 ergonomics) — soft does **not** pick.
3. Do not open GAP-D2 unless Madis switches.
