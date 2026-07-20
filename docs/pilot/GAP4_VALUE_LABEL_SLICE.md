# GAP4-VALUE-LABEL slice — minimal `T^{...}` value-label syntax feeding the seeded surface

**Date:** 2026-07-20 · **Marker:** `[GAP4-VALUE-LABEL]` · **Files:** `lexer.rs` (+1 op char), `ast.rs` (2 fields), `parser.rs` (helper + 2 hooks), `render.rs` (helper + 2 sites), `typecheck.rs` (seed harvest + wiring + tests) — diff +361/−15 across the five; `label.rs` untouched

## What landed (the surface slice's named "natural next fragment")

A `.vera` author can now produce a non-⊥ label from PLAIN SOURCE — the
GAP4-R2-SURFACE honest limit "seeds are test/API only" is closed:

```vera
fn store_row2(row: Str) -> Unit { row; }
fn main(console: Console) -> Unit uses {console} {
    let user_input: Str^{untrusted} = "row";
    store_row2(user_input);        // TYPE-ERROR [GAP4-R2-SURFACE] (E1)
}
```

1. **Syntax:** postfix `^{atom, ...}` after the annotation type at exactly two
   positions — fn params (`a: Str^{secret}`) and explicitly-annotated lets
   (`let t: Str^{secret} = ...`). Atoms this slice: the two DATA atoms
   `untrusted` / `secret` (authority stays on the `uses` clause). Unknown
   atoms and empty sets are parse errors. `^` now lexes as a plain op char.
2. **AST:** `Param.label` / `Stmt::Let.label` as `Vec<String>`, canonicalized
   (sorted + deduped) at parse; `#[serde(skip_serializing_if)]` keeps every
   unlabeled node's serialization — and therefore every existing store hash —
   byte-identical.
3. **Render:** labels render back (`Str^{secret, untrusted}` canonical order),
   so parse -> render -> parse is canonical-AST identical (pinned by test).
4. **Checker:** `collect_label_seeds` harvests `(fn, binding) -> Label` from
   annotations — params and arbitrarily NESTED lets (an annotation is never
   silently ignored) — and `check_program` feeds the EXISTING
   `[GAP4-R2-SURFACE]` pass with it. The walker and both enforcement points
   (named-fn param bound, `Console.print` ∅-data bound) are unchanged;
   annotation-free programs harvest an empty map (inert as before, lattice
   law + full suite).

## What this slice does and does NOT claim

| Claim | Status |
|-------|--------|
| "Seeds are test/API only" (surface honest limit) | **Closed** — E1 reject + E6 reject + secret-bound accept all demonstrable from plain source, no seeds. |
| CONF-P2 "ill-labeled flows rejected" — explicit-flow checker evidence on real source | **Advanced** (E1/E6 shapes) — still NOT the full CONF-P2 label gate (see next row). |
| R2 label-INFERENCE ergonomics gate | **NOT claimed / still OPEN** — labels here are fully explicit annotations; no inference, no corpus measurement (separate Madis-gated probe). |
| Full IFC / taint propagation / implicit flows | **NOT claimed** — one-hop Name-argument rule unchanged; `let y = x;` still drops the label (documented surface limit). |
| Label syntax beyond param/let (return types, nested `List<Str^{...}>`, lambda params) | **NOT claimed** — parse error there; smallest-position choice, document-before-widen. |
| Auth atoms in `^{...}` | **NOT claimed** — `uses` remains the authority surface. |
| Endorse / declassify / policies / quarantine / `infer` | **NOT touched.** |
| GAP-C2 / GAP-D2 / FixPatch | **NOT touched** — FixPatch stays `ephemeral: true`. |

## Honest limits

- **Labels are declarations, not inferences:** the binding's annotation IS its
  label; binding a labeled value to an unannotated name loses the label
  (one-hop rule). This is the R2-relevant ergonomics cost the OPEN gate will
  eventually measure — not hidden, not claimed solved.
- Value-position only at bindings; there is no `T^{...}` inside type
  constructors, returns, or lambda params (parse error — reserved, honest).
- The lexed `^` is not an operator anywhere else (expression use = parse
  error, unchanged behavior).
- No committed example uses labels yet (reject demos are unit tests per the
  `round_trip_all_examples` invariant); the accept shape is test-pinned.

## Tests (+5, suite 63 -> 68)

- `typecheck::tests::gap4vl_rejects_untrusted_let_arg_from_plain_source` —
  the milestone E1 reject with zero seeds.
- `typecheck::tests::gap4vl_secret_bound_param_accepts_and_console_print_rejects`
  — annotation-only accept/reject pair (secret bound + E6 leak).
- `typecheck::tests::gap4vl_nested_let_label_is_collected` — nested-block
  annotation is harvested, not silently dropped.
- `typecheck::tests::gap4vl_label_renders_and_reparses_identically` —
  canonical render round-trip + unlabeled nodes serialize with no `label` key
  (hash stability).
- `typecheck::tests::gap4vl_unknown_and_empty_label_atoms_are_parse_errors` —
  closed vocabulary pinned at the parse gate.

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib            # 68 passed (was 63)
cargo test -p vera --lib gap4vl_    # 5 passed
powershell -File docs\pilot\soft_smoke.ps1                 # SOFT-SMOKE PASS
cargo run -p vera -- --prove examples/prove_clamp.vera     # 6 proved (unchanged)
```

Backups: `crates/vera/src/{ast,lexer,parser,render,typecheck}.rs.bak_20260720_053453_gap4_value_label`.
