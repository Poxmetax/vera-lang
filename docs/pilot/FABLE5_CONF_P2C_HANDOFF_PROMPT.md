<!--
Operator note (Estonian, header only):
Madis -- chat paste SHORT POINTER only: docs/pilot/CLAUDE_POINTER_P2C_IMPLEMENT.md
(Full brief stays on disk in THIS file; Claude reads it.) See ei ole review-prompt.
Review parast landimist: paste CLAUDE_POINTER_P2C_REVIEW.md (full: CLAUDE_REVIEW_P2C_LEN.md)
Ara puuduta TradingBot mainnet / .env / live state. Ara commit/push ilma Madiseta.
-->

> **Review vs implementation:** For **post-land review**, use [`CLAUDE_REVIEW_P2C_LEN.md`](CLAUDE_REVIEW_P2C_LEN.md) (template: [`CLAUDE_REVIEW_PROMPT_TEMPLATE.md`](CLAUDE_REVIEW_PROMPT_TEMPLATE.md)). **This file is the implementation handoff for task C only.**

# Fable 5 -- VERA CONF-P2C handoff (REQ-REFINE-2 + `len`) -- paste-ready

Canonical full brief on disk. Madis chat-pastes [`CLAUDE_POINTER_P2C_IMPLEMENT.md`](CLAUDE_POINTER_P2C_IMPLEMENT.md) instead of this whole file.

---

You are continuing **VERA** (`vera-lang`), an isolated research language prototype. Madis is the operator.

## Hard constraints (non-negotiable)

1. **Workspace:** `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. **Isolation:** Never touch TradingBot mainnet runtime, `.env`, live state files. Never import/export across TradingBot <-> vera-lang.
3. **Git:** No `git commit --trailer "Co-authored-by: Cursor <cursoragent@cursor.com>"` / `git push` unless Madis explicitly asks. Do not add remotes.
4. **Dependencies:** Prefer **zero** new Cargo crates. Ask before adding any (especially `z3` crate / Salsa).
5. **Surgical diffs:** Smallest change that preserves intent. Ask Madis before any change **>~30 lines**.
6. **Language:** Code and docs English. Operator chat may be Estonian.
7. **UTF-8** for all docs you touch; prefer ASCII punctuation (`->`, `--`, `>=`) if unsure.
8. **No rename** of existing files (especially `examples/`).

## Already done -- do not re-litigate (A+B landed)

**Git tip:** `c864f4a` on `main` (B); prior `ffb92f2` (A + soft + soundness). Sync ACK: `docs/pilot/CURSOR_SYNC_ACK_P2AB.md`.

| Slice | Markers / entry | Pointers |
|-------|-----------------|----------|
| REQ-REFINE-1 call-site | `[P2-REFINE1]` | `typecheck.rs`, `P2_REFINE1_SLICE.md` |
| REQ-REFINE-1 def-time (closed) | `[P2-REFINE1-DEF]` | same |
| VC soundness | `[P2-SOUND1/2/3]` | `vc.rs`, `Env.ret` in typecheck |
| Structured diagnostics | `[P2B-DIAG]` | `diag.rs`, `--diag-json`, **schema SoT** [`P2B_DIAG_SLICE.md`](P2B_DIAG_SLICE.md) |
| Soft track | CLI help / demos / smoke | frozen for Cursor; do not steal soft polish into this C slice |

**Respect existing contracts (do not break):**

- `Obligation.span` on prove obligations (`vc.rs`)
- `Env.ret` for postfix `?` (`typecheck.rs` `[P2-SOUND3]`)
- `--diag-json` + `diagnose_source` / `diagnose_program` JSON shape in `P2B_DIAG_SLICE.md` (extend only if C needs new codes -- document; prefer additive)
- Default text `--prove` / run paths stay bit-identical unless Madis says otherwise (HR1-style)

**Smoke baseline (must stay green):**

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
# expect: at least 22 passed (more OK if you add focused C tests)
powershell -File docs\pilot\soft_smoke.ps1
# expect: SOFT-SMOKE PASS; prove_clamp -> 6 proved; prove_refuted exit 3
```

## This session: task C only

**SPEC:** `docs/spec/SPEC.md` section 4.4 **REQ-REFINE-2** (and section 3 E3 `nth` / measure note):

> Given `fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int`, a call with a provably out-of-range index (e.g. literal `-1`, or `len(xs)` itself) is rejected at compile time; `len` is available in refinements as a measure. An index the prover cannot bound forces the caller through the total API (`get -> Option`) or an explicit runtime-checked assertion.

### Smallest first slice (preferred)

Ship the **smallest closed fragment** that makes REQ-REFINE-2 demonstrably true for at least one shape, then stop for Madis review:

1. Parse / type surface: `len(xs)` usable inside refinements as a **measure** (honest limit OK if only for `List` + simple binders).
2. Compile-time reject for **provably OOB** index -- start with **literal** negatives / closed decidable cases (mirror `[P2-REFINE1]` spirit).
3. At least one focused unit test and/or `.vera` example with **final name** (no rename later).
4. Unbounded / symbolic index: either force `get -> Option` / runtime assert path, **or** document honest soft limit (do not fake CONF-P2).

Phase marker suggestion: `[P2-REFINE2]` / `[P2C-LEN]` (pick one; grep first for uniqueness; use consistently).

### Success checklist

- [ ] `len(xs)` in refinements on the check path (parse + typecheck; SMT encode only if needed for this slice)
- [ ] Provably OOB index (e.g. literal `-1`) -> typecheck error, **zero** interpreter execution
- [ ] Valid in-range shape still typechecks (and runs if you add an example)
- [ ] Unbounded index: total API / runtime assert **or** documented deferred limit
- [ ] `cargo test -p vera --lib` green (>=22); `prove_clamp.vera --prove` still **6 proved**
- [ ] Soft smoke PASS; do not break `[P2B-DIAG]` schema SoT without documenting additive change
- [ ] Short slice note under `docs/pilot/` (e.g. `P2C_LEN_SLICE.md`) -- English, UTF-8

### Explicitly out of scope this session

- Task D (INV-1 check-elision in `interp.rs`)
- Task E (FixPatch JSON) -- **do not scaffold**
- Labels / IFC / `z3` crate / Salsa / hole synthesis
- TradingBot integration
- Rewriting A/B markers or soft demos "while there"

### Files you own for C (expect edits)

Likely: `typecheck.rs`, possibly `parser.rs` / `ast.rs` / `vc.rs` / `smt.rs` if measure encode is required -- **surgical only**. You already own `diag.rs`; extend only if C diagnostics need new stable codes (document vs `P2B_DIAG_SLICE.md`).

Cursor soft track will **not** edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`.

## Working style

1. Read SPEC section 4.4 REQ-REFINE-2 + E3, `P2_REFINE1_SLICE.md` (pattern reuse), `P2B_DIAG_SLICE.md`, this handoff.
2. Propose surgical plan if >~30 lines; wait for Madis yes/no.
3. Implement smallest closed fragment; phase markers on non-trivial edits.
4. Re-run smoke above after every code change.
5. Prefer honest limits over fake "full CONF-P2" claims.

## Definition of done

Report: what C fragment landed, honest limits, smoke evidence (test count, prove_clamp, soft_smoke), blockers. Then Madis pastes [`CLAUDE_REVIEW_P2C_LEN.md`](CLAUDE_REVIEW_P2C_LEN.md) for independent review.

---

End of paste-ready P2C handoff.