# T05 — nonexhaustive_match_fixpatch

1. Run `--diag-json` on `initial/main.vera`. Confirm FixPatch kind `add-match-arms` and `ephemeral: true`.
2. Edit the source under `initial/` (or a work copy) to add the missing arm(s) so the match is exhaustive.
3. Re-check: program typechecks and runs (`cargo run -p vera -- <file>` exit 0).

Honest limits: one FixPatch kind only; ephemeral; no durable store (GAP-D2). Do not expand the language.