# VERA — Social posts (copy only — do not auto-post)

Ready-to-paste drafts. Tone: research-engineering, not startup hype.  
Repo: https://github.com/Poxmetax/vera-lang · Apache-2.0 · research prototype

---

## Twitter / X (short)

### A — Launch / discoverability

```
VERA: an AI-native research language (Rust toolchain, Apache-2.0).

Typed holes, refinements, thin Z3 VCs, ephemeral FixPatch.
Research prototype — not full IFC, not durable certs.

Easy to write. Hard to ship silent wrongness.

https://github.com/Poxmetax/vera-lang
```

### B — Technical hook

```
What exists today in VERA (honest):

• typed holes (?ident) — unfilled = error
• refinements + optional `vera --prove` via Z3
• ephemeral FixPatch on non-exhaustive match
• label lattice ≠ full IFC

Research prototype. Apache-2.0.
github.com/Poxmetax/vera-lang
```

### C — One-liner + CTA

```
Building toward a language LLMs can write with fewer silent bugs:
low ceremony surface, machine-checked substrate.

VERA — research prototype.
https://github.com/Poxmetax/vera-lang
```

---

## LinkedIn (medium)

### A — Project introduction

```
Open-sourced VERA — an AI-native research programming language.

Goal (one line): the language an LLM writes with the fewest shipped bugs — easy to write, hard to ship silent wrongness.

What ships today (honest scope):
• Static types (no null), refinements/contracts, typed holes
• Thin Z3 verification-condition path (`--prove` / `--prove-run`; Z3 optional on PATH)
• Content-addressed store + typed edit transactions
• Ephemeral FixPatch on structured diagnostics (not a durable proof cache)
• Label lattice pilot + thin seeded checker surface (E1/E6) — not full IFC; no label syntax or inference

This is a research prototype with a Rust reference toolchain, Apache-2.0.

If you work on LLM codegen, SMT-backed checking, or language design for agents, feedback and issues are welcome.

Repo: https://github.com/Poxmetax/vera-lang
```

### B — Engineering audience

```
VERA (Verified · Effect-typed · Reproducible · Agent-native) is a research-stage language aimed at agent-written programs.

Interesting surface area for reviewers:
1. Refinement types that check at runtime and can discharge VCs via Z3
2. Typed holes as first-class parse/type errors (synthesis deferred)
3. FixPatch JSON that is explicitly ephemeral — machine-visible honesty over marketing certs

Not claiming: production readiness, full IFC, or durable certificates.

Apache-2.0 · https://github.com/Poxmetax/vera-lang
```

---

## Reddit — r/rust / Rust Users Forum (longer, honest)

**Title options:**
- `VERA — AI-native research language in Rust (typed holes, refinements, thin Z3 VCs)`
- `Show: VERA research prototype — Rust toolchain, Apache-2.0, claim-honest scope`

**Body:**

```
Hi — sharing VERA, an AI-native *research* programming language with a Rust reference toolchain (Apache-2.0).

**One-line goal:** a low-ceremony surface over a machine-checked substrate — intended for studying LLM-written programs with fewer silent wrongness paths.

**What exists today (no hype):**
| Pillar | Honest status |
|--------|----------------|
| Typed holes | `?ident` parses; unfilled → type/runtime error (synthesis later) |
| Refinements + prove | Runtime checks + `vera --prove` / `--prove-run` via Z3 SMT-LIB2 subprocess |
| Content-addressed store | Parse → hash → render → parse round-trip; typed edit transactions |
| FixPatch | Ephemeral diagnostic/fix patch (`ephemeral: true`) — **not** a durable proof cache |
| Label lattice | Lattice pilot + thin seeded checker surface (E1/E6) — **not** full IFC; no label syntax or inference |

**Quick start** (needs Rust stable; Z3 on PATH only for `--prove`):

```bash
git clone https://github.com/Poxmetax/vera-lang.git
cd vera-lang
cargo run -p vera -- examples/hello.vera
cargo run -p vera -- --prove examples/prove_clamp.vera
cargo test -p vera
```

Happy to answer technical questions. Please treat “pilot” language carefully — gaps and limits are documented in `docs/pilot/KNOWN_GAPS.md`.

Repo: https://github.com/Poxmetax/vera-lang
```

---

## Claim checklist (before posting)

- [ ] Said “research prototype”
- [ ] Did not claim full IFC
- [ ] Did not claim durable FixPatch / durable certs
- [ ] Z3 described as optional for `--prove`
- [ ] Linked to GitHub, not to closed demos only
