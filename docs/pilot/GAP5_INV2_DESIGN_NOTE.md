# GAP5 — INV-2 keying design note (no durable store yet)

**Date:** 2026-07-20 · **Marker (code):** `[GAP5-INV2]` · **Code:** `crates/vera/src/vc.rs` (`ToolchainId`, `ProofCacheKey` + 1 test) — types only, **no on-disk cache**.

## The rule (SPEC INV-2, normative)

Every cache, memo table, incremental result, and proof certificate is keyed by
**content hash plus solver/model/toolchain version**. One keying scheme shared
by the compiler query engine, the runtime memoizer, and the proof cache. A
cache not keyed this way is a conformance violation.

## Proposed key tuple

```
ProofCacheKey {
    definition_hash: String,   // content-addressed store hash of the definition
    query_kind:      String,   // e.g. "typecheck", "prove/ensures[0]",
                               //      "prove/return_refine", "fixpatch"
    toolchain: ToolchainId {
        vera_version: String,  // CARGO_PKG_VERSION at result time, e.g. "0.1.0"
        solver_id:    String,  // solver + version, e.g. "z3-4.16.0";
                               // "none" for solver-free queries
    },
}
```

Example: clamp's proved ensures[0] under today's toolchain keys as
`("<clamp store hash>", "prove/ensures[0]", {"0.1.0", "z3-4.16.0"})`.

Unit test `gap5_inv2_key_distinguishes_toolchains` pins the semantics: same
fields = same key; a solver bump or different definition = different key.

## Scope today vs future

| Surface | Status |
|---------|--------|
| D's `ProvedSet` (run-after-prove elision) | **In-scope-correct without keys**: built from the same `Program` AST in the same process, dies with the process. No staleness path exists — this is why D did not need INV-2 plumbing. |
| Durable proof certificates (GAP-D2) | Future: MUST use `ProofCacheKey`; a lookup with a non-matching `ToolchainId` is a MISS (re-prove), never a hit. |
| FixPatch E JSON | E's patches must stay **ephemeral** (produced, applied-or-discarded within a run/review cycle) until keys exist. A persisted FixPatch would need the same key + the *target file content hash* it was computed against, else a stale patch could be applied to drifted code. |
| MCP compiler-service results | Same rule when caching across requests. |
| Query-engine memoization (U1/Salsa-class) | Same scheme when it arrives (Phase 2+); one scheme, three consumers. |

## Bump / compatibility rules

1. `vera_version` changes (any compiler behavior change) -> all cached
   verdicts MISS. No attempt at fine-grained compatibility in v0.1.
2. `solver_id` changes (Z3 upgrade, alternate prover) -> all *prove* verdicts
   MISS; solver-free query kinds (`typecheck`) keep `solver_id: "none"` and
   survive solver bumps.
3. Key equality is exact-string; there are no version ranges. Simplicity over
   cleverness until measurements justify more.
4. Renaming a definition does NOT invalidate (names are metadata; the hash is
   of the canonical AST — SPEC §6.1).

## Honest limits

- No durable store is implemented (explicitly out of scope; Madis gates it).
- `ToolchainId::current` takes the solver id as an argument; wiring actual Z3
  version discovery (`z3 --version`) into it lands with the first durable
  consumer, not now.
- OS/arch are NOT in the key (interpreter semantics are platform-independent
  by design; revisit for native backends in Phase 4).

## KNOWN_GAPS

GAP-5 -> **DESIGNED** (this note + typed key with test). "Implement durable
store" remains GAP-D2 (unchanged row).
