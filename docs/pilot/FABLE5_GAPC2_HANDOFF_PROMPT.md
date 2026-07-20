<!--
Operator: chat paste SHORT POINTER only -- CLAUDE_POINTER_GAPC2_IMPLEMENT.md
Full brief stays in THIS file. Not a review prompt. Not labels/IFC.
STATUS: prepared candidate. Madis-gated. Soft does NOT auto-pick vs GAP4-VALUE-LABEL.
-->

# Fable 5 -- VERA GAP-C2 handoff (SMT `len` measure encode)

Canonical full brief for **GAP-C2 only**. Madis pastes [`CLAUDE_POINTER_GAPC2_IMPLEMENT.md`](CLAUDE_POINTER_GAPC2_IMPLEMENT.md) into chat when ready.

**Why this task (candidate, not auto-default):**
- GAP-C1 closed the typecheck same-term `nth(xs, xs.len())` fragment. Soft ACK [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md); suite **59**.
- Prove tier still treats `Call` / `len` as **RUNTIME-CHECKED** (`encode_expr` rejects Call; QF_LIA slice has no measure). Agents may over-trust typecheck-only.
- Register: [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-C2; deferred SoT [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md) / [`PHASE2_VC_SLICE_REPORT.md`](PHASE2_VC_SLICE_REPORT.md).
- Thin VC/SMT encode fragment only -- not full REQ-REFINE-2; not labels; not GAP-D2.

**Gate:** Madis chooses this over (or after) [`GAP4-VALUE-LABEL`](FABLE5_GAP4_VALUE_LABEL_HANDOFF_PROMPT.md). Soft does **not** pick. Prefer GAP-C1 committed (done: `4fbf7df` / publish `0bc3c22`).

---

You are continuing **VERA** (`vera-lang`). Madis is the operator. This session implements **GAP-C2**: encode `len` as a measure in the VC/SMT path enough that a **relevant prove path** is honestly `[PROVED]` (unsat) or clearly unknown / RUNTIME-CHECKED -- never a fake PROVED.

## Hard constraints

1. Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. Never touch TradingBot mainnet / `.env` / live state.
3. No git commit/push unless Madis asks.
4. Prefer zero new Cargo crates; ask before any. Keep Z3 **subprocess** path (do not link `z3` crate unless Madis asks).
5. Surgical diffs; **ask Madis before >~30 lines**.
6. Code/docs English; UTF-8; prefer ASCII punctuation (`->`, `--`, `>=`).
7. No file renames (especially `examples/`).
8. Preserve A--E + GAP4 surface + GAP-C1 contracts: FixPatch additive/`ephemeral: true`; soft_smoke PASS; **prove_clamp still 6 proved** unless Madis expands the example.
9. Honest-limits table in the slice note; do not overclaim full REQ-REFINE-2, list-length propagation, or that every Call is now proved.
10. Never add examples that fail typecheck (`round_trip_all_examples`). Reject / RUNTIME demos stay unit tests or existing prove_runtime_hint pattern.
11. Do **not** open labels/IFC, value-label syntax, R2 ergonomics, or GAP-D2 unless Madis re-scopes.
12. Soundness-first: prefer honest RUNTIME-CHECKED over fake PROVED (`[P2-SOUND1/2/3]` habits).

## Preconditions

| Item | Status |
|------|--------|
| CONF-P2 A--E | **done** |
| GAP4-R2-SURFACE | **CLOSED** -- [`GAP4_R2_SURFACE_SLICE.md`](GAP4_R2_SURFACE_SLICE.md) |
| GAP-C1 same-term typecheck | **CLOSED** -- [`GAPC1_SYM_LEN_SLICE.md`](GAPC1_SYM_LEN_SLICE.md); ACK [`CURSOR_SYNC_ACK_GAPC1.md`](CURSOR_SYNC_ACK_GAPC1.md) |
| P2C decided-literal `len` | **LANDED** -- [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md); GAP-C2 = SMT encode leg |
| Baseline | expect **59** lib tests; prove_clamp **6** proved |

Debt register: [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-C2 row.

## Already done (do not re-open)

| Slice | Status |
|-------|--------|
| P2C decided-literal index reject + interp `len` measure | closed -- do not rewrite |
| GAP-C1 same-term typecheck reject | closed -- leave alone unless regression |
| GAP4 surface / pilot | leave alone |
| GAP4-VALUE-LABEL / R2 ergonomics | **out of scope** this session |
| GAP-D2 / FixPatch durable | **out of scope** |

## SPEC / SoT anchors

- SPEC §4.4: refinements checked by SMT over QF_LIA **plus measures like `len`**; REQ-REFINE-2 measure wording.
- [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md): `--prove` stays RUNTIME-CHECKED for Call; SMT encode deferred.
- [`PHASE2_VC_SLICE_REPORT.md`](PHASE2_VC_SLICE_REPORT.md): encode failure / unknown => RUNTIME-CHECKED; unsat => PROVED.
- Soft queue: [`SOFT_PARALLEL_QUEUE.md`](SOFT_PARALLEL_QUEUE.md) -- candidates Madis-gated (no auto-pick).

## What YOU must do (smallest closed fragment)

1. Read this brief + [`P2C_LEN_SLICE.md`](P2C_LEN_SLICE.md) + [`GAPC1_SYM_LEN_SLICE.md`](GAPC1_SYM_LEN_SLICE.md) + [`PHASE2_VC_SLICE_REPORT.md`](PHASE2_VC_SLICE_REPORT.md) + [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-C2 + existing `vc.rs` / `smt.rs` encode paths (especially where `Call` becomes RuntimeChecked).
2. Deliver a **thin** encode of `len` (uninterpreted / measure function, or the smallest fragment that makes one len-bound obligation honest) so that **at least one** relevant prove path is no longer a silent Catch-all RUNTIME-CHECKED for that measure -- either:
   - honest `[PROVED]` when Z3 returns unsat on a closed, sound obligation involving `len`, **or**
   - clear remaining RUNTIME-CHECKED / unknown with documented reason (no fake PROVED).
3. Marker: `[GAPC2-SMT-LEN]` (grep uniqueness first). Likely touch: `vc.rs` and/or `smt.rs` only. Ask before widening into typecheck / interp / parser.
4. Prefer the **smallest** fragment that closes the register debt for "SMT encode of len still open". Do **not** boil the ocean (full list theory, arbitrary Call encoding, general symbolic arithmetic from GAP-C1 soft cases).
5. Slice note: `docs/pilot/GAPC2_SMT_LEN_SLICE.md` with claimed vs not-claimed + HONEST LIMITS (what Call shapes stay RUNTIME-CHECKED; prove_clamp unchanged unless Madis expands).
6. Update [`KNOWN_GAPS.md`](KNOWN_GAPS.md) GAP-C2 row when closed (or PARTIAL if only a sub-encode lands -- say so).
7. Unit test(s) + soft_smoke PASS; prove_clamp still **6**; suite count documented (expect 59 + N).
8. Do **not** implement: value-label syntax, R2 ergonomics probe, GAP-D2, MCP, z3 crate, Salsa, rewriting GAP-C1 typecheck path.

### Smoke

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
cargo run -p vera -- --prove examples/prove_clamp.vera
# expect: soft_smoke PASS; prove_clamp -> 6 proved; suite 59+N
```

## Correct work (PASS bar)

- [ ] `len` participates in VC/SMT encode for a documented fragment (not only rejected as Call)
- [ ] Marker `[GAPC2-SMT-LEN]`; no fake PROVED on unsound encode
- [ ] Slice note with HONEST LIMITS / claimed-vs-not (what stays RUNTIME-CHECKED)
- [ ] soft_smoke PASS; prove_clamp 6; no typecheck-failing committed examples
- [ ] KNOWN_GAPS GAP-C2 updated honestly

## Out of scope (explicit)

- Value-label syntax / R2 ergonomics / full IFC
- GAP-C1 soft cases becoming proved (`xs.len()-1`, aliases, non-Name receivers) unless they fall out of the thin encode for free -- do not expand scope to chase them
- List-literal length propagation at typecheck (P2C soft)
- GAP-D2 durable FixPatch store
- F6 string Debug-escape polish
- Linking the `z3` crate

## Alternatives Madis may choose instead (do not invent)

| Alt | When |
|-----|------|
| GAP4-VALUE-LABEL | Madis wants author-visible labels feeding the seeded surface next |
| Thin R2 ergonomics probe | Madis explicitly asks measurement; does **not** close CONF-P2 gate alone |
| F6 string Debug-escape | polish day |
| GAP-D2 | only if durable certs wanted |

Soft does **not** default-pick. Both GAP-C2 and GAP4-VALUE-LABEL are prepared.

## Return (English short)

```text
VERDICT: DONE-GAPC2 | BLOCKED | PARTIAL
files: ...
marker: [GAPC2-SMT-LEN]
smoke: lib N; soft_smoke; prove_clamp 6
honest limits: what stays RUNTIME-CHECKED ...
next suggestion only (Madis decides)
```

End of GAP-C2 handoff.
