# VeraAgentBench v0.1 -- claim-less CLI probe

**Status:** claim-less probe pack for outside agents (agent under test).
**Rule:** Prefer claim-less. Not a leaderboard. Not IFC. Not durable FixPatch.

**Public trial note (2026-07-20):** claim-less scoreboard + pros/cons -> [`results/TRIAL_ROUND_2026-07-20_PUBLIC.md`](results/TRIAL_ROUND_2026-07-20_PUBLIC.md).

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
  README.md                 -- this file
  results/                  -- agent writes trial results here
  fixtures_failing/         -- intentional fails (never under examples/)
  task_T0N_*/prompt.md
  task_T0N_*/initial/       -- starting .vera
  task_T0N_*/checks/        -- check scripts using cargo run -p vera (.ps1 + POSIX .sh for the scored core)
  task_T0N_*/meta.json
```

## Scored core (7 tasks, claim-less)

| Order | ID   | Task                         | Honesty surface                                |
| ----- | ---- | ---------------------------- | ---------------------------------------------- |
| 1     | T02  | prove_clamp_discharge        | PROVED + refine/SMT discharge                  |
| 2     | T03  | prove_refuted_false_ensures  | REFUTED honesty (exit 3)                       |
| 3     | T04  | prove_runtime_checked_str    | RUNTIME-CHECKED honesty (not fake PROVED)      |
| 4     | T05  | nonexhaustive_match_fixpatch | ephemeral `add-match-arms` FixPatch            |
| 5     | T06' | round_trip_paren_identity    | paren round-trip honesty                       |
| 6     | T08  | hole_fill_clamp              | typed hole = fill by editing source            |
| 7     | T09  | fail_closed_binder           | front-door binder reject; no forged `[PROVED]` |

**Preflight (unscored):** T01 hello_console.
**Appendix / archive:** T07 refine_nth_len_ok -- demoted from the scored core
(run-only refine, overlaps T02); fixture + checks stay for archaeology.
T10 label_surface_reject -- optional appendix only, **not implemented**;
never "IFC solved".

**Historical note:** the 2026-07-20 trial ran the pre-scored-core batch
(T01-T08, T06 = store round-trip on hello); results under `results/` keep
that shape. The scored core above supersedes it for any future round.

## Runner note

No automated multi-model runner in v0.1. Agent runs checks manually from
`vera-lang/` root -- PowerShell or POSIX shell:

```powershell
cd <vera-lang checkout>
powershell -File bench\vera_agent_bench_v01\task_T02_prove_clamp_discharge\checks\check.ps1
```

```bash
cd <vera-lang checkout>
bash bench/vera_agent_bench_v01/task_T02_prove_clamp_discharge/checks/check.sh
```

Z3 must be on PATH for prove tasks. Scored-core tasks carry both `.ps1`
and `.sh`; T01/T07 keep `.ps1` only (unscored / archived).

## Agent tooling note -- Glob patterns

Some agent Glob tools find files only when the search root is set and the
pattern begins with a literal directory segment; others auto-prepend `**/`.
Prefer:

1. Search root = `bench/vera_agent_bench_v01` with pattern `**/*.vera`, or
2. Repo-relative `bench/vera_agent_bench_v01/**/*.vera` from the repo root.

After a glob, sanity-check the hit count (expect **10** `.vera` fixtures in
this bench -- 9 task initials incl. T09 + 1 under `fixtures_failing/`).
