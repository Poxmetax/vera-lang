# VeraAgentBench v0.1 — three-agent trial compare (soft note)

**Date:** 2026-07-20  
**Author:** soft track (Cursor) — compare-only; no `.rs`, no commit  
**Status:** SOFT NOTE — not a leaderboard claim  
**Sources:** `bench/vera_agent_bench_v01/results/RESULTS_FABLE_TRIAL_2026-07-20.md` (Fable); operator-reported Grok / Gemini web trials (same day)

---

## 1. Scoreboard

| Agent | Host | T01–T08 | Verdict | Notes |
|-------|------|---------|---------|--------|
| **Fable** (local Cursor / Claude) | Local CLI + repo + network as needed | **8/8 PASS** | Ran bench as designed | Local CLI; T05 FixPatch useful; T08 needed manual fill |
| **Grok** (web, cheap/free) | Browser sandbox | **All SKIP** | Env blocked | Cannot clone `github.com` (sandbox internet disabled); no `cargo` / `vera`; honest — no invented PASS; overclaim scan clean |
| **Gemini** (web, cheap/free) | Browser sandbox | **All SKIP (env)** | Env blocked | No clone/fetch, no `cargo`/`z3`, no local FS; honest SKIP; overclaim scan clean |

**Headline count:** Fable 8 PASS · Grok 0 PASS / 8 SKIP · Gemini 0 PASS / 8 SKIP(env).  
**Do not read as:** “VERA beats Grok/Gemini” or “Fable proves language superiority.”

---

## 2. Interpretation

**SKIP(env) ≠ VERA failure.** The web agents did not fail typecheck, prove, or FixPatch tasks — they never reached a runnable VERA CLI. The bench, as designed, requires:

- clone or local checkout of the repo  
- `cargo` / `vera` (and for prove tasks, `z3`)  
- writable local filesystem  

That measures **agent host capability** (shell + network + toolchain) more than **language value prop** (typed holes, refinements, diagnostics, FixPatch usefulness).

Free web agents without shell + network **cannot** run this bench as designed. Their clean honesty (no fake PASS, overclaim scan clean) is useful signal about **protocol discipline**, not about VERA’s surface quality.

Fable’s 8/8 PASS shows that, on a host that already has the toolchain, the v0.1 task set is **executable by an agent** — including that T05 FixPatch was useful and T08 still needed a manual fill. That is a host+agent+bench result, not a cross-model language ranking.

---

## 3. Implications for a GitHub-posted bench

If the goal is public, multi-agent comparison after posting the bench on GitHub, pick an explicit mode — do not mix them silently:

| Option | What it enables | What it does *not* claim |
|--------|-----------------|---------------------------|
| **(a)** Prebuilt binaries / release artifacts + Codespace / CodeSandbox | Web or light agents can run without local `cargo` | Still needs a real shell environment |
| **(b)** Operator-run harness that only pastes prompts / results | Fair compare across models; operator owns env | Agents are not “self-sufficient” runners |
| **(c)** “Paper eval” mode with pasted CLI transcripts | Offline / chat-only agents can still be scored on reasoning | Not a live CLI trial; oracle discipline must stay strict |
| **(d)** Keep web agents for **protocol honesty** tests only | SKIP vs invented-PASS is a real metric | Not a substitute for language / task PASS rates |

Recommendation shape for public posting: **label the mode in the README** so readers do not equate SKIP(env) with task failure or with VERA weakness.

---

## 4. Recommended next steps for Madis (pick 1–2)

1. **Decide the public-bench mode** — (a), (b), (c), or (d) above — before inviting more free-web agents; otherwise the scoreboard will keep looking like “2× all SKIP.”  
2. **Keep one local full-CLI trial** (Fable-class) as the smoke gate for task oracles; use cheap web agents only for honesty / overclaim scans unless (a) or (b) is in place.  
3. *(Optional)* Document T08’s manual-fill need in the task `meta` / results template so future local trials do not silently absorb human help into “PASS.”

---

## 5. Claim guard

- **Do not** overclaim VERA superiority from Fable-only PASS.  
- **Do not** treat Grok/Gemini SKIP as language or model capability rankings.  
- **Do** treat honest SKIP + clean overclaim scan as a positive protocol signal for constrained hosts.  
- Soft docs only this turn — no `.rs`, no git commit/push.

---

## End
