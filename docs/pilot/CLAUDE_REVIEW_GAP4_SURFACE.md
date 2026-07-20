<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAP4_SURFACE_REVIEW.md
Full review = THIS file. Soft-track filled verdict 2026-07-20 (Cursor).
Not an implement handoff. Optional: Claude re-verify before Madis commit.
-->

# Claude / soft review -- GAP4-R2-SURFACE (thin label typecheck surface)

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[GAP4-R2-SURFACE]` -- seeded label typecheck pass (E1/E6-shaped rejects) |
| **Date** | 2026-07-20 (post-land soft review; **not committed**) |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | Fable 5 GAP4-R2-SURFACE implement; soft ACK `CURSOR_SYNC_ACK_GAP4_SURFACE.md` |
| **Primary sources** | `GAP4_R2_SURFACE_SLICE.md` (SoT), `CURSOR_SYNC_ACK_GAP4_SURFACE.md`, `FABLE5_GAP4_R2_SURFACE_HANDOFF_PROMPT.md`, `GAP4_R2_PILOT_SLICE.md`, `KNOWN_GAPS.md`, SPEC §4.2 |
| **Reviewer** | Cursor soft-track (independent); optional Claude re-check via pointer |
| **Audience** | Madis + Claude -- **review / close-out only**, not implement |

---

## 0. Standing constraints

1. Isolation: no TradingBot mainnet / `.env` / live state.
2. No rename; no `_probe_*` temps.
3. No git commit/push unless Madis asks.
4. Soft does not edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`.
5. Do not claim full IFC, value-label syntax, R2 ergonomics closed, or GAP-D2.
6. Do not invent next implement scope in this review session.

---

## 1. What Fable claimed (verified)

- **Marker:** `[GAP4-R2-SURFACE]`
- **Files:** `crates/vera/src/typecheck.rs` only (+319/−1); `label.rs` untouched
- **Surface:** `check_program_labels(program, seeds)` -- seeds map `(fn, binding) -> Label`; E1 named-fn arg vs param bound; E6 `Console.print` vs ∅-data bound; both via `Label::flows_to` (data projection)
- **Front door:** `check_program` ends with EMPTY seeds (inert; all ⊥)
- **Tests:** +3 (`gap4_surface_*`); suite **56** (was 53)
- **Backup:** `typecheck.rs.bak_20260720_040347_gap4_r2_surface` -- **exclude** from commit
- **Not done / not claimed:** value-label syntax `T^{...}`; R2 inference ergonomics; CONF-P2 label gate on plain `.vera` text; taint propagation; implicit flows; policies / quarantine / `infer` / actors / endorse; GAP-D2 / FixPatch changes

---

## 2. Acceptance

| ID | Criterion | Verdict | Evidence |
|----|-----------|---------|----------|
| A1 | Checker-integrated E1-shaped reject (seeded) | **PASS** | `gap4_surface_rejects_untrusted_arg_into_bare_param_e1` |
| A2 | Checker-integrated E6-shaped reject (seeded) | **PASS** | `gap4_surface_rejects_secret_arg_into_console_print_e6` |
| A3 | Accept pair + Auth-vs-data-bound pin | **PASS** | `gap4_surface_accepts_bounded_sink_and_auth_handle` |
| A4 | Front door inert without seeds | **PASS** | empty-seed path + full suite / `round_trip_all_examples` still green |
| A5 | soft_smoke PASS; prove_clamp 6; lib **56** | **PASS** | Soft re-ran 2026-07-20: 56 passed; SOFT-SMOKE PASS; 6 proved |
| A6 | Honest limits / no overclaim | **PASS** | Slice + ACK tables: seeds ≠ syntax; R2 OPEN; not full IFC; FixPatch untouched |

---

## 3. Connection-consistency (both ends read)

| Hop | Writer | Reader | Verdict |
|-----|--------|--------|---------|
| Seeded source label on bare `Name` | seeds map / call-arg walk | `Label::flows_to` vs param / print bound | **MATCH** (one-hop rule documented) |
| Sink upper bound (param / Console.print) | seeds / ∅-data bound | same `flows_to` check | **MATCH** |
| Front door empty seeds | `check_program` wires EMPTY | lattice ⊥ ⊑ ⊥ inert | **MATCH** |
| FixPatch / GAP-D2 | untouched | `ephemeral: true` still holds | **MATCH** (no hop changed) |

---

## 4. Overclaim scan

| Claim risk | Status |
|------------|--------|
| Value-label syntax / author-writable non-⊥ labels | **not claimed** (seeds = test/API only) |
| R2 inference-ergonomics gate closed | **not claimed** (still OPEN) |
| Full IFC / taint / implicit flows | **not claimed** |
| CONF-P2 "ill-labeled flows" on plain `.vera` | **not claimed** |
| GAP-D2 durable store | **not touched** |
| "labels/IFC implemented" | **do not say this** |

---

## 5. Smoke (soft re-run 2026-07-20)

```text
cargo test -p vera --lib              -> 56 passed; 0 failed
cargo test -p vera --lib gap4_surface -> 3 passed
soft_smoke.ps1                        -> SOFT-SMOKE PASS
prove_clamp.vera                      -> 6 proved; 0 runtime-checked; 0 refuted
```

---

## 6. Return format (Estonian -- Madis paste / Claude optional re-check)

```text
## VERDICT
PASS

## Kokkuvote
GAP4-R2-SURFACE soft-review'i jargi PASS: seeded E1/E6 rejects typecheck'is,
front door empty-seed inert, baasliin 56 (oli 53), soft_smoke PASS, prove_clamp 6.
Seeds ≠ syntax; R2 ergonomics OPEN; full IFC / GAP-D2 ei ole claim'itud.
Working tree -- Madis peab ise commitima (exclude *.bak_*). Fable: STOP implement.

## Leiud
none (CRITICAL/HIGH/MED)

## Acceptance
A1 PASS | A2 PASS | A3 PASS | A4 PASS | A5 PASS | A6 PASS

## Smoke
56 tests; soft_smoke PASS; prove_clamp 6; gap4_surface 3 OK

## Next
Madis: cargo test -p vera --lib (kinnitus) -> commit GAP4 surface (exclude *.bak_*).
Fable: do NOT start next task until that commit. Soft next-recommended = GAP-C1 (see queue).
```

---

## Soft close-out rule

After Madis commits: update ACK commit hash if desired; then Madis may paste next **implement** pointer (`CLAUDE_POINTER_GAPC1_IMPLEMENT.md`) -- not before.
