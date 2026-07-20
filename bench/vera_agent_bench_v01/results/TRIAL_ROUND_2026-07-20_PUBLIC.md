# VeraAgentBench v0.1 — trial round 2026-07-20 (public, claim-less)

**Date:** 2026-07-20  
**Bench:** [`bench/vera_agent_bench_v01/`](../)  
**Rule:** Prefer claim-less. This is a soft trial note for humans and agents — **not** a leaderboard, **not** a language ranking.

---

## 1. What VeraAgentBench v0.1 is / is not

### Is

A small, hand-oracled CLI probe over surfaces that already exist in VERA today:

- run / typecheck
- `--prove` (honest `PROVED` / `RUNTIME-CHECKED` / `REFUTED`)
- `--diag-json` + ephemeral FixPatch `add-match-arms`
- `--round-trip`
- thin optional edits (match arm; fill `?body` by hand)

Tasks are machine-checkable (exit codes + substrings). Agents are expected to stay claim-less.

### Is not

- IFC / full label authoring
- Durable FixPatch / store edit-tx campaigns
- Hole synthesis as a product
- Cross-language “better than Python/Rust” comparison
- Production-ready tooling or a multi-model leaderboard
- Proof that free web chat agents can run the CLI without a real host

---

## 2. Scoreboard (same day)

| Agent | Mode | Result | Notes |
|-------|------|--------|-------|
| **Fable** (local) | Full local CLI | **8/8 PASS** | Clean FixPatch path on T05; T08 = manual `?body` fill (no synthesis claim) |
| **Gemini** | Mode B (operator-pasted CLI) | **T01–T04, T06 PASS**; **T05 INVALID**; **T07–T08 SKIP** | T05 fixture was already exhaustive (contaminated after Fable’s edit) — FixPatch path not honestly exercised |
| **Grok** | Mode B (operator-pasted CLI) | **T01–T07 PASS**; **T05** after restoring `fixtures_failing`; **T08 PASS\*** | T08 PASS\* = pre-filled / documented fixture state — not scored as hole synthesis |
| **Free chat** (Grok / Gemini web sandboxes) | No Mode B | **All SKIP(env)** | Host limitation (no clone / cargo / vera / writable FS) — **not** a language fail |

**How to read this:** Fable shows the task set is runnable on a full local host. Mode B shows operator-mediated CLI paste works when the fixture is clean. Free-web SKIP(env) measures sandbox capability, not VERA surface quality. Do **not** treat rows as a model ranking.

Internal detail notes (optional): [`RESULTS_FABLE_TRIAL_2026-07-20.md`](RESULTS_FABLE_TRIAL_2026-07-20.md). Gemini Mode B soft scoring notes: [`VERA_AGENT_BENCH_V01_GEMINI_MODE_B_2026-07-20.md`](../../../docs/pilot/VERA_AGENT_BENCH_V01_GEMINI_MODE_B_2026-07-20.md).

---

## 3. Pros (VERA + bench)

- **Triad exercised:** run / prove / diag+FixPatch / round-trip on real CLI oracles.
- **Ephemeral FixPatch** was useful when the fixture was still non-exhaustive (Fable T05).
- **Round-trip** task completed cleanly.
- **Honest prove labels** worked as designed (`REFUTED`, `RUNTIME-CHECKED` — not papered over as `PROVED`).
- **Mode B** works with an operator pasting CLI evidence.
- Agents under test largely **stayed claim-less** (no IFC / durable FixPatch / synthesis product claims in the recorded trials).

---

## 4. Cons / lessons

- **Fixture contamination:** editing `task_*/initial/` during a trial leaves the public/agent tree dirty for the next run (Gemini T05 INVALID). Hygiene: restore from SoT (`fixtures_failing/` or scaffold) after each trial; do not leave fixed copies in `initial/`.
- **Mode B is mostly oracle-following**, not hard unaided reasoning — valuable for surface smoke, weak as a deep capability claim.
- **Free web sandboxes** often cannot run CLI / cargo / network clone → honest SKIP(env).
- **T05 / T08 hygiene** must be explicit in protocol (non-exhaustive SoT; unfilled `?body` SoT; document PASS\* when pre-filled).
- **Cognitive load** of contracts / prove / diag JSON is real for agents and operators.
- **Early-stage surface** — thin pilot, small task set, hand oracles.

---

## 5. No overclaim (keep this clean)

This round does **not** claim:

- IFC or full labels
- Durable FixPatch
- Hole synthesis / automatic fill quality
- “Better than Python/Rust”
- Production-ready VERA or production-ready bench harness
- That SKIP(env) is a model or language failure
- That Mode B PASS equals deep autonomous reasoning

---

## Fixture hygiene (post-trial)

As of this note, trial `initial/` fixtures were restored toward SoT so the tree is not left contaminated:

- **T05:** from `fixtures_failing/nonexhaustive_light.vera` (Green arm absent)
- **T08:** unfilled `?body` hole form (scaffold SoT)

Future agents: edit a working copy or restore after PASS — do not commit a fixed `initial/` unless the task redesign intentionally changes the starting state.

---

## End
