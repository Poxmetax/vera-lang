# GAP3-RENDER-PAREN slice — precedence-aware canonical render

**Date:** 2026-07-20 · **Marker:** `[GAP3-RENDER-PAREN]` · **File:** `crates/vera/src/render.rs`

## What landed (PHASE12 F5 fix)

`render_expr_prec` now actually uses precedence: `prec_of` mirrors the
parser's ladder (if/match/lambda 0 < `||` 1 < `&&` 2 < comparisons 3
(non-associative) < `+ - ++` 4 < `* / %` 5 < unary 6 < postfix 7 < atoms 8);
a child that binds weaker than its position is parenthesized.

Rules that preserve AST shape exactly:
- left-assoc binops: equal-prec LEFT child bare, equal-prec RIGHT child
  parenthesized (`a - (b - c)` survives; `a - b - c` stays flat);
- comparisons are non-associative (grammar has ONE optional rel_op), so BOTH
  cmp children get parens (`(1 < 2) == true`);
- unary operands must bind at postfix strength (`-(1 + 2)`, `-(-x)`);
- postfix (call callee / field obj / `?` operand) at postfix strength
  (`(1 + 2).show()`).

| Probe | Before | After |
|-------|--------|-------|
| `let a: Int = (1 + 2) * 3;` → `--round-trip` | FAIL exit 1 (F5) | `round-trip OK program#ac007536e094f7f7`, exit 0 |
| `1 + 2 * 3` rendered | `1 + 2 * 3` | unchanged (no redundant parens — text-pinned test) |
| all 13 examples | green | green (natural-precedence ASTs get zero new parens) |

Tests: `gap3_mixed_precedence_shapes_round_trip` (6 shapes that failed
before), `gap3_no_redundant_parens_on_natural_precedence` (hash identity
cannot catch over-parenthesization, so the text is pinned). Suite 44 -> 46.

## Honest limits

| Item | Status |
|------|--------|
| String literal Debug-escape rendering (PHASE12 **F6**) | Untouched — separate minor; `format!("{:?}")` remains the escape story. |
| `Expr::Block` as an operand | Unreachable from parsed programs (blocks are not primaries); rendered bare, undocumented-AST-input beware. |
| Redundant-paren detection beyond the pinned case | One representative text pin; hash identity guards correctness everywhere else. |
| F6 minors (`==` any-type, bare method access) | Out of scope (register/PHASE12). |

## Verify

```powershell
cargo test -p vera --lib          # 46+ (gap3_* 2 passed)
cargo run -p vera -- --round-trip <file with (1 + 2) * 3>   # round-trip OK
powershell -File docs\pilot\soft_smoke.ps1
```

Backup: `crates/vera/src/render.rs.bak_20260720_013616_gap3_paren`.
