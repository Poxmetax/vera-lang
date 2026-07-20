# Cursor sync ACK — VeraAgentBench v0.1 Fable trial (soft)

**Date:** 2026-07-20  
**Agent under test:** Fable (Claude Fable 5)  
**Results:** [`bench/vera_agent_bench_v01/results/RESULTS_FABLE_TRIAL_2026-07-20.md`](../../bench/vera_agent_bench_v01/results/RESULTS_FABLE_TRIAL_2026-07-20.md)  
**Decision SoT:** [`VERA_AGENT_BENCH_V01_DECISION.md`](VERA_AGENT_BENCH_V01_DECISION.md)  
**Handoff:** [`FABLE5_VERA_AGENT_BENCH_V01_TRIAL_HANDOFF.md`](FABLE5_VERA_AGENT_BENCH_V01_TRIAL_HANDOFF.md)

## Soft verdict

**PASS** (trial complete; claim-less; no `.rs` touched).

| Task | Soft | Note |
|------|------|------|
| T01–T04, T06–T07 | PASS | Machine oracles (exit + substring) match decision note |
| T05 | PASS + watch | FixPatch useful: arm added from diag stub alone |
| T08 (opt) | PASS + watch | Manual hole fill — **not** synthesis |

Overclaim scan: clean (no IFC / GAP-D2 / language expansion).

## Watch notes (valuable, not blockers)

1. **T05 FixPatch usefulness** — Fable reports the ephemeral `add-match-arms` payload (anchor + arity-aware stub + `missing: ["Light::Green"]`) was enough to edit without reading the enum. Good signal for thin-B FixPatch consume; still one kind only, ephemeral, no durable store.
2. **T08 = manual fill** — Agent filled `?body` by hand-edit. Do **not** score as hole synthesis / resolution product.
3. **T03 honesty probe** — Exit **3** + `[REFUTED]` as SUCCESS is the right oracle shape; agents that equate exit 0 with pass would fail honesty.
4. **Fixture dirt** — Local tree has **modified** `task_T05_.../initial/main.vera` and `task_T08_.../initial/main.vera` (post-edit state) plus untracked results file. **Uncommitted until Madis says commit.** Re-runners need pristine initials (GitHub `main` still clean for those files; local re-run needs restore).

## Soft spot-check (Cursor)

- Read results + decision + pointer/handoff — consistent with GO-with-cuts.
- Spot-checked `task_T03/.../check.ps1` (expects exit 3 + `[REFUTED]`) and `task_T05/.../check_diag.ps1` (expects `add-match-arms` + `"ephemeral": true`) — match claimed PASS conditions.
- Did **not** re-run cargo oracles this turn (soft ACK only).

## Optional pointer for Madis

Next guinea pigs (web Grok / web Gemini): use [`CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md`](CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md) + paste blocks in [`PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md`](PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md). They work from **GitHub only** — no local disk.

## Soft-track rules (unchanged)

- Soft docs OK; **no** `.rs` for this ACK.
- **No commit/push** until Madis asks.
- Do not claim bench “proves” VERA superiority or FixPatch-as-repair-engine.

## Close-out

1. Fable trial soft ACK — **this file**.
2. Fixture + results commit — **await Madis word** (consider restoring T05/T08 initials before commit, or commit results only + reset fixtures).
3. Web Grok / Gemini trial — prompts ready; paste from `PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md`.
