# GitHub visibility checklist (VERA)

Maintainer checklist for [Poxmetax/vera-lang](https://github.com/Poxmetax/vera-lang).  
Do **not** post to public channels until Madis approves and E/README are ready.

Sources consulted: [GitHub SEO keywords](https://claudegithub.com/blog-github-seo-keywords), [Infrasity GitHub SEO 2026](https://www.infrasity.com/blog/github-seo), [GitHub Docs — social preview](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/customizing-your-repositorys-social-media-preview).

## 1. About box (GitHub UI — Madis clicks)

Repo → **About** (gear) → fill:

| Field | Value |
|-------|--------|
| **Description** | see paste-ready one-liner below |
| **Website** | leave empty until docs site exists; optional later: GitHub Pages |
| **Topics** | see list below (up to 20) |
| **Releases / Packages / Deployments** | leave unchecked until you have them |

### Paste-ready description (≤160 chars)

```
AI-native research language: typed holes, refinements, Z3 VCs, FixPatch. Rust toolchain. Apache-2.0 prototype — not full IFC.
```

Character count: 125.

### Topics (paste / click in UI)

```
programming-language
rust
research
ai
llm
typed-holes
refinement-types
formal-verification
smt
z3
interpreter
type-system
apache2
agents
```

Optional extras if slots remain: `static-analysis`, `compiler`, `verification-conditions`.

## 2. Social preview image (GitHub UI — Madis uploads)

Official guidance: PNG/JPG/GIF **under 1 MB**; best quality **1280×640** ([GitHub Docs](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/customizing-your-repositorys-social-media-preview)).

| File | Use |
|------|-----|
| [`docs/assets/social-preview.jpg`](assets/social-preview.jpg) | **Upload this** (1280×640 JPEG, &lt;1 MB) |
| [`docs/assets/social-preview.png`](assets/social-preview.png) | Source/edit asset only (oversized for GitHub UI) |

Steps:

1. Repo → **Settings** → **General** → **Social preview** → **Edit** → **Upload an image…**
2. Select `docs/assets/social-preview.jpg`
3. Confirm preview on a private Discord/Slack link (or Twitter Card Validator)

Canva MCP was unavailable (needs auth). Re-export in Canva at 1280×640 if you want a hand-tuned banner later.

## 3. Files that must be on `main` for discovery

- [x] `README.md` — elevator pitch, badges, Quick start, honest status
- [x] `LICENSE` — Apache-2.0 (enables license badge + GitHub license detection)
- [ ] Push/commit these when Madis is ready (soft track left them uncommitted by default)
- [ ] After E lands: bump Status table if FixPatch wording changes (still ephemeral)

## 4. Organic channels (AFTER README polish + Madis go — do not post yet)

| Channel | Timing / tip |
|---------|----------------|
| [Rust Users Forum](https://users.rust-lang.org/) | “Show your project” / research; link prove demos; honest limits |
| [r/rust](https://www.reddit.com/r/rust/) | Short Show post; avoid hype; weekends quieter |
| [Hacker News — Show HN](https://news.ycombinator.com/showhn.html) | Title: `Show HN: VERA – AI-native language with typed holes + Z3 VCs`; weekday morning UTC |
| [awesome-rust](https://github.com/rust-unofficial/awesome-rust) PR | Only after README + LICENSE on main; category e.g. Development tools / Formal methods if they fit |
| [awesome-formal-methods](https://github.com/formal-methods/awesome) / similar lists | Same: PR after polish; one line + honest scope |
| Discord (Rust Programming Language, etc.) | #showcase only; no spam |

**Do not:** buy stars, mass-DM, cross-post identical spam, or claim IFC / durable certificates.

## 5. Sanity checks after UI update

```text
https://github.com/Poxmetax/vera-lang
→ description visible
→ topics clickable
→ LICENSE detected as Apache-2.0
→ social card image on share preview
```
