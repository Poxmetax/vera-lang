<!--
Standing rule (Madis): long prompts get partially copied in chat.
Write the FULL detailed brief to a file (as usual after a task).
Chat paste is only a SHORT pointer that sends Claude to that one file.
-->

# Claude POINTER prompt template (one-file)

## Language

Chat with Madis may be Estonian. **All Claude paste / pointer / handoff prompt text: English only.** Never paste Estonian instruction blocks (e.g. do not use "Loe ja järgi", "Kontekst:", "Ära alusta").

## Standing rule

1. After (or before) work, write a **detailed** file under `docs/pilot/` the usual way:
   - Implement: `FABLE5_*_HANDOFF_PROMPT.md` or slice note with full done / must-do / pass bar
   - Review: `CLAUDE_REVIEW_<TOPIC>.md` from `CLAUDE_REVIEW_PROMPT_TEMPLATE.md` (sections 0-6)
2. Chat paste for Claude is a **short pointer** (about 10-15 lines) that:
   - names the role (implement vs review)
   - points to **that one file** as the source of truth
   - says: read it, follow it, do not invent from memory
3. **Never** paste the full detailed file into chat.

## Short pointer shape (copy and fill)

```text
# Claude pointer -- <TOPIC> <implement|review>

Read and follow this file. Do not invent scope from memory.

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Fable 5 / Claude Code (<implement|review>). Madis is the operator.

**Primary brief (read first, follow exactly):**
`docs/pilot/<DETAILED_FILE>.md`

**Also read these already-written files:** (optional but preferred -- speeds Claude; still short)
- `docs/pilot/<RELATED_1>.md`
- `docs/pilot/<RELATED_2>.md`

Follow the primary brief exactly. Do not invent scope from memory.
No git commit/push unless Madis asks. Stay inside `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
```

Primary file holds full done / must-do / pass bar. Supporting list = already-written SoTs (ACK, slices, SPEC) -- Claude reads from disk; Madis does not paste their bodies.

## Examples

| Session | Detailed file (on disk) | Short paste |
|---------|-------------------------|-------------|
| P2C implement | `FABLE5_CONF_P2C_HANDOFF_PROMPT.md` | `CLAUDE_POINTER_P2C_IMPLEMENT.md` |
| P2C review | `CLAUDE_REVIEW_P2C_LEN.md` | `CLAUDE_POINTER_P2C_REVIEW.md` |
| P2D implement | `FABLE5_CONF_P2D_HANDOFF_PROMPT.md` | `CLAUDE_POINTER_P2D_IMPLEMENT.md` |

Related: full review section template stays in `CLAUDE_REVIEW_PROMPT_TEMPLATE.md`.
