# Contributing to VERA

Thanks for considering a contribution. VERA is a **research prototype**. Prefer
small, reviewable changes over large speculative rewrites.

## Claim-less documentation

- Document what the code and tests actually do.
- Before claiming a feature, read [`README.md`](README.md) and [`AGENTS.md`](AGENTS.md)
  (honesty rules / what this is NOT).
- Do not add badge walls, star widgets, marketing copy, or "production-ready" language.

## Docs vs toolchain changes

- **Docs / examples / CI packaging / claim-less reports:** generally the right
  place for documentation and process PRs.
- **Core language / toolchain internals** (VC, SMT, typecheck, interpreter paths):
  expect careful review; prefer discussion in an Issue first if the change is
  large or changes soundness-related behavior.

When unsure, open an Issue.

## Development setup

Requirements: Rust stable (`cargo`), and [Z3](https://github.com/Z3Prover/z3/releases)
on `PATH` for prove-path tests.

```bash
cargo test -p vera
cargo run -p vera -- examples/hello.vera
```

Optional smoke (from a checkout root that contains `Cargo.toml`):

```powershell
powershell -File bench/soft_smoke.ps1
```

## Pull requests

1. Keep PRs small and focused.
2. Include or update tests when behavior changes.
3. Link related Issues.
4. Do **not** introduce mainnet trading dependencies or unrelated private paths.
5. Do **not** commit secrets or `.env` files.

## Issues

Bug reports and design questions via [GitHub Issues](https://github.com/Poxmetax/vera-lang/issues)
are preferred. Security: see [`SECURITY.md`](SECURITY.md).

## License

By contributing, you agree that your contributions are licensed under the
Apache-2.0 license of this repository.
