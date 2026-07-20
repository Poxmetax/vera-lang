# Claude pointer -- VeraAgentBench v0.1 trial debrief (Fable feedback)

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are **Fable** (Claude Code). Madis is the operator. This is a **soft debrief / feedback** pass — **not** a language-feature implement turn, **not** a new bench run unless Madis asks.

**Public SoT (read first):**  
[`bench/vera_agent_bench_v01/results/TRIAL_ROUND_2026-07-20_PUBLIC.md`](../../bench/vera_agent_bench_v01/results/TRIAL_ROUND_2026-07-20_PUBLIC.md)

**Your local trial log:**  
[`bench/vera_agent_bench_v01/results/RESULTS_FABLE_TRIAL_2026-07-20.md`](../../bench/vera_agent_bench_v01/results/RESULTS_FABLE_TRIAL_2026-07-20.md)

**Bench README:** [`bench/vera_agent_bench_v01/README.md`](../../bench/vera_agent_bench_v01/README.md)

---

## What happened (claim-less)

| Agent | Result |
|-------|--------|
| Fable (local) | **8/8 PASS** |
| Gemini Mode B | T01–T04, T06 PASS; **T05 INVALID** (contaminated exhaustive fixture); T07–T08 SKIP |
| Grok Mode B | T01–T07 PASS; T05 after restore from `fixtures_failing`; T08 PASS\* (pre-filled / documented) |
| Free web chat (no Mode B) | All **SKIP(env)** — host limit, not language fail |

### Pros worth keeping

- Triad + ephemeral FixPatch + round-trip + honest REFUTED / RUNTIME-CHECKED worked.
- Mode B works with an operator.
- Agents largely stayed claim-less.

### Cons / lessons

- **Fixture hygiene:** do not leave fixed copies in `task_*/initial/` after a trial. Restore from SoT (`fixtures_failing/nonexhaustive_light.vera` for T05; unfilled `?body` for T08).
- Mode B ≈ oracle-following, not hard reasoning.
- Free sandboxes often cannot run CLI.
- Cognitive load of contracts / prove / diag is real; surface is early-stage.

### No overclaim

Not IFC, not durable FixPatch, not hole synthesis, not “better than Python/Rust”, not production-ready.

---

## What Fable should do now

1. Read the public trial note; confirm agreement or note honest disagreements (claim-less).
2. Acknowledge fixture hygiene for any future bench guinea-pig runs.
3. **Do not** pick next hard work yourself. **Madis-gated only:** ERGO / F6 / GAP-D2 (and similar) — wait for an explicit pointer.
4. Soft docs / replies only unless Madis asks for code.
5. No git commit/push unless Madis asks. Stay in `vera-lang/`. Never touch TradingBot mainnet / `.env` / live state.

---

## End
