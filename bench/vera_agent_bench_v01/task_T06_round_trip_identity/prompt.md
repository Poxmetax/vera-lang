# T06′ — round_trip_paren_identity

Run `--round-trip` on the initial program. The fixture is paren-heavy with
mixed precedence (`(a + b) * c`, `a - (b - c)`, comparison over products).
Expect `round-trip OK` — parse → hash → render → parse → hash equality, so
the renderer's parenthesization must be honest (GAP-3 spirit), not merely
"looks the same".

Use only: `cargo run -p vera -- --round-trip <file>`.

not_claims: general formatter stability; F6 string-escape round-trip is an
alternate seed, not scored in this task.
