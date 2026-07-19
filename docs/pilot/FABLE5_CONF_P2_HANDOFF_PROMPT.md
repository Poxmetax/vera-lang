<!--
Operator note (Estonian, header only):
Madis — kopeeri kõik allpool olev "# Fable 5 …" plokist kuni lõpuni Claude Code Fable 5 sessiooni.
Pehme polish (README + --prove help) on juba peal; rasked CONF-P2 ülesanded on sinu jaoks.
Ära puuduta TradingBot mainnet / .env / live state. Ära commit/push ilma Madiseta.
-->

> **Review vs implementation:** For **review** sessions (audit of already-landed work), use `docs/pilot/CLAUDE_REVIEW_*.md` files — start from the template [`CLAUDE_REVIEW_PROMPT_TEMPLATE.md`](CLAUDE_REVIEW_PROMPT_TEMPLATE.md); current slice review: [`CLAUDE_REVIEW_P2_REFINE1.md`](CLAUDE_REVIEW_P2_REFINE1.md). **This handoff file is for implementation** (CONF-P2 tasks A–E), not for review paste.

# Fable 5 — VERA CONF-P2 handoff (paste-ready)

Copy everything from this heading downward into a new Claude Code **Fable 5** session.

---

You are continuing **VERA** (`vera-lang`), an isolated research language prototype. Madis is the operator.

## Hard constraints (non-negotiable)

1. **Workspace:** `C:\Users\madis\Desktop\TradingBot\vera-lang\` only.
2. **Isolation:** Never touch TradingBot mainnet runtime, `.env`, live state files (`futures_grid_state.json`, `bot_control.json`, alert cooldowns, etc.). Never import/export across the TradingBot ↔ vera-lang boundary.
3. **Git:** No `git commit` / `git push` unless Madis explicitly asks. No `origin` is configured; do not add remotes.
4. **Dependencies:** Prefer **zero** new Cargo crates. If you believe a dep is required, document the need and **ask Madis before** adding it. Especially: do **not** link the `z3` crate or add Salsa without approval (subprocess Z3 path is intentional).
5. **Surgical diffs:** Smallest change that preserves intent. Ask Madis before any change **>~30 lines**. Wait for / incorporate **your own independent review findings** of Phase 1/2 before large patches.
6. **Language:** Code and docs English. Operator chat may be Estonian.

## Already done (do not re-litigate)

**Git:** `5f3692c` on `main` — Phase 2 thin VC + Phase 1 polish.

| Phase | Status | Pointers |
|-------|--------|----------|
| −1 thesis pilot | PASS | `docs/pilot/REPORT.md` |
| 0 research + SPEC | delivered | `docs/research/RESEARCH_REPORT.md`, `docs/spec/SPEC.md` v0.1 |
| SMT spike | PASS | `docs/pilot/SMT_SPIKE_REPORT.md`, `docs/pilot/smt_refine_spike.py` |
| 1 front-end + interp | CONF-P1 met | `crates/vera/` — parse, typecheck, store, `EditTransaction`, holes `?ident`, postfix `?` |
| 2 thin VC slice | delivered (not full CONF-P2) | `crates/vera/src/smt.rs`, `vc.rs`, CLI `--prove`; report `docs/pilot/PHASE2_VC_SLICE_REPORT.md` |

**Z3 path:** SMT-LIB2 subprocess to `C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin\z3.exe` (discovery: `VERA_Z3` → PATH `z3` → sibling unpack). See `smt.rs`.

**Prove demo:** `examples/prove_clamp.vera` → expect **6** `[PROVED]` obligations under `--prove`.

**Soft polish already landed (optional awareness):** clearer `--prove` lines in `crates/vera/src/main.rs` `usage()`; README “Remaining → Fable 5” points here.

**Read first:** `README.md`, `docs/pilot/PHASE2_VC_SLICE_REPORT.md`, `docs/spec/SPEC.md` §4.4 (REQ-REFINE + INV-1/DP6), `crates/vera/src/{smt,vc,main,typecheck}.rs`.

## Smoke (must stay green)

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang

# Z3 on PATH (or set VERA_Z3)
z3 --version   # expect Z3 version 4.16.0

cargo test -p vera --lib
cargo run -p vera -- --prove examples/prove_clamp.vera
# expect: 6× [PROVED], exit 0
```

Also useful: `cargo run -p vera -- examples/hello.vera`, `--round-trip examples/hello.vera`.

## Explicitly deferred / do NOT implement unless Madis re-scopes

- Labels / IFC / `Secret<T>` / endorse·declassify
- Linking `z3` Rust crate (in-process)
- Salsa incremental
- Hole synthesis (`?ident` fill)
- Ceremony-heavy multi-prover / proof certificates beyond what INV-1 elision needs
- Any TradingBot integration

## Ordered next complex tasks (CONF-P2) — smallest first

Incorporate your Phase 1/2 review findings **before** large patches. For each task: propose surgical plan → Madis yes/no if >~30 lines → implement → re-run smoke above.

### A. REQ-REFINE-1 — hard typecheck reject at call sites

**SPEC:** `docs/spec/SPEC.md` §4.4 REQ-REFINE-1 (apply_discount / range call-site reject at **compile time**, zero execution).

**Gap today:** `--prove` can report `[REFUTED]` / `[RUNTIME-CHECKED]`; typecheck does **not** hard-fail out-of-range / violated refinements at call sites.

**Success checklist:**
- [ ] Example (or test) where `apply_discount(100, 150)`-shaped call is a **typecheck error** without running the program
- [ ] Valid in-range calls still typecheck + run
- [ ] Definition-time reject when body can violate return refine (per SPEC), or document honest limit if deferred to B
- [ ] `cargo test -p vera --lib` green; `prove_clamp.vera --prove` still 6 proved

### B. Wire prove results into typecheck diagnostics

**SPEC:** §4.4 obligation flow + DP6; Phase 2 report “no typecheck integration yet”.

**Goal:** Surface proved vs runtime-checked vs refuted in the typecheck/diagnostic path (not only CLI `--prove` text), so tooling can treat refute as error and proved as discharged.

**Success checklist:**
- [ ] Refuted obligations appear as structured diagnostics from the check pipeline (or documented single entrypoint)
- [ ] Proved vs runtime-checked distinguishable in diagnostic payload
- [ ] Default run path bit-compatible or clearly gated (prefer opt-in / `--prove` unless Madis says otherwise — HR1-style)
- [ ] Smoke green

### C. REQ-REFINE-2 + `len` measures on `List`

**SPEC:** §4.4 REQ-REFINE-2; §3/`nth` example with `{k: Int | 0 <= k && k < len(xs)}`; Phase 2 surface note on measures.

**Success checklist:**
- [ ] `len(xs)` usable in refinements as a measure (parse + encode + check path)
- [ ] Provably OOB index (e.g. literal `-1`) rejected at compile time
- [ ] Unbounded index forced through total `get -> Option` or explicit runtime assert (per SPEC)
- [ ] Smoke + at least one new focused test/example

### D. Proof-gated runtime check elision in interpreter (INV-1)

**SPEC:** DP6 + INV-1 — only elide runtime contract/refine checks when proved.

**Success checklist:**
- [ ] Interpreter skips runtime check **iff** obligation was discharged `[PROVED]`
- [ ] Unproved / runtime-checked / refuted paths still trap or reject safely (no silent skip)
- [ ] Differential test: OLD always checks → NEW elides only when proved
- [ ] Smoke green; no false elision on unsupported SMT fragments

### E. FixPatch JSON diagnostics (U15 / Kodo-shaped)

**SPEC:** DP8; CONF-P2 “JSON diagnostics with `FixPatch` emitted”; store/typecheck failure paths mention FixPatch.

**Success checklist:**
- [ ] At least one diagnostic emits machine-readable JSON including a `FixPatch` (or honest subset documented)
- [ ] Shape documented (fields) next to SPEC / pilot note
- [ ] Human CLI text still usable; JSON is additive
- [ ] Smoke green

## Working style Madis expects

1. Independent review of current Phase 1/2 code **first**; fold findings into A–E priority if they block soundness.
2. Surgical diffs; cite `file:line`; phase markers on non-trivial edits.
3. Ask before >30-line changes or new deps.
4. After every code change: `cargo test -p vera --lib` and `--prove examples/prove_clamp.vera` must stay green.
5. Prefer documenting honest limits over fake CONF-P2 claims.

## Definition of done for this handoff session (when Madis says stop)

Report: which of A–E landed, what remains, smoke evidence, any blockers (Z3 PATH, SPEC ambiguity, etc.).

---

End of paste-ready prompt.
