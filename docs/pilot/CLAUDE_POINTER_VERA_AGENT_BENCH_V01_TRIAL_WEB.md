# Claude/web pointer — VeraAgentBench v0.1 trial (GitHub / Codespace)

Read and follow this file. Do not invent scope from memory.

**Public repo (only SoT you can see):** https://github.com/Poxmetax/vera-lang  

You are the **agent under test**, not a language-feature implementer.  
You **cannot** see Madis’s local disk.

**Codespace / binary path (preferred for web trials):**  
[`docs/pilot/VERA_AGENT_BENCH_CODESPACE.md`](VERA_AGENT_BENCH_CODESPACE.md)

Bench root on `main`:

```text
bench/vera_agent_bench_v01/
```

Decision / rules (same as Fable trial):  
`docs/pilot/VERA_AGENT_BENCH_V01_DECISION.md`  
Ready pastes: `docs/pilot/PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md`

---

## Honest host constraint (read first)

Free web chat **does not** automatically get a shell or network. Prior trials: Grok sandbox internet disabled; Gemini network fetch restricted.

You may run this bench **only** in one of these modes (document which):

| Mode | Meaning |
|------|---------|
| **(A)** You can open/control a GitHub Codespace (or equivalent) with network |
| **(B)** Madis opened a Codespace and pastes terminal I/O to you (human-in-the-loop) |
| **(C)** You have network in another product mode (clone or curl release binary) |

If none of A/B/C → mark tasks **SKIP(env)** and STOP. Never invent PASS.  
Details: [`VERA_AGENT_BENCH_CODESPACE.md`](VERA_AGENT_BENCH_CODESPACE.md).

---

## Hard rules

1. Prefer **Codespace** (Mode A/B) over hoping the chat sandbox can `git clone`. Fallback: clone or release binary only if Mode C is real.
2. Attempt tasks **in order**: T01 → T06 first batch; T07 if time; T08 optional.
3. Use **only** vera CLI flags: run / `--prove` / `--diag-json` / `--round-trip`.
4. Prefer task `checks/*.ps1` when PowerShell is available; otherwise mirror the same exit-code + substring oracles with `./target/debug/vera …` or `cargo run -p vera -- …`.
5. **NOT** expand the language (no `.rs` feature work; no PR claiming new language features).
6. **NOT** claim IFC / full labels / GAP-D2 / durable FixPatch / hole synthesis.
7. **Honest results only.** Rate-limits / no shell / build fails → **STOP** and report **partial** / SKIP(env) — never invent PASS.
8. Return a RESULTS markdown **table in chat** to Madis (same columns as Fable). You may not be able to push files to his repo.

## Install / run (Codespace-first)

**Codespace URL pattern:**

```text
https://codespaces.new/Poxmetax/vera-lang?quickstart=1
```

After create, `postCreateCommand` installs Z3 and runs `cargo build -p vera`. Then:

```bash
cd /workspaces/vera-lang   # or the Codespace repo root
./target/debug/vera <path-to.vera>
./target/debug/vera --prove <path>          # needs z3 on PATH
./target/debug/vera --diag-json <path>
./target/debug/vera --round-trip <path>
# equivalent: cargo run -p vera -- …
```

**Mode C fallback** (only if you truly have network + exec):

```bash
git clone https://github.com/Poxmetax/vera-lang.git && cd vera-lang
# Rust stable + Z3 required for prove tasks
cargo build -p vera
# OR download vera-linux-x86_64 from a GitHub Release / Actions artifact when available
```

If you have **no** Codespace control, **no** Madis terminal paste loop, and **no** network/exec → SKIP(env) all tasks. Do not pretend.

## Output format (paste back to Madis)

```markdown
# VeraAgentBench v0.1 — trial results (<Agent>)

**Agent:** <Grok web | Gemini web>
**Date:** YYYY-MM-DD
**Mode:** A (agent Codespace) | B (Madis Codespace HITL) | C (network/binary) | none → SKIP(env)
**Source:** https://github.com/Poxmetax/vera-lang (Codespace / clone / binary)
**Rule:** honest PASS/FAIL/SKIP only; no language expansion claims

| Task | Verdict | Notes |
|------|---------|-------|
| T01 | | |
| T02 | | |
| T03 | | |
| T04 | | |
| T05 | | |
| T06 | | |
| T07 | | |
| T08 (opt) | SKIP / | |

## Overclaim scan
- [ ] Did not claim IFC / full labels
- [ ] Did not open GAP-D2
- [ ] Did not expand language features
- [ ] Used only vera CLI flags listed above
- [ ] Did not pretend a chat sandbox had a shell

## Stop
First batch complete or blocked → STOP for Madis review.
```
