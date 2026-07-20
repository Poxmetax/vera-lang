# Cursor sync ACK -- Fable landed GAPC2-SMT-LEN ([GAPC2-SMT-LEN], committed)

**Date:** 2026-07-20  
**Commit:** `f8b67cc` (TradingBot main; `vc.rs` + slice)  
**Publish:** code merge `f4f3cc7` on GitHub main / vera-lang subtree (`bcce500..f4f3cc7`; C2 snapshot `48e4bb9`, then VL `4033cd2`); soft-docs merge `43c8064` (`b3a248f` snapshot)  
**Soft review docs:** `19c83a2` (TradingBot main; review/ACK/queue/checklist + VeraAgentBench scaffold)  
**Sibling:** GAP4-VALUE-LABEL `[GAP4-VALUE-LABEL]` commit `28929dc` (same code publish merge) — soft ACK [`CURSOR_SYNC_ACK_GAP4_VALUE_LABEL.md`](CURSOR_SYNC_ACK_GAP4_VALUE_LABEL.md)  
**Soft review:** PASS -- [`CLAUDE_REVIEW_GAPC2_SMT_LEN.md`](CLAUDE_REVIEW_GAPC2_SMT_LEN.md) / pointer [`CLAUDE_POINTER_GAPC2_REVIEW.md`](CLAUDE_POINTER_GAPC2_REVIEW.md)  
**SoT:** [`GAPC2_SMT_LEN_SLICE.md`](GAPC2_SMT_LEN_SLICE.md)

## What landed (do not overclaim)

| Claim | Status |
|-------|--------|
| `len(xs)` / `xs.len()` as opaque Int SMT constant | **Yes** — `vera_len_<xs>` + axiom `>= 0` in `vc.rs` |
| Len-refined param assumptions assertable | **Yes** — enables honest PROVED on relevant ensures |
| REFUTED stays genuine for unrealizable len goals | **Yes** — `result >= 1` over `xs.len()` REFUTED |
| Call-site list-arg discharge | **NOT claimed** — stays RUNTIME-CHECKED ([P2-SOUND2]) |
| List theory / literal-length propagation | **NOT claimed** |
| Full REQ-REFINE-2 / `/` `%` / quantifiers | **NOT claimed** |
| Labels / IFC / value-label / R2 / GAP-D2 | **NOT touched** |

One file: `crates/vera/src/vc.rs` (+226/−0). `smt.rs` untouched. Backup: `vc.rs.bak_20260720_052207_gapc2_smt_len` — **exclude** from commit.

## Baseline (soft re-ran 2026-07-20)

At C2 land the suite was **63** (was 59; +4 `gapc2_*`). After sibling VALUE-LABEL the live suite is **68**:

- `cargo test -p vera --lib` -> **68** passed; 0 failed
- `cargo test -p vera --lib gapc2_` -> **4** passed
- `powershell -File docs\pilot\soft_smoke.ps1` -> **SOFT-SMOKE PASS** (exit 0)
- `cargo run -p vera -- --prove examples/prove_clamp.vera` -> **6** proved; 0 runtime-checked; 0 refuted (exit 0)

Suite chain (honest): **56** (GAP4 surface) → **59** (C1 `4fbf7df`) → **63** (C2 `f8b67cc`) → **68** (VL `28929dc`).

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`
- Every `examples/*.vera` must typecheck
- Soft docs expect **68** tests (post VL)
- Do **not** claim list theory or that GAP-C1 soft cases became proved

## Close-out

1. ~~**Madis / Fable:** commit GAP-C2~~ **DONE** — `f8b67cc`.
2. ~~Publish to `vera-github`~~ **DONE** — merge `f4f3cc7`.
3. Soft review/ACK/queue/checklist sync — this commit.
4. Next hard = Madis-gated (**GAP4-R2-ERGO** / F6 / GAP-D2) — soft does **not** pick.
