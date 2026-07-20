<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAP4_VALUE_LABEL_REVIEW.md
Full review = THIS file. Soft-track filled verdict 2026-07-20 (Cursor).
Not an implement handoff. Optional: Claude re-verify.
-->

# Claude / soft review -- GAP4-VALUE-LABEL (minimal `T^{...}` value-label syntax)

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[GAP4-VALUE-LABEL]` -- postfix `^{untrusted\|secret}` at fn-param + annotated-let; feeds existing GAP4-R2-SURFACE |
| **Date** | 2026-07-20 (post-land soft review; **committed** `28929dc`) |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | Fable GAP4-VALUE-LABEL implement; sibling GAP-C2 `f8b67cc` already landed; soft ACK `CURSOR_SYNC_ACK_GAP4_VALUE_LABEL.md` |
| **Primary sources** | `GAP4_VALUE_LABEL_SLICE.md` (SoT), `GAP4_R2_SURFACE_SLICE.md`, `CURSOR_SYNC_ACK_GAP4_SURFACE.md`, `KNOWN_GAPS.md`, SPEC §4.2 |
| **Reviewer** | Cursor soft-track (independent); optional Claude re-check via pointer |
| **Audience** | Madis + Claude -- **review / close-out only**, not implement |

---

## 0. Standing constraints

1. Isolation: no TradingBot mainnet / `.env` / live state.
2. No rename; no `_probe_*` temps.
3. Soft does not edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`, `parser.rs`, `ast.rs`, `lexer.rs`.
4. Do not claim full IFC, R2 inference ergonomics closed, auth atoms in `^{...}`, or GAP-D2.
5. Keep **GAP-C2** as a separate slice/review — do not merge verdicts.
6. Do not invent next implement scope in this review session.

---

## 1. What Fable claimed (verified)

- **Marker:** `[GAP4-VALUE-LABEL]`
- **Files:** `lexer.rs` (+1 op char), `ast.rs` (2 fields), `parser.rs` (helper + 2 hooks), `render.rs` (helper + 2 sites), `typecheck.rs` (seed harvest + wiring + tests); `label.rs` untouched
- **Surface:** postfix `^{atom,...}` after annotation type at **param** and **annotated-let** only; atoms `untrusted` / `secret`; canonicalize (sort+dedupe); render round-trip; `collect_label_seeds` → existing `[GAP4-R2-SURFACE]` pass
- **Evidence from plain source:** E1 reject + E6 reject + secret-bound accept (no test seeds)
- **Tests:** +5 (`gap4vl_*`); suite **68** (was 63)
- **Backup:** `*.bak_20260720_053453_gap4_value_label` -- **exclude** from commit
- **Not done / not claimed:** R2 inference; full IFC / taint / implicit flows; return-type / nested-type / lambda-param labels; auth atoms in `^{...}`; GAP-D2 / FixPatch changes

---

## 2. Acceptance

| ID | Criterion | Verdict | Evidence |
|----|-----------|---------|----------|
| A1 | E1-shaped reject from plain `.vera` (untrusted let → bare param) | **PASS** | `gap4vl_rejects_untrusted_let_arg_from_plain_source` |
| A2 | Secret-bound accept + E6 print reject (annotation-only) | **PASS** | `gap4vl_secret_bound_param_accepts_and_console_print_rejects` |
| A3 | Nested-let annotation harvested | **PASS** | `gap4vl_nested_let_label_is_collected` |
| A4 | Render round-trip + unlabeled hash stability | **PASS** | `gap4vl_label_renders_and_reparses_identically` |
| A5 | Unknown / empty atoms are parse errors | **PASS** | `gap4vl_unknown_and_empty_label_atoms_are_parse_errors` |
| A6 | soft_smoke PASS; prove_clamp 6; lib **68** | **PASS** | Soft re-ran 2026-07-20: 68 passed; 5 gap4vl; SOFT-SMOKE PASS; 6 proved |
| A7 | Honest limits / no overclaim | **PASS** | Slice + ACK: not IFC; R2 OPEN; one-hop; FixPatch ephemeral |

---

## 3. Connection-consistency (both ends read)

| Hop | Writer | Reader | Verdict |
|-----|--------|--------|---------|
| Annotation → seed map | `collect_label_seeds` (param / nested let) | `check_program` feeds existing surface pass | **MATCH** |
| Seeded Name arg vs param bound | surface walker (unchanged) | `Label::flows_to` | **MATCH** |
| Render label atoms | canonicalize at parse | render `^{...}` canonical order | **MATCH** (round-trip test) |
| Unlabeled serde | `skip_serializing_if` empty | store hash / round_trip_all_examples | **MATCH** |
| FixPatch / GAP-D2 | untouched | `ephemeral: true` | **MATCH** |

---

## 4. Overclaim scan

| Claim risk | Status |
|------------|--------|
| Full IFC / taint / implicit flows | **not claimed** |
| R2 inference-ergonomics gate closed | **not claimed** (still OPEN) |
| Labels on returns / nested types / lambdas | **not claimed** (parse error) |
| Auth atoms in `^{...}` | **not claimed** (`uses` remains authority) |
| GAP-C2 / GAP-D2 / FixPatch durable | **not touched** |
| "labels/IFC implemented" | **do not say this** |

---

## 5. Smoke (soft re-run 2026-07-20)

```text
cargo test -p vera --lib              -> 68 passed; 0 failed
cargo test -p vera --lib gap4vl_      -> 5 passed
soft_smoke.ps1                        -> SOFT-SMOKE PASS
prove_clamp.vera                      -> 6 proved; 0 runtime-checked; 0 refuted
```

---

## 6. Return format (Estonian -- Madis paste / Claude optional re-check)

```text
## VERDICT
PASS

## Kokkuvote
GAP4-VALUE-LABEL soft-review'i jargi PASS: T^{untrusted|secret} param/let,
collect_label_seeds → olemasolev surface, E1/E6 plain source'st, suite 68
(+5 gap4vl), soft_smoke PASS, prove_clamp 6. Mitte IFC; R2 OPEN; FixPatch
ephemeral. Commit 28929dc / publish f4f3cc7. Fable: STOP implement VL.

## Leiud
none (CRITICAL/HIGH/MED)

## Acceptance
A1 PASS | A2 PASS | A3 PASS | A4 PASS | A5 PASS | A6 PASS | A7 PASS

## Smoke
68 tests; soft_smoke PASS; prove_clamp 6; gap4vl 5 OK

## Next
Madis-gated next = GAP4-R2-ERGO / F6 / GAP-D2 — soft does not pick.
Fable next (approved separately): VeraAgentBench v0.1 trial guinea pig.
```

---

## Soft close-out rule

Code already committed (`28929dc`) and published (`f4f3cc7`). Soft ACK + this review sync docs only. Do **not** re-commit `.rs`. Keep GAP-C2 review separate.
