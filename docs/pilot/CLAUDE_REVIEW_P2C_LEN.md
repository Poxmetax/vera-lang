<!--
Operator note (Estonian):
Madis -- chat paste SHORT POINTER: docs/pilot/CLAUDE_POINTER_P2C_REVIEW.md
(Full review = THIS file on disk; Claude reads it.) PARAST C landimist. See ei ole implementation handoff.
Implementatsioon: paste CLAUDE_POINTER_P2C_IMPLEMENT.md (full: FABLE5_CONF_P2C_HANDOFF_PROMPT.md)
-->

# Claude review prompt -- P2C-LEN (REQ-REFINE-2 / `len` measures)

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[P2-REFINE2]` / `[P2C-LEN]` (whichever Fable used) -- REQ-REFINE-2 + `len` measures |
| **Date** | 2026-07-19 (post-land review; C = `976231b`) |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | A+B (`ffb92f2`, `c864f4a`); **C landed** `976231b` `[P2-REFINE2]` |
| **Primary sources** | `P2C_LEN_SLICE.md` (SoT), `FABLE5_CONF_P2C_HANDOFF_PROMPT.md`, `CURSOR_SYNC_ACK_P2C.md`, SPEC section 4.4, `P2B_DIAG_SLICE.md`, `P2_REFINE1_SLICE.md` |
| **Audience** | Claude (Fable 5 / Claude Code) -- **post-land review**, not implement |

> **Clear split:** This file is for **review after C lands** (Claude reads it from disk via [`CLAUDE_POINTER_P2C_REVIEW.md`](CLAUDE_POINTER_P2C_REVIEW.md)). To **implement** C, paste [`CLAUDE_POINTER_P2C_IMPLEMENT.md`](CLAUDE_POINTER_P2C_IMPLEMENT.md) (full brief: [`FABLE5_CONF_P2C_HANDOFF_PROMPT.md`](FABLE5_CONF_P2C_HANDOFF_PROMPT.md)). If the same session must do both, finish implement first, then switch to this prompt's section 3 checklist without drive-by scope expansion.

---

## 0. Standing constraints (always)

1. **Isolation:** Never touch TradingBot mainnet runtime, `.env`, live state. No import/export across TradingBot <-> vera-lang.
2. **No rename:** Do not rename files (especially under `examples/`). No `_probe_*` temps that later become final names.
3. **No git commit --trailer "Co-authored-by: Cursor <cursoragent@cursor.com>" / push** unless Madis explicitly asks. Do not add remotes.
4. **Surgical:** Prefer read + report. Code edits only if Madis re-scopes mid-review; ask before >~30 lines or new Cargo deps.
5. **Fable / hard-track ownership:** Do not "fix while reviewing" `vc.rs` / `smt.rs` / `typecheck.rs` / `interp.rs` / `diag.rs` unless Madis re-scopes.
6. **Language:** Code/docs English; operator-facing audit return in **Estonian** (see section 6).
7. **UTF-8** only; prefer ASCII punctuation if unsure.

---

## 1. What the prior session DID (evidence)

### A+B landed (preserve; do not re-litigate)

- **Commits:** `ffb92f2` (A + soundness + soft), `c864f4a` (B `[P2B-DIAG]`)
- **Markers:** `[P2-REFINE1]`, `[P2-REFINE1-DEF]`, `[P2-SOUND1/2/3]`, `[P2B-DIAG]`
- **Pre-C baseline was:** 22 lib tests; prove_clamp 6 proved; soft_smoke PASS

### C landed inventory (commit `976231b` -- verify, do not trust blindly)

- **Files touched:** `crates/vera/src/typecheck.rs`, `crates/vera/src/interp.rs`, `examples/refine_len_ok.vera`, `docs/pilot/P2C_LEN_SLICE.md`
- **Phase marker:** `[P2-REFINE2]`
- **Behavior claimed:** Kleene three-valued `&&`/`||` so literal OOB + `len` measure -> TypeError zero exec; `len(e)` eval in refine preds at runtime (guarded); in-range demo prints 20
- **Tests / examples:** +5 `refine2_*`, +3 `len_measure_*`; suite **30** (was 22); `examples/refine_len_ok.vera`
- **Docs:** `P2C_LEN_SLICE.md` = SoT for honest limits
- **Honest limits claimed:** symbolic `nth(xs, xs.len())` DEFERRED; unbounded index = runtime-checked + `get -> Option`; no new `--diag-json` codes
- **Cursor sync ACK:** `docs/pilot/CURSOR_SYNC_ACK_P2C.md`

**Proof pointers (re-run on review day):**

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
git show --stat 976231b
cargo test -p vera --lib
# expect: 30 passed
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- examples/refine_len_ok.vera
cargo test -p vera --lib -- refine2_
cargo test -p vera --lib -- len_measure_
```

---

## 2. What was EXPECTED of C when landed (acceptance criteria)

| Criterion | Source | Expected | Status claimed by implementer |
|-----------|--------|----------|-------------------------------|
| C1 `len(xs)` measure in refinements | SPEC section 4.4 REQ-REFINE-2 / E3 | usable on check path | fill at review |
| C2 provably OOB index rejected | handoff C / SPEC | typecheck error, zero exec (e.g. literal `-1`) | fill |
| C3 in-range / legal shape still ok | handoff C | typecheck (+ run if example) | fill |
| C4 unbounded index | SPEC | total `get -> Option` / runtime assert **or** documented soft limit | fill |
| C5 smoke green | handoff / ACK | **30** tests; prove_clamp 6; soft_smoke PASS | fill |
| C6 diag contract | P2B_DIAG_SLICE | no silent schema break; additive only if documented | fill |

**Non-goals C was allowed to skip:** full Flux-style inference, task D elision, task E FixPatch, labels/IFC, linking `z3` crate.

---

## 3. What YOU (Claude) MUST DO now (numbered) -- post-land review

1. **Read** Meta primary sources + Fable C slice note + this prompt.
2. **Inventory** claimed C markers/tests/examples (`Select-String` / grep); note mismatch vs implementer claim.
3. **Re-run smoke:**

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
# topic-specific: cargo test -- --nocapture <refine2|len filter>
# optional: cargo run -p vera -- --diag-json <new example>  # schema still matches P2B_DIAG_SLICE
```

4. **Adversarial check:** at least one case that must FAIL if C is sound (OOB literal / `len(xs)` as index if in scope). Ephemeral stdin or unit test only -- **no** permanent `_probe_*` renames.
5. **Connection-consistency:** writer <-> reader for measure `len` (parse/ast -> typecheck -> optional SMT/diag codes); cite both ends.
6. **Preserve A+B:** confirm `[P2-REFINE1]` / `[P2-REFINE1-DEF]` / `[P2B-DIAG]` / `Env.ret` / `Obligation.span` still hold.
7. **Write the audit** in section 6 format. Do **not** drive-by implement D/E.

---

## 4. What CORRECT work from you looks like (pass bar)

**PASS** if all of:

- Smoke in section 3 re-run and results cited (counts / exit codes / key strings).
- Each section 2 row has explicit **PASS / FAIL / PARTIAL** vs **your** evidence.
- Findings ranked CRITICAL -> HIGH -> MEDIUM -> LOW (or "none").
- Unsound "accept" paths for OOB indices hunted; found+cited or "searched, none" with what you tried.
- No drive-by renames, no mainnet touch, no unsolicited FixPatch scaffolding.
- Clear **VERDICT** + optional **next** proposals only (Madis decides).

**FAIL the review deliverable** if you:

- Assert green without re-running smoke.
- Treat prior docs as proof without inventory.
- Edit Fable-owned / out-of-scope files "to help."
- Commit / push / rename.
- Return only English prose without the Estonian section 6 structure.

---

## 5. Out of scope / do not

- Implementing C in this review session (unless Madis explicitly re-scopes).
- Tasks D/E, labels/IFC, `z3` crate, Salsa, hole synthesis, TradingBot integration.
- Breaking or silently changing `--diag-json` schema without documenting.

---

## 6. Return format (Estonian for Madis)

```text
## VERDICT
PASS | FAIL | PASS-WITH-FINDINGS

## Kokkuvote
2-4 lauset: mis on tosi parast sinu kontrolli (C + A+B sailimine).

## Leiud (CRITICAL -> LOW)
- [SEVERITY] fail:rida -- kirjeldus -- toend (kask / valjund)

## Acceptance vs sinu kontroll
| Criterion | Prior claim | Sinu verdict | Toend |
| ... | ... | PASS/FAIL/PARTIAL | ... |

## Smoke (tsiteeri)
cargo test: N passed
soft_smoke: SOFT-SMOKE PASS | FAIL (+ detail)
topic-specific: ...

## Next (ettepanekud ainult)
- ...
```

---

## How this pairs with the handoff

| File | When to paste |
|------|----------------|
| [`FABLE5_CONF_P2C_HANDOFF_PROMPT.md`](FABLE5_CONF_P2C_HANDOFF_PROMPT.md) | **Implement** C |
| **This file** | **Review** C after land |
| [`CLAUDE_REVIEW_PROMPT_TEMPLATE.md`](CLAUDE_REVIEW_PROMPT_TEMPLATE.md) | Mint future reviews |