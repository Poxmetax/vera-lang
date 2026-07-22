# 001 ? pipeline-smoke-language-name

## Goal

Add a trivial public crate-root helper `language_name() -> &'static str` that
returns `"vera"`, plus one unit test that asserts that exact string. This is a
deliberately minimal change whose real purpose is to smoke-test the
planner?executor harness end-to-end (spec lint, implement, Stop-gate acceptance)
without touching language semantics.

## Context & constraints

- Target crate: `crates/vera` (package name `vera` in `crates/vera/Cargo.toml`).
- Target file: `crates/vera/src/lib.rs` ? crate root already re-exports public API
  (`parse`, `check_program`, `prove_program`, ?) and already contains a
  `#[cfg(test)] mod tests { ? }` block (see existing tests such as
  `tests::round_trip_all_examples`).
- Place `language_name` as a new public free function at crate root (alongside the
  existing `pub use` re-exports), not inside a submodule.
- Place the new unit test inside the existing `#[cfg(test)] mod tests` so the
  cargo-test path is `tests::language_name_returns_vera`.
- Invariants that must not break: existing public API surface, existing tests in
  `mod tests`, Phase 1?2 front-end / interpreter / VC behaviour.
- Budget: 12GB RAM, CPU-only, Windows, stable Rust toolchain; no new crates;
  no `unsafe`; no network.
- Assumption (fallback where ambiguous): returning the lowercase ASCII string
  `"vera"` (not a display name) is the intended contract.

## Acceptance criteria

- `cargo test --package vera --lib tests::language_name_returns_vera`
- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`

## Required tests

- `tests::language_name_returns_vera` ? unit ? asserts
  `language_name() == "vera"` (exact byte string, no trim/case fold).

## Non-goals / do-not-touch

- No changes to `ast`, `parser`, `lexer`, `interp`, `typecheck`, `vc`, `smt`,
  `store`, `render`, `diag`, or `label` modules.
- No CLI / binary (`crates/vera/src/main.rs` if present) flag or help-text changes.
- No new dependencies in any `Cargo.toml`.
- No bench / trial-pack / docs edits.
- Do not rename or refactor the existing `mod tests` block beyond adding the one
  new test function.

## Risks

- Filter mismatch: acceptance uses the exact path
  `tests::language_name_returns_vera`; a differently named or nested test will
  trip the Stop-gate zero-match guard.
- Windows line endings: keep LF in edited Rust sources to satisfy `cargo fmt`.
- Acceptance runtime estimate: ~30?90 s beyond the existing battery (one lib unit
  test + fmt + clippy reuse of prior artifacts); well inside the ~580 s shared
  budget.
- No `unsafe` required or permitted for this change.

## Dependencies

None. First smoke-pipeline spec; no prerequisite `specs/NNN-*`.
