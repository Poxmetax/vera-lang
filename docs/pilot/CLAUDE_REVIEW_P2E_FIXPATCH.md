<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_P2E_REVIEW.md
Full review = THIS file. Soft-track filled verdict 2026-07-20 (Cursor).
Not an implement handoff. Optional: Claude re-verify before Madis commit.
-->

# Claude / soft review -- P2E-FIX (FixPatch JSON)

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | `[P2E-FIX]` -- machine-applicable FixPatch on structured diagnostics |
| **Date** | 2026-07-20 (post-land soft review; **not committed**) |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | Fable 5 CONF-P2 E DONE-E; gaps-before-E baseline was 50 |
| **Primary sources** | `P2E_FIXPATCH_SLICE.md` (SoT), `FABLE5_CONF_P2E_HANDOFF_PROMPT.md`, `CLAUDE_POINTER_P2E_IMPLEMENT.md`, `GAP5_INV2_DESIGN_NOTE.md`, `P2B_DIAG_SLICE.md`, SPEC DP8/CONF-P2 |
| **Reviewer** | Cursor soft-track (independent); optional Claude re-check via pointer |
| **Audience** | Madis + Claude -- **review only**, not implement |

---

## 0. Standing constraints

1. Isolation: no TradingBot mainnet / `.env` / live state.
2. No rename; no `_probe_*` temps.
3. No git commit/push unless Madis asks.
4. Soft does not edit: `vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`, `diag.rs`, `main.rs`, `store.rs`, `render.rs`, `label.rs`.
5. Do not claim durable FixPatch store (GAP-D2) or reopen GAP-2..5.

---

## 1. What Fable claimed (verified)

- **Marker:** `[P2E-FIX]`
- **Files:** `typecheck.rs` (`MatchFixInfo`, `TypeError` arity +2nd field, three non-exhaustive sites, `pattern_stub`), `diag.rs` (`FixPatch` + additive `Diagnostic.fix`), `store.rs` (constructor arity only -- `TypeError(msg, None)` x4), docs `P2E_FIXPATCH_SLICE.md` + `P2B_DIAG_SLICE.md` additive fix line
- **main.rs:** unchanged -- `--diag-json` serializes whole report
- **Tests:** +3 (`p2e_non_exhaustive_enum_match_carries_full_fix_payload`, `fixpatch_attached_to_non_exhaustive_match`, `fixpatch_omitted_on_fixless_type_error`); suite **53** (was 50)
- **Backups:** `{typecheck,store,diag}.rs.bak_20260720_022747_p2e_fix`, `P2B_DIAG_SLICE.md.bak_20260720_023416_p2e_fix_pointer` (present on disk)
- **Not done / not claimed:** durable INV-2 store; store-level FixPatch JSON (SPEC §6.2); arm bodies; multi TYPE-ERROR collection; literal-scrutinee exhaustiveness; full RepairPlan / MCP

---

## 2. Acceptance (E1-E6)

| ID | Criterion | Verdict | Evidence |
|----|-----------|---------|----------|
| E1 | >=1 diagnostic emits JSON FixPatch (documented shape) | **PASS** | Live `--diag-json` Signal demo: `fix.kind=add-match-arms`, `ephemeral=true`, `missing=["Signal::Sell(_)","Signal::Hold"]`, exit 1 |
| E2 | Additive schema; omit-not-null; human text usable | **PASS** | `skip_serializing_if`; test `fixpatch_omitted_on_fixless_type_error` asserts no `"fix"` key; text message shape preserved |
| E3 | Ephemeral contract machine-visible; no durable store claim | **PASS** | `FixPatch.ephemeral: true` always; slice + GAP5 note cite GAP-D2; no on-disk patch cache |
| E4 | Connection-consistency MatchFixInfo -> FixPatch | **PASS** | Writer `typecheck.rs` `at_fix` / `MatchFixInfo{span,missing}` -> reader `diag.rs` `diagnostic_from_type_error` maps `e.1` -> `FixPatch{kind,ephemeral,span,missing}` |
| E5 | soft_smoke PASS; prove_clamp 6; lib **53** | **PASS** | Soft re-ran 2026-07-20: 53 passed; SOFT-SMOKE PASS; 6 proved |
| E6 | Honest limits / no overclaim | **PASS** | Slice table matches code: one kind, fail-fast, start-only span, no arm bodies, literal match unchanged, store FixPatch not claimed |

---

## 3. Connection-consistency (both ends read)

| Hop | Writer | Reader | Verdict |
|-----|--------|--------|---------|
| `MatchFixInfo.span` / `missing` | `typecheck.rs` `at_fix` + Option/Result/enum sites | `diag.rs` `diagnostic_from_type_error` `e.1` -> `FixPatch` | **MATCH** |
| `FixPatch` JSON fields | `diag.rs` struct Serialize | `--diag-json` / serde consumers; soft demo | **MATCH** (`kind`, `ephemeral`, `span`, `missing`) |
| `TypeError` 2-tuple | `typecheck.rs` definition | `store.rs` x4 `TypeError(msg, None)` arity-only | **MATCH** (behavior unchanged; no store JSON surface) |
| Ephemeral guard | `diag.rs` `ephemeral: true` | `GAP5_INV2_DESIGN_NOTE.md` E-stays-ephemeral rule | **MATCH** |

---

## 4. Overclaim scan

| Claim risk | Status |
|------------|--------|
| Full RepairPlan / MCP | **not claimed** |
| Durable FixPatch / INV-2 store | **not claimed** (`ephemeral: true`; GAP-D2 open) |
| Store-level FixPatch (SPEC §6.2) | **not claimed** (arity-only in store) |
| Labels / IFC | **not touched** |
| All TYPE-ERRORs get fixes | **not claimed** (non-exhaustive match only) |
| Enum multi-missing message | **approved behavior change** (SPEC plural); single-missing text byte-identical per slice |
| TypeError arity | **approved** (all 6 construction sites + store x4) |

---

## 5. Smoke (soft re-run 2026-07-20)

```text
cargo test -p vera --lib          -> 53 passed; 0 failed
cargo test -p vera --lib fixpatch -> 2 passed
cargo test -p vera --lib p2e_     -> 1 passed
soft_smoke.ps1                    -> SOFT-SMOKE PASS; prove_clamp 6 proved
--diag-json Signal demo           -> TYPE-ERROR + fix block; exit 1
```

---

## 6. Return format (Estonian -- Madis paste / Claude optional re-check)

```text
## VERDICT
PASS

## Kokkuvote
P2E-FIX on soft-review'i jargi PASS: FixPatch JSON lisandub ainult non-exhaustive
match'ile, ephemeral:true on masinloetav, skeem on additive (omit-not-null),
baasliin 53 (oli 50), soft_smoke PASS, prove_clamp 6. Durable store / GAP-D2
ei ole claim'itud. Muudatused on working tree's -- Madis peab ise commitima.

## Leiud
none (CRITICAL/HIGH/MED); LOW note: TypeError arity + store.rs x4 None --
dokumenteeritud slice'is / KNOWN_GAPS ristviites; mitte regressioon.

## Acceptance (E1-E6)
E1 PASS | E2 PASS | E3 PASS | E4 PASS | E5 PASS | E6 PASS

## Smoke
53 tests; soft_smoke PASS; prove_clamp 6; diag-json fix block OK; exit 1

## Next
Madis: cargo test -p vera --lib (kinnitus) -> commit P2E (exclude *.bak_*);
optional soft push vera-github; do NOT open GAP-2..5 / GAP-D2.
```

---

## 7. Soft TODOs (this sync)

1. Soft baselines **50 -> 53** (README, COMMIT_CHECKLIST, SOFT_PARALLEL_QUEUE, KNOWN_GAPS)
2. `CURSOR_SYNC_ACK_P2E.md`
3. Queue: E **LANDED** awaiting Madis commit
4. Optional Claude re-review via this file + pointer (not required for soft PASS)

## Out of scope

GAP-D2 durable store; GAP-2..5 reopen; FixPatch auto-apply; labels/IFC; commit/push; mainnet.
