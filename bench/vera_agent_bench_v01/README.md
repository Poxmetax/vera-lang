# VeraAgentBench v0.1 — claim-less CLI probe (scaffold)

**Status:** trial scaffold for first guinea-pig run (Fable as **agent under test**).  
**Decision SoT:** [`docs/pilot/VERA_AGENT_BENCH_V01_DECISION.md`](../../docs/pilot/VERA_AGENT_BENCH_V01_DECISION.md)  
**Rule:** Prefer claim-less. Not a leaderboard. Not IFC. Not durable FixPatch.

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
