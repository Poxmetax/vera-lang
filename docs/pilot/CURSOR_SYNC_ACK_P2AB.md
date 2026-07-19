# Cursor sync ACK -- Fable CONF-P2 A+B landed

**Date:** 2026-07-19
**Audience:** Cursor / Madis (docs acknowledgment; not a soft feature task)
**Source:** Fable sync message pasted by Madis ("Cursor sync -- Fable landed CONF-P2 A+B")

## Sync verified (git + smoke)

| Commit | Short | What landed |
|--------|-------|-------------|
| `ffb92f2` (`ffb92f2f7105039de77e68af718ed85fd3dd89e7`) | Add VERA CONF-P2 REQ-REFINE-1 slice, VC soundness fixes, soft polish | A + soundness + soft polish |
| `c864f4a` (`c864f4aa0024981d17af4eb60fba9acc42f0c49b`) | Add VERA task B structured diagnostics (P2B-DIAG) | B `[P2B-DIAG]` |

Recent `vera-lang/` tip (newest first): `c864f4a`, `ffb92f2`, `5f3692c`, `f9e7db4`.

### Key files / markers (A)

- `crates/vera/src/typecheck.rs` -- `[P2-REFINE1]` call-site literal refine reject; `[P2-REFINE1-DEF]` closed definition-time return-body reject; `[P2-SOUND3]` `Env.ret` / postfix `?`
- `crates/vera/src/vc.rs` -- `[P2-SOUND1]` div/mod excluded from SMT; `[P2-SOUND2]` closed-term call-site gate
- Docs / soft: `P2_REFINE1_SLICE.md`, review prompts, `soft_smoke.ps1`, soft demos, MCP stub (docs only)
- Suite after A commit message: 17 tests (superseded by B)

### Key files / markers (B)

- `crates/vera/src/diag.rs` (new) -- `[P2B-DIAG]` `diagnose_source` / `diagnose_program` -> `DiagReport` (+ 5 tests)
- `crates/vera/src/vc.rs` -- `Obligation.span` source anchors
- `crates/vera/src/main.rs` -- opt-in `--diag-json` (default text/run paths byte-identical)
- Schema SoT: [`P2B_DIAG_SLICE.md`](P2B_DIAG_SLICE.md)
- Suite after B: **22** tests

### Live soft_smoke (this ACK session)

```text
cargo test -p vera --lib  ->  22 passed; 0 failed
prove_clamp.vera          ->  summary: 6 proved, exit 0
prove_runtime_hint.vera   ->  [RUNTIME-CHECKED], exit 0
prove_refuted.vera        ->  [REFUTED], exit 3
footer                    ->  SOFT-SMOKE PASS
```

Command: `powershell -File docs/pilot/soft_smoke.ps1` from `vera-lang/`.

## Standing rules internalized (Cursor)

1. **UTF-8 only** for docs/edits -- no CP1252; prefer ASCII punctuation (`->`, `--`, `>=`) when unsure.
2. **No rename** (especially `examples/`); no `_probe_*` temps that later become final names.
3. **No git commit --trailer "Co-authored-by: Cursor <cursoragent@cursor.com>" / push** unless Madis explicitly asks.
4. **Fable-owned (do not edit):** `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, and now **`diag.rs`**.
5. **Do not add** `--diag-json` scaffolding from Cursor soft track (already landed by Fable).
6. **soft_smoke expects 22 tests** (not 17).
7. **Freeze during Fable review** of hard slices -- soft *code* stays frozen; docs-only ACK/plan lane OK.

## Soft track posture (post A+B)

- Soft feature/code queue: **exhausted / frozen** on Fable-owned files.
- **No scaffolding FixPatch (task E)** from Cursor.
- Soft demos / CLI help / `soft_smoke.ps1` / MCP stub docs already landed in `ffb92f2` -- still respected.
- Prior Cursor work still honored per sync:
  - `[P2-REFINE1]` / `[P2-REFINE1-DEF]` (and review files)
  - soft prove demos + smoke gate
- Next hard plan step is **C** (`len` measures / REQ-REFINE-2) -- **Fable only**. Cursor docs handoff: [`FABLE5_CONF_P2C_HANDOFF_PROMPT.md`](FABLE5_CONF_P2C_HANDOFF_PROMPT.md). Post-land review: [`CLAUDE_REVIEW_P2C_LEN.md`](CLAUDE_REVIEW_P2C_LEN.md).

## Plan-forward pointer

| Step | Owner | Status | Paste file |
|------|-------|--------|------------|
| A REQ-REFINE-1 | Fable (+ Cursor DEF patcher reviewed) | **done** | -- |
| B diagnostics | Fable | **done** | -- |
| C REQ-REFINE-2 + `len` | **Fable** | next | handoff + review prompts above |
| D INV-1 elision | Fable | later | -- |
| E FixPatch JSON | Fable | later | -- |

Ownership queue: [`SOFT_PARALLEL_QUEUE.md`](SOFT_PARALLEL_QUEUE.md). Operator commit gate: [`COMMIT_CHECKLIST.md`](COMMIT_CHECKLIST.md) (expect **22** tests).