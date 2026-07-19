<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_P2D_IMPLEMENT.md
Full brief stays in THIS file. Do not paste this whole file into Claude chat.
-->

# Fable 5 -- VERA CONF-P2D handoff (INV-1 check-elision)

Canonical full brief for **task D only**. Madis pastes [`CLAUDE_POINTER_P2D_IMPLEMENT.md`](CLAUDE_POINTER_P2D_IMPLEMENT.md) into chat.

---

You are continuing **VERA** (`vera-lang`). Madis is the operator. This session implements **CONF-P2 task D**: proof-gated runtime check **elision** (INV-1 / DP6).

## Hard constraints

1. Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. Never touch TradingBot mainnet / `.env` / live state.
3. No git commit/push unless Madis asks.
4. Prefer zero new Cargo crates; ask before any.
5. Surgical diffs; ask before >~30 lines.
6. Code/docs English; UTF-8; prefer ASCII punctuation (`->`, `--`, `>=`).
7. No file renames (especially `examples/`).
8. Preserve A/B/C contracts: `Obligation.span`, `Env.ret`, `[P2B-DIAG]` schema SoT (`P2B_DIAG_SLICE.md`), soft_smoke / prove_clamp regression.

## Already done (do not re-open)

| Slice | Status | Pointers |
|-------|--------|----------|
| Phase -1 / 0 / 1 | done | README, SPEC, RESEARCH |
| Thin VC + `--prove` | done | `smt.rs`, `vc.rs`, `PHASE2_VC_SLICE_REPORT.md` |
| A REQ-REFINE-1 | done | `[P2-REFINE1]`, `[P2-REFINE1-DEF]`, `P2_REFINE1_SLICE.md` |
| B diagnostics | done | `[P2B-DIAG]`, `diag.rs`, `--diag-json`, `P2B_DIAG_SLICE.md` |
| C REQ-REFINE-2 / `len` | **must be landed (or blocked) before large D** -- see `P2C_LEN_SLICE.md` if present; if C not landed, ask Madis whether to wait |
| Soft demos / smoke | done | `soft_smoke.ps1`, prove_* examples |

**Sync / ownership:** `CURSOR_SYNC_ACK_P2AB.md`, `SOFT_PARALLEL_QUEUE.md`.

## SPEC anchors (D)

- **DP6:** when the prover discharges an obligation, the runtime check is elided (proof-gated, INV-1).
- **INV-1:** only correctness-preserving automatic transformations; elision must be proof-gated -- never speculative.
- **CONF-P2:** ">=1 contract SMT-proved end-to-end with its runtime check elided".

Today: interpreter always evaluates `requires` / `ensures` / refine preds at call/return (`interp.rs`). Prove path marks `[PROVED]` but does **not** skip those checks at run time.

## What YOU must do (smallest closed fragment)

1. Read this brief + `PHASE2_VC_SLICE_REPORT.md` + `interp.rs` contract-check sites + how `vc` / `diag` report PROVED.
2. Design a **proof-gated** elision path: only skip a runtime check when that obligation was **PROVED** for this definition/call in the current prove result (or an equivalent explicit proved-set passed into the interpreter). Unproved / RUNTIME-CHECKED / REFUTED must still check (or trap as today).
3. Prefer opt-in or clear wiring (e.g. run-after-prove / proved-set hook) so default `vera file.vera` stays safe if prove was not run -- **document** the chosen HR1-compatible behavior. Do not silently change semantics without a gate Madis can see.
4. Demonstrate on a known-proved case (e.g. `prove_clamp.vera` path): at least one proved `requires`/`ensures`/refine check is **not** re-evaluated at runtime (unit test or instrumentation assertion), while a non-proved contract still traps on violation.
5. Marker: `[P2D-ELIDE]` (grep uniqueness first). Touch `interp.rs` surgically; may need thin glue from `vc`/`diag`/`main` -- keep additive.
6. Do **not** implement FixPatch (E), labels, z3 crate, Salsa.
7. Slice note: `docs/pilot/P2D_ELISION_SLICE.md` (English, UTF-8): what elides, what does not, how prove results feed interp, smoke evidence.
8. Smoke:
```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- --prove examples/prove_clamp.vera
# prove_clamp still 6 proved
```

## Correct work (PASS bar)

- [ ] >=1 proved obligation's runtime check elided under the documented gate
- [ ] Unproved obligations still runtime-checked
- [ ] INV-1: no speculative elision (no skip without prove evidence)
- [ ] Marker `[P2D-ELIDE]`; slice note written
- [ ] soft_smoke PASS; prove_clamp 6 proved; lib tests green (count may rise)
- [ ] No FixPatch scaffold; no renames; no mainnet touch

## Out of scope

Task E FixPatch; full certificate store; CVC5; labels/IFC; rewriting A/B/C "while there"; claiming full CONF-P2.

## Return format (Estonian, short)

```text
## VERDICT
DONE-D | BLOCKED | PARTIAL

## Mis landis
...

## Smoke
...

## Piirangud / blockers
...

## Next
oota review -- CLAUDE_POINTER_P2D_REVIEW.md (after review file exists)
```

End of CONF-P2D handoff.
