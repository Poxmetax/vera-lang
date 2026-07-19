<!--
Operator: chat paste SHORT POINTER -- CLAUDE_POINTER_GAP5_IMPLEMENT.md
DESIGN NOTE (+ optional tiny keyed stub). Not durable cert DB unless Madis expands.
Not FixPatch E.
STATUS: DESIGNED/LANDED 23f2e46 [GAP5-INV2] -- typed key only; no durable store.
-->

# Fable 5 -- VERA GAP-5 handoff (INV-2 keying design)

**DESIGNED / LANDED** `23f2e46` `[GAP5-INV2]` · [`GAP5_INV2_DESIGN_NOTE.md`](GAP5_INV2_DESIGN_NOTE.md). Campaign ACK: [`CURSOR_SYNC_ACK_GAPS_BEFORE_E.md`](CURSOR_SYNC_ACK_GAPS_BEFORE_E.md). Baseline now **50**. FixPatch stays EPHEMERAL until INV-2 keys wired (GAP-D2). This file is historical implement brief.

Canonical brief for **GAP-5 only**. Pointer: [`CLAUDE_POINTER_GAP5_IMPLEMENT.md`](CLAUDE_POINTER_GAP5_IMPLEMENT.md).

---

You are continuing **VERA**. Madis is the operator. Close the **documentation / design** hole for SPEC **INV-2** (no stale results): every cache / memo / proof certificate keyed by **content hash + solver/model/toolchain version**.

## Why (KNOWN_GAPS)

D's `ProvedSet` is same-process only (fine). Persistent certificates / FixPatch apply / MCP must not paint into a corner with unversioned blobs. E handoff already forbids claiming durable patches without INV-2 -- this slice makes the scheme **explicit and written down**.

## Hard constraints

`vera-lang/` only; no mainnet; no commit unless asked; prefer **docs + optional tiny Rust type/stub**; ask before implementing a real persistent store; no renames; honest-limits; do **not** start FixPatch E apply-to-disk; soft_smoke still green if you touch code.

## SPEC anchor

- INV-2 (normative): key by content hash **plus** solver/model/toolchain version; one scheme shared by compiler query engine, runtime memoizer, proof cache.
- SPEC §6.4 caching / provenance notes.

## What YOU must do (default = design note; expand only if Madis asks)

1. Write `docs/pilot/GAP5_INV2_DESIGN_NOTE.md` covering:
   - Proposed key tuple (fields, examples)
   - What is in-scope today (same-process D set) vs future (durable proof / patch / MCP cache)
   - How FixPatch E JSON should stay ephemeral until keys exist
   - Compatibility / bump rules when toolchain or Z3 version changes
2. Update `KNOWN_GAPS.md` GAP-5: mark **DESIGNED** (or CLOSED-for-design) with path to the note; keep "implement durable store" as separate future row if needed
3. Optional thin code (only if fits <~30 lines and Madis-friendly): e.g. a `ToolchainId` / `ProofCacheKey` struct + unit test of key equality -- **no** on-disk cache required
4. Marker if code: `[GAP5-INV2]`

## Out of scope (unless Madis expands)

Persistent SQLite/file proof DB; Salsa; wiring all of U1 query engine; implementing E; changing D runtime elision semantics.

## Return (English short)

VERDICT DONE-GAP5-DESIGN | PARTIAL | BLOCKED; doc path; any code; next (E only when Madis green-lights after gaps).

End of GAP-5 handoff.
