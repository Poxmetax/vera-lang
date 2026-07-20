# VeraAgentBench v0.1 — Gemini Mode B soft score (2026-07-20)

**Status:** SOFT NOTE — honest scoring of operator-pasted Mode B results; no `.rs`, no commit  
**Agent:** Gemini (Mode B — operator-run / pasted CLI evidence)  
**Date:** 2026-07-20  
**Compare baseline:** Fable local trial `bench/vera_agent_bench_v01/results/RESULTS_FABLE_TRIAL_2026-07-20.md` (8/8 PASS)

---

## 0. Disk facts (verified this turn)

| Check | Result |
|-------|--------|
| `task_T05_.../initial/main.vera` | **Already includes** `Light::Green => "green"` — match is **exhaustive** |
| Comment on that file | Still says “intentionally non-exhaustive” / Expect FixPatch for `Light::Green` — **stale vs body** |
| `fixtures_failing/nonexhaustive_light.vera` | **Correct SoT:** only `Light::Red` arm; Green missing |
| `prompt.md` | Step 1: `--diag-json` → FixPatch `add-match-arms` + `ephemeral: true`; Step 2: edit missing arm(s); Step 3: run exit 0 |
| `checks/check_diag.ps1` | Requires `add-match-arms` + `"ephemeral": true` in diag JSON |
| `checks/check_after_edit.ps1` | Requires post-edit run exit 0 |
| Fable results note | T05 PASS: ran diag (missing Green), **then 1-line edit added Green to the fixture** |
| `git status` | `initial/main.vera` is **modified** (uncommitted Fable residue) |

**Contamination root cause:** Fable’s successful FixPatch trial left the **fixed** copy in `initial/`. Gemini Mode B therefore saw an already-exhaustive fixture and could not honestly exercise the intended FixPatch → edit path.

> **Footnote (post-trial hygiene):** After this Mode B trial, T05 `initial/main.vera` was restored from `fixtures_failing/nonexhaustive_light.vera` (MD5 match to SoT). The pre-restore copy was quarantined under `bench/vera_agent_bench_v01/_operator_archive/`. Historical Mode B scores in this note are unchanged.

---

## 1. Scoreboard (Gemini Mode B — soft)

Operator report: Mode B T01–T06 PASS, T07–T08 SKIP. Terminal evidence treated as real for oracles where applicable.

| Task | Gemini Mode B (honest) | Fable | Notes |
|------|------------------------|-------|-------|
| T01 | **PASS** | PASS | Pasted evidence accepted |
| T02 | **PASS** | PASS | Pasted evidence accepted |
| T03 | **PASS** | PASS | Pasted evidence accepted |
| T04 | **PASS** | PASS | Pasted evidence accepted |
| **T05** | **INVALID** *(or PASS\* fixture contaminated)* | **PASS** (clean FixPatch path) | Fixture already exhaustive → FixPatch path **not** exercised; not a clean Mode B PASS for intended task |
| T06 | **PASS** | PASS | Pasted evidence accepted |
| T07 | **SKIP** (optional OK) | PASS | Optional skip allowed |
| T08 | **SKIP** (optional OK) | PASS | Optional skip allowed |

**Headline (honest):** Gemini Mode B ≈ **5 clean PASS + 1 INVALID/PASS\* (T05) + 2 SKIP** — **not** “6/6 core PASS.”  
**Fable:** **8/8 PASS** on a still-non-exhaustive T05 initial (then left fixture dirty).

Do **not** score Gemini T05 as clean PASS. Do **not** treat Mode B as tied with Fable on FixPatch usefulness.

---

## 2. T05 contamination warning

- **Intended task:** non-exhaustive match → `--diag-json` emits `add-match-arms` / `ephemeral: true` → agent adds missing arm(s) → run exit 0.
- **On disk at Gemini trial time:** `initial/main.vera` already had `Light::Green` → diag would **not** emit the FixPatch oracle (`check_diag.ps1` would FAIL if run against current initial).
- **Verdict label:** **INVALID** preferred; **PASS\*** only if explicitly footnoted “fixture contaminated — FixPatch path not tested.”
- **SoT for restore:** `fixtures_failing/nonexhaustive_light.vera` (Green arm absent). Matches prompt + check_diag design.

---

## 3. vs Fable 8/8

| Dimension | Fable | Gemini Mode B |
|-----------|-------|----------------|
| Core T01–T06 | 6/6 PASS (clean T05) | 5 PASS + T05 INVALID |
| Optional T07/T08 | both PASS | both SKIP (OK) |
| FixPatch usefulness signal | Yes (diag → edit) | **No** (contaminated fixture) |
| Host | Local full CLI | Mode B (operator paste) |

Fable remains the only clean FixPatch (T05) data point until `initial/` is restored and Mode B is re-run.

---

## 4. Recommended next step (one line)

**Restore `task_T05_.../initial/main.vera` from `fixtures_failing/nonexhaustive_light.vera` (keep Fable’s fixed copy under `results/` if needed); Madis commits; then re-run Mode B T05 only.**

Optional keep path suggestion:  
`bench/vera_agent_bench_v01/results/T05_main_after_fable_fix_2026-07-20.vera`

---

## 5. Claim guard

- Soft note only — no language / model superiority claim from this Mode B paste.
- T07/T08 SKIP is protocol-OK, not a fail.
- Earlier soft compare (`VERA_AGENT_BENCH_V01_TRIAL_COMPARE_2026-07-20.md`) covered web **SKIP(env)** trials; this note is **Mode B** (pasted CLI) and supersedes that Gemini row for Mode B scoring only.
- Fixture restore was **not** applied in the original Mode B scoring turn (document-first). See footnote above for the later post-trial restore.

---

## End
