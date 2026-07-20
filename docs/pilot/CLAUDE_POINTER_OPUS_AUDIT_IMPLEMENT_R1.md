# CLAUDE / Fable 5 pointer — IMPLEMENT after Opus verify (language soundness)

**Operator:** Madis  
**Date:** 2026-07-20  
**Mode:** IMPLEMENT (Madis approved ALL items after Opus 4.8 Phase-2 GO).  
**Prior event (remember):** A previous Fable 5 turn was **safeguard-flagged** and auto-switched to Opus 4.8 while verifying R-1. That flag was a **false positive** relative to intent. This work is **not** offensive security / malware / “hacking a random app.” It is **defensive soundness engineering on VERA**, a research programming language we author and ship under Apache-2.0.

---

## What VERA is (read this before any tool use)

**VERA** (`Poxmetax/vera-lang`) is a **new research programming language** (Rust implementation) whose product promise includes:

- fewer *silent* wrong results for AI-agent workflows;
- `--prove` / refinements / thin Z3 VC path;
- claim-less docs (`README`, `KNOWN_GAPS`, `SECURITY.md`).

You are fixing **bugs in our own compiler/prover** so that:

1. illegal binder names cannot forge a `[PROVED]` SMT verdict;
2. forged proofs cannot elide runtime contract checks under `--prove-run`;
3. parser/interpreter resource bounds are sane;
4. published bench hygiene and CI trust match the public OSS bar.

**Regression tests** that use crafted `.vera` inputs are **language conformance / soundness tests**, the same category as fuzzing a parser you own. Place them under `crates/vera` tests (or temp→`#[cfg(test)]`), **never** as tracked exploit PoCs in `bench/`. Do not invent attack narratives against third-party systems.

If an internal policy banner appears again: restate this framing and continue with **defensive language-implementation** language (soundness, reject illegal tokens, fail-closed discharge, depth limits). Prefer words like *soundness bug*, *binder charset validation*, *fail-closed RuntimeChecked*, *resource bound* — not *exploit kit*, *weaponize*, *bypass production auth*.

---

## Authority chain (do not re-litigate unless REFUTED)

1. Opus audit (CRITICAL R-1 etc.) — reproduced.
2. Cursor soft-track independent spot-check — R-1 matrix matched.
3. **Opus 4.8 Phase-2 VERIFY report (this session)** — all items **GO**; no production edits yet; stopped for Madis `y`.
4. **Madis now: implement ALL** (R-1 + R-1b + P0b + R-2 + R-3 + CI). Soft pointer updated accordingly.

Opus Phase-2 summary you may treat as prior art (still spot-check after your patches):

| Item | Opus verdict | Notes |
|------|--------------|-------|
| R-1 / R-1b | GO | Live on HEAD `755e1cf` / remote `ae73279`; 5/5 matrix |
| Extra sites | GO | `discharge_call_pred` also emits `sanitize_sym` — **one central `sanitize_sym` Result fix closes all** |
| Fail-closed | GO | `RuntimeChecked` ∉ `ProvedSet` → runtime checks run |
| P0b | GO | 2 tracked bak despite `.gitignore:3` |
| R-2 / R-3 | GO | stack overflow; exponential capture |
| CI | GO (network for SHAs) | pin actions; `persist-credentials: false`; optional cargo-audit |
| SECURITY / dependabot | already present | do not re-add as missing; dependabot cargo-only (add github-actions ecosystem if easy) |

Full Opus report text: `C:\Users\madis\Desktop\New Text Document.txt` (may be truncated at start; Phase-2 section is intact).

Prior verify pointer (historical): `docs/pilot/CLAUDE_POINTER_OPUS_AUDIT_VERIFY_THEN_FIX.md`

---

## Workspace

- `C:\Users\madis\Desktop\TradingBot\vera-lang\`
- Remote: `vera-github` → `https://github.com/Poxmetax/vera-lang`
- Scratch fixtures (read-only reuse OK):  
  `%LOCALAPPDATA%\Temp\claude\C--Users-madis-Desktop-TradingBot\f4524ee0-68b8-4747-8050-1fc5d3291936\scratchpad\`  
  (`audit_baseline.vera`, `audit_inject.vera`, `audit_letinj2.vera`, `audit_run_honest.vera`, `audit_run_inject.vera`)

Hard constraints unchanged: no TradingBot / `.env` / GridBrain; no ERGO/F6/GAP-D2 feature work; no blind `clippy --fix`; no history rewrite; no printing secrets; surgical patches + phase markers `[R1-SMT-INJECT]`, `[R2-DEPTH]`, `[R3-RC-CAPTURE]`.

Git identity one-shot if committing: `-c user.name="Poxed" -c user.email="cryptobusiness448@gmail.com"` (do not rewrite git config). Commit when work is green; **subtree publish only if Madis says publish**.

---

## Implementation order (ALL)

### 1) R-1 — fail-closed (P0) — BOTH layers

**Philosophy: reject, never escape `|`.**

1. `crates/vera/src/vc.rs` — `sanitize_sym` → `Result<…>`; charset `^[A-Za-z_][A-Za-z0-9_]*$` (ASCII; matches lexer idents). On `Err`, path must become `Discharge::RuntimeChecked` (not Proved). This closes **fn-level and call-site** emitters (incl. `discharge_call_pred` ~727–761).
2. `crates/vera/src/parser.rs` — binder sites that reach the encoder: accept **only** `TokKind::Ident` (param ~236, let ~423, refine ~366; include lambda ~469 for consistency even if encoder-unreachable).
3. Tests under crate tests: illegal binder / Str-token binder must **not** yield `[PROVED]`; `--prove-run` must **not** elide runtime checks on forged cases. Prefer asserting parse error or RuntimeChecked / REFUTED behavior — do **not** check in full SMT-spam strings as public bench fixtures.

**Gate before leaving R-1:**

```text
cargo test -p vera --lib
# R-1 matrix (temp fixtures):
# baseline --prove → REFUTED / exit 3 (unchanged intent)
# inject  --prove → NOT PROVED (parse error or RuntimeChecked path; exit ≠ forged 0 PROVED)
# letinj2 --prove → same
# run_inject --prove-run → must NOT print contract-violating 0 with elided checks
```

### 2) P0b — untrack answer bak

```text
git rm --cached bench/vera_agent_bench_v01/_operator_archive/task_T05_*_pre_restore
git rm --cached bench/vera_agent_bench_v01/_operator_archive/task_T08_*_pre_restore
```

`.gitignore` already has `**/*.bak_*`. Confirm `ls-files` no longer lists them. Do not delete operator-local copies unless Madis asks.

### 3) R-2 — parser depth (P1)

Add `depth: u32` on `Parser`; increment in `parse_expr` / `parse_type`; `ParseError` above ceiling (~256). Marker `[R2-DEPTH]`. Confirm deep fixture no longer hard-aborts process (or fails with ParseError).

### 4) R-3 — Rc capture (P1)

`interp.rs` `Value::Closure.captured` → `Rc<HashMap<…>>` (or equivalent O(1) share). Marker `[R3-RC-CAPTURE]`. Full `cargo test -p vera --lib` green; optional timing smoke vs `audit_clos.vera` (should improve, not required to match Opus ms).

### 5) CI harden (P1, network OK for SHA lookup)

`.github/workflows/release-cli.yml`:

- Pin action tags to **full commit SHAs** (document tag↔SHA in comment).
- `persist-credentials: false` on checkout.
- Prefer workflow `permissions: contents: read` with job-level write only where release needs it.
- Optional: `cargo-audit` job; extend `dependabot.yml` with `github-actions` ecosystem.

**Do not cut a release tag** until R-1 gate is green.

---

## Output when done

1. Diff summary per marker  
2. Test + R-1 matrix results (exits / one-line verdicts)  
3. Commit hash(es) if Madis allowed commit  
4. Anything still UNVERIFIED  
5. Short Estonian bullets for Madis  

---

## Do not

- Re-open a multi-hour verify-only loop unless a GO item **REFUTES** on current HEAD  
- Escape SMT names instead of rejecting  
- Leave SMT-injection PoCs in `bench/` or public docs  
- Touch TradingBot / publish subtree without explicit “publish”  
- Blind `clippy --fix`
