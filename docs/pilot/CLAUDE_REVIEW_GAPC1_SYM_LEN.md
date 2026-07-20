<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAPC1_REVIEW.md
Full review = THIS file. Soft-track filled verdict 2026-07-20 (Cursor).
Not an implement handoff. Optional: Claude re-verify.
-->

# Claude / soft review -- GAPC1-SYM-LEN (symbolic same-term len-as-index reject)

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[GAPC1-SYM-LEN]` -- symbolic same-term `nth(xs, xs.len())` compile-time reject |
| **Date** | 2026-07-20 (post-land soft review) |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | Fable GAPC1-SYM-LEN implement; code commit `4fbf7df` (publish merge `0bc3c22`); soft ACK `CURSOR_SYNC_ACK_GAPC1.md` |
| **Primary sources** | `GAPC1_SYM_LEN_SLICE.md` (SoT), `CURSOR_SYNC_ACK_GAPC1.md`, `FABLE5_GAPC1_HANDOFF_PROMPT.md`, `P2C_LEN_SLICE.md`, `KNOWN_GAPS.md`, SPEC REQ-REFINE-2 |
| **Reviewer** | Cursor soft-track (independent); optional Claude re-check via pointer |
| **Audience** | Madis + Claude -- **review / close-out only**, not implement |

---

## 0. Standing constraints

1. Isolation: no TradingBot mainnet / `.env` / live state.
2. No rename; no `_probe_*` temps.
3. Soft does not edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`.
4. Do not claim full REQ-REFINE-2, general symbolic arithmetic, GAP-C2 SMT `len` encode, or list-literal length propagation.
5. Soft cases (`xs.len()-1`, other-list `.len()`, aliases, Kleene `||` guard) are **design**, not bugs.
6. Do not invent next implement scope in this review session (next = TBD Madis-gated).

---

## 1. What Fable claimed (verified)

- **Marker:** `[GAPC1-SYM-LEN]`
- **Files:** `crates/vera/src/typecheck.rs` only (+191/−0)
- **Surface:** `check_sym_len_arg_refine` beside `check_lit_arg_refine`; shape gate requires bare `<name>.len()`; param→arg subst for same-`Name` receiver/list; reflexivity under Kleene `&&`/`||`/`!`
- **SPEC case:** `nth(xs, xs.len())` / `nth_c1(data, data.len())` → `TypeError` with `[GAPC1-SYM-LEN]`, zero execution
- **Tests:** +3 (`gapc1_*`); suite **59** (was 56)
- **Backup:** `typecheck.rs.bak_20260720_044832_gapc1_sym_len` -- **exclude** from commit
- **Not done / not claimed:** GAP-C2 SMT encode; `xs.len()-1` / cross-term / aliases / non-`Name` receivers; FixPatch for this class; GAP4 surface contracts unchanged

---

## 2. Acceptance

| ID | Criterion | Verdict | Evidence |
|----|-----------|---------|----------|
| A1 | Same-term `data.len()` as index rejected statically | **PASS** | `gapc1_rejects_len_of_same_list_as_index` |
| A2 | Soft controls stay soft (BinOp / other list) | **PASS** | `gapc1_len_minus_one_and_other_list_stay_soft` |
| A3 | Kleene `unknown \|\| false` stays soft (design) | **PASS** | `gapc1_kleene_or_guard_stays_soft` |
| A4 | GAP4 / FixPatch untouched | **PASS** | slice claims + suite still pins `gap4_surface_*` / FixPatch tests |
| A5 | soft_smoke PASS; prove_clamp 6; lib **59** | **PASS** | Soft re-ran 2026-07-20: 59 passed; SOFT-SMOKE PASS; 6 proved |
| A6 | Honest limits / no overclaim | **PASS** | Slice + ACK tables: same-term fragment only; GAP-C2 OPEN |

---

## 3. Connection-consistency (both ends read)

| Hop | Writer | Reader | Verdict |
|-----|--------|--------|---------|
| Arg shape `<recv>.len()` | call-site arg walk | `check_sym_len_arg_refine` shape gate | **MATCH** |
| param→arg subst (same Name) | callee params receiving `recv` | pred `len(<param>)` → symbolic binder | **MATCH** |
| Reflexivity decide | `pred_holds_for_sym_len` / `sym_len_term` | Kleene combine (same as P2C lit path) | **MATCH** |
| Soft fall-through | unknown / non-shape | runtime / prove (unchanged P2C) | **MATCH** (design) |
| FixPatch / GAP-D2 | untouched | no new diag code / fix kind | **MATCH** |

---

## 4. Overclaim scan

| Claim risk | Status |
|------------|--------|
| Full REQ-REFINE-2 / general symbolic arithmetic | **not claimed** |
| `xs.len() - 1` compile-time reject | **not claimed** (soft by design) |
| Aliased bindings (`let ys = xs; … ys.len()`) | **not claimed** (soft by design) |
| GAP-C2 SMT `len` measure encode | **not claimed / still OPEN** |
| List-literal length propagation | **not claimed** |
| "labels/IFC" / value-label / R2 ergonomics | **not touched** |
| Soft cases are bugs | **do not say this** — they are intentional design |

---

## 5. Smoke (soft re-run 2026-07-20)

```text
cargo test -p vera --lib              -> 59 passed; 0 failed
cargo test -p vera --lib gapc1_       -> 3 passed
soft_smoke.ps1                        -> SOFT-SMOKE PASS
prove_clamp.vera                      -> 6 proved; 0 runtime-checked; 0 refuted
```

---

## 6. Return format (Estonian -- Madis paste / Claude optional re-check)

```text
## VERDICT
PASS

## Kokkuvote
GAPC1-SYM-LEN soft-review'i jargi PASS: same-term nth(xs, xs.len())
compile-time TypeError, baasliin 59 (oli 56), soft_smoke PASS, prove_clamp 6.
Soft cases (len-1, other-list, aliases, Kleene ||) = design, mitte bug.
GAP-C2 / full REQ-REFINE-2 ei ole claim'itud. Code commit 4fbf7df / publish 0bc3c22.
Next = TBD Madis-gated (GAP-C2 voi value-label) -- soft ei vali.

## Leiud
none (CRITICAL/HIGH/MED)

## Acceptance
A1 PASS | A2 PASS | A3 PASS | A4 PASS | A5 PASS | A6 PASS

## Smoke
59 tests; soft_smoke PASS; prove_clamp 6; gapc1_ 3 OK

## Next
Madis: TBD next hard task (GAP-C2 or value-label) -- soft does not pick.
Fable: STOP until Madis pastes next implement pointer.
```

---

## Soft close-out rule

After soft docs commit + push: ACK carries real hashes; next hard task remains **TBD Madis-gated** (do not default-pick GAP-C2 or value-label).
