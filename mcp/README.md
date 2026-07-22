# VERA MCP compiler service (Phase 3 stub)

**Status:** server stub only -- no server implementation yet.
**Date:** 2026-07-19 (seam note 2026-07-22)
**Marker:** `[SOFT-MCP-STUB]`

> **Persistence seam landed (in-crate).**
> A thin store-facing seam exists in the `vera` crate -- `vera::mcp`
> (`mcp_get_cert` + `McpWriteSession`): an INV-2-keyed, advisory-lock-correct
> primitive a future MCP server binds to for durable proof-certificate
> persistence. This is NOT the server -- no JSON-RPC, no protocol loop, no
> tool schemas (those stay CONF-P3, below). Cert verdicts only; live FixPatch
> stays `ephemeral: true`. Implementation: `crates/vera/src/mcp.rs`.

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

Names below are **provisional**. Final schemas ship with CONF-P3.

| Tool (provisional) | Maps to today | Returns (goal) |
|--------------------|---------------|----------------|
| `vera.typecheck` | `check_program` / CLI typecheck path | structured OK / `TypeError` JSON |
| `vera.prove` | `prove_program` / `vera --prove` | per-obligation `proved` / `runtime-checked` / `refuted` (+ exit semantics) |
| `vera.round_trip` | `CodebaseStore::round_trip_ok` / `--round-trip` | content-hash identity check |
| `vera.edit_tx` | `CodebaseStore::apply(EditTransaction)` | commit result or `StaleBase` / `Type` conflict (+ later `FixPatch`) |
| `vera.hash` | store load / `--hash-only` | definition hashes + namespace summary |

Out of early MCP scope (later phases):

- REQ-REFINE-1/2 hard rejects, check-elision (INV-1), FixPatch JSON
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

- No Rust/Python MCP server code in this directory yet
- No new Cargo dependencies for an MCP server here
- No second codebase store

## When to implement

Only after an explicit Phase 3 / CONF-P3 track opens (or a default-off scaffold
for an MCP server). This stub is docs-only until then.

## Pointers

- Spec: [`docs/spec/SPEC.md`](../docs/spec/SPEC.md) -- DP8, §6 store, §10 CONF-P3
- Seam: [`crates/vera/src/mcp.rs`](../crates/vera/src/mcp.rs)
- CLI prove demos: [`examples/README.md`](../examples/README.md)
