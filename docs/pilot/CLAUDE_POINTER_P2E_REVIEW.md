# Claude pointer -- P2E review (post-land)

Workspace: `C:\Users\madis\Desktop\TradingBot\vera-lang\`

You are Claude / Fable (review only). Madis is the operator.
Task E landed (working tree, **not committed**): marker `[P2E-FIX]`. Soft review already filled:
[`CLAUDE_REVIEW_P2E_FIXPATCH.md`](CLAUDE_REVIEW_P2E_FIXPATCH.md) (Cursor soft-track, 2026-07-20) -- **PASS**.
Do **not** implement durable FixPatch store / GAP-D2. Do **not** reopen GAP-2..5.
Do **not** edit `vc.rs` / `smt.rs` / `typecheck.rs` / `interp.rs` / `diag.rs` / `main.rs` / `store.rs`.

**Primary review brief (read first):**
`docs/pilot/CLAUDE_REVIEW_P2E_FIXPATCH.md`

**Also read:**
- `docs/pilot/P2E_FIXPATCH_SLICE.md` -- SoT for FixPatch claims / honest limits
- `docs/pilot/FABLE5_CONF_P2E_HANDOFF_PROMPT.md`
- `docs/pilot/CLAUDE_POINTER_P2E_IMPLEMENT.md`
- `docs/pilot/GAP5_INV2_DESIGN_NOTE.md` -- ephemeral contract
- `docs/pilot/P2B_DIAG_SLICE.md` -- additive `fix` field
- `docs/pilot/CURSOR_SYNC_ACK_P2E.md`
- `docs/spec/SPEC.md` (DP8, CONF-P2 FixPatch)

Re-run smoke if Madis wants a second pair of eyes; cite real numbers. Soft baseline is **53**.
No git commit/push unless Madis asks. Stay in `vera-lang/`; never touch TradingBot mainnet / `.env` / live state.
