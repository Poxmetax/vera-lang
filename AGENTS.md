# AGENTS.md -- VERA (published trial pack)

> **Status: published trial pack (claim-less)**
> VeraAgentBench scored-core fixtures live under [`bench/vera_agent_bench_v01/`](bench/vera_agent_bench_v01/).
> This is a **research-prototype probe surface** for outside agents -- not a leaderboard product.
> Headline stays honesty surfaces only. **Not** IFC-done. **Not** an MCP compiler service. **Not** a cache-skip / durable-store speed story.

**Bench overview:** [`bench/README.md`](bench/README.md)
**Bench tasks:** [`bench/vera_agent_bench_v01/`](bench/vera_agent_bench_v01/)

---

## What this is (claim-less)

VERA is a **research prototype** language/toolchain with a CLI agents can hit:

- run / typecheck (via run)
- `--prove` -> honest **PROVED / REFUTED / RUNTIME-CHECKED** (not "all green")
- `--diag-json` -> diagnostics; FixPatch kind `add-match-arms` is **ephemeral** when present
- `--round-trip` -> store/render round-trip check

## What this is NOT

- Full IFC / "labels solved"
- Durable FixPatch / proof-cache **product** or speed leaderboard
- Hole **synthesis** product (holes = fill by **editing source**)
- MCP compiler service (docs stub only unless separately gated)
- Cross-language agent leaderboard

## Trial pack (published -- claim-less)

Scored core (see [`bench/README.md`](bench/README.md)): **T02, T03, T04, T05, T06' (parens), T08, T09 (fail-closed binder)**.
T01 = smoke. T07 demoted. T10 labels = optional appendix only (not scored; never "IFC solved").

## How to run

```text
cargo run -p vera -- <file.vera>
cargo run -p vera -- --prove <file.vera>
cargo run -p vera -- --diag-json <file.vera>
cargo run -p vera -- --round-trip <file.vera>
```

Z3 must be on `PATH` for `--prove`. Prefer task `checks/` scripts when present. Scored-core tasks ship both `checks/check*.ps1` and POSIX `checks/check*.sh`.

## Honesty rules for agents under test

1. Do not edit toolchain sources to make a task pass.
2. Report the triad as printed -- REFUTED / RUNTIME-CHECKED are valid successes when the oracle asks for them.
3. Do not claim durable repair, synthesis, IFC, or cache-skip speed from a green task.

---

*Published trial pack entrypoint. Claim-less. Not IFC / MCP / skip-as-headline.*
