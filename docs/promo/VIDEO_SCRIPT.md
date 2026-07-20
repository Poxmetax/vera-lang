# VERA — Promo video script (~45–60s)

**Tone:** research-engineering briefing. Calm, precise. No startup-bro cadence.  
**Visual preference:** dark slate / muted teal; code overlays; no neon hype.  
**Target length:** 45–60 seconds spoken (~110–130 words).

---

## On-screen title card (0–3s)

**VERA**  
AI-native research language · Apache-2.0  
github.com/Poxmetax/vera-lang

---

## Narration (full script)

VERA is an AI-native research programming language — a low-ceremony surface over a machine-checked substrate.

Today it ships typed holes, refinement contracts, a thin Z3 verification path, and ephemeral FixPatch diagnostics.

Refinements check at runtime. When Z3 is on your PATH, `vera --prove` can discharge verification conditions — like proving a clamp always lands in range.

FixPatch is honest about its limits: machine-applicable, and always marked ephemeral — not a durable certificate store.

Label lattices have a math pilot plus a thin seeded checker surface for explicit-flow E1/E6 rejects. They are not full information-flow control, and there is no label syntax or inference yet.

VERA is a research prototype. Easy to write. Hard to ship silent wrongness.

---

## Beat sheet (for editors / HeyGen)

| Time | Visual | VO beat |
|------|--------|---------|
| 0–5s | Wordmark + “research prototype” badge | Intro: AI-native research language |
| 5–15s | Pillars strip: holes · refine · Z3 · FixPatch | What ships today |
| 15–30s | Code: `prove_clamp` snippet + `--prove` | Runtime checks + optional prove |
| 30–42s | JSON: `ephemeral: true` FixPatch | Honest limits on FixPatch |
| 42–50s | “≠ full IFC” label callout | Labels ≠ IFC |
| 50–60s | GitHub URL + Apache-2.0 | CTA + close slogan |

---

## Forbidden phrases (do not ad-lib)

- “production ready”, “enterprise”, “guarantees correctness”
- “full IFC”, “information-flow control shipped”
- “durable certs”, “proof cache”, “never wrong”
- “revolutionary”, “game-changer”, “10x”

---

## HeyGen generation notes

- Preferred mode: `generate`, landscape, professional narrator (neutral accent).
- Style: technical product explainer, not lifestyle ad.
- Session / asset IDs recorded in `ASSETS.md` when generation succeeds.

---

## Face-safe regeneration (2026-07-20 polish)

**Problem:** Floating titles / kinetic text / burned captions can cover the avatar face.

**Hard layout rules for next render:**
- Keep avatar face + upper torso clear at all times.
- All on-screen text, lower-thirds, code callouts, and animations: **lower third** or **far left/right side panels only** — never over eyes/forehead/mouth.
- Prefer non-captioned export (`video_url`); do **not** use `captioned_video_url` as the local promo cut.
- No neon hype; dark slate / muted teal.

**Beat sheet (unchanged VO; safe-zone visuals):**

| Time | Visual (safe zone) | VO beat |
|------|--------------------|---------|
| 0–5s | Lower-third title: VERA · research prototype | Intro |
| 5–15s | Side or lower strip: holes · refine · Z3 · FixPatch | What ships |
| 15–30s | Side panel: `prove_clamp` / `--prove` | Runtime + optional prove |
| 30–42s | Lower-third: `ephemeral: true` | FixPatch honesty |
| 42–50s | Side callout: labels ≠ full IFC | Label pilot |
| 50–60s | Lower-third CTA: github + slogan | Close |

**Blocker (this session):** HeyGen free plan — 3 videos/month used; MCP returns `quota_exceeded`. Re-run after upgrade or reset; keep narration above verbatim.
