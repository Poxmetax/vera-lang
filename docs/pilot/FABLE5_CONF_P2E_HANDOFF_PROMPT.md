<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_P2E_IMPLEMENT.md
Full brief stays in THIS file. Not a review prompt.
-->

# Fable 5 -- VERA CONF-P2E handoff (FixPatch JSON)

Canonical full brief for **task E only**. Madis pastes [`CLAUDE_POINTER_P2E_IMPLEMENT.md`](CLAUDE_POINTER_P2E_IMPLEMENT.md) into chat -- **GREEN-LIT 2026-07-20** (gaps-before-E complete; baseline **50** -- see [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md)).

---

You are continuing **VERA** (`vera-lang`). Madis is the operator. This session implements **CONF-P2 task E**: machine-applicable **FixPatch** on structured diagnostics (DP8 / CONF-P2).

## Hard constraints

1. Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. Never touch TradingBot mainnet / `.env` / live state.
3. No git commit/push unless Madis asks.
4. Prefer zero new Cargo crates; ask before any.
5. Surgical diffs; ask before >~30 lines.
6. Code/docs English; UTF-8; prefer ASCII punctuation (`->`, `--`, `>=`).
7. No file renames (especially `examples/`).
8. Preserve A-D contracts: refine markers, `ProvedSet` / `--prove-run`, soft_smoke / prove_clamp regression.
9. **Diag schema SoT:** `docs/pilot/P2B_DIAG_SLICE.md`. FixPatch must be **additive** (optional field(s) omitted when absent -- not `null`). Do not break existing diagnostic keys or `--diag-json` consumers.
10. **Honest-limits habit:** slice note must include an HONEST LIMITS table; sync tone = do not overclaim.
11. **Never** add examples that fail typecheck (`round_trip_all_examples`).

## Preconditions / sibling work

| Item | Status |
|------|--------|
| GAP-1 duplicate-fn typecheck reject | **CLOSED** `5c98c75` `[P2-DUPFN]` -- keep example fn names unique |
| GAP-5 / INV-2 | **DESIGNED** `23f2e46` `[GAP5-INV2]` -- cite [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md); do not invent a durable unversioned proof/patch store |
| Gaps campaign | **COMPLETE** GAP-1..5 -- baseline **50**; GAP-4 = lattice-math only (not IFC) |

Debt register: `docs/pilot/KNOWN_GAPS.md`.

## Already done (do not re-open)

| Slice | Status | Pointers |
|-------|--------|----------|
| A-D | done | commits through `77f7077` `[P2D-ELIDE]` |
| Soft smoke | green | **50** tests baseline post gaps-before-E; prove_clamp **6** proved |

## SPEC anchors (E)

- **DP8:** diagnostics are structured JSON with machine-applicable `FixPatch` / `RepairPlan`.
- **CONF-P2:** "JSON diagnostics with `FixPatch` emitted".
- Examples in SPEC: non-exhaustive `match` -> FixPatch adding arms; store typecheck failure may carry FixPatch.

## INV-2 / GAP-5 (must not paint into a corner)

- SoT: [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md) (`23f2e46` / `[GAP5-INV2]` typed key; **no** on-disk cache).
- D's `ProvedSet` is **same-process only** -- fine; do not pretend FixPatch JSON is a durable certificate.
- **FixPatch JSON stays EPHEMERAL** (produced, applied-or-discarded within a run/review cycle) until INV-2 keys are wired for a durable consumer (GAP-D2). A persisted FixPatch would need the same key + target content hash, else a stale patch could apply to drifted code.
- If you emit patches that an agent might store/replay later, document in the slice note: **future durable apply MUST key by content hash + solver/toolchain version (INV-2)** -- out of scope to implement the store now, but **forbidden** to claim patches are permanently valid without that keying.
- Prefer ephemeral "suggest this edit" patches tied to current diagnostic span/message.
- Do **not** claim labels/IFC implemented (GAP-4 = lattice-math evidence only -- [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md)).

## What YOU must do (smallest closed fragment)

1. Read this brief + `P2B_DIAG_SLICE.md` + `diag.rs` + `KNOWN_GAPS.md` (GAP-1, GAP-5).
2. Pick **one** deterministic diagnostic case that can carry a mechanical FixPatch (e.g. TYPE-ERROR with a suggested replacement text/span, or a documented PROVE-REFUTED stub shape). Ship that case well.
3. Define serializable **FixPatch** (Kodo-shaped subset OK): document exact fields in `docs/pilot/P2E_FIXPATCH_SLICE.md`. Honest subset > fake full RepairPlan.
4. Attach FixPatch to **at least one** `--diag-json` diagnostic when computable; omit field when not.
5. Marker: `[P2E-FIX]` (grep uniqueness first). Likely `diag.rs` (+ emission sites); text CLI stays usable; JSON additive.
6. Unit test(s) + soft_smoke PASS; prove_clamp still 6 proved.
7. Update `P2B_DIAG_SLICE.md` with additive FixPatch pointer (do not rewrite whole SoT).
8. Do **not** build MCP server, auto-apply-to-store, labels/IFC, or persistent cert DB.

### Smoke

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- --prove examples/prove_clamp.vera
# plus your --diag-json case showing FixPatch
```

## Correct work (PASS bar)

- [ ] >=1 diagnostic emits JSON including FixPatch (documented shape)
- [ ] Additive schema; human text CLI usable
- [ ] HONEST LIMITS table includes: no durable INV-2 store; GAP-1 composition noted
- [ ] Marker `[P2E-FIX]`; soft_smoke PASS; prove_clamp 6
- [ ] No typecheck-failing examples; no overclaim of full RepairPlan / MCP

## Out of scope

Full RepairPlan planner; durable certificate store; GAP-2/3/4 implementation; Phase 3 MCP server; rewriting A-D.

## Return format (Estonian, short)

```text
## VERDICT
DONE-E | BLOCKED | PARTIAL

## Mis landis
...

## Smoke
...

## Piirangud / blockers
...

## Next
oota review -- CLAUDE_POINTER_P2E_REVIEW.md (when soft prepares it); do not reopen GAP-2..5 unless Madis re-scopes
```

End of CONF-P2E handoff.
