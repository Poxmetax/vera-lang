# Phase 2 VC slice — Z3 subprocess path

**Date:** 2026-07-19 · **Status:** thin end-to-end slice delivered (not full CONF-P2)  
**Binary:** `C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin\z3.exe` (v4.16.0)

## Approach

1. **Z3 integration:** SMT-LIB2 over **subprocess** to `z3.exe` (`crates/vera/src/smt.rs`). Discovery order: `VERA_Z3` → `z3` on PATH → sibling unpack under TradingBot. The `z3` Rust crate was **not** linked on this Windows host (avoids MSVC/DLL link friction); subprocess is the supported Phase 2 path.
2. **VC generation** (`crates/vera/src/vc.rs`): for each function, encode a pure Int/Bool/`ite` body, assume `requires` + param `{x:Int|pred}`, then ask Z3 whether `¬ensures` / `¬return-refine` is satisfiable. **unsat ⇒ proved**; encode failure / `unknown` ⇒ **runtime-checked**; **sat ⇒ refuted**.
3. **Call sites:** when arguments encode as closed SMT terms, discharge callee `requires` and arg refinements the same way.
4. **CLI:** `vera <file> --prove` prints `[PROVED]` / `[RUNTIME-CHECKED]` / `[REFUTED]` and a summary (exit 3 if any refuted).

## Example

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
cargo run -p vera -- --prove examples/prove_clamp.vera
```

Expected: ≥1 `[PROVED]` on `clamp` return refine and/or ensures; call-site `lo <= hi` for literal triples also proved.

## What this does **not** yet claim (remaining CONF-P2)

- `/` and `%` are **excluded** from the encodable fragment ([P2-SOUND1], independent review 2026-07-19): SMT-LIB `div`/`mod` are Euclidean while the interpreter truncates toward zero, so encoding them let `--prove` mark obligations PROVED that the runtime then trapped (`x / 2` at `x = -7`). They stay runtime-checked.
- Call-site discharge requires **closed literal arguments** ([P2-SOUND2], same review): an open (variable) argument previously reached Z3 as an undeclared/unconstrained symbol and produced spurious REFUTED; such call sites are now reported RUNTIME-CHECKED.
- **REQ-REFINE-1 call-site + closed definition-time:** [P2-REFINE1] / [P2-REFINE1-DEF] in typecheck.rs (see docs/pilot/P2_REFINE1_SLICE.md). Requires-guided / param-dependent bodies still soft.
- **Task B structured diagnostics: delivered** as `[P2B-DIAG]` — `--diag-json` CLI mode + documented entrypoints `diagnose_source` / `diagnose_program` (`crates/vera/src/diag.rs`); prove tiers (proved / runtime-checked / refuted) machine-readable with source spans; refuted ⇒ `severity: error`, exit 3; default text paths byte-identical (see [`P2B_DIAG_SLICE.md`](P2B_DIAG_SLICE.md)).
- Full SMT-backed refine reject for non-literal args (still REFUTED/RUNTIME via --prove only).
- **REQ-REFINE-2** / `len` measures on `List`.
- Label lattice / IFC, JSON `FixPatch`, multi-prover (CVC5), proof certificates / check elision in the interpreter.
- Linking the `z3` crate for in-process solving (optional later; subprocess is intentional).

## Verify

```powershell
cargo build -p vera
cargo test -p vera --lib
cargo run -p vera -- examples/hello.vera
cargo run -p vera -- --prove examples/prove_clamp.vera
```

## Spec anchors

- SPEC.md §4.4 obligation flow + REQ-REFINE (Phase 2 gate)
- Pilot spike: `docs/pilot/smt_refine_spike.py` / `SMT_SPIKE_REPORT.md`
