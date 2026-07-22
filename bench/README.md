# VERA bench -- claim-less evidence and trial pack

Public, hand-oracled evidence that the VERA approach catches real bug
classes before runtime, plus a small CLI probe pack for outside agents.
Research prototype. Not a leaderboard. Claim-less by design.

## Contents

- **Thesis pilot evidence** -- [`REPORT.md`](REPORT.md): six bug-class
  buckets (input validation, SQL injection, crypto misuse, hard-coded
  credentials, unhandled error paths, out-of-bounds), each shown (a)
  shipping silently in idiomatic Python and (b) caught before runtime by
  a verified-substrate style. Every catch is backed by real tool output
  with recorded exit codes.
- **SMT / refinement evidence** -- [`SMT_SPIKE_REPORT.md`](SMT_SPIKE_REPORT.md):
  Z3 discharge of value-range and bounds properties for buckets 1 and 6,
  with zero execution of the program under test.
- **Bucket sources** -- `bucket*_baseline.py` (idiomatic Python that
  ships the bug) vs `bucket*_vera.py` (verified-substrate proxy that
  catches it), with `*_violation.py` files showing the pre-runtime catch.
  Shared substrate in `vera_substrate.py`. Property/contract tests in
  `test_bucket*.py`.
- **Agent trial pack** -- [`vera_agent_bench_v01/`](vera_agent_bench_v01/):
  hand-oracled tasks (T01-T09) that exercise the VERA CLI surfaces --
  `--prove` (PROVED / REFUTED / RUNTIME-CHECKED), `--diag-json`,
  `--round-trip`. See [`../AGENTS.md`](../AGENTS.md) for the agent-facing
  entry point and honesty rules.

## Running the bucket evidence

The buckets are a self-contained Python demonstration (proxy for VERA's
static type/label layer, before the toolchain existed). They import each
other relatively, so run them from this directory:

```text
cd bench
mypy --strict .              # buckets 2,3,4,5 fail here (static catch)
pytest test_bucket1.py test_bucket6.py   # buckets 1,6 fail on baseline
python smt_refine_spike.py   # Z3 discharge for buckets 1 and 6 (exit 0)
```

Requires `mypy`, `hypothesis`, `icontract`, and `z3-solver`.

## Running the agent trial pack

The trial pack targets the real VERA CLI. See
[`vera_agent_bench_v01/README.md`](vera_agent_bench_v01/README.md) for
per-task detail and [`../AGENTS.md`](../AGENTS.md) for how to run and the
honesty rules for agents under test.

```text
cargo run -p vera -- --prove <file.vera>
```

Z3 must be on `PATH` for `--prove`.

## What this is NOT

Not a leaderboard. Not a claim of full information-flow control. Not a
durable-store / cache-skip speed story. Not a synthesis product. The
evidence here is exactly what it says: bug classes caught before runtime,
with recorded tool output.
