# Web pastes — VeraAgentBench v0.1 (Grok + Gemini)

**Repo:** https://github.com/Poxmetax/vera-lang  
**Shared rules:** [`CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md`](CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md)  
**Codespace path:** [`VERA_AGENT_BENCH_CODESPACE.md`](VERA_AGENT_BENCH_CODESPACE.md)  
**Decision:** [`VERA_AGENT_BENCH_V01_DECISION.md`](VERA_AGENT_BENCH_V01_DECISION.md)  
**Fable soft ACK (local only; web agents ignore):** [`CURSOR_SYNC_ACK_VERA_AGENT_BENCH_V01_FABLE_TRIAL.md`](CURSOR_SYNC_ACK_VERA_AGENT_BENCH_V01_FABLE_TRIAL.md)

Copy **one** block into the web chat. Agents have **no** access to Madis’s PC.

**Prefer §§3–4 (Codespace variants).** §§1–2 are legacy clone-first pastes kept for Mode C hosts that already have network+shell.

---

## Honest constraint (operator reminder)

| Mode | What it is |
|------|------------|
| **(A)** | Agent opens/controls a Codespace with network |
| **(B)** | Madis opens Codespace; pastes terminal I/O to the agent |
| **(C)** | Agent has network elsewhere (clone or curl binary) |

Free chat sandboxes without A/B/C → expect SKIP(env). Do not pretend they gain a shell.

Codespace URL: `https://codespaces.new/Poxmetax/vera-lang?quickstart=1`

---

## 1. Paste for Grok (web) — legacy clone-first (Mode C only)

```text
You are the agent under test for VeraAgentBench v0.1 — NOT a language implementer.

PUBLIC REPO (your only SoT — you cannot see my local disk):
  https://github.com/Poxmetax/vera-lang

HONEST HOST RULE: Use Mode C only if you truly have network + shell.
If you cannot clone/exec: mark all tasks SKIP(env) and STOP. Never invent PASS.
Prefer Codespace Mode A/B — see docs/pilot/VERA_AGENT_BENCH_CODESPACE.md on main.

1) Clone or browse that repo on branch main.
2) Open bench/vera_agent_bench_v01/ (README + task_T0N_* folders).
3) Optional context: docs/pilot/VERA_AGENT_BENCH_V01_DECISION.md and
   docs/pilot/CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md

Setup (minimal):
  git clone https://github.com/Poxmetax/vera-lang.git && cd vera-lang
  cargo run -p vera -- <file.vera>
  cargo run -p vera -- --prove <file>          # needs Z3 on PATH
  cargo run -p vera -- --diag-json <file>
  cargo run -p vera -- --round-trip <file>
If build/Z3 fails on free tier: SKIP prove tasks honestly; do not invent PASS.

Rules:
- Only vera CLI flags above. No .rs language expansion. No IFC / GAP-D2 / durable FixPatch / hole-synthesis claims.
- First batch: T01 → T06 in order. T07 if time. T08 optional (manual hole fill ≠ synthesis).
- Prefer checks/*.ps1 if you have PowerShell; else mirror exit-code + substring oracles from those scripts.
- Rate limits / tool failure → STOP and report partial results. Never fake PASS.

Return to me in chat a RESULTS markdown table with columns Task | Verdict | Notes
(T01–T08), Mode letter, plus an overclaim scan checklist. Then STOP for my review.
```

---

## 2. Paste for Gemini (web) — legacy clone-first (Mode C only)

```text
Role: agent under test for VeraAgentBench v0.1 (CLI probe). You are not implementing VERA language features.

Your only source of truth is the public GitHub repository:
  https://github.com/Poxmetax/vera-lang
(You do not have access to any private local machine.)

HONEST HOST RULE: Mode C only if you have real network + exec.
Otherwise SKIP(env) all tasks and stop. Prefer Codespace (Mode A/B):
  docs/pilot/VERA_AGENT_BENCH_CODESPACE.md

Steps:
1. Clone or browse https://github.com/Poxmetax/vera-lang (main).
2. Work from bench/vera_agent_bench_v01/ — each task has prompt.md, initial/, checks/, meta.json.
3. Read docs/pilot/CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md if present; follow those hard rules.

How to run VERA after clone:
  cargo run -p vera -- <path>
  cargo run -p vera -- --prove <path>       # Z3 required
  cargo run -p vera -- --diag-json <path>
  cargo run -p vera -- --round-trip <path>
Free/cheap tier: if cargo or Z3 is unavailable, mark prove tasks SKIP (env) and continue what you can. Do not invent PASS.

Constraints:
- Allowed: vera CLI only (run / --prove / --diag-json / --round-trip).
- Forbidden: expanding the language via .rs edits; claiming IFC, full labels, GAP-D2, durable FixPatch, or hole synthesis.
- Order: T01 through T06 first. T07 optional if time. T08 optional (filling ?body by edit is OK; do not call it synthesis).
- Honesty: if rate-limited or stuck, stop and report partial results.

Deliverable: paste a markdown RESULTS table (Task | Verdict | Notes for T01–T08), Mode letter, and an overclaim scan back in this chat, then stop for operator review.
```

---

## 3. Paste for Grok (web) — Codespace-first (preferred)

```text
You are the agent under test for VeraAgentBench v0.1 — NOT a language implementer.

PUBLIC REPO: https://github.com/Poxmetax/vera-lang
CODESPACE DOCS: docs/pilot/VERA_AGENT_BENCH_CODESPACE.md
POINTER: docs/pilot/CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md

HONEST MODES (pick one; say which in RESULTS):
  (A) You can open/control a GitHub Codespace with network for this repo
  (B) I (operator) opened Codespace; I will paste terminal stdout/stderr/exit codes when you ask
  (C) You have network+exec elsewhere (clone or curl release binary vera-linux-x86_64)
If none of A/B/C: mark ALL tasks SKIP(env) and STOP. Do NOT pretend this chat sandbox has a shell.

Codespace URL pattern (operator may already have one open):
  https://codespaces.new/Poxmetax/vera-lang?quickstart=1
After create: Rust + z3 + cargo build -p vera (postCreateCommand).

Run from repo root:
  ./target/debug/vera <file.vera>
  ./target/debug/vera --prove <file>       # needs z3
  ./target/debug/vera --diag-json <file>
  ./target/debug/vera --round-trip <file>
Mode B: propose ONE command at a time; wait for my paste; score from real exit codes only.

Bench: bench/vera_agent_bench_v01/ — T01→T06 first; T07 if time; T08 optional.
Rules: only those CLI flags; no .rs language expansion; no IFC / GAP-D2 / durable FixPatch / hole-synthesis claims.
Never invent PASS.

Return RESULTS markdown: Mode | Task | Verdict | Notes (T01–T08) + overclaim scan. Then STOP.
```

---

## 4. Paste for Gemini (web) — Codespace-first (preferred)

```text
Role: agent under test for VeraAgentBench v0.1. You do NOT implement VERA language features.

SoT: https://github.com/Poxmetax/vera-lang
Read if reachable: docs/pilot/VERA_AGENT_BENCH_CODESPACE.md and
  docs/pilot/CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md

HOST MODES — state which you are using:
  (A) You control a GitHub Codespace for Poxmetax/vera-lang (network + shell)
  (B) Operator Codespace: you propose commands; operator pastes terminal I/O back
  (C) Other host with network (git clone or download release asset vera-linux-x86_64)
If not A/B/C: SKIP(env) every task and stop. Free chat without shell/network is not a CLI host.

Codespace: https://codespaces.new/Poxmetax/vera-lang?quickstart=1
Expected tools after create: z3 on PATH, ./target/debug/vera built.

Commands (repo root):
  ./target/debug/vera <path>
  ./target/debug/vera --prove <path>
  ./target/debug/vera --diag-json <path>
  ./target/debug/vera --round-trip <path>
In Mode B, one command per turn; verdicts only from operator-pasted exit codes / substrings.

Tasks: bench/vera_agent_bench_v01/ in order T01–T06; T07 optional; T08 optional (edit fill ≠ synthesis).
Forbidden: .rs feature work; IFC / full labels / GAP-D2 / durable FixPatch / hole-synthesis claims.
Honesty: no fake PASS.

Deliverable: RESULTS table (Mode, Task, Verdict, Notes for T01–T08) + overclaim checklist, then stop.
```

---

## 5. Short paste for Madis (Estonian-ready steps)

Use this yourself, then give agents §3 or §4:

```text
1) Ava https://github.com/Poxmetax/vera-lang → Code → Codespaces → Create on main
   (või https://codespaces.new/Poxmetax/vera-lang?quickstart=1).
2) Oota postCreate (Rust build + z3). Smoke: ./target/debug/vera --help
3) Ütle agentidele Mode B: sina jooksutad käsud Codespace’is ja kleebid terminali väljundi chatti.
4) Kleebi Grokile §3 või Geminiile §4 failist docs/pilot/PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md
5) Kui agent ütleb SKIP(env) ilma A/B/C-ta — see on aus, mitte VERA viga.
```
