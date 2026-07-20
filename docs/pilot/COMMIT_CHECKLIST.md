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
| `cargo test -p vera --lib` | **68** tests pass (post `[GAP4-VALUE-LABEL]` `28929dc`; was 63 after `[GAPC2-SMT-LEN]` `f8b67cc`; was 59 after `[GAPC1-SYM-LEN]` `4fbf7df`) |
| `examples/prove_clamp.vera` | summary contains **`6 proved`**, exit **0** |
| `examples/prove_runtime_hint.vera` | at least one **`[RUNTIME-CHECKED]`**, exit **0** |
| `examples/prove_refuted.vera` | **`[REFUTED]`**, exit **3** |
| Script footer | `SOFT-SMOKE PASS` |

## Freeze / race lesson

- Soft track was **frozen** mid-review because parallel soft work raced Fable 5 CONF-P2.
- Madis resumed with **"hästi, jätka"** — continue carefully.
- **Never rename files** during soft work (especially `examples/`). No `_probe_*` temps that later get renamed; new files use **final names only**.
- **Do not edit** Fable-owned: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, **`diag.rs`**.
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
- `vera-lang/docs/pilot/CURSOR_SYNC_ACK_P2AB.md` (Cursor sync ACK A+B)
- `vera-lang/docs/pilot/FABLE5_CONF_P2C_HANDOFF_PROMPT.md` (task C implement handoff)
- `vera-lang/docs/pilot/CLAUDE_REVIEW_P2C_LEN.md` (task C post-land review)
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

**Claude paste rule:** Paste POINTER files to Claude, not full handoffs. Template: [CLAUDE_POINTER_PROMPT_TEMPLATE.md](CLAUDE_POINTER_PROMPT_TEMPLATE.md). Review full bodies still live in [CLAUDE_REVIEW_PROMPT_TEMPLATE.md](CLAUDE_REVIEW_PROMPT_TEMPLATE.md) / topic files (e.g. [CLAUDE_REVIEW_P2_REFINE1_DEF.md](CLAUDE_REVIEW_P2_REFINE1_DEF.md); next C full: [CLAUDE_REVIEW_P2C_LEN.md](CLAUDE_REVIEW_P2C_LEN.md)).

**Sync:** [CURSOR_SYNC_ACK_GAP4_VALUE_LABEL.md](CURSOR_SYNC_ACK_GAP4_VALUE_LABEL.md) (VL LANDED soft re-verify PASS, baseline **68**, code `28929dc` / publish `f4f3cc7`); [CURSOR_SYNC_ACK_GAPC2.md](CURSOR_SYNC_ACK_GAPC2.md) (`f8b67cc` / `f4f3cc7`); prior [CURSOR_SYNC_ACK_GAPC1.md](CURSOR_SYNC_ACK_GAPC1.md) (`4fbf7df` / `0bc3c22`); [CURSOR_SYNC_ACK_GAP4_SURFACE.md](CURSOR_SYNC_ACK_GAP4_SURFACE.md) (`658e14b` / `34d7459`); [CURSOR_SYNC_ACK_P2E.md](CURSOR_SYNC_ACK_P2E.md) (`ddc3d6a` / `3c72ce4`). Soft smoke expects **68** tests. Debt: [KNOWN_GAPS.md](KNOWN_GAPS.md). Soft commit candidates: this checklist + queue + C2/VL ACK/review + bench scaffold (+ any soft baseline bumps); **exclude** `*.bak_*`.


See [`SOFT_PARALLEL_QUEUE.md`](SOFT_PARALLEL_QUEUE.md): **ACTIVE (post-freeze, no-rename rule)**. Task E **CLOSED**; GAP4-R2-SURFACE **CLOSED** (`658e14b`); GAP-C1 **CLOSED** (`4fbf7df`); GAP-C2 **CLOSED** (`f8b67cc`); GAP4-VALUE-LABEL **CLOSED** (`28929dc`); next hard = **TBD Madis-gated** (GAP4-R2-ERGO / F6 / GAP-D2 — do not pick).