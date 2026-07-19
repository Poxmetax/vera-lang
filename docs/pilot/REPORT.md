# VERA Phase -1 Thesis Pilot — Report

**Date:** 2026-07-19 · **Author:** AI agent (single session) · **Location:** `vera-lang/docs/pilot/` (isolated greenfield; imports nothing from the TradingBot runtime)

**Question under test (the one load-bearing, still-UNVERIFIED assumption):**
1. Does a *verified-substrate* style catch, **before runtime**, the bug classes that idiomatic Python ships silently?
2. Can that style be **authored fluently** at acceptable cost (not so much ceremony that an LLM would avoid it)?

**Proxy for VERA (no toolchain built yet):** Python 3.13 + `mypy --strict` (stands in for VERA's static type/label layer) + `icontract` `@require`/`@ensure` (stands in for `requires`/`ensures`) + `hypothesis` property tests (contracts-as-oracles, plan U9) + a ~90-line hand-rolled substrate (`vera_substrate.py`: `Option`/`Result`, `Tainted`/`Trusted`, `Secret`) emulating the unified effect+capability+taint label and the CaMeL "untrusted data can't reach a privileged sink" idea.

---

## Per-bucket results

| # | Bucket | CWE | (a) ships bug? | (b) caught before runtime? | Mechanism (real tool) | Authoring cost (Δ logical SLOC, b−a) |
|---|--------|-----|----------------|----------------------------|-----------------------|--------------------------------------|
| 1 | Input validation | CWE-20 | **Yes** — `apply_discount(100,150)` → `-50.0`, silent | **Yes** | `icontract` `@require`/`@ensure` boundary + `hypothesis` property fails on baseline | **+4** |
| 2 | Injection (SQL) | CWE-89 | **Yes** — `' OR '1'='1` returns all rows | **Yes** | **`mypy --strict` type error**: `Tainted[str]` not assignable to `Trusted[str]` sink | **−1** (+ shared taint types) |
| 3 | Crypto misuse | CWE-327 | **Yes** — bare unsalted MD5 | **Yes** | **`mypy --strict` type error**: `Literal['md5']` not in `ApprovedHash` | **+7** |
| 4 | Hard-coded creds | CWE-259 | **Yes** — secret in source + leaks to logs/JSON | **Partial→Yes** | **`mypy --strict` type error**: `str` literal not assignable to `Secret[str]` sink; **+** runtime `Secret` redaction / non-serialization | **+5** |
| 5 | Unhandled None / error path | null class | **Yes** — happy path runs, missing key → runtime `TypeError` | **Yes** | **`mypy --strict` type error**: `Result[int,str]` has no `*` operator (must handle `Err`) | **+1** |
| 6 | Out-of-bounds / int bounds | CWE-787 | **Yes** — negative index silently wrong; large index runtime crash | **Yes** | `icontract` `@require(0<=i<len)` boundary + `hypothesis` property fails on baseline | **+1** |

**Counts:** (a) shipped the bug silently in **6 / 6** buckets. (b) caught it before shipping in **6 / 6** buckets.
Of the 6 catches: **4 are truly static** (`mypy --strict` type errors — buckets 2, 3, 4, 5, caught with zero execution) and **2 are contract + property** (buckets 1, 6 — caught at the contract boundary and by a `hypothesis` property, i.e. before ship but not at compile time).

---

## Verdict: **PASS** → build proceeds

- **Catch axis:** 6/6 = **100%** caught, vs the ~70% PASS bar and ~40% FAIL floor. Comfortably PASS.
- **Authoring axis:** 6/6 buckets authored in valid substrate that passed `mypy --strict` on the **first** tool run (13 modules, exit 0), with low ceremony — **median +2.5, mean +2.8 logical lines** of core code per bucket over the baseline, plus a **one-time** 64-logical-line shared substrate that a real VERA would ship as stdlib. That is well within "fluently authorable" (≥70% bar). PASS.
- No bucket landed in the FAIL band and none required softening the surface.

### Why this is a real result, not a rigged one
- The baselines are **fully type-annotated and pass `mypy --strict` themselves** (13-file clean run, exit 0). So the catches in (b) are *not* "Python had no types" — they come specifically from the substrate discipline (labels, `Result`, contracts, refinement-style bounds), which is exactly VERA's claim.
- Every catch is backed by real tool output with recorded exit codes (below), not assertion.

---

## Honest caveats (what the coordinator must weigh before Phase 0/1)

1. **Single most important caveat — "before runtime" means two different things here, and only 4/6 are truly static.** Buckets 2–5 are caught by `mypy --strict` with **zero execution** (the strong form VERA promises). Buckets 1 and 6 (and the *provenance* half of bucket 4) are caught by **runtime contracts + test-time properties** — before the bug reaches production, but *not* at compile time. Real VERA claims SMT-proved **refinement types** (`{x:Int | 0<=x<len}`, plan U8) that would move buckets 1 and 6 into the static column; the proxy can't show that because **Z3/CVC5/Dafny are not installed**. So the pilot validates the *thesis direction* strongly, but the "static proof of value-range/bounds" mechanism is assumed, not demonstrated.

2. **Bucket 4 (hard-coded creds) is the weakest-fidelity proxy.** The `mypy` catch rejects passing a bare plaintext literal to a `Secret[str]` sink (the naive form), and the runtime `Secret` genuinely refuses to log/serialize. But the type system does **not** prove the secret *came from a capability/env* rather than `Secret("admin123")` — a determined author can still hardcode by wrapping. Real VERA would tie `Secret` construction to a capability handle; the proxy cannot fully enforce provenance. Score it "caught" for the leak/serialization vector, "partial" for the hardcode-provenance vector.

3. **`Result` has an `unwrap()` escape hatch** (like Rust). Bucket 5's static catch holds only if the author doesn't reach for `.unwrap()`; `unwrap()` type-checks and defers to runtime. VERA's story is the same, so this is representative, not a proxy artifact.

4. **Authoring fluency is n=6, one author, one session, no time pressure.** Low line-count delta is encouraging evidence the style is authorable, but it does not discharge the plan's real worry (the FP study: LLMs revert to imperative under unfamiliar ceremony) at scale or for weaker models. Treat "fluently authorable" as *supported*, not *proven*.

5. **The proxy emulates VERA crudely.** Hand-rolled `Tainted/Trusted/Secret` wrappers stand in for a unified label lattice; there are no effect rows, capture sets, content-addressed substrate, or SMT. The pilot tests the *claim* (strict substrate catches what dynamic Python ships) — not VERA's specific mechanisms.

---

## Commands run + real exit codes (reproducible evidence)

Run from `vera-lang/docs/pilot/` with `python` = CPython 3.13.13. Raw logs in `logs/`.

| Command | Exit | Result | Log |
|---------|------|--------|-----|
| `python -m mypy --strict vera_substrate.py bucket{1..6}_baseline.py bucket{1..6}_vera.py` (13 files) | **0** | `Success: no issues found in 13 source files` — substrate + baselines + vera modules all type-clean | `logs/mypy_clean.txt` |
| `python -m mypy --strict bucket2_vera_violation.py` | **1** | `error: Argument 2 to "query_by_name" has incompatible type "Tainted[str]"; expected "Trusted[str]"` | `logs/mypy_violation_b2.txt` |
| `python -m mypy --strict bucket3_vera_violation.py` | **1** | `error: Argument 2 to "hash_password" has incompatible type "Literal['md5']"; expected "Literal['sha256', 'sha512']"` | `logs/mypy_violation_b3.txt` |
| `python -m mypy --strict bucket4_vera_violation.py` | **1** | `error: Argument 1 to "connect" has incompatible type "str"; expected "Secret[str]"` | `logs/mypy_violation_b4.txt` |
| `python -m mypy --strict bucket5_vera_violation.py` | **1** | `error: Unsupported operand types for * ("Ok[int]"/"Err[str]" and "int")` | `logs/mypy_violation_b5.txt` |
| `python -m pytest -v` | **0** | `21 passed, 2 xfailed in 5.25s` (the 2 `xfail`s are the baseline property tests that hypothesis *proves* fail — bugs caught) | `logs/pytest.txt` |

Tool versions (recorded): mypy 2.3.0, hypothesis 6.156.7, icontract 2.7.3, pytest 9.1.1. No `pip install` was run.

### How the "before runtime" catch is evidenced per bucket
- **Buckets 2,3,4,5:** the `bucketN_vera_violation.py` file is the buggy code an LLM would emit; `mypy --strict` rejects it (exit 1) with **no execution**.
- **Buckets 1,6:** the `hypothesis` property test on the baseline is marked `xfail(strict=True)` — pytest confirms hypothesis actually *finds* a counterexample (negative price / negative index), i.e. the property would have blocked the bug at test time; the same property passes on the VERA-style version.

---

## Files

- `vera_substrate.py` — shared VERA-style proxy: `Option`/`Result`, `Tainted`/`Trusted`, `Secret`.
- `bucketN_baseline.py` — idiomatic Python with the latent bug (all pass `mypy --strict`).
- `bucketN_vera.py` — VERA-style implementation.
- `bucketN_vera_violation.py` (buckets 2–5) — the buggy usage that `mypy --strict` rejects (static catch evidence).
- `test_bucketN.py` — behavioral + `hypothesis` property tests.
- `logs/` — raw tool output + exit codes; `metrics.txt` — line counts / logical SLOC.
