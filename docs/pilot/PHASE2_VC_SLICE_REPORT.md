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

- Full **REQ-REFINE-1** compile-time reject of `apply_discount(100, 150)` as a hard type error (slice reports REFUTED/RUNTIME; no typecheck integration yet).
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
