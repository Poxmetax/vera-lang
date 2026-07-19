<!--
Operator note (Estonian):
Madis — kopeeri sellest mallist uus CLAUDE_REVIEW_<TOPIC>.md fail iga kord,
kui annad Claude'ile (Fable 5 / Claude Code) ülevaatuseks midagi.
Täida sektsioonid 1–4 konkreetsete failide, markerite ja smoke-käskudega.
See mall on STANDING RULE: review-prompt peab alati ütlema (1) mida tehti,
(2) mida oodati, (3) mida Claude peab tegema, (4) milline on korrektne väljund.
-->

# Claude review prompt — <TOPIC>

## Meta

| Field | Fill |
|-------|------|
| **Topic / marker** | e.g. `[P2-REFINE1]`, soft polish, Phase N |
| **Date** | YYYY-MM-DD |
| **Workspace** | `C:\Users\madis\Desktop\TradingBot\vera-lang\` only |
| **Prior session** | who / which agent (soft track, Fable 5, surgical_builder, …) |
| **Primary sources** | paths to slice notes, handoff, SPEC anchors, `git status` notes |
| **Audience** | Claude (Fable 5 / Claude Code) — **review**, not implement |

---

## 0. Standing constraints (always)

1. **Isolation:** Never touch TradingBot mainnet runtime, `.env`, live state (`futures_grid_state.json`, `bot_control.json`, alert cooldowns). No import/export across TradingBot ↔ vera-lang.
2. **No rename:** Do not rename files (especially under `examples/`). No `_probe_*` temps that later become “final” names.
3. **No git commit --trailer "Co-authored-by: Cursor <cursoragent@cursor.com>" / push** unless Madis explicitly asks. Do not add remotes.
4. **Surgical:** Prefer read + report. Code edits only if Madis re-scopes mid-review; ask before >~30 lines or new Cargo deps.
5. **Fable / hard-track ownership:** Do not “fix while reviewing” `vc.rs` / `smt.rs` / other Fable-owned dirty work unless the review prompt explicitly scopes that file and Madis agrees.
6. **Language:** Code/docs English; operator-facing audit return in **Estonian** (see §6).

---

## 1. What the prior session DID (evidence)

> Fill with **concrete** evidence — not vibes. Cite files, markers, tests, examples.

- **Files touched:** `<path>` (+ optional line / function)
- **Phase markers:** `[MARKER]` — where they appear
- **Behavior claimed:** one short paragraph of what landed
- **Tests / examples added:** names + expected pass/fail shape
- **Docs written:** slice report / queue / README lines
- **Explicitly NOT done (deferred):** bullet list
- **Dirty-tree / race notes:** e.g. “`vc.rs` may have unrelated `[P2-SOUND*]` — leave alone”

**Proof pointers (copy real commands you or prior session ran):**

```powershell
# e.g. Select-String / cargo test filter / soft_smoke footer
```

---

## 2. What was EXPECTED of that work (acceptance criteria)

> Map prior work to plan / handoff / SPEC. Mark each criterion MET / PARTIAL / DEFERRED / FAIL.

| Criterion | Source | Expected | Status claimed by prior |
|-----------|--------|----------|-------------------------|
| … | handoff task X / SPEC §… | … | MET / PARTIAL / … |

Also state **non-goals** the prior session was allowed to skip.

---

## 3. What YOU (Claude) MUST DO now (numbered)

1. **Read** the primary sources listed in Meta (slice + handoff + SPEC anchors + this prompt).
2. **Inventory** the claimed markers/tests/examples (grep / `Select-String`); note any mismatch vs §1.
3. **Re-run smoke** (adjust to topic; default soft baseline):

```powershell
cd C:\Users\madis\Desktop\TradingBot\vera-lang
$env:Path = "C:\Users\madis\.cargo\bin;" + $env:Path + ";C:\Users\madis\Desktop\TradingBot\z3-4.16.0-x64-win\bin"
cargo test -p vera --lib
powershell -File docs\pilot\soft_smoke.ps1
# topic-specific: cargo test -- --nocapture <filter> ; cargo run -p vera -- <example>
```

4. **Adversarial check:** try at least one case that should FAIL if the feature is sound (negative example, unit test, or **ephemeral** stdin/`cargo test` — do **not** rename / leave probe files).
5. **Connection-consistency:** writer ↔ reader of any new diagnostic / marker / error string (both ends cited).
6. **Write the audit** in the §6 return format. Do **not** drive-by refactor.

---

## 4. What CORRECT work from you looks like (pass bar)

**PASS** if all of:

- Smoke commands in §3 re-run and results cited (counts / exit codes / key strings).
- Each acceptance row in §2 has an explicit **PASS / FAIL / PARTIAL** vs **your** evidence (not prior session’s claim alone).
- Findings ranked **CRITICAL → HIGH → MEDIUM → LOW** (or “none”).
- Unsound “accept” paths (if in scope) explicitly hunted; either found + cited or “searched, none found” with what you tried.
- No drive-by renames, no mainnet touch, no unsolicited large patches.
- Clear **VERDICT** + optional **next** proposals only (Madis decides).

**FAIL the review deliverable** if you:

- Assert green without re-running smoke.
- Treat prior docs as proof without inventory.
- Edit Fable-owned / out-of-scope files “to help.”
- Commit / push / rename.
- Return only English prose without the Estonian §6 structure Madis needs.

---

## 5. Out of scope / do not

- Full CONF-P2 / unrelated Phase tasks unless listed in Meta.
- Linking `z3` crate, Salsa, labels/IFC, hole synthesis — unless this review’s topic.
- TradingBot integration.
- Rewriting history / inventing git remotes.
- Creating permanent `_probe_*` example files.

---

## 6. Return format (Estonian for Madis)

```text
## VERDICT
PASS | FAIL | PASS-WITH-FINDINGS

## Kokkuvõte
2–4 lauset: mis on tõsi pärast sinu kontrolli.

## Leiud (CRITICAL → LOW)
- [SEVERITY] fail:rida — kirjeldus — tõend (käsk / väljund)

## Acceptance vs sinu kontroll
| Criterion | Prior claim | Sinu verdict | Tõend |
| ... | ... | PASS/FAIL/PARTIAL | ... |

## Smoke (tsiteeri)
cargo test: N passed
soft_smoke: SOFT-SMOKE PASS | FAIL (+ detail)
topic-specific: ...

## Next (ettepanekud ainult)
- ...
```

---

## How to mint a new review file

**Standing paste rule:** Madis pastes a short **POINTER** into Claude chat -- not this full template body. See [`CLAUDE_POINTER_PROMPT_TEMPLATE.md`](CLAUDE_POINTER_PROMPT_TEMPLATE.md). Full detail stays on disk (this file / `CLAUDE_REVIEW_<TOPIC>.md` sections 0-6).

1. Copy this template -> `docs/pilot/CLAUDE_REVIEW_<TOPIC>.md`
2. Fill Meta + sections 1-4 with **real** state from the slice/handoff/`git status`
3. Mint short pointer `docs/pilot/CLAUDE_POINTER_<TOPIC>_REVIEW.md` (role + read-order + must-do + return format)
4. Madis pastes **only** the POINTER into a **new** Claude review session; Claude reads the full review file from disk
5. Implementation: full handoff in `FABLE5_*_HANDOFF_PROMPT.md`; chat paste = `CLAUDE_POINTER_*_IMPLEMENT.md` (never the full handoff body)

