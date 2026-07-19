# GAP4-R2-PILOT slice — label lattice thin pilot (executable evidence)

**Date:** 2026-07-20 · **Marker:** `[GAP4-R2-PILOT]` · **Files:** `crates/vera/src/label.rs` (new), `lib.rs` (module + export)

## What landed (handoff shape A: internal lattice module + tests)

First executable evidence for SPEC §4.2 / DP4 — the one novel type-system
concept. `label.rs` implements exactly the SPEC-normative semantics:

- `Atom` = `Auth(name)` | `Untrusted` | `Secret`; `Label` = finite atom set.
- Lattice: `join` = ∪, `meet` = ∩, `leq` = ⊆, `bottom` = ∅ ("lower is better").
- **(SUB-LABEL)** `flows_to(bound)` — a declared label is an upper bound;
  tests encode the E1 shape (∅-data sink rejects `untrusted` — injection) and
  the E6 shape (∅-data sink rejects `secret` — leak; a `secret`-typed sink
  accepts it).
- **(TAINT-PROP)** `taint_prop` joins DATA atoms only — an authority atom
  (capability handle) in the computation does not taint the result.

Tests (3): lattice laws (commutativity/idempotence/identity/order),
sink-bound accept/reject matrix, data-only taint propagation. Suite 46 -> 49.

## What this slice does and does NOT claim

| Claim | Status |
|-------|--------|
| "Zero implementation evidence" for the label concept | **Closed** — the lattice math + sink-bound mechanics run and pass. |
| R2 label-**inference ergonomics** gate (SPEC §4.2 inference stance) | **NOT claimed** — needs surface integration + corpus measurement; remains the open R2 risk. |
| CONF-P2 "ill-labeled flows rejected" (E1/E5/E6 in the checker) | **NOT claimed** — no parser/typecheck integration yet; `uses` clause remains MVP's only label surface. |
| Implicit flows | **NOT claimed** — SPEC's own [UNVERIFIED/OPEN] item, untouched. |
| R2 fallback (split auth/data checkers) | Not triggered — no ergonomics failure observed because ergonomics were not yet measured. |

## Verify

```powershell
cargo test -p vera --lib gap4_    # 3 passed
```

Backup (lib.rs): `crates/vera/src/lib.rs.bak_20260720_013616_gap4_label`.
