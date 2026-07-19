# VERA MCP compiler service (Phase 3 stub)

**Status:** stub only -- no server implementation yet.
**Date:** 2026-07-19
**Marker:** `[SOFT-MCP-STUB]`

This directory is reserved for the future **Model Context Protocol (MCP)** surface
that exposes the VERA compiler as a service (not only as the `vera` CLI).

## Why this exists

SPEC design principle **DP8** (*Machine-readable everything*):

> Diagnostics are structured JSON with machine-applicable `FixPatch`/`RepairPlan`;
> the compiler is a service (MCP) as well as a CLI; the codebase is a queryable
> store, not text files. *(-> §6, Phase 3)*

Conformance gate **CONF-P3** (SPEC §10) includes:

> the MCP compiler-service answers **typecheck** / **prove**

Until Phase 3 lands, agents use the CLI (`cargo run -p vera -- ...`) and the
in-process Rust API (`vera` crate). This README is the ownership stake so the
layout advertised in the root README is real.

## Planned tool surface (informative; not implemented)

Names below are **provisional**. Final schemas ship with CONF-P3; do not invent
server code here while Fable owns CONF-P2 hard work.

| Tool (provisional) | Maps to today | Returns (goal) |
|--------------------|---------------|----------------|
| `vera.typecheck` | `check_program` / CLI typecheck path | structured OK / `TypeError` JSON |
| `vera.prove` | `prove_program` / `vera --prove` | per-obligation `proved` / `runtime-checked` / `refuted` (+ exit semantics) |
| `vera.round_trip` | `CodebaseStore::round_trip_ok` / `--round-trip` | content-hash identity check |
| `vera.edit_tx` | `CodebaseStore::apply(EditTransaction)` | commit result or `StaleBase` / `Type` conflict (+ later `FixPatch`) |
| `vera.hash` | store load / `--hash-only` | definition hashes + namespace summary |

Out of early MCP scope (stay on hard CONF-P2 / later phases):

- REQ-REFINE-1/2 hard rejects, check-elision (INV-1), FixPatch JSON (Fable CONF-P2)
- `infer` / actors / policy / fuel (CONF-P3 agentic remainder)
- schema absorption of *other* MCP APIs into VERA (`use schema::mcp(...)` -- SPEC §7.6)

## Relationship to the CLI

| Concern | CLI (`crates/vera`) | MCP (this dir, future) |
|---------|---------------------|-------------------------|
| Authoring / CI scripts | primary | optional wrap |
| Agent edit->verify loop | possible via subprocess | **intended** primary (DP8) |
| Store truth | content-addressed defs | same store; MCP must not invent a second codebase |

Invariant (SPEC §6.2): a committed codebase always typechecks. Any MCP
`edit_tx` tool must preserve that -- same gate as the in-process API.

## Non-goals for this stub

- No Rust/Python MCP server code in this PR slice
- No new Cargo dependencies
- No edits to Fable-owned modules (`vc.rs`, `smt.rs`, `typecheck.rs`, `interp.rs`)
- No renames of existing files

## When to implement

Only after Madis opens a Phase 3 / CONF-P3 track (or an explicit soft item that
scaffolds an MCP server behind a default-off toggle). Soft parallel queue for
CONF-P2 polish is exhausted; this stub is docs-only safe-lane work.

## Pointers

- Spec: [`docs/spec/SPEC.md`](../docs/spec/SPEC.md) -- DP8, §6 store, §10 CONF-P3
- Soft ownership: [`docs/pilot/SOFT_PARALLEL_QUEUE.md`](../docs/pilot/SOFT_PARALLEL_QUEUE.md)
- Commit gate: [`docs/pilot/COMMIT_CHECKLIST.md`](../docs/pilot/COMMIT_CHECKLIST.md)
- CLI prove demos: [`examples/README.md`](../examples/README.md)