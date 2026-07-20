# VERA — Promo assets registry

Updated 2026-07-20 (polish pass). Do not commit secrets. Export URLs may expire.

## Local files

| Path | Description |
|------|-------------|
| `docs/promo/SLOGANS.md` | 12 claim-honest taglines |
| `docs/promo/SOCIAL_POSTS.md` | Twitter/X, LinkedIn, Reddit drafts (copy only) |
| `docs/promo/DEMO_SNIPPETS.md` | prove_clamp, refine_len, FixPatch JSON |
| `docs/promo/VIDEO_SCRIPT.md` | ~45–60s narration + beat sheet + face-safe notes |
| `docs/promo/ASSETS.md` | This registry |
| `docs/assets/vera_github_social_1200x630.png` | Canva export (~158 KB, 1200×630) — polish v2 |
| `docs/assets/vera_github_social_1200x630.png.bak_*_pre_polish` | Pre-polish backup |
| `docs/assets/vera_promo_48s.mp4` | HeyGen promo (~6.9 MB, ~48 s) — same render; regen blocked |
| `docs/assets/vera_promo_thumb.jpg` | HeyGen thumbnail (~202 KB) |

## Canva (polish v2 — 2026-07-20)

| Item | Value |
|------|-------|
| Generation job | `721e9459-fdfe-434a-a74f-57b5ebae355e` |
| Selected candidate | `dg-0b52fc45-a681-43b7-802b-883bb7f70f04` |
| Source design ID (1600×900) | `DAHP4MK5QHo` |
| Source edit URL | https://www.canva.com/d/AUbOKgNZGg29hUl |
| Resized design ID (1200×630) | `DAHP4DLlIP4` |
| Resized edit URL | https://www.canva.com/d/GLc49hot2m-TgbO |
| Resized view URL | https://www.canva.com/d/1wW1MxMHuh6zbuM |
| Design title | VERA — research prototype social |
| Local PNG | `docs/assets/vera_github_social_1200x630.png` |

**Copy on card (claim-honest):**
- VERA
- Easy to write. Hard to ship silent wrongness.
- AI-native research language · research prototype · Apache-2.0
- github.com/Poxmetax/vera-lang

**Layer-clean confirmation:** New design (no baked “Future of Research Programming” background image). Single teal accent graphic (top-right); no duplicate wordmarks peeking under headline. Hierarchy: wordmark → slogan → meta/URL.

### Superseded (v1 — keep for archaeology)

| Item | Value |
|------|-------|
| Design ID | `DAHP4AslynE` |
| Edit URL | https://www.canva.com/d/sSUxIS02Mv5G3-G / https://www.canva.com/d/n9WVgRDq8JbzCl2 |
| Issue | Background image baked hype title + overlapping teal VERA ghost layers under editable text |

## HeyGen

| Item | Value |
|------|-------|
| Session ID | `2fe1ae2f5f3749dd8c2346971e851d1f` |
| Session URL | https://app.heygen.com/video-agent/2fe1ae2f5f3749dd8c2346971e851d1f |
| Video ID | `86365cd88f9244ac826d11595708418b` |
| Video page | https://app.heygen.com/videos/86365cd88f9244ac826d11595708418b |
| Mode | `generate` (landscape) |
| Status | **completed** (~47.9 s) |
| Title | VERA: AI-Native Research Language Briefing |
| Account plan | free — **3 free videos/month exhausted** (2026-07-20 polish) |
| Script source | `docs/promo/VIDEO_SCRIPT.md` |
| Local MP4 | `docs/assets/vera_promo_48s.mp4` |
| Local thumb | `docs/assets/vera_promo_thumb.jpg` |

**Face-safe status:** NOT regenerated this pass. MCP `create_video_agent` / chat mode returned `quota_exceeded` (429). Prefer **non-captioned** `video_url` over `captioned_video_url` (burned captions can cover face). Full face-safe remake needs plan upgrade or next billing cycle — prompt ready in `VIDEO_SCRIPT.md` § Face-safe regeneration.

CDN URLs (signed; may expire — prefer local copies or HeyGen video page):
- Use MCP `get_video` with video ID above; avoid `captioned_video_url` for local promo cut.

## Claim guardrails (assets must respect)

- Research prototype only
- FixPatch = ephemeral
- Labels ≠ full IFC
- Z3 optional for `--prove`
