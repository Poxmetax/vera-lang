# VeraAgentBench v0.1 — Codespace / release-binary path (honest)

**Repo:** https://github.com/Poxmetax/vera-lang  
**Bench:** `bench/vera_agent_bench_v01/`  
**Dev container:** [`.devcontainer/devcontainer.json`](../../.devcontainer/devcontainer.json)  
**Release workflow:** [`.github/workflows/release-cli.yml`](../../.github/workflows/release-cli.yml)  
**Web pointer:** [`CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md`](CLAUDE_POINTER_VERA_AGENT_BENCH_V01_TRIAL_WEB.md)  
**Pastes:** [`PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md`](PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md)

This document exists because **free web chat sandboxes do not magically gain a shell**. Grok (prior trial) reported sandbox internet disabled (cannot clone). Gemini reported network fetch restricted. A Codespace or a downloaded `vera` binary only helps if the agent can actually **reach** that environment.

---

## Honest constraint — three modes (pick one; label it)

| Mode | Who has the shell / network | When it works | When it fails |
|------|----------------------------|---------------|---------------|
| **(A) Agent-controlled Codespace** | The web product can open or drive a GitHub Codespace (or equivalent remote VM) with network | Agent clones/builds once, runs `vera` / checks, returns RESULTS | Free chat with no Codespace integration — **do not pretend** |
| **(B) Human-in-the-loop Codespace** | Madis opens Codespace; pastes terminal I/O (or file contents) to the agent | Agent reasons + proposes commands; Madis runs them; agent scores from real exit codes | Slow; still valid if oracles stay honest |
| **(C) Network in another product mode** | Agent/host has outbound network (Cursor local, paid agent with tools, curl-capable sandbox, etc.) | Clone repo **or** `curl` release binary + run bench | Classic free Grok/Gemini chat with no network — **STOP / SKIP(env)** |

**Do not claim:** “paste this into free Grok/Gemini and they will run CLI.”  
**Do claim:** “with Mode A, B, or C, the same task oracles apply.”

SKIP(env) ≠ VERA task failure. See [`VERA_AGENT_BENCH_V01_TRIAL_COMPARE_2026-07-20.md`](VERA_AGENT_BENCH_V01_TRIAL_COMPARE_2026-07-20.md).

---

## Codespace URL pattern

After `.devcontainer` is on `main`:

```text
https://codespaces.new/Poxmetax/vera-lang?quickstart=1
```

Or from the repo page: **Code → Codespaces → Create codespace on main**.

Expected after create (see `postCreateCommand`):

- Rust stable (image)
- `z3` on `PATH` (`apt`)
- `cargo build -p vera` completed
- Working directory: repo root (`/workspaces/vera-lang` or similar)

Smoke:

```bash
z3 --version
./target/debug/vera --help
./target/debug/vera bench/vera_agent_bench_v01/task_T01_hello_console/initial/main.vera
```

---

## How Madis opens Codespace once (Mode B setup)

1. Open https://github.com/Poxmetax/vera-lang  
2. **Code → Codespaces → Create codespace on `main`** (or use the URL above).  
3. Wait for post-create (Rust build + Z3). If it fails, paste the log — do not invent PASS.  
4. Keep the Codespace tab open while chatting with Grok/Gemini.  
5. For each agent turn: paste the **Codespace-variant** block from [`PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md`](PROMPTS_WEB_GROK_GEMINI_BENCH_V01.md), then paste terminal output back when the agent asks.

---

## How agents should use it

**Preferred order:**

1. If you already control a Codespace / remote shell with this repo → run tasks there (Mode A).  
2. If Madis is pasting terminal I/O → propose exact commands; wait for real stdout/stderr/exit codes (Mode B).  
3. If you have network but no Codespace → clone **or** download release binary (Mode C).  
4. If none of A/B/C → mark all tasks **SKIP(env)** and stop. Never invent PASS.

Allowed CLI only:

```bash
./target/debug/vera <path.vera>
./target/debug/vera --prove <path>
./target/debug/vera --diag-json <path>
./target/debug/vera --round-trip <path>
# or: cargo run -p vera -- …
```

---

## Binary download fallback (Mode C)

When a GitHub Release exists (tag `v*` built by `release-cli` workflow):

```bash
# Example — replace TAG with a real release tag once published
curl -fsSL -o vera \
  "https://github.com/Poxmetax/vera-lang/releases/download/TAG/vera-linux-x86_64"
chmod +x vera
./vera --help
# Z3 still required on PATH for --prove tasks
```

Until a tag release exists: use **workflow_dispatch** artifact `vera-linux-x86_64` from Actions, or build in Codespace.

**Honest limit:** downloading a binary still requires a host that can execute it and (for prove) has `z3`. Chat-only sandboxes without download/exec remain SKIP(env).

---

## Results honesty

Same rules as the web pointer: honest PASS / FAIL / SKIP only; no `.rs` language expansion; no IFC / GAP-D2 / durable FixPatch / hole-synthesis claims. Return a RESULTS table in chat.
