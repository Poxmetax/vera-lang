# Contributing to VERA

Thanks for considering a contribution. VERA is a **research prototype**. Prefer
small, reviewable changes over large speculative rewrites.

## Claim-less documentation

- Document what the code and tests actually do.
- Before claiming a feature, read [`docs/pilot/KNOWN_GAPS.md`](docs/pilot/KNOWN_GAPS.md).
- Do not add badge walls, star widgets, marketing copy, or “production-ready” language.

## Soft track vs hard track

This tree is developed in a private monorepo and published as a standalone
subtree. Contributors on GitHub see only this public repository.

- **Soft track** (docs, examples metadata, CI packaging, claim-less reports):
  generally the right place for documentation and process PRs.
- **Hard track** (core language / toolchain internals such as VC, SMT, typecheck,
  interpreter paths): expect careful review; prefer discussion in an Issue first
  if the change is large or changes soundness-related behavior.

Exact ownership of individual files may shift; when unsure, open an Issue.

## Development setup

Requirements: Rust stable (`cargo`), and [Z3](https://github.com/Z3Prover/z3/releases)
on `PATH` for prove-path tests.

```bash
cargo test -p vera
cargo run -p vera -- examples/hello.vera
```

Optional soft smoke (from a checkout that includes the pilot scripts):

```powershell
powershell -File docs/pilot/soft_smoke.ps1
```

## Pull requests

1. Keep PRs small and focused.
2. Include or update tests when behavior changes.
3. Link related Issues; mention any known-gap IDs you touch.
4. Do **not** introduce TradingBot / mainnet / private monorepo dependencies.
5. Do **not** commit secrets, `.env` files, or unrelated private paths.

## Issues

Bug reports and design questions via [GitHub Issues](https://github.com/Poxmetax/vera-lang/issues)
are preferred. Security: see [`SECURITY.md`](SECURITY.md).

## License

By contributing, you agree that your contributions are licensed under the
Apache-2.0 license of this repository.
