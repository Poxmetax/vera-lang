# VERA-lang commit checklist (operator)

**Date:** 2026-07-19  
**Scope:** Next `vera-lang/` commit only. Soft track resumes post soft-FREEZE with a **no-rename** rule.

## Before commit (mandatory)

Run soft smoke from the TradingBot repo root (or any cwd; script resolves `vera-lang`):

```powershell
powershell -File vera-lang\docs\pilot\soft_smoke.ps1
```

Do this **immediately before** `git add` / commit — not earlier in the session and then forgotten.

### Expected smoke outcomes

| Check | Expect |
|-------|--------|
| `cargo test -p vera --lib` | **17** tests pass (soft-freeze baseline 7 + `[P2-REFINE1]`/`[P2-REFINE1-DEF]` 7 + `[P2-SOUND3]` 3) |
| `examples/prove_clamp.vera` | summary contains **`6 proved`**, exit **0** |
| `examples/prove_runtime_hint.vera` | at least one **`[RUNTIME-CHECKED]`**, exit **0** |
| `examples/prove_refuted.vera` | **`[REFUTED]`**, exit **3** |
| Script footer | `SOFT-SMOKE PASS` |

## Freeze / race lesson

- Soft track was **frozen** mid-review because parallel soft work raced Fable 5 CONF-P2.
- Madis resumed with **"hästi, jätka"** — continue carefully.
- **Never rename files** during soft work (especially `examples/`). No `_probe_*` temps that later get renamed; new files use **final names only**.
- **Do not edit** Fable-owned: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`.
- No parallel soft renames during review.

## Soft artifacts to include (from `git status --short -- vera-lang/`)

Paths as of 2026-07-19 soft resume (re-run `git status --short -- vera-lang/` before commit; do not invent):

### Soft-owned / soft-safe (include when committing soft slice)

- `vera-lang/crates/vera/src/main.rs` — soft CLI help / Z3 path / exit-code hints
- `vera-lang/README.md` — soft docs touch (verify diff is soft-only)
- `vera-lang/examples/README.md`
- `vera-lang/examples/prove_runtime_hint.vera`
- `vera-lang/examples/prove_refuted.vera`
- `vera-lang/docs/pilot/SOFT_PARALLEL_QUEUE.md`
- `vera-lang/docs/pilot/soft_smoke.ps1`
- `vera-lang/docs/pilot/COMMIT_CHECKLIST.md` (this file)
- `vera-lang/mcp/README.md` — Phase 3 MCP stub (docs only; `[SOFT-MCP-STUB]`)
- `vera-lang/docs/pilot/FABLE5_CONF_P2_HANDOFF_PROMPT.md` (handoff; ok in same commit if Madis wants docs together)
- `vera-lang/docs/pilot/PHASE12_REVIEW_FINDINGS.md`
- `vera-lang/docs/pilot/PHASE2_VC_SLICE_REPORT.md` (modified — include only if review agrees)

### Fable / hard-track — do **not** treat as soft

- `vera-lang/crates/vera/src/vc.rs` — **Fable-owned**; only commit if Madis/Fable CONF-P2 slice is intentional in the same commit

## Explicitly do **not** commit

- `crates/vera/src/*.bak_*` patcher backups (e.g. `typecheck.rs.bak_20260719_*_p2_refine1_def`) — keep on disk, exclude from commit
- TradingBot **mainnet** / TIER 1–3 runtime files
- Z3 unpack tree / binary install under `z3-*-win/`
- `.env` or any API keys / Telegram tokens
- Unrelated root-repo dirty files outside `vera-lang/`

## Soft track status

**Claude review (standing rule):** when pasting a review to Claude, use [CLAUDE_REVIEW_PROMPT_TEMPLATE.md](CLAUDE_REVIEW_PROMPT_TEMPLATE.md) / current [CLAUDE_REVIEW_P2_REFINE1.md](CLAUDE_REVIEW_P2_REFINE1.md) — not the Fable implementation handoff.


See [`SOFT_PARALLEL_QUEUE.md`](SOFT_PARALLEL_QUEUE.md): **ACTIVE (post-freeze, no-rename rule)**. Soft feature queue is **exhausted** pending Fable CONF-P2 / Madis new soft items.