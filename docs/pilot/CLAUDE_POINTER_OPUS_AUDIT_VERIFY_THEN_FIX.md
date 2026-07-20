# CLAUDE / Fable 5 pointer — Opus audit VERIFY-THEN-FIX

**Operator:** Madis  
**Date:** 2026-07-20  
**Mode:** VERIFY FIRST. Do **not** write production patches until Madis replies `y` to your verification report.  
**Track:** Hard (Rust + CI + bench hygiene). Soft docs only if needed for the report.

---

## Role

You are Fable 5 (Claude Code) on `vera-lang`. Independently re-verify every load-bearing claim from the Opus GitHub/security audit, against the **actual program** on this machine. Claim-less discipline: VERIFIED / REFUTED / UNVERIFIED with command or `file:line`. Never present inference as measurement.

After verification, produce a short GO/NO-GO per work item. **Stop.** Wait for Madis `y` before any patcher / edit / commit / push.

---

## Workspace

- Root: `C:\Users\madis\Desktop\TradingBot\vera-lang\`
- Public remote: `vera-github` → `https://github.com/Poxmetax/vera-lang` (do not push until Madis says so)
- Audit dump (truncated): `C:\Users\madis\Desktop\New Text Document.txt` (starts mid-report at C-2)
- Opus scratch fixtures (if still present):  
  `%LOCALAPPDATA%\Temp\claude\C--Users-madis-Desktop-TradingBot\f4524ee0-68b8-4747-8050-1fc5d3291936\scratchpad\`  
  Key files: `audit_baseline.vera`, `audit_inject.vera`, `audit_letinj2.vera`, `audit_run_honest.vera`, `audit_run_inject.vera`  
  If missing, recreate **outside the repo** (temp dir) from the fixture bodies below — never leave exploit PoCs inside tracked tree.

---

## Hard constraints

1. Stay inside `vera-lang/`. Never touch `.env*`, `futures_grid_bot/`, `crew_memory/`, TradingBot live state, or GridBrain.
2. No `git commit` / `git push` / `git add` until Madis explicitly approves after your verify report.
3. Do not start ERGO / F6 / GAP-D2 feature work.
4. Do not run `cargo clippy --fix` blindly (intentional `None` branch risk in `render.rs`).
5. Do not rewrite git history for path leaks (`C:\Users\madis\…`).
6. Never print credentials, tokens, or secret values.
7. Surgical rule if later approved to patch: smallest change; phase markers; >30 lines of hard logic → ask first.
8. R-1 fix philosophy if later approved: **reject, do not escape** `|` / illegal binder names → fall to RuntimeChecked / parse error (fail-closed). Patch **both** ends: parser Ident-only **and** `sanitize_sym` → `Result`.

---

## Work items Madis wants (ALL) — verify each, then wait

| ID | Priority | Intent (do NOT implement yet) |
|----|----------|-------------------------------|
| **R-1** | P0 | SMT-LIB injection via binder name (`\|` in Str token) → false `[PROVED]` + `--prove-run` elides runtime checks |
| **R-1b** | P0 | Second site: `let` binder injection |
| **P0b** | P0 | Untrack published answer bak: T05/T08 under `_operator_archive/*.bak_*` (`git rm --cached`; `.gitignore` already has `**/*.bak_*`) |
| **R-2** | P1 | Parser unbounded recursion → process stack overflow |
| **R-3** | P1 | Exponential closure capture (`env.values.clone()` per lambda) |
| **CI** | P1 | Pin Actions to SHA; `persist-credentials: false`; optional `cargo-audit` job |
| **Hygiene** | P3 | Note only: absolute paths in `soft_smoke.ps1` etc. — verify count; do not mass-rewrite unless Madis expands scope |

**Already done (do not re-add as missing):** `SECURITY.md`, `CONTRIBUTING.md`, `.github/dependabot.yml` — confirm present on disk + `vera-github/main`.

---

## Phase 1 — Independent verification matrix (mandatory)

Reproduce **yourself**. Prefer Opus scratch fixtures; recreate in temp if needed.

### R-1 / R-1b (CRITICAL) — must reproduce

**Structural (read both ends):**

- `crates/vera/src/vc.rs` — `sanitize_sym` ≈ `format!("|{name}|")` without escaping `|`
- `crates/vera/src/parser.rs` — param / let / refine binder: `let name = self.advance().text` without `TokKind::Ident` gate
- `crates/vera/src/smt.rs` — first matching sat line from stdout/stderr chain

**Empirical (expected if still open):**

| Fixture | Command | Expect |
|---------|---------|--------|
| baseline (param `x: Int`, body `0`, `ensures result >= 999`) | `cargo run -q -p vera -- --prove <file>` | `[REFUTED]` / exit **3** |
| inject (param name Str token containing `\|` + SMT spam) | same | `[PROVED]` / exit **0** |
| letinj2 | same | `[PROVED]` / exit **0** |
| run_honest (calls `evil(1)`) | `--prove-run` | refuted path / **not** silent success |
| run_inject | `--prove-run` | `[PROVED]` + `elided … runtime check` + prints `0` / exit **0** |

**Fixture bodies (recreate in temp if scratch gone):**

```text
# audit_baseline.vera
fn evil(x: Int) -> Int
    ensures result >= 999
{ 0 }
fn main(console: Console) -> Unit uses {console} {
    console.print("hi");
}

# audit_inject.vera  (param name is a string token — exact bytes matter)
fn evil("x| Int) (assert false) (check-sat) (declare-const |zz": Int) -> Int
    ensures result >= 999
{ 0 }
fn main(console: Console) -> Unit uses {console} {
    console.print("hi");
}
```

(Use Opus scratch copies for `letinj2` / `run_*` when available.)

Record: exact exits, one-line verdict strings, whether your numbers match Opus.

### P0b — bak on GitHub

```text
gh api repos/Poxmetax/vera-lang/contents/bench/vera_agent_bench_v01/_operator_archive --jq ".[].name"
git -C vera-lang ls-files "bench/vera_agent_bench_v01/_operator_archive/*.bak_*"
git -C vera-lang check-ignore -v <each bak path>   # expect ignore rule vs still tracked
```

Expect: two T05/T08 `*.bak_*` **tracked** despite `.gitignore` `**/*.bak_*`.

### R-2 / R-3

- Confirm no depth limit in `parser.rs` (grep).
- Confirm `interp.rs` closure capture clones full env.
- Optional empirical: Opus `audit_deep_*.vera` / `audit_clos.vera` — only if safe; do not hang the session on 240KB deadlock fixtures unless Madis asks.

### CI

Read `.github/workflows/release-cli.yml`: `contents: write`, unpinned `@stable` / `@v4` tags. Confirm no `cargo-audit` job yet.

### Clean checks

```text
cargo build -p vera
cargo test -p vera --lib
```

Expect build OK; note test count (README claims 68 — measure, don't assume).

### Profile / packaging (context only)

Public profile may already show Name=Madis, bio, `Poxmetax/Poxmetax` README — audit started before that. Do not spend time “fixing” empty profile.

---

## Phase 2 — Verification report (output format)

Deliver to Madis **before any write**:

1. Assumption register (load-bearing claims + VERIFIED/REFUTED/UNVERIFIED)
2. Table: Item | Opus claim | Your result | Match?
3. GO/NO-GO for implementing each of: R-1, R-1b, P0b, R-2, R-3, CI
4. Proposed surgical patch outline (files + approach) — **no code yet**
5. Risks (esp. prove-run elision semantics, intentional-design false positives)
6. Explicit list of what you did **not** re-run
7. Short Estonian summary (6–8 bullets)

**STOP** and wait for Madis `y` / partial approve (e.g. `R-1+P0b only`).

---

## Phase 3 — Implement (ONLY after Madis `y`)

Order if all approved:

1. **R-1 both layers** (parser Ident-only at all binder sites + `sanitize_sym` reject → RuntimeChecked/error). Add tests/fixtures under test tree or temp→promoted tests — no exploit left in `bench/`.
2. **Re-run** R-1 matrix: inject must **fail closed** (no false PROVED; runtime checks not elided on forged prove).
3. **P0b:** `git rm --cached` the two archive baks; commit; subtree-publish only if Madis says publish.
4. **R-2** depth cap; **R-3** `Rc` capture map — surgical.
5. **CI** pin SHAs + `persist-credentials: false`; optional cargo-audit with network approval.
6. Do **not** cut a release tag until R-1 is green.

Patcher ceremony: follow project soft/hard norms; LF; py_compile N/A for Rust — `cargo test` + R-1 matrix as gate.

---

## Do not

- Patch on Opus’s word alone
- Escape SMT names instead of rejecting
- Publish release binaries before R-1 closed
- Commit TradingBot monorepo secrets
- Blind `clippy --fix`
- Silent oracle/bench semantics changes without versioning note

---

## Success (verify phase)

- [ ] R-1 differential reproduced or honestly REFUTED with evidence
- [ ] P0b bak still public/tracked confirmed
- [ ] CI file claims confirmed
- [ ] Report delivered; **no production edits** until `y`
