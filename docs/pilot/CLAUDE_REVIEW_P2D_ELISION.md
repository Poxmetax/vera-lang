<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_P2D_REVIEW.md
Full review = THIS file. Not an implement handoff.
-->

# Claude review prompt -- P2D-ELIDE (INV-1 check-elision)

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[P2D-ELIDE]` -- proof-gated runtime check elision |
| **Date** | 2026-07-20 (post-land) |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | Fable 5 CONF-P2 D; C review PASS `976231b` |
| **Primary sources** | `P2D_ELISION_SLICE.md` (SoT), `FABLE5_CONF_P2D_HANDOFF_PROMPT.md`, `CURSOR_SYNC_ACK_P2D.md`, SPEC DP6/INV-1/CONF-P2, `P2B_DIAG_SLICE.md` |
| **Audience** | Claude -- **review only**, not implement |

---

## 0. Standing constraints

1. Isolation: no TradingBot mainnet / `.env` / live state.
2. No rename; no `_probe_*` temps.
3. No git commit/push unless Madis asks.
4. Prefer read + report; ask before >~30 lines or new crates.
5. Do not edit while reviewing: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs` unless Madis re-scopes.
6. Return audit in **Estonian** (section 6). UTF-8; prefer ASCII punctuation.

---

## 1. What the prior session DID (claimed -- verify)

- **Marker:** `[P2D-ELIDE]`
- **Files:** `vc.rs` (`Obligation.fn_name` / `ensures_index`, `ProvedSet`; dup-fn excluded), `interp.rs` (`with_proved`; elide PROVED return_refine + ensures[i]; never elide requires/param-refine entry), `main.rs` (`--prove-run` opt-in; default run byte-identical; `--prove` wins over both flags), `lib.rs` export
- **Tests:** +4 `elide_*`; suite **34** (was 30)
- **Docs:** `docs/pilot/P2D_ELISION_SLICE.md`
- **CONF-P2 claim:** clamp 2x ensures + return refine SMT-proved and runtime checks elided via `--prove-run` (armed 3, elided 9 on 3 calls)
- **INV-1 claim:** elision only from same-process `prove_program` on same Program AST
- **Diag:** `--diag-json` schema unchanged (new Obligation fields not serialized)
- **Backups:** `{vc,interp,main,lib}.rs.bak_20260720_000130_p2d_elide`
- **Not done:** call_requires / call_arg_refine elision; persistent certificate store; typecheck reject of dup fn names

**Re-run:**

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
# expect: 34 passed
cargo test -p vera --lib -- elide_
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- --prove-run examples/prove_clamp.vera
# expect: armed 3; outputs 5/0/10; elided 9; exit 0
cargo run -p vera -- --prove-run examples/prove_refuted.vera
# expect: not running; exit 3
cargo run -p vera -- examples/prove_clamp.vera
# expect: no [P2D-ELIDE] lines (default = zero elision)
```

---

## 2. What was EXPECTED (acceptance)

| ID | Criterion | Source | Review status |
|----|-----------|--------|---------------|
| D1 | >=1 proved obligation runtime check elided under documented gate | handoff D / CONF-P2 | fill |
| D2 | Unproved / RUNTIME-CHECKED / REFUTED still check or safe-fail | INV-1 / handoff | fill |
| D3 | No speculative elision (proof-gated only) | INV-1 / DP6 | fill |
| D4 | Default run path unchanged (HR1) | handoff | fill |
| D5 | soft_smoke PASS; prove_clamp 6 proved; lib tests green | baseline | fill (expect 34) |
| D6 | `--diag-json` schema unchanged or additive documented | P2B SoT | fill |

---

## 3. What YOU must do

1. Read Meta sources + this prompt; inventory markers/tests vs section 1.
2. Re-run smoke commands above; cite real numbers.
3. Adversarial: RuntimeChecked body still traps; REFUTED does not run; dup-fn does not steal elision; requires never skipped.
4. Connection-consistency: `ProvedSet` writer (vc) <-> reader (interp); CLI arming path both ends.
5. Do not start E / FixPatch. Write Estonian section 6 audit.

---

## 4. Correct review deliverable

**PASS** if: smoke cited; D1-D6 explicit PASS/FAIL/PARTIAL; findings CRITICAL->LOW or none; no drive-by edits; clear VERDICT.

**FAIL deliverable if:** assert green without re-run; edit Fable files "to help"; claim full CONF-P2 beyond D fragment; skip Estonian section 6.

---

## 5. Out of scope

Implement E; persistent certificates; call-site elision; labels/IFC; z3 crate; commit/push; mainnet.

---

## 6. Return format (Estonian)

```text
## VERDICT
PASS | FAIL | PARTIAL

## Kokkuvote
2-4 lauset

## Leiud
CRITICAL / HIGH / MED / LOW (voi "none")

## Acceptance (D1-D6)
...

## Smoke
34 tests? soft_smoke? prove-run clamp/refuted? default run?

## Next
ettepanekud ainult (E FixPatch / backlog)
```
