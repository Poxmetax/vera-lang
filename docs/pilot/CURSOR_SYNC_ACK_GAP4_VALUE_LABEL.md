# Cursor sync ACK -- Fable landed GAP4-VALUE-LABEL ([GAP4-VALUE-LABEL], committed)

**Date:** 2026-07-20  
**Commit:** `28929dc` (TradingBot main; code + slice)  
**Publish:** code merge `f4f3cc7` on GitHub main / vera-lang subtree (`bcce500..f4f3cc7`; VL snapshot `4033cd2`); soft-docs merge `43c8064` (`b3a248f` snapshot)  
**Soft review docs:** `19c83a2` (TradingBot main; review/ACK/queue/checklist + VeraAgentBench scaffold)  
**Sibling:** GAP-C2 `[GAPC2-SMT-LEN]` commit `f8b67cc` (same code publish merge) — soft ACK [`CURSOR_SYNC_ACK_GAPC2.md`](CURSOR_SYNC_ACK_GAPC2.md)  
**Soft review:** PASS -- [`CLAUDE_REVIEW_GAP4_VALUE_LABEL.md`](CLAUDE_REVIEW_GAP4_VALUE_LABEL.md) / pointer [`CLAUDE_POINTER_GAP4_VALUE_LABEL_REVIEW.md`](CLAUDE_POINTER_GAP4_VALUE_LABEL_REVIEW.md)  
**SoT:** [`GAP4_VALUE_LABEL_SLICE.md`](GAP4_VALUE_LABEL_SLICE.md)

## What landed (do not overclaim)

| Claim | Status |
|-------|--------|
| Postfix `T^{untrusted\|secret}` at fn-param + annotated-let | **Yes** — lexer `^`; AST `Param.label` / `Stmt::Let.label`; parse canonicalize |
| Render round-trip of labels | **Yes** — canonical atom order |
| Unlabeled serialization / store hash stability | **Yes** — `skip_serializing_if` empty labels |
| `collect_label_seeds` → existing GAP4-R2-SURFACE | **Yes** — E1/E6/secret-bound from plain source |
| R2 inference-ergonomics gate | **NOT claimed / still OPEN** |
| Full IFC / taint / implicit flows | **NOT claimed** |
| Labels on returns / nested types / lambdas | **NOT claimed** |
| Auth atoms in `^{...}` | **NOT claimed** |
| FixPatch / GAP-D2 | **NOT touched** — FixPatch stays `ephemeral: true` |
| GAP-C2 SMT `len` | **NOT this slice** — sibling `f8b67cc` |

Files: `lexer.rs`, `ast.rs`, `parser.rs`, `render.rs`, `typecheck.rs`. `label.rs` untouched. Backups `*.bak_20260720_053453_gap4_value_label` — **exclude** from commit.

## Baseline (soft re-ran 2026-07-20)

- `cargo test -p vera --lib` -> **68** passed; 0 failed (was 63; +5 `gap4vl_*`; 0 regression)
- `cargo test -p vera --lib gap4vl_` -> **5** passed
- `powershell -File docs\pilot\soft_smoke.ps1` -> **SOFT-SMOKE PASS** (exit 0)
- `cargo run -p vera -- --prove examples/prove_clamp.vera` -> **6** proved; 0 runtime-checked; 0 refuted (exit 0)

Suite chain: **59** (C1) → **63** (C2) → **68** (this VL). Prior committed baseline before C2/VL: GAP-C1 `4fbf7df` (59).

## Soft-track rules

- Do **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`, `parser.rs`, `ast.rs`, `lexer.rs`
- Every `examples/*.vera` must typecheck (no labeled reject demos under `examples/`)
- Soft docs expect **68** tests
- Do **not** claim full IFC or R2 ergonomics closed; do **not** open GAP-D2

## Close-out

1. ~~**Madis / Fable:** commit VALUE-LABEL~~ **DONE** — `28929dc`.
2. ~~Publish to `vera-github`~~ **DONE** — merge `f4f3cc7`.
3. Soft review/ACK/queue/checklist sync — this commit.
4. Remaining OPEN for labels: **GAP4-R2-ERGO** (Madis-gated). Next hard also candidates: F6 / GAP-D2 — soft does **not** pick.
