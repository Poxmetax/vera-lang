# Cursor sync ACK -- Fable landed GAP4-R2-SURFACE ([GAP4-R2-SURFACE], awaiting Madis commit)

**Date:** 2026-07-20  
**Commit:** *(none yet -- working tree only; Madis commits after review)*  
**Soft review:** soft re-verify PASS (this ACK)  
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
| CONF-P2 "ill-labeled flows rejected" on plain `.vera` text | **NOT claimed** — author cannot yet produce non-⊥ labels |
| FixPatch / GAP-D2 | **NOT touched** — FixPatch stays `ephemeral: true` |

One file: `crates/vera/src/typecheck.rs` (+319/−1). `label.rs` untouched. Backup on disk: `typecheck.rs.bak_20260720_040347_gap4_r2_surface` — **exclude** from commit.

## Baseline (soft re-ran 2026-07-20)

- `cargo test -p vera --lib` -> **56** passed; 0 failed (was 53; +3 `gap4_surface_*`; 0 regression)
- `powershell -File docs\pilot\soft_smoke.ps1` -> **SOFT-SMOKE PASS** (exit 0)
- `cargo run -p vera -- --prove examples/prove_clamp.vera` -> **6** proved; 0 runtime-checked; 0 refuted (exit 0)

Prior committed baseline: **[P2E-FIX]** `ddc3d6a` (suite 53); publish merge `3c72ce4` on GitHub main / vera-lang subtree.

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`
- Every `examples/*.vera` must typecheck
- Soft docs expect **56** tests
- Do **not** claim full IFC, label syntax, or R2 ergonomics closed; do **not** open GAP-D2

## Next

1. **Madis:** review SoT + this ACK; re-run `cargo test -p vera --lib` (and optionally `soft_smoke.ps1`) then commit GAP4 surface code+docs; **exclude** `*.bak_*` backups.
2. Optional: soft push to `vera-github` after commit (Madis decides).
3. Remaining hard surface for labels: value-label annotation (feeds same pass) and/or R2 inference-ergonomics measurement — Madis gates; not soft-steal.
