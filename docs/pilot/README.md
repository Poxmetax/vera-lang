# VERA Phase -1 Thesis Pilot

A cheap, pre-build experiment that tests VERA's core thesis **before** any
toolchain is written: *does a familiar surface + a strict, machine-verified
substrate make an LLM ship fewer bugs, and can that style be authored fluently?*

It is **not** the VERA language. It is the closest runnable **proxy**, built from
tools already installed, and it lives fully isolated under `vera-lang/` — it does
not import or touch any TradingBot runtime file.

## What it does

For each of six CWE bug buckets it writes two implementations of one small task:

- **(a) baseline** — idiomatic, fully type-annotated Python with the realistic
  latent bug an LLM would emit.
- **(b) VERA-style** — the same task under a strict substrate: `mypy --strict`
  types, `Option`/`Result` discipline, `icontract` `@require`/`@ensure`
  contracts, `hypothesis` property tests, and a hand-rolled taint/secret label
  (`Tainted`/`Trusted`/`Secret`) emulating VERA's unified effect+capability+taint
  label and the CaMeL "untrusted data can't reach a privileged sink" idea.

The measured question: does (b) catch **before runtime** the bug that (a) ships
silently, and at what authoring cost? See `REPORT.md` for the verdict.

## Buckets

1. Input validation (CWE-20) · 2. SQL injection (CWE-89) · 3. Crypto misuse
(CWE-327) · 4. Hard-coded creds (CWE-259) · 5. Unhandled None / error paths ·
6. Out-of-bounds indexing / int bounds (CWE-787).

## Layout

```
vera_substrate.py          shared VERA-style proxy (Option/Result, Tainted/Trusted, Secret)
bucketN_baseline.py        idiomatic Python with the latent bug
bucketN_vera.py            VERA-style implementation
bucketN_vera_violation.py  (buckets 2-5) buggy usage that mypy --strict rejects
test_bucketN.py            behavioral + hypothesis property tests
logs/                      raw mypy + pytest output, exit codes, metrics
REPORT.md                  per-bucket table, PASS/FAIL/RESCOPE verdict, caveats
```

## Re-run it

From this directory (`vera-lang/docs/pilot/`), with `python` = CPython 3.13.13:

```powershell
# 1. VERA-style modules + baselines must type-check clean (expect exit 0):
python -m mypy --strict vera_substrate.py `
  bucket1_baseline.py bucket1_vera.py bucket2_baseline.py bucket2_vera.py `
  bucket3_baseline.py bucket3_vera.py bucket4_baseline.py bucket4_vera.py `
  bucket5_baseline.py bucket5_vera.py bucket6_baseline.py bucket6_vera.py

# 2. Each violation file must FAIL type-check (expect exit 1 with a type error):
python -m mypy --strict bucket2_vera_violation.py
python -m mypy --strict bucket3_vera_violation.py
python -m mypy --strict bucket4_vera_violation.py
python -m mypy --strict bucket5_vera_violation.py

# 3. Behavioral + property tests (expect: passed, with 2 xfailed baseline props):
python -m pytest -v
```

## Versions used (verified 2026-07-19)

CPython 3.13.13 · mypy 2.3.0 · hypothesis 6.156.7 · icontract 2.7.3 ·
pytest 9.1.1. No `pip install` was run; all deps were pre-installed.
