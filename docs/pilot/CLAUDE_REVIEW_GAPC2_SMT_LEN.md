<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAPC2_REVIEW.md
Full review = THIS file. Soft-track filled verdict 2026-07-20 (Cursor).
Not an implement handoff. Optional: Claude re-verify.
-->

# Claude / soft review -- GAPC2-SMT-LEN (opaque `len` measure in VC/SMT)

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[GAPC2-SMT-LEN]` -- `len` as opaque Int constant + `>= 0` axiom in VC encode |
| **Date** | 2026-07-20 (post-land soft review; **committed** `f8b67cc`) |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | Fable GAPC2-SMT-LEN implement; prior GAP-C1 `4fbf7df`; soft ACK `CURSOR_SYNC_ACK_GAPC2.md` |
| **Primary sources** | `GAPC2_SMT_LEN_SLICE.md` (SoT), `GAPC1_SYM_LEN_SLICE.md`, `P2C_LEN_SLICE.md`, `KNOWN_GAPS.md`, SPEC §4.4 |
| **Reviewer** | Cursor soft-track (independent); optional Claude re-check via pointer |
| **Audience** | Madis + Claude -- **review / close-out only**, not implement |

---

## 0. Standing constraints

1. Isolation: no TradingBot mainnet / `.env` / live state.
2. No rename; no `_probe_*` temps.
3. Soft does not edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`.
4. Do not claim list theory, literal-length propagation, call-site WP widening, or full REQ-REFINE-2.
5. Keep **GAP4-VALUE-LABEL** as a separate slice/review — do not merge verdicts.
6. Do not invent next implement scope in this review session.

---

## 1. What Fable claimed (verified)

- **Marker:** `[GAPC2-SMT-LEN]`
- **Files:** `crates/vera/src/vc.rs` only (+226/−0); `smt.rs` untouched (plain subprocess runner)
- **Surface:** `len(xs)` / `xs.len()` → one symbol `vera_len_<xs>` + axiom `(assert (>= … 0))`; Name receivers only; len-free scripts byte-identical
- **Headline:** len-refined param no longer kills len-free ensures → honest **[PROVED]**; `result >= 1` over `xs.len()` stays **[REFUTED]** (c=0 realizable)
- **Tests:** +4 (`gapc2_*`); suite **59 → 63** at land (later **68** after VALUE-LABEL)
- **Backup:** `vc.rs.bak_20260720_052207_gapc2_smt_len` -- **exclude** from commit
- **Not done / not claimed:** list content / literal length; call-site list-arg discharge; `/` `%`; labels / IFC / GAP-D2

---

## 2. Acceptance

| ID | Criterion | Verdict | Evidence |
|----|-----------|---------|----------|
| A1 | Len-refined param enables proved ensures | **PASS** | `gapc2_len_param_refine_assumption_enables_proved_ensures` |
| A2 | Body `xs.len()` nonneg ensures proved by axiom | **PASS** | `gapc2_len_body_nonneg_ensures_proved_by_axiom` |
| A3 | Refutable `result >= 1` stays REFUTED | **PASS** | `gapc2_refutable_len_ensures_stays_honest` |
| A4 | Call-site list args stay RUNTIME-CHECKED | **PASS** | `gapc2_call_site_len_args_stay_runtime_checked` |
| A5 | soft_smoke PASS; prove_clamp 6; suite green | **PASS** | Soft re-ran 2026-07-20: **68** lib (post-VL); 4 gapc2; SOFT-SMOKE PASS; 6 proved |
| A6 | Honest limits / no overclaim | **PASS** | Slice + ACK: opaque only; [P2-SOUND2] call-site gate kept |

---

## 3. Connection-consistency (both ends read)

| Hop | Writer | Reader | Verdict |
|-----|--------|--------|---------|
| Encode `len(xs)` / `xs.len()` | `encode_expr` / `len_sym` | SMT script symbols | **MATCH** (one symbol per name) |
| Decl + axiom | `collect_len_syms*` | `discharge_goal` assumptions | **MATCH** |
| Len-free path | empty collect | prove_clamp scripts unchanged | **MATCH** (6 proved still) |
| Call-site list args | [P2-SOUND2] gate | RUNTIME-CHECKED | **MATCH** (pinned) |
| Labels / FixPatch | untouched | surface + ephemeral | **MATCH** |

---

## 4. Overclaim scan

| Claim risk | Status |
|------------|--------|
| List theory / `[1,2].len() == 2` | **not claimed** |
| Call-site discharge with list arguments | **not claimed** (RUNTIME-CHECKED) |
| Full REQ-REFINE-2 / general symbolic arithmetic | **not claimed** |
| `/` `%` encode / quantifiers / `z3` crate | **not touched** |
| Labels / IFC / value-label / R2 / GAP-D2 | **not touched** |
| Soft GAP-C1 cases silently became proved | **do not claim** — they did not |

---

## 5. Smoke (soft re-run 2026-07-20)

```text
cargo test -p vera --lib              -> 68 passed; 0 failed  (suite after sibling VL)
cargo test -p vera --lib gapc2_       -> 4 passed
soft_smoke.ps1                        -> SOFT-SMOKE PASS
prove_clamp.vera                      -> 6 proved; 0 runtime-checked; 0 refuted
```

Suite chain honesty: **59** (C1) → **63** (C2 land) → **68** (VALUE-LABEL). This review's C2 delta is +4 at the 59→63 step.

---

## 6. Return format (Estonian -- Madis paste / Claude optional re-check)

```text
## VERDICT
PASS

## Kokkuvote
GAPC2-SMT-LEN soft-review'i jargi PASS: opaque vera_len_<xs> + >=0 axiom,
headline PROVED + REFUTED pinned, call-site RUNTIME-CHECKED, +4 gapc2
(59→63), prove_clamp 6 unchanged. Mitte list theory. Commit f8b67cc /
publish f4f3cc7 (with VL). Fable: STOP implement C2.

## Leiud
none (CRITICAL/HIGH/MED)

## Acceptance
A1 PASS | A2 PASS | A3 PASS | A4 PASS | A5 PASS | A6 PASS

## Smoke
68 tests (post-VL); soft_smoke PASS; prove_clamp 6; gapc2 4 OK

## Next
Madis-gated next = GAP4-R2-ERGO / F6 / GAP-D2 — soft does not pick.
```

---

## Soft close-out rule

Code already committed (`f8b67cc`) and published (`f4f3cc7`). Soft ACK + this review sync docs only. Do **not** re-commit `.rs`. Keep VALUE-LABEL review separate.
