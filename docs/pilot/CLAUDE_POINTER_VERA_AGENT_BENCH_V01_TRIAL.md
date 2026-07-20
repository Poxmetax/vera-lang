# Claude pointer -- VeraAgentBench v0.1 trial (Fable guinea pig)

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are **Fable — the agent under test**, not a language-feature implementer.
Madis approved GO-with-cuts for VeraAgentBench v0.1. Soft scaffold lives at:

`bench/vera_agent_bench_v01/`

**Full handoff:** [`FABLE5_VERA_AGENT_BENCH_V01_TRIAL_HANDOFF.md`](FABLE5_VERA_AGENT_BENCH_V01_TRIAL_HANDOFF.md)  
**Decision SoT:** [`VERA_AGENT_BENCH_V01_DECISION.md`](VERA_AGENT_BENCH_V01_DECISION.md)

## Hard rules

1. Attempt tasks **in order** (T01 → T06 first batch; T07 if time; T08 optional).
2. Use **only** vera CLI: run / `--prove` / `--diag-json` / `--round-trip` (via `cargo run -p vera -- …`).
3. Log results honestly to `bench/vera_agent_bench_v01/results/` (copy `RESULTS_TEMPLATE.md`).
4. **NOT** expand the language (no `.rs` edits for features).
5. **NOT** claim IFC / full labels / R2 ergonomics closed.
6. **NOT** open GAP-D2 / durable FixPatch.
7. **STOP** after the first batch for Madis review.

Baseline context (do not re-prove as language work): suite **68**, soft_smoke PASS, prove_clamp **6**. Soft reviews: VL PASS, C2 PASS.

Stay in `vera-lang/`. Never touch TradingBot mainnet / `.env` / live state.
