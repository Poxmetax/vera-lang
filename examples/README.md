# VERA examples

One-line index of examples/*.vera. Prove demos need Z3 on PATH (see repo README).

| File | Purpose |
|------|---------|
| hello.vera | Minimal Console hello-world |
| clamp.vera | Contracts on clamp (runtime-checked; Phase 1 style) |
| refine_rt.vera | Refinement return type checked at runtime |
| adt.vera | User struct + enum + match |
| option_result.vera | Option / Result + exhaustive match |
| list_demo.vera | List literals and total accessors |
| lambda_list.vera | Lambdas + List map/filter/fold |
| propagate.vera | Postfix ? Option propagation |
| prove_clamp.vera | --prove: SMT-proved clamp refinements (expect 6 proved) |
| prove_abs.vera | --prove: SMT-proved abs_nonneg (expect 2 proved; Int unary-minus) |
| prove_runtime_hint.vera | --prove: Str ensures -> [RUNTIME-CHECKED] |
| prove_refuted.vera | --prove: false Int ensures -> [REFUTED], exit 3 |
| refine_call_ok.vera | [P2-REFINE1] in-range refined calls typecheck+run |
| refine_len_ok.vera | [P2-REFINE2] in-range `nth` + `len` measure; prints 20 |

Optional smoke: `powershell -File bench/soft_smoke.ps1`
