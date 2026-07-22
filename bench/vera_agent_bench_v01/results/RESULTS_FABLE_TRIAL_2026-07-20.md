# VeraAgentBench v0.1 — trial results (Fable guinea pig)

**Agent:** Fable (Claude Fable 5 / Claude Code)
**Date:** 2026-07-20
**Workspace:** `vera-lang/`
**Rule:** honest PASS/FAIL only; no language expansion claims

| Task | Verdict | Notes |
|------|---------|-------|
| T01 | **PASS** | run exit 0; stdout `Hello, VERA`; check.ps1 PASS |
| T02 | **PASS** | `--prove` exit 0; `summary: 6 proved, 0 runtime-checked, 0 refuted`; check.ps1 PASS |
| T03 | **PASS** | `--prove` exit **3**; `[REFUTED] bad ensures[0] — sat (counterexample exists)`; honest-REFUTED recognized, no PROVED claim; check.ps1 PASS |
| T04 | **PASS** | `--prove` exit 0; `[RUNTIME-CHECKED] tag ensures[0] — unsupported expr kind for SMT slice`; check.ps1 PASS |
| T05 | **PASS** | step 1: `--diag-json` exit 1 with `fix.kind: "add-match-arms"`, `ephemeral: true`, `missing: ["Light::Green"]` (check_diag PASS); step 2: added `Light::Green => "green",` arm (1-line edit to the fixture, guided by the FixPatch anchor 9:5 + stub); step 3: run exit 0, prints `red` (check_after_edit PASS) |
| T06 | **PASS** | `--round-trip` exit 0; `round-trip OK  program#8bc8faa3ce870982`; check.ps1 PASS |
| T07 | **PASS** | run exit 0; prints `20` (len-measure refine in-range call); check.ps1 PASS |
| T08 (opt) | **PASS** | check_initial_rejects PASS (unfilled `?body` rejected first — pinned pre-state); filled hole with the standard ite clamp body by hand-edit; run exit 0, prints `5`; check_after_fill PASS. **No synthesis claim** — this was a manual edit. |

## Observations (agent's honest notes, no product claims)

- Every oracle was machine-checkable from exit code + substring; zero human
  judgment needed for T01–T04/T06/T07.
- The T05 FixPatch payload was genuinely sufficient to apply the fix without
  reading the enum declaration: anchor span + arity-aware stub named the
  exact missing arm. Ephemeral flag was visible in the JSON as documented.
- T03's non-zero exit (3) being the SUCCESS condition is a good honesty probe
  — an agent that only pattern-matches "exit 0 = good" would misreport it.
- No task required (or tempted) a `.rs` edit; the CLI surface was enough.

## Overclaim scan (must stay clean)

- [x] Did not claim IFC / full labels
- [x] Did not open GAP-D2
- [x] Did not expand language features (only bench fixture `.vera` edits: T05 arm, T08 hole fill)
- [x] Used only vera CLI flags listed in prompts (run / `--prove` / `--diag-json` / `--round-trip`)

## Stop

First batch complete (T01–T06 + T07 + optional T08) → **STOP for maintainer review.**
