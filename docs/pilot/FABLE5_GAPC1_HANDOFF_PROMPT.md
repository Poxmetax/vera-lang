<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAPC1_IMPLEMENT.md
Full brief stays in THIS file. Not a review prompt. Not labels/IFC.
STATUS: recommended next hard task AFTER Madis commits GAP4-R2-SURFACE.
GATE: do not start until GAP4 surface is committed (exclude bak).
-->

# Fable 5 -- VERA GAP-C1 handoff (symbolic len-as-index compile-time reject)

Canonical full brief for **GAP-C1 only**. Madis pastes [`CLAUDE_POINTER_GAPC1_IMPLEMENT.md`](CLAUDE_POINTER_GAPC1_IMPLEMENT.md) into chat when ready.

**Why this task (default next after GAP4 surface):**
- GAP4-R2-SURFACE closed the "no checker surface" remainder (seeded). Soft review PASS; suite **56**.
- Remaining label work (value-label syntax, R2 ergonomics probe) is **Madis-gated** and easy to overclaim as IFC progress -- not the default.
- GAP-C1 is plan-faithful refine debt from [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md): SPEC REQ-REFINE-2 cites `nth(xs, xs.len())` / `len(xs)`-as-index; P2C deferred it honestly to runtime.
- Thin typecheck fragment; not full IFC; not GAP-D2; not F6 polish.

**Gate:** Madis must **commit** `[GAP4-R2-SURFACE]` first (exclude `*.bak_*`). Do not start on a dirty uncommitted GAP4 tree unless Madis explicitly overrides.

---

You are continuing **VERA** (`vera-lang`). Madis is the operator. This session implements **GAP-C1**: one thin compile-time reject for the deferred symbolic `len`-as-index case.

## Hard constraints

1. Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. Never touch TradingBot mainnet / `.env` / live state.
3. No git commit/push unless Madis asks.
4. Prefer zero new Cargo crates; ask before any.
5. Surgical diffs; ask before >~30 lines.
6. Code/docs English; UTF-8; prefer ASCII punctuation (`->`, `--`, `>=`).
7. No file renames (especially `examples/`).
8. Preserve A--E + GAP4 surface contracts: FixPatch additive/`ephemeral: true`; label seeds pass stays green; soft_smoke / prove_clamp regression.
9. Honest-limits table in the slice note; do not overclaim full REQ-REFINE-2 or SMT `len` encode (GAP-C2).
10. Never add examples that fail typecheck (`round_trip_all_examples`). Reject demos stay out of `examples/` or use unit tests only.
11. Do **not** open labels/IFC, value-label syntax, R2 ergonomics corpus, or GAP-D2 unless Madis re-scopes.

## Preconditions

| Item | Status |
|------|--------|
| CONF-P2 A--E | **done** |
| GAP4-R2-SURFACE | **must be committed** before you start -- SoT [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md); ACK [`CURSOR_SYNC_ACK_GAP4_SURFACE.md`](CURSOR_SYNC_ACK_GAP4_SURFACE.md) |
| P2C decided-literal `len` | **LANDED** -- [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md); GAP-C1 = the deferred symbolic fragment |
| Baseline after GAP4 commit | expect **56** lib tests; prove_clamp **6** proved |

Debt register: [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-C1 row.

## Already done (do not re-open)

| Slice | Status |
|-------|--------|
| P2C decided-literal index reject | closed fragment -- do not rewrite |
| GAP4 surface / pilot | leave alone unless regression |
| GAP-C2 SMT `len` measure encode | **out of scope** |
| GAP-D2 / FixPatch durable | **out of scope** |

## SPEC / SoT anchors

- SPEC REQ-REFINE-2 / `nth` + `len` wording (cite from SPEC when writing the slice).
- [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md) honest limit: `nth(xs, xs.len())` not rejected this slice -- needs symbolic same-term reasoning.
- Soft queue: [`SOFT_PARALLEL_QUEUE.md`](SOFT_PARALLEL_QUEUE.md) next-recommended = this task.

## What YOU must do (smallest closed fragment)

1. Read this brief + [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md) + [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-C1 + existing `refine2_*` / `len` typecheck paths.
2. Deliver **one** demonstrable typecheck reject for the deferred symbolic case (preferred shape: `nth(xs, xs.len())` or equivalent same-term `len(xs)`-as-index), with marker `[GAPC1-SYM-LEN]` (grep uniqueness first).
3. Prefer spanning the smallest AST-equality / param→arg substitution fragment that catches the SPEC case; ask Madis before widening into general symbolic arithmetic.
4. Keep non-literal / unbounded indices soft → runtime as already designed (do not pretend full symbolic solver).
5. Slice note: `docs/pilot/GAPC1_SYM_LEN_SLICE.md` with claimed vs not-claimed + HONEST LIMITS (not GAP-C2; not full REQ-REFINE-2; not list-length propagation for arbitrary literals unless already in P2C).
6. Update [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-C1 row when closed (or PARTIAL if only a sub-case lands -- say so).
7. Unit test(s) + soft_smoke PASS; prove_clamp still 6; suite count documented (expect 56 + N).
8. Do **not** implement: SMT `len` encode, value-label syntax, R2 ergonomics probe, GAP-D2, MCP, z3 crate, Salsa.

### Smoke

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- --prove examples/prove_clamp.vera
# expect: soft_smoke PASS; prove_clamp -> 6 proved; suite 56+N
```

## Out of scope (explicit)

- Value-label syntax / R2 ergonomics corpus (Madis-gated label track -- not default)
- Full IFC / taint / implicit flows
- GAP-C2 SMT measure encode
- GAP-D2 durable FixPatch store
- F6 string Debug-escape polish
- Rewriting P2C decided-literal path

## Alternatives Madis may choose instead (do not invent)

| Alt | When |
|-----|------|
| Thin value-label annotation feeding GAP4 seeds pass | Madis wants language surface next |
| Thin R2 ergonomics probe (measurement only) | Madis explicitly asks; does **not** close CONF-P2 gate alone |
| F6 string Debug-escape | polish day |
| GAP-D2 | only if durable certs wanted |

Default remains **GAP-C1**.

## Return (English short)

```text
VERDICT: DONE-GAPC1 | BLOCKED | PARTIAL
files: ...
marker: [GAPC1-SYM-LEN]
smoke: lib N; soft_smoke; prove_clamp 6
honest limits: ...
next suggestion only (Madis decides)
```

End of GAP-C1 handoff.
