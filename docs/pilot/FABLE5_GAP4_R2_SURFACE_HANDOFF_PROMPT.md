<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAP4_R2_SURFACE_IMPLEMENT.md
Full brief stays in THIS file. Not a review prompt. Not full IFC.
STATUS: recommended next hard task after CONF-P2 E (awaiting Madis paste / green-light).
-->

# Fable 5 -- VERA GAP4-R2-SURFACE handoff (thin label typecheck surface)

Canonical full brief for **GAP4-R2-SURFACE only** (post-E next hard slice). Madis pastes [`CLAUDE_POINTER_GAP4_R2_SURFACE_IMPLEMENT.md`](CLAUDE_POINTER_GAP4_R2_SURFACE_IMPLEMENT.md) into chat when ready.

**Why this task (register order):** CONF-P2 A--E are closed for implement (E LANDED soft-review PASS, awaiting Madis commit). Remaining CONF-P2 surface called out in [`CURSOR_SYNC_ACK_P2E.md`](CURSOR_SYNC_ACK_P2E.md): **labels / R2 ergonomics (GAP-4 OPEN gate)**. Lattice math already exists (`[GAP4-R2-PILOT]` / `label.rs`). Next closed fragment = **wire a thin surface into the checker** so one explicit-flow reject is real -- not prose, not full IFC.

---

You are continuing **VERA** (`vera-lang`). Madis is the operator. This session implements **GAP4-R2-SURFACE**: one thin **label typecheck surface** on top of the existing lattice pilot.

## Hard constraints

1. Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. Never touch TradingBot mainnet / `.env` / live state.
3. No git commit/push unless Madis asks.
4. Prefer zero new Cargo crates; ask before any.
5. Surgical diffs; ask before >~30 lines.
6. Code/docs English; UTF-8; prefer ASCII punctuation (`->`, `--`, `>=`).
7. No file renames (especially `examples/`).
8. Preserve A--E contracts: refine markers, `ProvedSet` / `--prove-run`, FixPatch additive/`ephemeral: true`, soft_smoke / prove_clamp regression.
9. **Honest-limits habit:** slice note must include an HONEST LIMITS / claimed-vs-not table; sync tone = do not overclaim.
10. **Never** add examples that fail typecheck (`round_trip_all_examples`). Demo reject cases stay out of `examples/` (same pattern as P2E non-exhaustive demo).
11. **Do NOT jump to full IFC.** No policies, quarantine, `infer`, actors, endorse/declassify, implicit flows.

## Preconditions

| Item | Status |
|------|--------|
| CONF-P2 A--D | **done** (review PASS where applicable) |
| CONF-P2 E FixPatch | **LANDED** `[P2E-FIX]` -- soft review PASS; **prefer Madis commit before you start** so `typecheck.rs` / baseline are clean on `main` |
| GAP-4 lattice pilot | **LANDED** `d4aebd3` `[GAP4-R2-PILOT]` -- [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md); `label.rs` has `Atom`/`Label`/`flows_to`/`taint_prop` |
| GAP-5 / INV-2 | **DESIGNED** -- cite note; do not build durable store (GAP-D2) unless Madis re-scopes |
| Baseline | **53** lib tests expected post-E; prove_clamp **6** proved |

Debt register: [`KNOWN_GAPS.md`](KNOWN_GAPS.md) (GAP-4 OPEN = R2 ergonomics / CONF-P2 label gate).

## Already done (do not re-open / do not re-implement)

| Slice | Status | Pointers |
|-------|--------|----------|
| GAP4-R2-PILOT | lattice-math evidence ONLY | `label.rs`, [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md) |
| P2E-FIX | ephemeral FixPatch JSON | [`P2E_FIXPATCH_SLICE.md`](P2E_FIXPATCH_SLICE.md) |
| GAP-D2 | durable store | **out of scope** unless Madis explicitly switches task |

## SPEC anchors (read, stay thin)

- SPEC §4.2 unified label lattice; DP4 one label; risk **R2** (inference ergonomics).
- CONF-P2 wording: ill-labeled flows rejected (E1/E5 injection + E6 leak shapes) **and** label-inference ergonomics gate -- this slice advances **checker reject evidence**, not the full ergonomics corpus gate.
- Implicit flows remain **[UNVERIFIED/OPEN]** -- out of scope.
- MVP today: `uses` clause is the main authority surface; value labels `T^{...}` may be post-MVP -- pick the **smallest** surface that makes one E1-or-E6-shaped reject executable in typecheck (document which surface you chose).

## What YOU must do (smallest closed fragment)

1. Read this brief + [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md) + `crates/vera/src/label.rs` + SPEC §4.2 + [`KNOWN_GAPS.md`](KNOWN_GAPS.md).
2. Deliver **one** demonstrable typecheck reject (or accept+reject pair) that uses the existing lattice, e.g.:
   - **Preferred:** typecheck path that rejects an E1-shaped or E6-shaped **explicit** flow using `Label::flows_to` (sink upper bound), **or**
   - Minimal parse + typecheck of a tiny value-label / annotated sink on **one** demo path (if grammar change needed, ask Madis before >~30 lines / parser widen).
3. Prefer **reusing** `label.rs` APIs; do not rewrite the lattice.
4. Marker: `[GAP4-R2-SURFACE]` (grep uniqueness first). Likely touch: `typecheck.rs` (+ maybe thin AST/parser only if unavoidable and approved).
5. Slice note: `docs/pilot/GAP4_R2_SURFACE_SLICE.md` with:
   - what surface landed
   - claimed vs not-claimed table
   - HONEST LIMITS: **not** full IFC; **not** R2 ergonomics gate closed; **not** implicit flows; **not** GAP-D2; FixPatch stays ephemeral
6. Update [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-4 row: PARTIAL/progress toward surface -- **do not** mark R2 ergonomics gate CLOSED unless you also ship a documented ergonomics probe Madis accepts (default: leave gate OPEN, close only "no checker surface").
7. Unit test(s) + soft_smoke PASS; prove_clamp still 6 proved; suite count documented (expect 53 + N).
8. Do **not** implement: policies, quarantine, `infer`, actors, endorse/declassify, durable INV-2 store, MCP server, z3 crate, Salsa.

### Smoke

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- --prove examples/prove_clamp.vera
# plus your reject/accept demo (not in examples/ if it fails typecheck)
```

## Correct work (PASS bar)

- [ ] >=1 typecheck path rejects an ill-labeled **explicit** flow (E1 or E6 shape documented)
- [ ] Reuses `[GAP4-R2-PILOT]` lattice; marker `[GAP4-R2-SURFACE]`
- [ ] Slice note with HONEST LIMITS / claimed-vs-not (no full IFC / no R2 gate closed by default)
- [ ] soft_smoke PASS; prove_clamp 6; no typecheck-failing committed examples
- [ ] KNOWN_GAPS GAP-4 updated honestly (surface progress vs gate still OPEN)

## Out of scope

Full IFC; R2 full corpus ergonomics measurement (optional light probe only if Madis asks); GAP-D2 durable store; FixPatch expansion; GAP-C1/C2 symbolic len; Phase 3 MCP; rewriting A--E.

## Alternates (Madis may switch -- do not self-switch)

1. **GAP-D2** durable INV-2 keys / FixPatch store -- only if Madis wants a durable consumer now (cite [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md)).
2. **GAP-C1** symbolic `nth`/`len` compile-time reject fragment -- refine debt, not the remaining CONF-P2 label surface.

## Return format (English short)

```text
## VERDICT
DONE-GAP4-SURFACE | PARTIAL | BLOCKED

## What landed
...

## Honest limits
...

## Smoke
lib tests: N; soft_smoke; prove_clamp 6

## Next
...
```

End of GAP4-R2-SURFACE handoff.
