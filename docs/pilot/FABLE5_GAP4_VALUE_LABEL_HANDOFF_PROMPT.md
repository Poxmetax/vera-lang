<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md
Full brief stays in THIS file. Not a review prompt. Not full IFC. Not R2 gate close.
STATUS: prepared candidate. Madis-gated. Soft does NOT auto-pick vs GAP-C2.
NAME: GAP4-VALUE-LABEL -- next labels step after GAP4-R2-SURFACE (minimal syntax -> existing seeds pass).
NOT: GAP4-R2-ERGO (ergonomics measurement) -- that remains a later Madis-gated probe.
-->

# Fable 5 -- VERA GAP4-VALUE-LABEL handoff (minimal value-label syntax seed)

Canonical full brief for **GAP4-VALUE-LABEL only**. Madis pastes [`CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md`](CLAUDE_POINTER_GAP4_VALUE_LABEL_IMPLEMENT.md) into chat when ready.

**Why this task (candidate, not auto-default):**
- GAP4-R2-SURFACE closed the seeded checker pass (E1/E6 explicit-flow rejects). Soft ACK [`CURSOR_SYNC_ACK_GAP4_SURFACE.md`](CURSOR_SYNC_ACK_GAP4_SURFACE.md).
- Honest limit of that slice: **seeds are test/API only** -- a `.vera` author still cannot produce a non-⊥ label. Natural next fragment (named in surface SoT): **minimal value-label annotation that feeds the same pass**.
- This is **not** the R2 inference-ergonomics gate (CONF-P2 label gate stays OPEN until Madis accepts a measurement probe). This is **not** full IFC.
- Slice name **GAP4-VALUE-LABEL** (not GAP4-R2-ERGO): syntax seed -> surface wiring. Ergonomics = separate later probe if Madis asks.

**Gate:** Madis chooses this over (or after) [`GAP-C2`](FABLE5_GAPC2_HANDOFF_PROMPT.md). Soft does **not** pick. Prefer GAP-C1 / surface committed (done).

---

You are continuing **VERA** (`vera-lang`). Madis is the operator. This session implements **GAP4-VALUE-LABEL**: the thinnest **value-label syntax** (or equivalent annotation) that lets **one** author-visible / parseable label reach the existing `[GAP4-R2-SURFACE]` checker pass -- enough that E1- or E6-shaped reject evidence is no longer seeds-only.

## Hard constraints

1. Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. Never touch TradingBot mainnet / `.env` / live state.
3. No git commit/push unless Madis asks.
4. Prefer zero new Cargo crates; ask before any.
5. Surgical diffs; **ask Madis before >~30 lines** (parser/AST widen needs explicit approval).
6. Code/docs English; UTF-8; prefer ASCII punctuation (`->`, `--`, `>=`).
7. No file renames (especially `examples/`).
8. Preserve A--E + GAP4 surface + GAP-C1 contracts: FixPatch additive/`ephemeral: true`; soft_smoke / prove_clamp regression; surface seeds pass stays green.
9. **Honest-limits habit:** slice note must include HONEST LIMITS / claimed-vs-not; sync tone = do not overclaim.
10. Never add examples that fail typecheck (`round_trip_all_examples`). Reject demos stay unit tests (or soft-fail outside `examples/`).
11. **Do NOT jump to full IFC.** No policies, quarantine, `infer`, actors, endorse/declassify, **implicit flows**, taint-through-computation expansion.
12. **Do NOT claim R2 / CONF-P2 label-inference ergonomics gate closed.** Leave gate OPEN unless Madis separately accepts an ergonomics probe (out of default scope).

## Preconditions

| Item | Status |
|------|--------|
| CONF-P2 A--E | **done** |
| GAP4-R2-PILOT | **LANDED** -- lattice math; reuse `label.rs` |
| GAP4-R2-SURFACE | **CLOSED** -- seeded pass; SoT [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md) |
| GAP-C1 | **CLOSED** -- leave refine path alone |
| Baseline | expect **59** lib tests; prove_clamp **6** proved |

Debt register: [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-4 -- R2 ergonomics / value-label remaining OPEN.

## Already done (do not re-open / do not re-implement)

| Slice | Status | Pointers |
|-------|--------|----------|
| GAP4-R2-PILOT | lattice-math evidence ONLY | `label.rs`, [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md) |
| GAP4-R2-SURFACE | seeded checker pass | [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md) -- **reuse**; feed it |
| GAP-C2 SMT `len` | separate candidate | **out of scope** this session |
| GAP-D2 | durable store | **out of scope** |

## SPEC anchors (read, stay thin)

- SPEC §2 / grammar note: value labels `T^{...}` -- `^` reserved; post-MVP surface; MVP authority surface remains `uses`.
- SPEC §4.2 unified label lattice; DP4; risk **R2** (inference ergonomics) -- this slice advances **language surface**, not the ergonomics corpus gate.
- CONF-P2: ill-labeled flows rejected **and** label-inference ergonomics gate -- this slice may unlock plain-source reject evidence; it does **not** by itself pass the R2 gate.
- Implicit flows remain **[UNVERIFIED/OPEN]** -- out of scope.

## What YOU must do (smallest closed fragment)

1. Read this brief + [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md) + [`GAP4_R2_PILOT_SLICE.md`](GAP4_R2_PILOT_SLICE.md) + `label.rs` + SPEC §4.2 / value-label notes + [`KNOWN_GAPS.md`](KNOWN_GAPS.md).
2. Deliver **one** minimal surface that produces a non-⊥ label from parsed source (or the smallest annotation Madis approves), wired into the **existing** `check_program_labels` / seeds machinery -- preferred: feed the same pass rather than a second checker.
3. Prefer SPEC shape `T^{...}` if grammar cost is small; if parser widen would exceed ~30 lines, **STOP and ask Madis** with options (e.g. narrower annotation vs defer).
4. Marker: `[GAP4-VALUE-LABEL]` (grep uniqueness first). Likely touch: thin parser/AST + typecheck seed population from annotations; **do not rewrite** lattice; avoid rewriting the full surface walk.
5. Slice note: `docs/pilot/GAP4_VALUE_LABEL_SLICE.md` with:
   - what syntax/annotation landed
   - claimed vs not-claimed table
   - HONEST LIMITS: **surface ≠ IFC**; **R2 gate still OPEN**; **no implicit flows**; seeds may remain for tests; FixPatch stays ephemeral; not GAP-C2
6. Update [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-4 row honestly: value-label syntax progress -- **do not** mark R2 inference-ergonomics gate CLOSED.
7. Unit test(s) + soft_smoke PASS; prove_clamp still 6; suite count documented (expect 59 + N).
8. Do **not** implement: policies, quarantine, `infer`, actors, endorse/declassify, durable INV-2 store, MCP, z3 crate, Salsa, GAP-C2, R2 corpus ergonomics measurement (unless Madis explicitly adds it as a separate light probe -- default: no).

### Smoke

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- --prove examples/prove_clamp.vera
# expect: soft_smoke PASS; prove_clamp -> 6 proved; suite 59+N
```

## Correct work (PASS bar)

- [ ] >=1 path where a parsed annotation / value-label produces a non-⊥ label that the surface pass enforces (accept and/or reject documented)
- [ ] Reuses `[GAP4-R2-SURFACE]` / `[GAP4-R2-PILOT]` lattice; marker `[GAP4-VALUE-LABEL]`
- [ ] Slice note with HONEST LIMITS (no full IFC; R2 gate OPEN; no implicit flows)
- [ ] soft_smoke PASS; prove_clamp 6; no typecheck-failing committed examples
- [ ] KNOWN_GAPS GAP-4 updated honestly (syntax progress; gate still OPEN)

## Out of scope (explicit)

- Full IFC / taint propagation through computation / implicit flows
- R2 full corpus ergonomics measurement (CONF-P2 label gate) -- later Madis-gated probe, not this default
- Endorse / declassify / policies / quarantine / `infer` / actors
- GAP-C2 SMT `len` encode
- GAP-D2 durable FixPatch store
- F6 string Debug-escape polish

## Alternatives Madis may choose instead (do not invent)

| Alt | When |
|-----|------|
| GAP-C2 SMT `len` encode | Madis wants prove-tier measure honesty next |
| Thin R2 ergonomics probe only (GAP4-R2-ERGO) | Madis explicitly asks measurement without syntax -- does **not** close gate alone; usually weaker than value-label first |
| F6 string Debug-escape | polish day |
| GAP-D2 | only if durable certs wanted |

Soft does **not** default-pick. Both GAP-C2 and GAP4-VALUE-LABEL are prepared.

## Return (English short)

```text
VERDICT: DONE-GAP4-VALUE-LABEL | BLOCKED | PARTIAL
files: ...
marker: [GAP4-VALUE-LABEL]
smoke: lib N; soft_smoke; prove_clamp 6
honest limits: surface != IFC; R2 gate OPEN; no implicit flows
next suggestion only (Madis decides)
```

End of GAP4-VALUE-LABEL handoff.
