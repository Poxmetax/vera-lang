# VeraAgentBench v0.1 — claim-less CLI probe (scaffold)

**Status:** trial scaffold for first guinea-pig run (Fable as **agent under test**).  
**Decision SoT:** [`docs/pilot/VERA_AGENT_BENCH_V01_DECISION.md`](../../docs/pilot/VERA_AGENT_BENCH_V01_DECISION.md)  
**Rule:** Prefer claim-less. Not a leaderboard. Not IFC. Not durable FixPatch.

**Public trial note (2026-07-20):** claim-less scoreboard + pros/cons → [`results/TRIAL_ROUND_2026-07-20_PUBLIC.md`](results/TRIAL_ROUND_2026-07-20_PUBLIC.md).

## What this is

Hand-oracled tasks that exercise existing CLI surfaces:

- run / typecheck (via run or `--diag-json`)
- `--prove` (PROVED / RUNTIME-CHECKED / REFUTED)
- `--diag-json` + ephemeral FixPatch `add-match-arms`
- `--round-trip`

## What this is NOT

- Long-horizon / MCP / store edit-tx campaigns
- Hole synthesis product
- Cross-lang Python/Rust comparison
- Label/IFC authoring leaderboard
- Language feature implementation work

## Layout

```
bench/vera_agent_bench_v01/
  README.md                 — this file
  results/                  — agent writes trial results here
  fixtures_failing/         — intentional fails (never under examples/)
  task_T0N_*/prompt.md
  task_T0N_*/initial/       — starting .vera
  task_T0N_*/checks/        — PowerShell check scripts using cargo run -p vera
  task_T0N_*/meta.json
```

## First batch (trial)

| ID | Task | Bucket |
|----|------|--------|
| T01 | hello_console | A |
| T02 | prove_clamp_discharge | A |
| T03 | prove_refuted_false_ensures | A |
| T04 | prove_runtime_checked_str | A |
| T05 | nonexhaustive_match_fixpatch | B thin |
| T06 | round_trip_identity | A |
| T07 | refine_nth_len_ok | A |
| T08 | hole_fill_clamp (optional) | B thin |

**First-batch stop for Madis review:** complete T01–T06 (or through T07 if time), then STOP. T08 optional.

## Runner note

No automated multi-model runner in v0.1. Agent runs checks manually or via `checks/check.ps1` from `vera-lang/` root:

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
powershell -File bench\vera_agent_bench_v01\task_T01_hello_console\checks\check.ps1
```

Z3 must be on PATH for prove tasks.

## Agent tooling note — Glob patterns

Cursor Glob (this environment) finds files when `target_directory`/`path` is set and the pattern begins with a literal directory segment (tool docs auto-prepend `**/` when missing). **Some other agent Glob tools** (e.g. Claude Code) may still return **0 hits** for that combination.

Prefer:

1. Search root = `bench/vera_agent_bench_v01` with pattern `**/*.vera`, or
2. Monorepo-relative `vera-lang/bench/vera_agent_bench_v01/**/*.vera` from the TradingBot root.

After a glob, sanity-check hit count (expect **9** `.vera` fixtures in this bench). Broad `**/*` may also surface `*.bak_*` under `_operator_archive/` — those are operator archives, **not** scored `initial/main.vera`.

Crate/`docs` soft-patch `*.bak_*` archaeology is **local-only** under `vera-lang/_operator_archive/crate_src_bak/` (and `docs_bak/`); see [`../../_operator_archive/README.md`](../../_operator_archive/README.md). Bench `task_*` pre-restore baks in this folder remain tracked.
