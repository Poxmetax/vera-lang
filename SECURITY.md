# Security Policy

## Scope

VERA (`vera-lang`) is an **Apache-2.0 research prototype**, not a production runtime.
Security expectations match that status: report issues early; do not assume hardened
deployment defaults.

This repository is **standalone**. It does not include, depend on, or accept
contributions that pull in private trading systems, secrets, or unrelated monorepo paths.

## Supported versions

Only the default branch (`main`) is considered for security discussion. Tagged
releases, if any, inherit the same research-prototype caveats unless a tag notes
otherwise.

## Reporting a vulnerability

**Preferred:** use [GitHub Security Advisories](https://github.com/Poxmetax/vera-lang/security/advisories/new)
for private disclosure.

If advisories are unavailable, open a **private** channel via GitHub Issues only
after confirming the report contains **no secrets** and no exploit payload against
third-party systems. Prefer describing impact and reproduction at the language /
toolchain layer (parser, typechecker, VC/Z3 path, CLI).

Please **do not**:

- Open a public issue with a working exploit against unrelated services
- Attach credentials, `.env` contents, or private project paths
- Expect a bug bounty — **none is offered**

## Response

Reports are reviewed as capacity allows. There is no guaranteed SLA. Fixes may
land as documentation clarifications, tests, or code changes depending on severity
and research priorities.

## Non-goals

This policy does not cover third-party dependencies beyond normal Dependabot /
maintainer review, nor does it cover operator-private work outside this public
repository.
