# Fable handoff — VeraAgentBench v0.1 trial (guinea pig)

**Date:** 2026-07-20  
**Operator:** Madis  
**Role:** Fable is the **agent under test**, not the implementer of new VERA language features.  
**Pointer (paste short):** [`CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL.md`](CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL.md)  
**Decision:** [`VERA_AGENT_BENCH_V01_DECISION.md`](VERA_AGENT_BENCH_V01_DECISION.md) — GO with cuts

---

## 0. Context (already done — do not redo as hard work)

| Item | Status |
|------|--------|
| `[GAPC2-SMT-LEN]` | committed `f8b67cc`; soft review **PASS** |
| `[GAP4-VALUE-LABEL]` | committed `28929dc`; soft review **PASS** |
| Publish | `f4f3cc7` on GitHub main |
| Suite | **68**; soft_smoke PASS; prove_clamp **6** |

Next hard language work (GAP4-R2-ERGO / F6 / GAP-D2) is **Madis-gated** and **out of scope** for this trial.

---

## 1. Mission

Run the first VeraAgentBench batch using only the existing CLI. Measure whether an agent can complete thin A / thin-B tasks honestly.

Scaffold root:

```
bench/vera_agent_bench_v01/
```

Each task: `prompt.md` + `initial/` + `checks/` + `meta.json`.

---

## 2. First batch (do these)

| Order | Task folder | Goal |
|-------|-------------|------|
| 1 | `task_T01_hello_console` | run → stdout has `Hello, VERA` |
| 2 | `task_T02_prove_clamp_discharge` | `--prove` → `summary: 6 proved` |
| 3 | `task_T03_prove_refuted_false_ensures` | `--prove` → `[REFUTED]`, exit 3 |
| 4 | `task_T04_prove_runtime_checked_str` | `--prove` → ≥1 `[RUNTIME-CHECKED]` |
| 5 | `task_T05_nonexhaustive_match_fixpatch` | `--diag-json` has `add-match-arms`; edit arms; run OK |
| 6 | `task_T06_round_trip_identity` | `--round-trip` → `round-trip OK` |

**If time before stop:** T07 (`refine_nth_len_ok`).  
**Optional:** T08 (`hole_fill_clamp`) — fill `?body` by edit; **no synthesis claim**.

**STOP** after T01–T06 (or T07) for Madis review. Do not expand the bench.

---

## 3. Allowed tools

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo run -p vera -- <file>
cargo run -p vera -- --prove <file>
cargo run -p vera -- --diag-json <file>
cargo run -p vera -- --round-trip <file>
# or task checks:
powershell -File bench\vera_agent_bench_v01\task_T01_hello_console\checks\check.ps1
```

Z3 must be on PATH for prove tasks.

---

## 4. Forbidden

1. Edit `vc.rs` / `smt.rs` / `typecheck.rs` / `interp.rs` / `diag.rs` / `main.rs` / `store.rs` / `render.rs` / `label.rs` / `parser.rs` / `ast.rs` / `lexer.rs` to “make the bench pass.”
2. Claim full IFC, label inference, or CONF-P2 ergonomics closed.
3. Open GAP-D2 or invent durable FixPatch.
4. Put failing fixtures under `examples/` (round_trip invariant).
5. Cross-lang leaderboard claims.
6. Auto-expand to 12–20 / bucket C tasks.

---

## 5. Results logging

Copy `bench/vera_agent_bench_v01/results/RESULTS_TEMPLATE.md` → a dated results file (e.g. `RESULTS_FABLE_TRIAL_2026-07-20.md`). Fill PASS/FAIL honestly. Include overclaim scan checkboxes.

---

## 6. Return format (to Madis)

```text
## VERA_AGENT_BENCH_V01_TRIAL
agent: Fable
batch: T01-T06 (T07/T08 if attempted)
results: <path>
table: T01=... T02=... ...
overclaims: none | <list>
stop: YES — awaiting Madis review
```

---

## 7. English paste for Madis → Fable (chat)

```text
You are Fable — agent under test for VeraAgentBench v0.1 (not a language implementer).

Read and follow:
  docs/pilot/CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL.md
Full brief:
  docs/pilot/FABLE5_VERA_AGENT_BENCH_V01_TRIAL_HANDOFF.md

Workspace: C:\Users\madis\Desktop\TradingBot\vera-lang\
Scaffold: bench/vera_agent_bench_v01/

Attempt T01→T06 in order (T07 if time; T08 optional). Use only vera CLI
(run / --prove / --diag-json / --round-trip). Log honest results under
bench/vera_agent_bench_v01/results/. Do NOT expand the language, claim IFC,
or open GAP-D2. STOP after the first batch for Madis review.
```
