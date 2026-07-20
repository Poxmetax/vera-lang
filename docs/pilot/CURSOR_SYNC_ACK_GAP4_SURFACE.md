# Cursor sync ACK -- Fable landed GAP4-R2-SURFACE ([GAP4-R2-SURFACE], committed)

**Date:** 2026-07-20  
**Commit:** `658e14b` (TradingBot main)  
**Publish:** merge `34d7459` on GitHub main / vera-lang subtree  
**Soft review:** PASS -- [`CLAUDE_REVIEW_GAP4_SURFACE.md`](CLAUDE_REVIEW_GAP4_SURFACE.md) / pointer [`CLAUDE_POINTER_GAP4_SURFACE_REVIEW.md`](CLAUDE_POINTER_GAP4_SURFACE_REVIEW.md)  
**SoT:** [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md)

## What landed (do not overclaim)

| Claim | Status |
|-------|--------|
| Thin seeded label typecheck pass | **Yes** — `check_program_labels(program, seeds)` in `typecheck.rs`; wires `Label::flows_to` |
| E1 injection reject (named-fn arg vs param bound) | **Yes** — at call sites, seeded |
| E6 leak reject (`Console.print` vs ∅-data bound) | **Yes** — at call sites, seeded |
| Front door inert without seeds | **Yes** — `check_program` runs pass with EMPTY seeds (all labels ⊥) |
| Value-label syntax (`T^{...}`) | **NOT claimed** — seeds are test/API only; SPEC keeps syntax post-MVP |
| R2 inference-ergonomics gate | **NOT claimed / still OPEN** |
| Implicit flows / taint propagation / full IFC | **NOT claimed** |
| CONF-P2 "ill-labeled flows" on plain `.vera` text | **NOT claimed** — author cannot yet produce non-⊥ labels |
| FixPatch / GAP-D2 | **NOT touched** — FixPatch stays `ephemeral: true` |

One file: `crates/vera/src/typecheck.rs` (+319/−1). `label.rs` untouched. Backup on disk: `typecheck.rs.bak_20260720_040347_gap4_r2_surface` — **exclude** from commit.

## Baseline (at GAP4 land; soft re-ran 2026-07-20)

- `cargo test -p vera --lib` -> **56** passed; 0 failed (was 53; +3 `gap4_surface_*`; 0 regression)
- `powershell -File docs\pilot\soft_smoke.ps1` -> **SOFT-SMOKE PASS** (exit 0)
- `cargo run -p vera -- --prove examples/prove_clamp.vera` -> **6** proved; 0 runtime-checked; 0 refuted (exit 0)

Prior committed baseline: **[P2E-FIX]** `ddc3d6a` (suite 53); publish merge `3c72ce4`.

**Current soft baseline (post GAPC1):** **59** — see [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md).

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`
- Every `examples/*.vera` must typecheck
- Soft docs at GAP4 land expected **56** tests; live baseline now **59** after GAP-C1
- Do **not** claim full IFC, label syntax, or R2 ergonomics closed; do **not** open GAP-D2

## Close-out

1. ~~**Madis:** commit GAP4 surface~~ **DONE** — `658e14b`.
2. ~~Publish to `vera-github`~~ **DONE** — merge `34d7459`.
3. Follow-on: **[GAPC1-SYM-LEN]** landed + committed `4fbf7df` / publish `0bc3c22` — soft ACK [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md).
4. Remaining OPEN for labels: value-label annotation and/or R2 inference-ergonomics — Madis gates; not soft-steal.
