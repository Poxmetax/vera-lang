# 002 -- gitattributes-eol-pins

## Goal

Create a `.gitattributes` at the vera-lang root pinning LF line endings for Rust
sources, Markdown, and VERA sources (`*.rs`, `*.md`, `*.vera`), and add
machine-checked guards proving the pins exist and that the existing
`.git-blame-ignore-revs` remains well-formed. Roadmap item: maintenance -- repo
hygiene that stops CRLF churn from breaking `cargo fmt --check` and polluting
diffs and blame on Windows checkouts.

## Context & constraints

- ROADMAP: absent (docs/ROADMAP.md does not exist in this checkout).
- New file: `.gitattributes` at the vera-lang root -- verified absent today;
  this spec creates it.
- Existing file: `.git-blame-ignore-revs` at the vera-lang root -- verified
  present; contains `#`-prefixed comment lines, blank lines, and one 40-char
  lowercase hex hash line. It must remain byte-for-byte unchanged.
- Test host: `crates/vera/src/lib.rs` -- the existing `#[cfg(test)] mod tests`
  block already locates the vera-lang root by joining `CARGO_MANIFEST_DIR` with
  `../..` (see the `examples_dir` helper used by
  `tests::round_trip_all_examples`); the two new unit tests reuse that pattern.
- Workspace: root `Cargo.toml` lists member `crates/vera` (package name `vera`
  in `crates/vera/Cargo.toml`).
- Required `.gitattributes` content -- exactly these three pin lines, nothing
  else, with a trailing newline:

```text
*.rs text eol=lf
*.md text eol=lf
*.vera text eol=lf
```

- Invariants: no other tracked file's committed content changes except the test
  additions in `crates/vera/src/lib.rs`; no renormalization commit.
- Budget: 12GB RAM, CPU-only, Windows, stable toolchain; no new dependencies;
  no network.
- Assumption: hash lines in `.git-blame-ignore-revs` are full 40-char lowercase
  hex (SHA-1 repo); `#`-prefixed comment lines and blank lines are permitted.
- Assumption: both tests compare trimmed lines (tolerating a trailing CR on
  read) so they hold even if a Windows working copy materializes the files with
  CRLF.

## Acceptance criteria

- `cargo test --package vera --lib tests::gitattributes_pins_eol`
- `cargo test --package vera --lib tests::blame_ignore_revs_well_formed`
- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`

## Required tests

- `tests::gitattributes_pins_eol` -- unit, inside the existing `mod tests` in
  `crates/vera/src/lib.rs` -- reads `.gitattributes` at the vera-lang root;
  asserts the file exists and that its trimmed lines include each of the three
  pin lines `*.rs text eol=lf`, `*.md text eol=lf`, and `*.vera text eol=lf`.
- `tests::blame_ignore_revs_well_formed` -- unit, same module -- reads
  `.git-blame-ignore-revs` at the vera-lang root; asserts the file exists, that
  every non-empty trimmed line not starting with `#` is exactly 40 lowercase
  ASCII hex characters, and that at least one such hash line is present.

## Non-goals / do-not-touch

- No renormalization: do not run `git add --renormalize`, do not re-stage or
  re-commit existing files to flip their stored line endings.
- No edits to `.git-blame-ignore-revs`.
- No changes to `.gitignore`, CI/workflow files, `Cargo.toml`, or `Cargo.lock`.
- No changes to language modules (`ast`, `parser`, `lexer`, `interp`,
  `typecheck`, `vc`, `smt`, `store`, `render`, `diag`, `label`) beyond the two
  test functions added to `mod tests`.
- No `git config` changes (e.g. `blame.ignoreRevsFile`, `core.autocrlf`) --
  per-user settings are out of scope.

## Risks

- Retroactivity: this changes nothing retroactively -- files already committed
  with CRLF renormalize only when they are next staged/touched. Any future bulk
  renormalization is a separate task, and its commit hash would then belong in
  `.git-blame-ignore-revs`.
- Windows working copies: with `core.autocrlf=true` checked-out files may carry
  CRLF, so both tests must compare trimmed lines rather than raw bytes.
- Blame behavior: `.git-blame-ignore-revs` only takes effect where
  `blame.ignoreRevsFile` is configured (GitHub honors the file automatically);
  this spec guards its presence and well-formedness, not client git config.
- Acceptance runtime: two tiny unit tests plus the standing gates, reusing
  battery artifacts; well under the ~580 s shared budget.
- No `unsafe` required or permitted.

## Dependencies

None. Independent of specs/001-pipeline-smoke-language-name: both add unit
tests to the same `mod tests` block in `crates/vera/src/lib.rs`, but the edits
are additive and non-conflicting.
