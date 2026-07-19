# SMT / refinement spike — REQ-REFINE discharge

**Date:** 2026-07-19 · **Tool:** `z3-solver` 5.0.0 (Python bindings; no `z3` CLI required)  
**Script:** [`smt_refine_spike.py`](smt_refine_spike.py) · **Exit:** 0

## Question

Phase -1 caveat 1 / RESEARCH_REPORT **R1** / SPEC §4.4 **REQ-REFINE**: can value-range and bounds properties (pilot buckets 1 and 6) be caught **statically, with zero execution of the program under test**, via SMT?

## Method

Pure Z3 queries over the same contracts the pilot used as `icontract` preconditions/postconditions. No VERA program is interpreted.

| Check | Meaning | Expected | Result |
|-------|---------|----------|--------|
| `b1_proved` | `price>=0 ∧ 0≤pct≤100 ⇒ result≥0` for `result = price*(1-pct/100)` | `unsat` (proved) | **PASS** |
| `b1_counterexample` | Drop the pct bound → find silent CWE-20 input | `sat` | **PASS** (`price=1, pct=101`) |
| `b6_proved` | `0≤offset<length ⇒ in-bounds` | `unsat` | **PASS** |
| `b6_counterexample` | Drop the bound → find CWE-787-class input | `sat` | **PASS** (`length=1, offset=-1`) |

## Verdict

**PASS — REQ-REFINE discharged for buckets 1 and 6.**

Together with the original Phase -1 pilot (6/6 caught pre-ship; 4/6 already static via mypy), **all six pilot buckets now have a demonstrated static or SMT-static path**:

| Bucket | Static path |
|--------|-------------|
| 1 Input validation | **SMT** (this spike) + contracts |
| 2 Injection | mypy taint labels |
| 3 Crypto misuse | mypy `Literal` allowlist |
| 4 Hard-coded creds | mypy `Secret[str]` sink |
| 5 Unhandled None | mypy `Result` discipline |
| 6 Out-of-bounds | **SMT** (this spike) + contracts |

## Honest limits (still open)

1. This spike proves the *mathematical* contracts of the two bucket bodies — it is not yet a VERA compiler VC generator that reads `.vera` source and emits these queries automatically. Wiring that into Phase 2's type/refinement checker is the remaining engineering work.
2. Predicates here stay in a decidable fragment (linear real arithmetic / integer bounds). Harder refinements may still timeout (R4).
3. Implicit-flow IFC remains **[UNVERIFIED/OPEN]** per SPEC §4.2.

## Re-run

```powershell
cd vera-lang\docs\pilot
python smt_refine_spike.py
# expect exit 0 and VERDICT PASS
```
