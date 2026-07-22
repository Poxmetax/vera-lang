# T09 — fail_closed_binder

Run `--prove` on the initial program. The refinement binder is a string
literal (`"r|s"`) instead of an identifier. Expect a parse-time reject:
exit **1**, an `error:` line, and **no** `[PROVED]` line anywhere — the
toolchain must not forge proof output for a program it cannot parse
(fail-closed, R1 spirit).

Use only: `cargo run -p vera -- --prove <file>`.

not_claims: this fixture demonstrates the front-door identifier-binder
reject; it is not an SMT-injection corpus and not exploit tooling.
