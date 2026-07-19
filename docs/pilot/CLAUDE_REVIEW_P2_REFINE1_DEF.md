<!--
Operator note (Estonian):
Madis — kopeeri see FAIL (alates "# Claude review prompt — P2-REFINE1-DEF") uude Claude
review sessiooni (mitte implementation handoff). Eesmärk: kontrollida definition-time
return-refine slice'i, mitte uut koodi kirjutada.
-->

# Claude review prompt — P2-REFINE1-DEF

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[P2-REFINE1-DEF]` — REQ-REFINE-1 **definition-time** closed return-body refine hard reject |
| **Date** | 2026-07-19 |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | surgical_builder / Cursor CONF-P2 continuation (Madis: "jätka plaani järgi") |
| **Primary sources** | `docs/pilot/P2_REFINE1_SLICE.md` (DEF section), `docs/pilot/FABLE5_CONF_P2_HANDOFF_PROMPT.md` task A, `docs/spec/SPEC.md` §4.4 REQ-REFINE-1, `crates/vera/src/typecheck.rs`, `apply_p2_refine1_def.py` |
| **Audience** | Claude (Fable 5 / Claude Code) — **review**, not implement |

---

## 0. Standing constraints (always)

1. **Isolation:** Never touch TradingBot mainnet runtime, `.env`, live state (`futures_grid_state.json`, `bot_control.json`, alert cooldowns). No import/export across TradingBot ↔ vera-lang.
2. **No rename:** Do not rename files (especially under `examples/`). No `_probe_*` temps that later become "final" names.
3. **No git commit --trailer "Co-authored-by: Cursor <cursoragent@cursor.com>" / push** unless Madis explicitly asks. Do not add remotes.
4. **Surgical:** Prefer read + report. Code edits only if Madis re-scopes mid-review; ask before >~30 lines or new Cargo deps.
5. **Fable / hard-track ownership:** `vc.rs` / `smt.rs` may be dirty with `[P2-SOUND*]`. **Do not edit them** while reviewing P2-REFINE1-DEF unless documenting a conflict.
6. **Language:** Code/docs English; operator-facing audit return in **Estonian** (see §6).

---

## 1. What the prior session DID (evidence)

- **Files touched:** `crates/vera/src/typecheck.rs` — `check_fn` hook + `check_ret_refine_body` + `eval_closed_int_expr`; unit tests `refine1_def_*`
- **Phase markers:** `[P2-REFINE1-DEF]` on hook, helpers, error string, tests
- **Behavior claimed:** At **definition time**, if return type is `{binder: Int | pred}` and the body is a **closed** Int literal / unary-minus / closed `if` tree (empty stmts, no free names), evaluate body; if pred is decidably `false` → `TypeError` containing `[P2-REFINE1-DEF]` (zero interpreter execution). Param-dependent bodies stay soft.
- **Tests added:**
  - `refine1_def_rejects_negative_literal_return` — `bad() -> {r| r >= 0} { -1 }` → err
  - `refine1_def_accepts_nonneg_literal_return` — `{ 0 }` → Ok
  - `refine1_def_rejects_closed_ite_false_branch` — `if 1 < 0 { 1 } else { -1 }` → err
  - `refine1_def_soft_on_param_dependent_body` — `id(x) -> {r|…} { x }` → Ok (soft)
- **Docs:** appended DEF section to `P2_REFINE1_SLICE.md`; README Phase 2 one-liner; `SOFT_PARALLEL_QUEUE.md` A → done (closed fragment); `PHASE2_VC_SLICE_REPORT.md` note; this review file
- **Patcher:** `apply_p2_refine1_def.py` (backup `typecheck.rs.bak_*_p2_refine1_def`)
- **Explicitly NOT done:** requires-guided binds; stmt/let dataflow; REQ-REFINE-2; handoff B/C/D/E; any `vc.rs` edit
- **Dirty-tree note:** `vc.rs` may still carry Fable `[P2-SOUND*]` — left alone

**Proof pointers (live smoke from implementer session 2026-07-19):**

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
# → 17 passed (includes refine1_* + refine1_def_* + vc/smt)
powershell -File docs\pilot\soft_smoke.ps1
# → SOFT-SMOKE PASS; prove_clamp 6 proved; prove_refuted exit 3
```

Marker inventory hint:

```powershell
Select-String -Path crates\vera\src\typecheck.rs -Pattern 'P2-REFINE1-DEF|fn check_ret_refine_body|fn refine1_def_'
Select-String -Path crates\vera\src\vc.rs -Pattern 'P2-REFINE1-DEF'   # expect 0
```

---

## 2. What was EXPECTED of that work (acceptance criteria)

| Criterion | Source | Expected | Status claimed by prior |
|-----------|--------|----------|-------------------------|
| A1 call-site out-of-range literal | handoff A / SPEC §4.4 | typecheck error, zero exec | MET (prior `[P2-REFINE1]`) |
| A2 in-range call still ok | handoff A | typecheck + run | MET |
| A3 definition-time body vs return refine | handoff A / SPEC "body that could return a negative" | hard reject when decidable | MET for **closed lit/ite** (`[P2-REFINE1-DEF]`) |
| A4 smoke green | handoff | `cargo test` + prove_clamp 6 | MET (17 tests; 6 proved) |
| Requires-guided / param bodies | handoff A honest limit | may defer | DEFERRED (soft) documented |

**Non-goals this slice was allowed to skip:** full SMT definition-time, handoff B diagnostics, REQ-REFINE-2, INV-1 elision, FixPatch, `vc.rs` merges.

---

## 3. What YOU (Claude) MUST DO now (numbered)

1. **Read** this file → `P2_REFINE1_SLICE.md` (DEF section) → handoff task A → SPEC §4.4 REQ-REFINE-1 → `check_ret_refine_body` / `eval_closed_int_expr` / `refine1_def_*` in `typecheck.rs`.
2. **Inventory** markers/tests (commands in §1); note mismatch vs claimed.
3. **Re-run smoke:**

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
cargo test -p vera --lib -- refine1_def_
```

4. **Adversarial check:** confirm negative return still rejects; confirm param-body soft does **not** spuriously reject; try (ephemeral unit-test style only — no permanent `_probe_*` files) whether a closed nested `if` that returns `-1` is rejected.
5. **Connection-consistency:** writer `format!("[P2-REFINE1-DEF] body returns …")` ↔ readers: unit asserts + slice doc.
6. **Write the audit** in §6 Estonian format. Do **not** drive-by refactor or touch `vc.rs`.

---

## 4. What CORRECT work from you looks like (pass bar)

**PASS** if all of:

- Smoke commands in §3 re-run and results cited (17 lib tests / SOFT-SMOKE PASS / refine1_def_ filter).
- Each acceptance row in §2 has explicit **PASS / FAIL / PARTIAL** vs **your** evidence.
- Findings ranked CRITICAL → LOW (or "none").
- Unsound "accept" paths for **closed** falsifying bodies hunted; either found + cited or "searched, none found" with what you tried.
- No drive-by renames, no mainnet touch, no unsolicited large patches.
- Clear **VERDICT** + optional **next** (handoff B) — Madis decides.

**FAIL the review deliverable** if you assert green without re-running smoke, treat prior docs as proof without inventory, edit Fable-owned `vc.rs`, commit/push/rename, or skip Estonian §6 structure.

---

## 5. Out of scope / do not

- Handoff B/C/D/E implementation
- Linking `z3` crate, Salsa, labels/IFC, hole synthesis
- TradingBot integration
- Creating permanent `_probe_*` example files
- "Finishing" requires-guided binds unless Madis re-scopes

---

## 6. Return format (Estonian for Madis)

```text
## VERDICT
PASS | FAIL | PASS-WITH-FINDINGS

## Kokkuvõte
2–4 lauset: kas [P2-REFINE1-DEF] definition-time closed slice peab vastu sinu kontrollile.

## Leiud (CRITICAL → LOW)
- [SEVERITY] fail:rida — kirjeldus — tõend (käsk / väljund)

## Acceptance vs sinu kontroll
| Criterion | Prior claim | Sinu verdict | Tõend |
| A3 def-time closed | MET | PASS/FAIL/PARTIAL | ... |
| A4 smoke | MET | ... | ... |

## Smoke (tsiteeri)
cargo test: N passed
soft_smoke: SOFT-SMOKE PASS | FAIL
refine1_def_ filter: …

## Next (ettepanekud ainult)
- handoff B (prove tiers → typecheck/CLI diagnostics) — väikseim slice
- …
```

---

End of P2-REFINE1-DEF review prompt.