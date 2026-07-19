# P2-DUPFN micro-slice — reject duplicate function names

**Date:** 2026-07-20 · **Marker:** `[P2-DUPFN]` · **Files:** `crates/vera/src/typecheck.rs` (+ one interp test rewrite)

## Why now (GAP-1 from the D-review backlog)

The typechecker rejected duplicate *type* names but silently accepted duplicate
*function* names; every name-keyed map (interpreter fns, vc callee lookup)
resolved to the **last** declaration, so an earlier `fn f` was shadowed with no
diagnostic. This poisons name-keyed reasoning — the `[P2D-ELIDE]` proved set
already had to defend against it — and every new program written on top of the
ambiguity would have made a later reject a breaking change. Cheapest moment to
close it is now (operator-approved micro-slice, 2026-07-20).

## What landed

- `check_program` builds the fn map with an explicit duplicate check, mirroring
  the existing duplicate-type style: second declaration -> compile-time
  `TypeError` at its span, message `[P2-DUPFN] duplicate function <name>`.
- New unit test `dupfn_rejects_duplicate_function_names`.
- `elide_excludes_duplicate_fn_names` rewritten to **skip typecheck on
  purpose**: it now asserts the front-door reject AND still exercises the
  `ProvedSet` duplicate-exclusion as defense-in-depth for API callers that go
  straight to `prove_program` / `Interpreter` (neither requires
  `check_program`). The guard stays — belt and suspenders.

| Case | Result |
|------|--------|
| `fn f ... fn f ... main` (CLI) | `error: 4:1: [P2-DUPFN] duplicate function f`, exit 1 |
| Same program via API without typecheck | ProvedSet still excludes `f`; interp runs last-wins (unchanged) |
| All examples / prior suite | unaffected (no duplicates existed) |

## Honest limits

- Function vs struct/enum name collision needs no check: fn names are lowercase
  idents, type names uppercase TypeIdents — lexically disjoint namespaces.
- API paths (`prove_program`, `Interpreter::new/with_proved`) intentionally do
  not force typecheck; their dup-fn behavior is unchanged (last wins + proved
  set exclusion). The front door for programs is `check_program` / CLI, which
  now rejects.

## Verify

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo test -p vera --lib          # 35 passed (was 34)
cargo test -p vera --lib dupfn    # 1 passed
powershell -File docs\pilot\soft_smoke.ps1   # SOFT-SMOKE PASS; prove_clamp 6 proved
```

Backups: `crates/vera/src/{typecheck,interp}.rs.bak_20260720_005810_p2_dupfn`.
