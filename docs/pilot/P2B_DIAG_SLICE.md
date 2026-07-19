# P2B-DIAG slice — structured pipeline diagnostics (handoff task B)

**Date:** 2026-07-19 · **Marker:** `[P2B-DIAG]` · **Files:** `crates/vera/src/diag.rs` (new), `vc.rs` (`Obligation.span`), `main.rs` (`--diag-json`), `lib.rs` (exports)

## What landed

Machine-readable diagnostics for the whole **parse → typecheck → prove** pipeline (SPEC DP8), behind an opt-in CLI flag. Default paths are untouched: the text `--prove` report is byte-identical, and the interpreter run path is unreachable from diag mode (diagnostics never execute the program).

- **Documented entrypoints** (`lib.rs` re-exports): `diagnose_source(file, src, with_prove)` and `diagnose_program(file, &Program, with_prove)` → `DiagReport`.
- **CLI:** `vera <file> --diag-json` (parse + typecheck) · `vera <file> --prove --diag-json` (+ Phase-2 obligations). Pretty-printed JSON on stdout.
- **Spans:** prove obligations now carry a source anchor (`Obligation.span`) — fn declaration span for `ensures` / `return_refine`, call-expression span for `call_requires` / `call_arg_refine`. Typecheck/parse errors expose their `line:col` as a structured field instead of a string prefix.

## JSON shape

Top level (`DiagReport`):

| Field | Meaning |
|---|---|
| `tool`, `version` | `"vera"`, crate version |
| `file` | path as given to the CLI / entrypoint |
| `ok` | `true` iff no errors **and** nothing refuted |
| `summary` | `{errors, proved, runtime_checked, refuted}` counts |
| `diagnostics` | array of `Diagnostic` |

Per diagnostic:

| Field | Meaning |
|---|---|
| `source` | pipeline stage: `parse` \| `typecheck` \| `prove` |
| `severity` | `error` (parse/type errors, refuted) \| `info` (proof tiers) |
| `code` | stable machine code: `PARSE-ERROR`, `TYPE-ERROR`, `PROVE-PROVED`, `PROVE-RUNTIME-CHECKED`, `PROVE-REFUTED`, `PROVE-ERROR` |
| `message` | human-readable one-liner |
| `target`, `kind` | obligation identity (prove only): fn/call target + `ensures` / `return_refine` / `call_requires` / `call_arg_refine` |
| `status` | prove tier: `proved` \| `runtime-checked` \| `refuted` |
| `reason` | runtime-checked reason / refuted detail |
| `span` | `{line, col}` when known |

Optional fields are omitted (not `null`) when absent.

## Exit contract (unchanged semantics)

`DiagReport::exit_code()` mirrors the CLI: **1** = any parse/typecheck/prove-infrastructure error · **3** = any refuted obligation · **0** = ok. Same codes as the text mode ([SOFT-EXIT-HELP]).

## Example (real output)

```json
{
  "tool": "vera", "version": "0.1.0", "file": "examples/prove_refuted.vera",
  "ok": false,
  "summary": { "errors": 0, "proved": 0, "runtime_checked": 0, "refuted": 1 },
  "diagnostics": [ {
      "source": "prove", "severity": "error", "code": "PROVE-REFUTED",
      "message": "bad ensures[0] (ensures): refuted — sat (counterexample exists)",
      "target": "bad ensures[0]", "kind": "ensures", "status": "refuted",
      "reason": "sat (counterexample exists)", "span": { "line": 5, "col": 1 }
  } ]
}
```

## Honest limits

| Item | Status |
|---|---|
| Typecheck errors are fail-fast (one per report, matching `check_program`) | collecting multiple type errors = later slice |
| Span granularity | fn-decl / call-expr / error-token level; no end-spans or ranges |
| `FixPatch` machine-applicable edits | **not this slice** — handoff task E |
| Proof-gated check elision | **not this slice** — handoff task D |
| Text `--prove` output | untouched (byte-identical), still the human default |

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib -- diag::
# expect: 5 passed (22 total suite)
cargo run -p vera -- --prove --diag-json examples/prove_refuted.vera   # refuted JSON, exit 3
cargo run -p vera -- --prove --diag-json examples/prove_clamp.vera     # summary.proved = 6, exit 0
cargo run -p vera -- --diag-json examples/hello.vera                   # ok:true, empty, exit 0
cargo run -p vera -- --prove examples/prove_clamp.vera                 # text mode unchanged: 6 proved
powershell -File docs/pilot/soft_smoke.ps1                             # SOFT-SMOKE PASS
```
