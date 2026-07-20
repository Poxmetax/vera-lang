# P2E-FIX slice — machine-applicable FixPatch on structured diagnostics (handoff task E)

**Date:** 2026-07-20 · **Marker:** `[P2E-FIX]` · **Files:** `crates/vera/src/typecheck.rs` (payload), `diag.rs` (`FixPatch` + `Diagnostic.fix`), `store.rs` (constructor-arity only)

## What landed (SPEC DP8 / CONF-P2 "JSON diagnostics with FixPatch emitted")

The one deterministic case SPEC §4.1 itself names: a **non-exhaustive `match`**
is a compile error "with the missing constructors named in the diagnostic (and
a `FixPatch` adding the arms)". That case now emits exactly that:

1. **`typecheck.rs`** — `MatchFixInfo { span, missing }` (plain data, serde-free):
   at the three non-exhaustive sites (Option / Result / user enum) the error is
   built with `TypeError::at_fix`, carrying the match expression's span plus the
   uncovered arms as **valid, arity-aware pattern stubs** (`pattern_stub`:
   `Signal::Sell` + 1 field -> `Signal::Sell(_)`; 0 fields -> bare name).
   `TypeError` gained a second tuple field `Option<MatchFixInfo>` (all 6
   construction sites adapted; `Display` unchanged — text CLI output identical
   shape). The user-enum message now names **ALL** missing variants
   (`missing Sell, Hold`), matching SPEC §4.1 plural wording; the
   single-missing message text is byte-identical to before.
2. **`diag.rs`** — serializable `FixPatch` + additive `Diagnostic.fix` field
   (`skip_serializing_if` — omitted when absent, never `null`, per the P2B
   schema contract). `diagnostic_from_type_error` maps the payload; parse /
   prove / prove-error diagnostics carry no fix.
3. **CLI:** no `main.rs` change — `--diag-json` serializes the whole report, so
   the `fix` block rides along; human text mode is untouched.

## FixPatch JSON shape (documented fields — honest subset, not a RepairPlan)

| Field | Meaning |
|---|---|
| `kind` | fix kind; this slice ships exactly one: `"add-match-arms"` |
| `ephemeral` | always `true` this slice — consumers MUST NOT store/replay the patch against drifted code (durable apply requires INV-2 keying — [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md) / GAP-D2) |
| `span` | anchor: the `match` expression this patch targets (`{line, col}`) |
| `missing` | valid arm **pattern stubs** to add, e.g. `["None"]`, `["Signal::Sell(_)", "Signal::Hold"]`; arm bodies are the consumer's choice (VERA has no `todo` construct) |

## Demo (real output, 2026-07-20)

`vera --diag-json p2e_demo_nonexhaustive.vera` (enum `Signal { Buy, Sell(Int), Hold }`, match covers only `Buy`) — exit 1:

```json
{
  "source": "typecheck",
  "severity": "error",
  "code": "TYPE-ERROR",
  "message": "non-exhaustive match on Signal: missing Sell, Hold",
  "span": { "line": 11, "col": 5 },
  "fix": {
    "kind": "add-match-arms",
    "ephemeral": true,
    "span": { "line": 11, "col": 5 },
    "missing": [ "Signal::Sell(_)", "Signal::Hold" ]
  }
}
```

Text mode for the same file (unchanged human shape):
`error: 11:5: non-exhaustive match on Signal: missing Sell, Hold`

The demo file is deliberately NOT in `examples/` — committed examples must
typecheck (`round_trip_all_examples`).

## Honest limits

| Item | Status |
|---|---|
| **No durable INV-2 store** | FixPatch is **EPHEMERAL** (produced, applied-or-discarded within one run/review cycle). A persisted/replayed patch would need `ProofCacheKey`-style keying **plus the target file content hash** it was computed against — [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md); implementing that store is GAP-D2, out of E. The JSON carries `ephemeral: true` so the contract is machine-visible. |
| GAP-1 composition | The payload is only computed on the `check_program` front door, which since `[P2-DUPFN]` rejects duplicate `fn` names **before** match checking — so a fix can never be attributed to a shadowed duplicate. Patch identity is span-based, not fn-name-based. |
| One fix kind only | Only non-exhaustive `match` carries a fix. Other TYPE-ERRORs, PARSE-ERROR, PROVE-* have no mechanical fix this slice (field omitted). Not a full `RepairPlan` planner; no MCP server. |
| Fail-fast unchanged | One TYPE-ERROR per report (P2B contract) — a program with several non-exhaustive matches gets one fix at a time. |
| Span granularity | Start-only `{line, col}` (P2B limit — no end-spans/byte ranges), so the patch is **anchor + pattern stubs**, not a byte-range text edit; the consumer locates the match at the anchor and appends arms before its closing brace. |
| Arm bodies | Not generated (no `todo` construct in VERA); `missing` entries are pattern-side only. |
| Literal-scrutinee matches (Int/Str/Bool) | Exhaustiveness is not statically checked for these today (pre-existing; runtime "no arm matched" trap unchanged, `interp.rs`) — so they carry no fix either. Same surface as the existing check, no new claim. |
| Store transactions | `EditTransaction` typecheck failures now carry the same payload inside `StoreError::Type` for free, but the store has no JSON surface — store-level FixPatch (SPEC §6.2) is **not claimed**. |

## Tests (+3, suite 50 -> 53)

- `typecheck::tests::p2e_non_exhaustive_enum_match_carries_full_fix_payload` — message names ALL missing variants; stubs arity-aware (`Shape::Line(_)`, `Shape::Rect(_, _)`); span pinned.
- `diag::tests::fixpatch_attached_to_non_exhaustive_match` — end-to-end enum + Option: kind/ephemeral/missing/span==diagnostic-span; JSON round-trip asserts the `fix` block.
- `diag::tests::fixpatch_omitted_on_fixless_type_error` — additive-schema proof: fix-less TYPE-ERROR serializes with **no** `"fix"` key.

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib            # 53 passed (was 50)
cargo test -p vera --lib fixpatch   # 2 passed
cargo test -p vera --lib p2e_       # 1 passed
powershell -File docs\pilot\soft_smoke.ps1                 # SOFT-SMOKE PASS
cargo run -p vera -- --prove examples/prove_clamp.vera     # 6 proved (unchanged)
# demo: any .vera with a non-exhaustive match via --diag-json shows the fix block
```

Backups: `crates/vera/src/{typecheck,store,diag}.rs.bak_20260720_022747_p2e_fix`,
`docs/pilot/P2B_DIAG_SLICE.md.bak_20260720_023416_p2e_fix_pointer`.
