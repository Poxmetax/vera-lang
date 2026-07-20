# VeraAgentBench v0.1 — evaluation decision (pre-implementation)

**Date:** 2026-07-20  
**Author:** soft track (Cursor) — evaluate-only; no bench scaffold  
**Status:** DECISION NOTE — not a shipping README  
**Rule:** Prefer claim-less. Soft docs only; do not race Fable on `.rs`.

---

## 0. Proposal summary (Madis)

1. Small honest bench: **12–20 tasks** in buckets **A** single-shot / **B** edit&maintain / **C** long-horizon.  
2. Concrete ideas: refinement+prove, typed holes, FixPatch, Z3-known bug, store round-trip, multi-step CLI, intentional bad state, etc.  
3. Metrics: silent bug rate, verification success, edit integrity, intervention count, recovery success, hole resolution; compare vs Python/Rust on same tasks.  
4. Layout: `task_XX/{prompt.md,initial/,oracle|checks/,meta.json}` + simple runner.  
5. Next steps: 8 tasks first, hand-write oracles, try 2–3 models, then expand.

---

## 1. Overall verdict

**GO with cuts** — ship a **thin, claim-less v0.1** (recommended **6–8 tasks**, not 12–20) focused on what the CLI and existing examples already demonstrate: run / typecheck / `--diag-json` / `--prove` / `--round-trip`. **Cut or defer** long-horizon (C), content-addressed store-as-agent-API loops, FixPatch multi-turn “agent repair” narratives, hole-synthesis tasks, and Python/Rust head-to-head as a headline claim. Layout + hand-written oracles + a tiny runner are fine when Madis gates implementation; **do not** scaffold a full bench tree until that gate.

---

## 2. Grounding — current VERA reality (SoT read 2026-07-20)

| Surface | Honest status |
|---------|----------------|
| README | Research prototype; holes parse + error; refinements + Z3 prove; store round-trip + typed edit transactions; FixPatch ephemeral; labels = lattice + thin surface — **not** full IFC |
| Examples | 13 `.vera` files (hello, clamp, ADT, Option/Result, lists, prove_*, refine_*) — all must typecheck |
| `--prove` / `--prove-run` | Real; PROVED / RUNTIME-CHECKED / REFUTED; Z3 on PATH |
| `--diag-json` | Real; structured diagnostics; FixPatch rides when present |
| FixPatch `[P2E-FIX]` | **One kind only:** `add-match-arms`; **ephemeral**; pattern stubs, **no arm bodies**; fail-fast one diagnostic; **no** durable store (GAP-D2); **no** store JSON FixPatch (GAP-E1) |
| Store | Library `CodebaseStore` + `EditTransaction` (Insert/Replace/Delete fn); CLI `--round-trip` / `--hash-only`; **no** agent CLI for edit txs; **no** MCP server (Phase 3 stub docs only) |
| Typed holes | `?ident` parses; typecheck + runtime reject unfilled; **no synthesis** |
| Labels / GAP-4 | Surface seeded pass **landed** (docs). **`[GAP4-VALUE-LABEL]` code is present on the hard working tree** (parser/AST/render/typecheck + unit tests with `T^{...}`); soft docs (`KNOWN_GAPS`, `SOFT_PARALLEL_QUEUE`) still list it as PREPARED / not soft-ACKed; **no** `examples/*.vera` with `^{` yet. Treat label-authoring bench tasks as **provisional** until soft ACK + slice note + optional example |
| GAP-C2 | Soft register says opaque `len` SMT encode landed (working tree); suite count in soft docs may lag — do not pin bench claims to suite numbers |
| MCP | Docs stub only — agents use CLI or in-process Rust API |

**Implication:** the proposal over-assumes agent-facing store loops, FixPatch repair cycles, hole filling-as-product-feature, and long-horizon tooling. Those are thinner than a 12–20 task / A+B+C marketing shape implies.

---

## 3. Idea-by-idea scorecard

### Buckets

| Bucket | Verdict | Note |
|--------|---------|------|
| **A — single-shot** | **Ready now** (core of v0.1) | Prompt → one artifact → CLI check |
| **B — edit & maintain** | **Needs cuts** | Text edit + re-check is OK; store-tx / FixPatch loops are thin |
| **C — long-horizon** | **Cut from v0.1** | No MCP, no durable certs, no multi-session agent API |

### Concrete task ideas

| Idea | Verdict | Detail |
|------|---------|--------|
| Refinement + prove (clamp / Int ensures) | **Ready now** | Reuse `prove_clamp`, `prove_refuted`, `prove_runtime_hint` patterns |
| Honest RUNTIME-CHECKED / REFUTED recognition | **Ready now** | Oracle = exit code + substring; teaches models not to fake PROVED |
| Typed-hole fill | **Overclaim risk** if framed as synthesis | Ready as **“edit source until typecheck+run”**; cut “synthesis / hole resolution product” |
| FixPatch consume (`add-match-arms`) | **Ready now (narrow)** | `--diag-json` → agent adds arms → typecheck; **not** durable / multi-kind / auto-apply |
| FixPatch multi-step agent loop / repair planner | **Cut from v0.1** | Needs feature: more kinds + apply tooling + GAP-D2 |
| Z3-known bug / wrong ensures | **Ready now** | Pair with `prove_refuted`-style oracle |
| Store round-trip | **Ready now (CLI)** | `--round-trip` on agent-produced or edited source |
| Store edit-transaction agent API | **Needs feature / Overclaim** | Library-only; no CLI/MCP; GAP-E1 no FixPatch on store JSON |
| Multi-step CLI (prove then run, diag then edit) | **Ready now (thin B)** | Cap at 2–3 CLI steps; not “long-horizon” |
| Intentional bad state (non-exhaustive match, bad refine) | **Ready now** | Keep failing fixtures **outside** `examples/` (round_trip invariant) |
| Label / IFC / E1–E6 authoring | **Provisional / Needs soft ACK** | VALUE-LABEL may be in tree; do **not** claim IFC; defer headline until ACK + example policy |
| Cross-lang Python/Rust same-task leaderboard | **Cut from v0.1 headline** | See §4 pitfalls; optional appendix later with matched oracles |

---

## 4. Metrics — honest now vs human/sim

| Metric | Honest automation now? | Notes |
|--------|------------------------|-------|
| **Verification success** | **Yes** | `--prove` exit + `[PROVED]` / `[REFUTED]` / `[RUNTIME-CHECKED]` counts |
| **Typecheck / diag success** | **Yes** | exit 0 vs 1; optional JSON schema smoke |
| **Edit integrity** | **Partial yes** | Post-edit `typecheck` + `--round-trip`; not full “semantic edit intent” |
| **Silent bug rate** | **Human/oracle** | Needs hand-written expected behavior / golden output; “silent” ≠ typecheck green |
| **Intervention count** | **Human/sim** | Define what counts as intervention; runner can log turns if harness exists |
| **Recovery success** | **Human/sim** | Needs a defined recovery protocol; FixPatch is not a recovery engine |
| **Hole resolution** | **Misnamed** | Measure “filled hole → typechecks/runs”; do **not** claim synthesis quality |
| **vs Python / Rust** | **Pitfall-heavy** | Different type systems, no Z3 VC path in Python, Rust borrow≠refinement; “same task” text ≠ same failure modes. Defer comparative claims until oracles are language-normalized and labeled **proxy**, not proof of VERA superiority |

---

## 5. Minimum viable v0.1 scope (recommended 6–8 tasks)

Suggested **first GitHub post** set (names provisional; oracles hand-written; fixtures may live under `bench/` later — **not created by this note**):

| ID | Name | Bucket | Check sketch |
|----|------|--------|--------------|
| T01 | `hello_console` | A | run prints expected line |
| T02 | `prove_clamp_discharge` | A | `--prove` → 6 proved (or pinned count) |
| T03 | `prove_refuted_false_ensures` | A | `--prove` → REFUTED, exit 3 |
| T04 | `prove_runtime_checked_str` | A | ≥1 `[RUNTIME-CHECKED]` |
| T05 | `nonexhaustive_match_fixpatch` | B thin | `--diag-json` has `add-match-arms`; agent edits; typecheck+run |
| T06 | `round_trip_identity` | A | `--round-trip` OK on agent output or fixed seed |
| T07 | `refine_nth_len_ok` | A | typecheck+run `refine_len_ok`-shaped program |
| T08 *(optional)* | `hole_fill_clamp` | B thin | start with `?body`; agent fills; typecheck+run — **no synthesis claim** |

**Explicitly not in first post:** C long-horizon, store edit-tx campaigns, durable FixPatch, label/IFC leaderboard, Python/Rust comparison tables, hole-synthesis scores.

**Layout (when gated):** `task_XX/{prompt.md,initial/,checks/,meta.json}` + thin runner invoking `cargo run -p vera -- …` is enough. Prefer **checks/** scripts over claiming an “oracle interpreter.”

---

## 6. What NOT to claim on a GitHub README for the bench

Do **not** say or imply:

1. VERA has a durable FixPatch / proof cache / agent repair loop (GAP-D2 open; FixPatch `ephemeral: true`).  
2. FixPatch is a general `RepairPlan` (one kind: `add-match-arms`).  
3. Typed holes have synthesis or automatic fill.  
4. Content-addressed store is an agent-facing product API (library + round-trip CLI only today).  
5. Long-horizon / multi-session agent evaluation is supported.  
6. Labels / IFC / CONF-P2 label-inference ergonomics are closed (R2 gate OPEN; VALUE-LABEL needs soft ACK before any authoring claim).  
7. MCP compiler service exists (stub docs only).  
8. Bench proves fewer silent bugs than Python/Rust (without matched oracles + explicit proxy disclaimer).  
9. “12–20 tasks across A/B/C” as shipped v0.1 if only 6–8 land.  
10. Research prototype ≠ production language — keep that framing.

**Allowed framing:** small, hand-oracled probe of CLI surfaces agents can already hit; metrics that are machine-checkable called out separately from human-scored ones.

---

## 7. Implementation gate (soft)

| Step | Action |
|------|--------|
| Now | This decision only — **no** full bench scaffold |
| Optional later | Tiny stub dir + 1 smoke task **only if** Madis says yes and Fable is not conflicting on paths |
| Before public post | Hand oracles for T01–T07; soft-smoke green; no failing fixtures under `examples/` |
| Soft ACK | If VALUE-LABEL soft-ACKed, may add **one** label reject/accept task with honest “not IFC” meta |

---

## 8. Assumption register (R19)

| Claim | Load-bearing? | Status |
|-------|---------------|--------|
| CLI flags `--prove`, `--diag-json`, `--round-trip` exist and match README | YES for A-tasks | VERIFIED — README + `main.rs` usage |
| FixPatch kind set = {`add-match-arms`}, ephemeral | YES for FixPatch task scope | VERIFIED — `P2E_FIXPATCH_SLICE.md` / `diag.rs` |
| No hole synthesis | YES if cutting synthesis metrics | VERIFIED — typecheck/interp unfilled reject |
| Store agent CLI / MCP absent | YES for cutting C + store loops | VERIFIED — MCP stub README; store is lib API |
| VALUE-LABEL soft-documented as shipped | YES for label tasks in v0.1 | **UNVERIFIED / lag** — code markers + tests in hard tree; soft ACK/slice/examples absent → **defer label tasks** |
| 12–20 / A+B+C needed for useful first post | YES for full proposal | **Rejected** — 6–8 A + thin B sufficient |

---

## 9. Recommendation one-liner

**GO with cuts:** claim-less 6–8 task CLI probe; cut C, cut store-as-agent-API, cut FixPatch-as-repair-engine, cut cross-lang leaderboard, defer labels until soft ACK; write oracles by hand; scaffold only after Madis gates.

---

*End of decision note. No bench tree created by this document.*
