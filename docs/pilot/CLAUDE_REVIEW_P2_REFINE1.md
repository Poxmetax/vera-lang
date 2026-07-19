<!--
Operator note (Estonian):
Madis — kopeeri see FAIL (alates "# Claude review prompt — P2-REFINE1") uude Claude
(Fable 5 / Claude Code) sessiooni üLEVAATUSEKS. See EI ole implementatsiooni-handoff.
Implementatsiooniks kasuta FABLE5_CONF_P2_HANDOFF_PROMPT.md.
-->

# Claude review prompt — P2-REFINE1

## Meta

| Field | Value |
|-------|-------|
| **Topic / marker** | `[P2-REFINE1]` — REQ-REFINE-1 **call-site** literal Int refine hard reject |
| **Date** | 2026-07-19 |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | Soft / parallel CONF-P2 slice (typecheck only); Fable may still own dirty `vc.rs` |
| **Primary sources** | `docs/pilot/P2_REFINE1_SLICE.md`, handoff task **A** in `docs/pilot/FABLE5_CONF_P2_HANDOFF_PROMPT.md`, `docs/pilot/PHASE2_VC_SLICE_REPORT.md`, `docs/spec/SPEC.md` §4.4 REQ-REFINE-1, README Phase 2 line |
| **Audience** | Claude — **adversarial review** of landed call-site slice; not a greenfield implement session |
| **Template** | `docs/pilot/CLAUDE_REVIEW_PROMPT_TEMPLATE.md` (standing rule) |

---

## 0. Standing constraints (always)

1. **Isolation:** Never touch TradingBot mainnet, `.env`, live state files. No cross-boundary imports.
2. **No rename:** Especially `examples/`. No leftover `_probe_*` files.
3. **No git commit --trailer "Co-authored-by: Cursor <cursoragent@cursor.com>" / push** unless Madis asks.
4. **Surgical review:** Prefer read + re-smoke + report. Do **not** drive-by refactor `typecheck.rs` / `vc.rs` / `smt.rs` during this review unless Madis re-scopes mid-session and you propose a surgical plan first.
5. **Dirty tree awareness:** `vc.rs` may contain Fable mid-flight `[P2-SOUND1]` / `[P2-SOUND2]` (and related tests). **Do not assume `vc.rs` is clean or unchanged.** Leave Fable VC work alone unless you are documenting a **conflict** with P2-REFINE1 (cite both ends; do not “fix” VC unless asked).
6. **Language:** Evidence/commands English; **return audit in Estonian** (§6).

---

## 1. What the prior session DID (evidence)

### Behavior landed

Call-site **compile-time** hard reject when all hold:

1. Parameter type is `{binder: Int | pred}`
2. Argument is an `Int` literal
3. `pred` is a closed comparison / `&&` / `||` / `!` tree over the binder + literals
4. Evaluation yields `false` → `TypeError` containing `[P2-REFINE1]` (**zero interpreter execution**)

In-range literals still typecheck. Definition-time body vs return refine was **intentionally deferred**.

### Concrete inventory (as of soft_smoke 2026-07-19)

| Artifact | Location / note |
|----------|-----------------|
| Core hook | `crates/vera/src/typecheck.rs` — call path invokes `check_lit_arg_refine` after `types_equal` (~call-site loop; marker comment `[P2-REFINE1]`) |
| Helper | `check_lit_arg_refine` + `pred_holds_for_lit` / related — doc comment cites REQ-REFINE-1 call-site slice |
| Error string | `"[P2-REFINE1] arg {} = {value} violates parameter refinement"` |
| Unit tests | `refine1_rejects_out_of_range_literal_call` — `apply_discount(100, 150)` → err with `[P2-REFINE1]` |
| | `refine1_accepts_in_range_literal_call` — `apply_discount(100, 10)` → Ok |
| | `refine1_rejects_negative_literal_call` — present in current tree (negative literal path) |
| Positive example | `examples/refine_call_ok.vera` — in-range `apply_discount(100, 10)` / `(0, 0)` |
| Slice doc | `docs/pilot/P2_REFINE1_SLICE.md` |
| Queue | `docs/pilot/SOFT_PARALLEL_QUEUE.md` — task A **partial** (literal call-site done; definition-time open) |
| Phase 2 report | `docs/pilot/PHASE2_VC_SLICE_REPORT.md` — notes REQ-REFINE-1 call-site landed; definition-time still open |
| README | Phase 2 line mentions `[P2-REFINE1]` + pointer to slice |

### Explicitly NOT done by this slice

| Item | Status |
|------|--------|
| Non-literal / unevaluable preds | Soft — prove / runtime (unchanged) |
| Definition-time body vs return refine | **Deferred** (handoff A allows defer) |
| Full CONF-P2 / REQ-REFINE-2 / check-elision / FixPatch | Not this slice |
| `vc.rs` / Z3 encoder edits for REFINE1 | **Not touched by design** (composes with Fable P2-SOUND on `vc.rs`) |

### Live smoke snapshot (docs author re-ran; re-verify yourself)

```text
cargo test -p vera --lib  →  13 passed (includes refine1_* + vc/smt tests)
soft_smoke.ps1            →  SOFT-SMOKE PASS
  prove_clamp             →  summary: 6 proved, exit 0
  prove_runtime_hint      →  ≥1 [RUNTIME-CHECKED]
  prove_refuted           →  [REFUTED], exit 3
```

Marker count hint: `Select-String` on `typecheck.rs` for `P2-REFINE1` → multiple hits (hook + helper + tests). `vc.rs` may show `P2-SOUND*` hits — **Fable-owned**, not REFINE1.

---

## 2. What was EXPECTED of that work (acceptance criteria)

Sources: handoff **task A** (`FABLE5_CONF_P2_HANDOFF_PROMPT.md`), SPEC §4.4 REQ-REFINE-1, `P2_REFINE1_SLICE.md`.

| # | Criterion | Expected | Prior claim |
|---|-----------|----------|-------------|
| A1 | Out-of-range literal call (e.g. `apply_discount(100, 150)`) is a **typecheck error** without running the program | `TypeError` + `[P2-REFINE1]` | MET (unit test) |
| A2 | Valid in-range calls still typecheck + run | typecheck Ok + example runs | MET (`refine_call_ok.vera` + accept test) |
| A3 | Definition-time reject when body can violate return refine **or** honest defer documented | defer OK if documented | DEFERRED (slice table + helper doc) |
| A4 | `cargo test -p vera --lib` green; `prove_clamp.vera --prove` still **6 proved** | smoke green | MET at soft_smoke time |
| A5 | No `vc.rs` edit for this slice (compose with Fable) | typecheck-only | CLAIMED — verify no REFINE1 markers in `vc.rs`; Fable P2-SOUND dirt is separate |

**Non-goals for prior work:** REQ-REFINE-2, prove↔typecheck diagnostics wiring, INV-1 elision, FixPatch, labels/IFC.

---

## 3. What YOU (Claude) MUST DO now (numbered)

1. **Read** (in order): this file → `P2_REFINE1_SLICE.md` → handoff task A → SPEC §4.4 REQ-REFINE-1 → relevant `typecheck.rs` regions (`check_lit_arg_refine`, call-site loop, `refine1_*` tests) → `examples/refine_call_ok.vera`. Skim `PHASE2_VC_SLICE_REPORT.md` for context only.
2. **Inventory** markers/tests (do not trust §1 blindly):

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
Select-String -Path crates\vera\src\typecheck.rs -Pattern 'P2-REFINE1|fn refine1_|check_lit_arg_refine'
Select-String -Path crates\vera\src\vc.rs -Pattern 'P2-REFINE1|P2-SOUND'   # expect SOUND maybe; REFINE1 should be absent
Test-Path examples\refine_call_ok.vera
```

3. **Re-run smoke** and cite real output:

```powershell
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
cargo test -p vera --lib -- refine1_
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- examples\refine_call_ok.vera
```

4. **Negative / adversarial checks** (soundness of “accept”):
   - Confirm unit tests still reject `pct=150` (and negative literal test if present).
   - Mentally or via **ephemeral** check (inline test source / one-off `cargo test` without **renaming** or committing probe examples): boundary cases `pct=101`, `pct=-1`, `price=-1` if covered by refine; `pct=0` / `pct=100` should **accept** if pred is `0 <= d && d <= 100`.
   - Hunt **unsound accept**: literal that clearly violates pred but typechecks (false negative). If you find one → CRITICAL/HIGH with minimal repro.
   - Confirm non-literal args still soft (no false hard-reject that breaks prove_clamp / open-arg paths).

5. **Connection-consistency:**
   - Writer: error format / marker in `check_lit_arg_refine`.
   - Readers: unit asserts on `"[P2-REFINE1]"`; docs/slice claim; example comment.
   - Confirm call-site only (named fn params with `Type::Refine`); note whether first-class/`Fn` call path **intentionally** skips refine hard-reject (document as finding or intentional limit).

6. **Return** the Estonian audit (§6). Propose next steps only — **do not** implement definition-time refine or edit `vc.rs` in this review session unless Madis explicitly expands scope.

---

## 4. What CORRECT work from you looks like (pass bar)

**Your review deliverable PASSes** if:

- You re-ran §3 smokes and cited counts/exits (not copied from this prompt alone).
- Each A1–A5 row has **your** PASS / FAIL / PARTIAL with evidence.
- You explicitly state whether you found any **unsound accept** of a bad literal (or “none found” + what you tried).
- Fable `vc.rs` dirt is acknowledged without being “fixed” casually.
- No renames, no commit, no mainnet touch, no drive-by CONF-P2 features.

**FAIL the review deliverable** if you assert green without re-running tests, rubber-stamp the slice doc, or ship a large unsolicited patch.

---

## 5. Out of scope / do not

- Implement definition-time return-refine reject (handoff A defer → later / task B adjacent).
- REQ-REFINE-2 / `len` measures, INV-1 elision, FixPatch, prove↔typecheck diagnostics.
- Edit `vc.rs` / `smt.rs` to “clean up” P2-SOUND work.
- Rename examples; add permanent `_probe_*` files.
- TradingBot / `.env` / git push.
- Claim full CONF-P2 or full REQ-REFINE-1 (definition-time still open).

---

## 6. Return format (Estonian for Madis)

```text
## VERDICT
PASS | FAIL | PASS-WITH-FINDINGS

## Kokkuvõte
2–4 lauset: kas [P2-REFINE1] call-site slice peab vastu sinu kontrollile.

## Leiud (CRITICAL → LOW)
- [SEVERITY] path:line — kirjeldus — tõend

## Acceptance vs sinu kontroll
| # | Criterion | Prior | Sinu | Tõend |
| A1 | out-of-range literal hard reject | MET | … | … |
| A2 | in-range still ok | MET | … | … |
| A3 | definition-time / honest defer | DEFERRED | … | … |
| A4 | lib tests + prove_clamp 6 proved | MET | … | … |
| A5 | no REFINE1 vc.rs dependency | CLAIMED | … | … |

## Smoke (tsiteeri oma jooksud)
cargo test -p vera --lib: …
refine1_ filter: …
soft_smoke: …
refine_call_ok.vera: …

## Unsound-accept hunt
Tried: …
Result: none found | FOUND (detail)

## Next (ettepanekud ainult)
- e.g. definition-time slice, Fn-call refine path, boundary tests, …
```

---

End of P2-REFINE1 review prompt.
