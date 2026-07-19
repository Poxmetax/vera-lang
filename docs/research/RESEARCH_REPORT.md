# VERA Research Report — Phase 0

**Version:** 1.0 (Phase 0 deliverable) · **Date:** 2026-07-19 · **Status:** complete, pending operator review
**Companion document:** [`../spec/SPEC.md`](../spec/SPEC.md) (the language design specification this report grounds)
**Inputs:** the VERA project plan (`vera_ai-native_language_5ea95226.plan.md`) and the Phase -1 pilot report ([`../pilot/REPORT.md`](../pilot/REPORT.md), verdict PASS, 2026-07-19).

**Epistemic labeling convention (used throughout both Phase 0 documents):**

- **[VERIFIED]** — backed by a primary source verified either during the plan's research pass (plan §10) or re-verified during this Phase 0 session, or by first-party pilot evidence with recorded exit codes.
- **[DESIGN CHOICE]** — a decision VERA makes; defensible but not empirically forced.
- **[UNVERIFIED/OPEN]** — plausible but not yet demonstrated; must not be treated as fact. Each such item appears in §4 (risks) with a discharge plan.

---

## Table of contents

- [1. Problem statement and empirical grounding](#1-problem-statement-and-empirical-grounding)
  - [1.1 The question](#11-the-question)
  - [1.2 What the third-party evidence says](#12-what-the-third-party-evidence-says)
  - [1.3 The core tension](#13-the-core-tension)
  - [1.4 First-party evidence: the Phase -1 pilot](#14-first-party-evidence-the-phase--1-pilot)
  - [1.5 Design consequences](#15-design-consequences)
- [2. Prior-art analysis](#2-prior-art-analysis)
  - [2.1 The 2026 AI-first wave](#21-the-2026-ai-first-wave)
  - [2.2 Unison — the content-addressed substrate](#22-unison--the-content-addressed-substrate)
  - [2.3 Mojo — progressive hardening and AI hardware](#23-mojo--progressive-hardening-and-ai-hardware)
  - [2.4 Effects and capabilities: Koka, Flix, Pony, Effekt, Scala 3](#24-effects-and-capabilities-koka-flix-pony-effekt-scala-3)
  - [2.5 Verification: SPARK, Dafny, Flux, and the prover-as-oracle](#25-verification-spark-dafny-flux-and-the-prover-as-oracle)
  - [2.6 Security: CaMeL, Jif, LIO, object capabilities](#26-security-camel-jif-lio-object-capabilities)
  - [2.7 Toolchain and smart-layer source techniques](#27-toolchain-and-smart-layer-source-techniques)
- [3. The synthesis map](#3-the-synthesis-map)
  - [3.1 How the pieces compose](#31-how-the-pieces-compose)
  - [3.2 Where the genuine novelty is](#32-where-the-genuine-novelty-is)
  - [3.3 Where VERA is deliberately not novel](#33-where-vera-is-deliberately-not-novel)
- [4. Open research risks, ranked](#4-open-research-risks-ranked)
- [5. Sources](#5-sources)

---

## 1. Problem statement and empirical grounding

### 1.1 The question

Which programming language does an LLM write with the fewest shipped bugs — and can one be *designed* so that the answer is "this one," for both ordinary software and autonomous agents?

Every language in use today was designed for human authors. LLM authors have a different failure profile: they are extremely fluent in high-training-mass, low-ceremony languages, and they ship a characteristic set of silent defects (unvalidated input, injection, crypto misuse, hard-coded credentials, unhandled error paths, bounds errors). VERA's bet is that a language can keep the surface LLMs are fluent in while making that defect set unrepresentable or statically caught underneath.

### 1.2 What the third-party evidence says

**[VERIFIED]** Four independent lines of evidence, all cited in the project plan (§10) and re-checked this session:

1. **Kharma, Choi, AlKhanafseh, Mohaisen — "Security and Quality in LLM-Generated Code: A Multi-Language, Multi-Model Analysis" ([arXiv 2502.01853](https://arxiv.org/abs/2502.01853), accepted to IEEE TDSC).** 200 tasks × 4 languages (Python, Java, C++, C) × 5 LLM families, evaluated with per-program unit tests plus SonarQube and CodeQL. Findings: Python and Java achieve higher compilation and semantic-correctness rates and *fewer security findings* than C and C++; C/C++ output shows more memory-safety issues, hard-coded secrets, and cryptographic misuse; models frequently fail to use modern security features (e.g. Java 17 facilities) and reproduce outdated idioms. Notably for VERA's bucket design, hard-coded credentials (CWE-259) is called out as a prominent Python weakness.
2. **Zhang, Kothari — "Holistic Evaluation of State-of-the-Art LLMs for Code Generation" ([arXiv 2512.18131](https://arxiv.org/abs/2512.18131)).** 944 real-world LeetCode problems across five languages, six models. Establishes a defect taxonomy that any "AI-native" language must answer to: compile-time errors, runtime errors, functional failures, and algorithmic suboptimality — and shows large between-model variance (DeepSeek-R1 and GPT-4.1 lead), meaning language design cannot assume a top-tier author.
3. **"Perish or Flourish? A Holistic Evaluation of Large Language Models for Code Generation in Functional Programming" ([arXiv 2601.02060](https://doi.org/10.48550/arxiv.2601.02060)).** FPBench: 721 tasks in Haskell, OCaml, Scala, with Java as an imperative baseline, across GPT-3.5/4o/5. Findings: error rates remain significantly higher in *purely* functional languages than in hybrid (Scala) or imperative (Java) ones, and LLMs frequently emit non-idiomatic, imperative-patterned code in FP languages. Crucially, static-analysis feedback enables *partial self-repair*. This is the strongest warning against a ceremony-heavy or unfamiliar surface — and the strongest endorsement of a machine-readable diagnostic-and-repair loop.
4. **[ai-coding-lang-bench](https://github.com/mame/ai-coding-lang-bench)** — a community benchmark comparing LLM coding accuracy across languages; corroborates the training-mass effect (mainstream dynamic languages are written most accurately).

Two summary facts fall out. First, *what LLMs write best* (familiar, permissive, high-training-mass surfaces) and *what ships fewest security defects* (strict, checked substrates) are today different languages. Second, LLMs respond well to structured, machine-readable feedback loops — self-repair works when the tooling tells the model precisely what is wrong.

### 1.3 The core tension

**[VERIFIED]** The evidence in 1.2 yields the tension the plan (§1) is built on:

- Dynamic, permissive, familiar languages (Python/JS/Ruby): highest LLM fluency, lowest ceremony — and the defect classes above ship *silently* (nothing in the toolchain objects).
- Strict/verified languages (Rust, SPARK, Haskell): whole bug classes eliminated — but LLMs struggle with the ceremony, revert to non-idiomatic imperative patterns (arXiv 2601.02060), and first-try validity drops.

Every prior attempt picks a side. VERA's thesis is that the tension is resolvable because the *surface* and the *substrate* are separable: keep the surface within the LLM's fluency distribution; move the strictness into a machine-verified substrate whose feedback is machine-readable and machine-actionable (the arXiv 2601.02060 self-repair result is what makes this loop credible).

### 1.4 First-party evidence: the Phase -1 pilot

**[VERIFIED — first-party]** The plan gated all build work on a cheap falsification attempt of the thesis (Phase -1). It ran on 2026-07-19 and PASSED. Full detail with recorded exit codes: [`../pilot/REPORT.md`](../pilot/REPORT.md).

**Setup.** No VERA toolchain exists, so the pilot used the closest runnable proxy: Python 3.13 + `mypy --strict` (proxy for the static type/label layer) + `icontract` (proxy for `requires`/`ensures`) + `hypothesis` (contracts-as-oracles, plan U9) + a ~90-line hand-rolled substrate (`Option`/`Result`, `Tainted`/`Trusted`, `Secret`) emulating the unified label idea. Six bug buckets from arXiv 2502.01853 / MITRE CWE. Per bucket: (a) idiomatic Python and (b) substrate-style code, then: does (b) reject before runtime what (a) ships silently, and at what authoring cost?

**Results (condensed from the pilot's per-bucket table):**

| # | Bucket (CWE) | (a) ships silently? | (b) caught before runtime? | Mechanism | Cost (Δ logical SLOC) |
|---|---|---|---|---|---|
| 1 | Input validation (CWE-20) | Yes | Yes | contract + property test | +4 |
| 2 | SQL injection (CWE-89) | Yes | Yes | **static** type error (taint) | −1 |
| 3 | Crypto misuse (CWE-327) | Yes | Yes | **static** type error | +7 |
| 4 | Hard-coded creds (CWE-259) | Yes | Partial→Yes | **static** type error + runtime redaction | +5 |
| 5 | Unhandled None/error | Yes | Yes | **static** type error (`Result`) | +1 |
| 6 | Out-of-bounds (CWE-787) | Yes | Yes | contract + property test | +1 |

Catch axis: 6/6 (bar was ≥~70%). Authoring axis: all six authored valid on the *first* `mypy --strict` run (13 modules, exit 0), median +2.5 / mean +2.8 logical lines per bucket over baseline, plus a one-time 64-logical-line shared substrate that a real VERA ships as stdlib. The baselines themselves are fully type-annotated and `mypy --strict`-clean, so the catches come from the *substrate discipline*, not from "Python had no types."

**The pilot's caveats, reported honestly (they shape §4 of this report):**

1. **Only 4/6 catches are truly static.** Buckets 2–5 are compile-time (zero execution). Buckets 1 and 6 were caught by runtime contracts + property tests — before ship, but not statically. Real VERA claims SMT-proved refinement types (plan U8) would move buckets 1 and 6 into the static column; the proxy could not demonstrate this (no Z3/CVC5/Dafny installed). **The static value-range/bounds mechanism is therefore assumed, not demonstrated** → risk R1, and a hard Phase 2 conformance requirement in the spec (SPEC §4.4, REQ-REFINE-1/2).
2. **Bucket 4 is the weakest-fidelity proxy.** The leak/serialization vector is genuinely caught; the *provenance* vector is not — a determined author can hardcode a secret by wrapping it. The spec closes this by tying `Secret` construction to capability handles (SPEC §5.4, §8).
3. **`Result` has an `unwrap()` escape hatch** that defers to runtime. VERA's story is the same (representative, not a proxy artifact).
4. **Authoring fluency is n=6, one author, one session.** "Fluently authorable" is *supported*, not *proven*, especially given the FP-reversion finding of arXiv 2601.02060 → risk R3.
5. **The proxy emulates VERA crudely** (wrapper types, no effect rows/capture sets/content-addressing/SMT). It tests the claim's *direction*, not VERA's specific mechanisms.

### 1.5 Design consequences

The evidence base licenses exactly the thesis in plan §1 — *familiar, low-ceremony surface; strict, machine-verified substrate; a prover (not tests) as ground-truth oracle; a tight structured edit→verify→auto-repair loop* — with two v2 reshapes already folded in:

- **Unify effects and capabilities into one mechanism** (capture sets), per Effekt and Scala 3 capture checking **[VERIFIED sources, plan §10]**.
- **Avoid the gradual-typing performance trap**: "Is Sound Gradual Typing Dead?" (POPL 2016) measured 35x–104x slowdowns from pervasive typed/untyped contract boundaries **[VERIFIED, plan §10]**; VERA therefore uses erasable/concrete types plus inferred refinements, never pervasive boundary checks (plan U13).

These are elaborated as normative design principles in SPEC §1.

---

## 2. Prior-art analysis

Format per entry: what it does → what VERA takes → what VERA rejects or does differently → citation. Citation provenance is annotated in §5.

### 2.1 The 2026 AI-first wave

The existence of five independent 2026 designs is itself evidence: the "language for AI authors" problem is recognized, and each project solved a *different slice*. None combines a verified substrate, a unified label model, and a content-addressed store — that gap is VERA's position (§3.2).

#### 2.1.1 Turn — agentic computation as language constructs

**What it does. [VERIFIED]** Turn ([arXiv 2603.08755](https://arxiv.org/abs/2603.08755)) is a compiled, actor-based language for agentic software — statically typed at schema-inference boundaries, dynamically typed at the value level ("targeted strictness"). Five language-level constructs: *Cognitive Type Safety* (LLM inference is a typed primitive: the compiler generates a JSON Schema from a struct definition and the VM validates model output before binding), a *confidence operator* for control flow gated on model certainty, an Erlang-derived *actor model* (each agent gets an isolated context window, persistent memory, mailbox), a *capability-based identity system* (opaque unforgeable handles from the VM host; raw credentials never enter agent memory), and *compile-time schema absorption* (`use schema::openapi(...)`; graphql/fhir/mcp adapters in development). Rust-based bytecode VM; open source.

**What VERA takes.** Nearly the whole agentic layer: infer-as-typed-primitive, the confidence operator, actor isolation, unforgeable capability handles, schema absorption (plan §2, Phase 3; SPEC §7).

**What VERA rejects or does differently.** (a) Turn is *dynamically typed at the value level*; VERA is statically typed throughout with inference — the 1.2 evidence says the dynamic core is precisely where silent defects live. (b) Turn validates the *shape* of LLM output but does not track its *provenance*; in VERA every `infer` result carries the `untrusted` label, so shape-valid-but-malicious data still cannot reach privileged sinks (SEC1; SPEC §7.1). (c) Turn has no contracts/SMT layer; VERA adds one.

#### 2.1.2 AICore — the determinism-first agent language

**What it does. [VERIFIED]** AICore ([github.com/keaz/aicore](https://github.com/keaz/aicore)) is an "agent-native, IR-first" language for human+AI collaboration: deterministic tokenization/parsing/IR IDs/formatting ("same input, same AST"), structured JSON diagnostics (stable codes, spans, fixes), a type + effect checker, design-by-contract (`requires`/`ensures`/`invariant`), LLVM backend, reproducible builds (lockfile, checksums). Semantics are Rust-inspired: ownership/borrows, ADTs, exhaustive match, `Result`/`Option`, no null, trait generics.

**What VERA takes.** Determinism as a product value (plan U10 extends it to the runtime), structured machine-readable diagnostics, the precedent that contracts and an effect checker belong in an agent-facing language.

**What VERA rejects or does differently.** AICore's program-of-record is still files + IR; VERA's is a content-addressed semantic store where edits are typed transactions (plan U16) — determinism of *representation* rather than determinism of *reformatting*. VERA also unifies effects with capabilities and taint (AICore's effect system is effects-only), and aims for SMT-*proved* contracts rather than checked-only.

#### 2.1.3 Kodo — provenance, confidence, and the closed repair loop

**What it does. [VERIFIED]** Kodo ([github.com/rfunix/kodo](https://github.com/rfunix/kodo)) is a compiled AI-agent language whose distinctive feature is compiler-enforced authorship/trust metadata: `@authored_by`, `@confidence` (0.0–1.0), `@reviewed_by`. Confidence propagates *transitively* (a function's effective confidence is the min over its call graph); effective confidence below 0.8 blocks compilation until a human review annotation is present. Contracts are Z3-verified. Diagnostics carry unique codes (E0001–E0699) and byte-offset `FixPatch` objects plus multi-step `RepairPlan`s; `kodoc fix` applies them, closing the compile→fix→recompile loop. Ships an MCP server. Kodo's own docs honestly note the trust root is external (source-text annotations are forgeable without repo-level protections).

**What VERA takes.** The `FixPatch`/`RepairPlan` diagnostic contract (plan U15), provenance + confidence metadata with deploy gates, and the adaptive-verification-budget idea (verify harder where confidence is lower, plan U3).

**What VERA rejects or does differently.** In VERA, provenance metadata attaches to content-addressed definitions in the store, not to source-text annotations — an agent cannot textually edit a review into existence without producing a different hash (the external root-of-trust problem remains, and the spec says so; SPEC §6.4). Confidence gates ride the same store rather than a separate subsystem.

#### 2.1.4 Zerolang (Zero) — the graph is the program

**What it does. [VERIFIED — newly verified this session]** Zero ([zerolang.ai](https://zerolang.ai/), [github.com/vercel-labs/zerolang](https://github.com/vercel-labs/zerolang); Vercel Labs, first released 2026-05-15, Apache-2.0, pre-1.0) is an experimental graph-native systems language: the semantic graph *is* the program database. Agents author through `zero query` and `zero patch` — checked, structure-aware edits guarded by graph hashes, shape rules, and type facts; stale or invalid patches fail before touching the store. Humans read `.0` projections (readable text renders), with `zero export`/`import`/`verify-projection` making the projection boundary explicit. JSON diagnostics with stable codes and typed repair metadata; explicit capabilities; effects in signatures; sub-10 KiB native binaries (self-reported).

**What VERA takes.** Independent confirmation (converging with Unison, §2.2) that *program-as-database + checked patches + human projections* is the right agent-editing model — this de-risks plan U14/U16. The hash-guarded stale-edit rejection maps directly onto VERA's typed transactions.

**What VERA rejects or does differently.** Zero is a C-family systems runtime without a formal verification layer, refinements, or taint tracking. VERA's store is content-addressed at definition granularity (Unison-style: the hash *is* the identity, names are metadata) rather than a mutable graph guarded by hashes; and VERA layers types/proofs/labels above the store.

#### 2.1.5 Karn — token density as the design center

**What it does. [VERIFIED that the repo claims it; numbers self-reported]** Karn ([github.com/karn-lang/karn](https://github.com/karn-lang/karn), MIT) is a token-minimal language for AI agents: one intent per operator (`->`, `!`, `?`, `??`, `|>`, `|~`, `&`, `*`, `%`), every I/O returns `Ok|Err`, a machine-readable JSON spec for agent consumption, multi-target codegen (C/JS/HTML/Python). Claims ~2.1 tokens/LOC vs Python's ~6.8 (76% fewer). **[UNVERIFIED]** The density numbers are self-reported by a single-author, very-low-adoption project (3 GitHub stars at check time); treat as directional, not established.

**What VERA takes.** The token-economy *goal* is real (context windows and output limits are hard constraints). VERA adopts it as U14: a token-dense canonical LLM projection rendered from the same AST as the human syntax — density as a *view*, not as the semantics.

**What VERA rejects.** Making density *the* surface. An operator-soup surface trades away exactly the training-mass familiarity that §1.2 shows drives LLM accuracy, and sacrifices human reviewability. VERA's bet: two projections from one AST dominate one compromise syntax.

### 2.2 Unison — the content-addressed substrate

**What it does. [VERIFIED]** [Unison](https://github.com/unisonweb/unison) identifies every definition by a hash of its AST (names resolved to hashes, binders normalized); names are metadata attached to hashes. Consequences: no builds in the traditional sense, perfect incremental compilation and test caching (results keyed by hash), instant non-breaking renames, semantic rather than textual diffs, and dependencies that cannot drift. Unison also pioneered *abilities* (algebraic effects) in a mainstream-ish functional language.

**What VERA takes.** The substrate wholesale (plan §2): codebase-as-database, hash-keyed caching (the enabling mechanism for INV-2 and the U1 query engine), never-broken codebase, semantic diffs. Abilities also inform the effect design lineage.

**What VERA does differently.** (a) VERA's primary *write* interface for agents is a typed transactional edit API (U16) with a token-dense projection (U14) — Unison's UCM workflow targets humans. (b) VERA layers contracts/refinements/SMT and the label lattice above the store; Unison verifies types only. (c) VERA attaches provenance/confidence metadata (Kodo-style) at the store level.

### 2.3 Mojo — progressive hardening and AI hardware

**What it does. [VERIFIED]** [Mojo](https://mojolang.org/docs/vision/) is a Python-family language for AI compute: progressive typing/hardening (start permissive, tighten to strict `fn` semantics), compile-time metaprogramming in the same language, MLIR-first compilation reaching CPUs/GPUs/accelerators.

**What VERA takes.** The *progressive hardening posture* (prototype permissively, tighten with types/effects/contracts as verification demands — plan §1), MLIR-first lowering for the Phase 4 release backend (U4), and the same-language compile-time metaprogramming direction (natural fit with AST-as-database; post-MVP).

**What VERA rejects.** Python *source compatibility* (a non-goal: it drags in semantics that defeat verification), and GPU-first priorities in v1 (Phase 4 stretch; plan §9 non-goals).

### 2.4 Effects and capabilities: Koka, Flix, Pony, Effekt, Scala 3

**What they do. [VERIFIED]**

- [Koka](https://koka-lang.github.io/koka/doc/book.html): row-polymorphic effect types (purity and exception-safety visible in signatures) and **Perceus** precise reference counting with reuse (FBIP: functional-but-in-place).
- [Flix](https://doc.flix.dev/effect-system.html): a polymorphic effect system positioned explicitly as *supply-chain resistance* — a dependency cannot silently do I/O its signature does not declare.
- [Pony](https://www.ponylang.io/media/papers/fast-cheap.pdf): reference capabilities ("deny capabilities") giving data-race freedom by construction in an actor language.
- [Effekt](https://effekt-lang.org/tour/captures): capability-passing style — effects are provided as capabilities, and *capture sets* on types track which capabilities a value closes over.
- [Scala 3 capture checking](https://www.scala-lang.org/api/3.x/docs/experimental/capture-checking/basics.html) (experimental): capture sets `T^{c}` on types in a production-adjacent compiler.

**What VERA takes.** The v2 unification (plan §1, U12): *a capability is the token that authorizes an effect*, so one capture-set-style mechanism expresses both "what the caller must provide" and "where the value may escape." Effekt/Scala provide the mechanism; Koka/Flix provide the "effects in signatures = provable purity + supply-chain resistance" motivation; Pony provides the actor-adjacent deny-by-default discipline (and later, Phase 4 optional ownership). Perceus/FBIP is the Phase 4 memory-management plan (S9).

**What VERA rejects or does differently.** Koka-style effect *rows* as a separate dimension from capabilities — two overlapping mechanisms is exactly what v2 collapsed. And none of these systems carries *taint/secrecy* in the same annotation; that extension is VERA's (§3.2, risk R2).

### 2.5 Verification: SPARK, Dafny, Flux, and the prover-as-oracle

**What they do. [VERIFIED]**

- [SPARK/Ada with GNATprove](https://learn.adacore.com/courses/intro-to-spark/chapters/05_Proof_Of_Functional_Correctness.html): industrial contract-based proof (`requires`/`ensures` proven, not just tested); contracts double as runtime checks and test oracles; documented honesty about what proof does *not* cover (e.g. timing side channels).
- [Dafny](https://ar5iv.labs.arxiv.org/html/1606.02022): SMT-backed verification-aware language — specs and proofs in the development loop, automation via Z3.
- [Flux](https://ranjitjhala.github.io/static/flux-pldi23.pdf) (PLDI'23): liquid/refinement types for Rust; mostly-*inferred* refinements discharge the ubiquitous bug classes (bounds, overflow, div-by-zero) with near-zero annotation burden — per the plan's verification pass: roughly half the spec lines and an order of magnitude faster verification vs. prior Rust proof tooling on evaluated benchmarks.
- ["The Prover Is the Judge"](https://arxiv.org/html/2607.14340v1) (arXiv 2607.14340): frames the LLM-authoring loop around a machine-checkable oracle — the model iterates until the prover accepts, instead of self-declaring success.

**What VERA takes.** The entire verification stance (plan §2, U7–U9): `requires`/`ensures`/`invariant` runtime-checked by default and SMT-proved when possible; *inferred refinements below contracts* as the workhorse for the common bug classes (this is the designated mechanism for moving pilot buckets 1 and 6 into the static column — SPEC §4.4); contracts as property-test oracles (U9); multi-prover dispatch (Z3/CVC5, U7); and the prover-as-oracle loop philosophy.

**What VERA rejects or does differently.** (a) Mandatory specification: Dafny/SPARK-grade proof as a *requirement* would trigger the ceremony-reversion failure mode (arXiv 2601.02060); in VERA contracts are optional, never required to compile (plan §9). (b) Flux is Rust-hosted; VERA must reimplement refinement inference for its own core — an acknowledged risk (R1), not a free reuse.

### 2.6 Security: CaMeL, Jif, LIO, object capabilities

**What they do. [VERIFIED]**

- [CaMeL — "Defeating Prompt Injections by Design"](https://arxiv.org/abs/2503.18813) (arXiv 2503.18813): defends agent systems by extracting control/data flow from the *trusted* query only; untrusted tool/LLM data flows through a *quarantined* LLM with no tool access; capabilities plus security policies are enforced at tool boundaries. Per the plan's verification pass, CaMeL solved 77% of AgentDojo tasks with provable security — as a Python *framework around* an agent system.
- [Jif](https://www.cs.cornell.edu/jif/): static information-flow control labels (confidentiality + integrity, decentralized label model) on Java types.
- [LIO](http://www.scs.stanford.edu/%7Edm/home/papers/stefan:lio.pdf): dynamic IFC in Haskell — a floating "current label" that rises as sensitive data is observed, enforced at I/O.
- [E / object-capability model](http://erights.org/elib/capability/overview.html): no ambient authority; authority travels only through unforgeable references; attenuation, revocable forwarders, membranes, POLA.

**What VERA takes.** SEC1: CaMeL's architecture *as language semantics* — quarantined contexts are functions whose capability set is empty by type; policies are declarations evaluated at every effect gate; taint labels make "untrusted data cannot drive a privileged action" a type error rather than a framework convention (SPEC §7.4–7.5, §8). From Jif/LIO: the label discipline (static where possible, LIO's context-label idea as the reference design for dynamic islands and implicit-flow control — the latter explicitly OPEN in v0.1). From E/ocap: SEC4 wholesale — attenuation, revocation, membranes riding the capture-set system.

**What VERA rejects or does differently.** (a) A separate taint subsystem (Jif's label sublanguage is famously heavyweight): VERA folds IFC into the *one* label lattice the type checker already infers (§3.2; ergonomics risk R2). (b) CaMeL-as-framework: conventions enforced outside the language can be skipped; VERA's versions are compiler-enforced.

### 2.7 Toolchain and smart-layer source techniques

Compact entries; each feeds a numbered plan item (U/S/SEC). All **[VERIFIED]** via plan §10 primary sources unless noted.

- **Salsa / rust-analyzer query architecture** ([rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/queries/salsa.html), [rust-analyzer](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/architecture.md)) → **U1**: the whole compiler as a memoized, red-green, demand-driven query graph. VERA keys queries by content hash (stronger invalidation than file mtimes). Combined with §2.2, this is the near-instant edit→verify loop.
- **Cranelift aegraph mid-end** ([aegraph](https://cfallin.org/blog/2026/04/09/aegraph/)) → **U2**: single-pass acyclic e-graph optimizer with ISLE rewrite rules; fast dev-tier codegen. VERA pairs it with S10 so rewrites are proof-gated.
- **MLIR progressive lowering** ([MLIR Toy ch.5](https://mlir.llvm.org/docs/Tutorials/Toy/Ch-5/)) → **U4**: dialect stack down to LLVM for release/GPU targets (Mojo precedent). VERA reuses dialects rather than inventing IR.
- **WASI Preview 2** ([WASI](https://github.com/WebAssembly/WASI/blob/v0.2.1/README.md)) → **U5**: component model with unforgeable handles and no ambient authority — the *runtime* enforcement of VERA's capability model; agent code runs least-authority in a real sandbox, not just in the type system.
- **GraalVM Truffle** ([Truffle](http://lafo.ssw.uni-linz.ac.at/papers/2012_SPLASH_Truffle.pdf)) → **S6**: tiered interpret→JIT→AOT with guarded, deoptimizable speculation. VERA requires stable speculations (deopt-cycle guard).
- **Adapton** ([PLDI'14](http://matthewhammer.org/adapton/adapton-pldi2014.pdf)) → **S8**: demanded-computation-graph incremental *runtime* — the Salsa idea extended past compile time, legal only for provably pure code.
- **Perceus/FBIP** ([Perceus TR](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/11/perceus-tr-v4.pdf)) → **S9** and the default memory model: precise RC, in-place reuse at refcount 1 — imperative speed from functional code, no GC pauses.
- **Alive2** ([AliveToolkit](https://github.com/AliveToolkit/alive2/)) → **S10**: translation validation via SMT (it found dozens of real LLVM miscompiles — 47 per the plan's verification pass); VERA applies it to its own aggressive rewrites so "smart speed provably cannot introduce bugs."
- **MLGO** ([LLVM MLGO](https://llvm.org/docs/MLGO.html)) → **S12**: ML-guided optimization *advisory only* — learned models choose among proven-equivalent options and never make correctness decisions; models pinned for deterministic builds (INV-1/INV-2 compliant by design).
- **Smyth / Hazel / Hazelnut / JetBrains MPS** ([Smyth](https://uchicago-pl.github.io/smyth/), [Hazel](https://hazel.org/), [Hazelnut POPL'17](https://plv.colorado.edu/papers/hazelnut-popl17.pdf), [MPS](https://www.jetbrains.com/help/mps/basic-notions.html)) → **S1/S2**: typed holes with live bidirectional synthesis (Smyth: ~66% fewer examples needed vs Myth, per plan verification); Hazelnut measured 44.2% of real edit states as syntactically malformed — the class VERA's structural edit API makes *unrepresentable*; MPS is the industrial projectional-editing precedent.
- **Microsoft Coyote** ([coyote](https://microsoft.github.io/coyote/)) → **U11**: systematic, reproducible exploration of actor interleavings for concurrency testing.
- **monad-par / deterministic parallel Haskell** ([monad-par](https://simonmar.github.io/bib/papers/monad-par.pdf)) → **S7**: purity-driven parallelism with a *guaranteed deterministic* result — the resolution of the S7-vs-U10 tension (INV-3).
- **SLSA + sigstore** ([SLSA v1.2](https://slsa.dev/spec/v1.2/), [provenance](https://slsa.dev/spec/v1.1/provenance)) → **SEC6**: signed non-forgeable build provenance, keyless signing, transparency logs; VERA adds capability-scoped dependencies (a package's authority bounded by declared effects/captures).
- **Proof-Carrying Code** ([Necula '97](https://www.cs.tufts.edu/comp/150CMP/papers/necula97pcc.pdf)) → **SEC2**: ship a machine-checkable proof; verify with a small independent checker (checking ≪ proving), shrinking the TCB and catching prover bugs — VERA's answer to "who watches the verifier."
- **Wasmtime fuel/epoch** ([Config docs](https://docs.wasmtime.dev/api/wasmtime/struct.Config.html)) → **SEC3**: deterministic per-instruction fuel metering + memory limits; aligns resource limits with the determinism thesis.
- **"Is Sound Gradual Typing Dead?"** ([POPL'16](https://www2.ccs.neu.edu/racket/pubs/popl16-tfgnvf.pdf)) → **U13** (negative result VERA designs around): pervasive typed/untyped contract boundaries cost 35x–104x on the gradual-typing lattice benchmarks. VERA: erasable types, inferred refinements, dynamic islands explicit and bounded.

---

## 3. The synthesis map

### 3.1 How the pieces compose

VERA is a *composition*, and the plan's claim is that the composition is coherent. The stack, bottom-up, with the source of each layer:

| Layer | What it is | Borrowed from | Plan items |
|---|---|---|---|
| Store | Content-addressed AST store; names as metadata; edits as typed transactions; dual human/LLM projection | Unison; Zerolang (convergent); Kodo (provenance metadata) | U1, U14, U16 |
| Types | HM inference, ADTs, no null, exhaustive match, `Option`/`Result`, erasable/concrete types | Rust, AICore; POPL'16 (what to avoid) | U13 |
| **Labels** | **One lattice: capabilities/effects + taint + secrecy as a single annotation** | **Effekt/Scala (captures) + Jif/LIO (IFC) + Pony/E (caps) + CaMeL (policies) — the fusion itself is VERA's** | U12, SEC1, SEC4, SEC5 |
| Proofs | Refinements (inferred) below contracts (`requires`/`ensures`), runtime-checked by default, SMT-proved when possible; contracts as test oracles; PCC certificates | Flux/LiquidHaskell, SPARK, Dafny, "Prover Is the Judge", Necula | U7, U8, U9, SEC2 |
| Loop | Query-engine compiler; JSON diagnostics + FixPatch/RepairPlan; typed holes + verifier-gated synthesis; MCP compiler-service | Salsa/rust-analyzer, Kodo, AICore, Smyth/Hazel, Zerolang | U1, U15, S1–S3, S11 |
| Runtime | Deterministic by default; all nondeterminism as effects; fuel metering; actors with isolated context; record/replay; deterministic test scheduler | AICore, Turn, Pony/Erlang, Wasmtime, Coyote, monad-par | U10, U11, SEC3, S7 |
| Agentic | infer-as-typed-primitive (+ `untrusted` label), confidence operator, quarantine/planner split, policy gates, schema absorption | Turn + CaMeL (fused) | Phase 3, SEC1 |
| Backends | Cranelift aegraph (dev) / MLIR→LLVM (release) / WASM+WASI P2 (sandbox); Perceus+FBIP memory; translation-validated rewrites; advisory ML | Cranelift, MLIR/Mojo, WASI, Koka, Alive2, MLGO | U2, U4, U5, S6–S12 |
| Supply chain | SLSA provenance + sigstore + capability-scoped dependencies + reproducible builds | SLSA/sigstore, Flix (motivation) | SEC6 |

The glue that makes the layers safe to compose is the three integration invariants (plan §7), promoted to normative spec rules (SPEC §1.3): **INV-1** only correctness-preserving transforms are automatic; **INV-2** every cache keyed by content hash + solver/model version; **INV-3** implicit parallelism restricted to provably pure/commutative code. The one real tension found in the plan's integration analysis (implicit parallelism S7 vs deterministic-by-default U10) is resolved by INV-3.

### 3.2 Where the genuine novelty is

Two claims, stated with epistemic care:

1. **The unified label lattice.** One annotation on values and signatures carries effects, capabilities, and taint/secrecy — `Str^{net, untrusted}` is "a string whose production needed the `net` capability and whose content is untrusted." CaMeL capabilities + Jif labels + Effekt captures collapse into a single lattice; policies, effect checking, and secrecy all read the annotation the type checker already infers. **[UNVERIFIED as a novelty claim in the strong sense]**: the plan's research pass did not find this fusion shipping in any surveyed language, and neither did this session's — but absence-of-evidence across a survey is not proof of absence. More importantly, its *inference ergonomics are unproven* (risk R2; Phase 2 spike required).
2. **The coherent stack as one artifact.** Each neighbor holds a piece: Turn the agentic primitives, Unison/Zero the store, Kodo the repair loop and provenance, AICore determinism and contracts, SPARK/Dafny/Flux the proofs, CaMeL the injection defense — but no surveyed system runs *prover-as-oracle + content-addressed substrate + unified labels + agentic primitives* as one design where each part strengthens the others (proofs cached by content hash; labels enforced at effect gates the runtime already has; synthesis gated by the prover; agent edits that cannot break the codebase). VERA's differentiator is the synthesis, not any single primitive — the plan says this (§9) and this report's survey confirms it is the defensible framing.

### 3.3 Where VERA is deliberately not novel

Everything else is intentionally borrowed, because each piece is someone's proven result: surface familiarity (Python/Ruby lineage), HM inference, ADTs + exhaustive matching (ML lineage via Rust), SMT solvers (Z3/CVC5 as-is), refinement inference (Flux's recipe), content addressing (Unison's recipe), capability runtime (WASI P2), backends (Cranelift/MLIR/LLVM), memory management (Perceus), concurrency testing (Coyote), supply chain (SLSA/sigstore), synthesis (Smyth-style). Anti-novelty is a stated design value: **rejected** ideas (plan §7) include naive gradual-typing boundaries, unguarded speculation, ML with authority over correctness, unrestricted `eval`, implicit nondeterministic parallelism, and non-hash-keyed caches.

---

## 4. Open research risks, ranked

Each risk: statement → evidence → consequence if it goes badly → mitigation/discharge plan. These are the honest holes in the case; none is currently a blocker, all are gated.

### R1 — The SMT/refinement static-bounds mechanism is undemonstrated (highest priority)

- **Statement.** VERA's claim that value-range/bounds bugs (pilot buckets 1 and 6) move from "caught by runtime contract/property test" to "caught statically, zero execution" rests on inferred refinement types + SMT (U8, U7). No artifact of ours has demonstrated this: the pilot proxy had no SMT layer (Z3/CVC5/Dafny not installed). **[UNVERIFIED/OPEN — pilot caveat 1.]**
- **Evidence for plausibility.** Flux (PLDI'23) demonstrates exactly this mechanism for Rust with mostly-inferred annotations **[VERIFIED]**; the pilot demonstrated the *non-static* half (contracts catch the same buckets pre-ship) **[VERIFIED, first-party]**.
- **Consequence if false.** The thesis degrades, not dies: buckets 1/6 remain runtime-contract-caught (still pre-ship), but the "statically caught" promise and part of the differentiation vs. AICore/Kodo weakens.
- **Discharge.** Phase 2 conformance requirements REQ-REFINE-1/2 (SPEC §4.4) define the exact demonstrations: static rejection of the pilot's bucket-1 out-of-range call and bucket-6 out-of-bounds index, with no execution. The spec instructs Phase 2 to build this *first*, as a spike, before the full contract layer.

### R2 — Unified-label-lattice inference ergonomics are unproven

- **Statement.** Folding effects + capabilities + taint + secrecy into one inferred annotation might produce label blow-up, incomprehensible errors, or annotation burden that breaks LLM fluency. The fusion is VERA's own synthesis; nothing shipping was found to copy the ergonomics from. **[UNVERIFIED/OPEN — plan §9 novelty risk.]**
- **Evidence for plausibility.** Each ingredient works alone (Effekt/Scala captures; Jif/LIO labels; the pilot's crude taint wrappers were authored fluently, n=6) **[VERIFIED per ingredient]**.
- **Consequence if false.** Fall back to two simpler mechanisms (capability captures; separate binary taint) — less elegant, still functional; SEC1 survives in CaMeL's original two-subsystem shape.
- **Discharge.** Phase 2 type-checker spike (plan §9): measure inferred-label size and annotation counts on the example corpus; gate on "labels stay implicit except at module boundaries and sinks."

### R3 — LLM authoring fluency at scale

- **Statement.** The pilot's fluency evidence is n=6 buckets, one author (a frontier model), one session, no time pressure. Weaker models, larger programs, or long-horizon agent sessions may revert to imperative/dynamic patterns, as observed for FP languages. **[UNVERIFIED/OPEN — pilot caveat 4 + arXiv 2601.02060.]**
- **Mitigation by construction.** Familiar surface (§1.5); contracts optional; refinements inferred; FixPatch repair loop (the 2601.02060 self-repair result says feedback loops recover much of the gap); typed holes let the model *state intent* instead of fighting syntax.
- **Discharge.** Continuous: every phase's example corpus doubles as an authoring benchmark; track first-try validity and repair-loop convergence per model tier. No single gate — this is a running metric with a floor (≥~70%, the pilot bar).

### R4 — SMT undecidability and prover limits

- **Statement.** Some verification conditions will be undecidable or time out; a prover-as-oracle loop that frequently answers "unknown" erodes trust and flow. **[VERIFIED as a general property of SMT; open as a quantitative question for VERA's workload.]**
- **Mitigation.** Multi-prover dispatch (U7); refinements restricted to decidable fragments (QF_LIA in MVP-adjacent scope) for the common cases (U8); runtime-check fallback is always sound (contracts never *must* be proved to compile); interactive/human proof escape for the rare hard cases; adaptive verification budget (U3).
- **Discharge.** Phase 2 metrics: proportion of obligations auto-discharged, time-out rate, fallback rate — reported per release.

### R5 — The gradual-typing performance trap (mitigated; keep watching)

- **Statement.** If progressive hardening were implemented as sound gradual typing with pervasive boundary contracts, mixed programs could slow down 35x–104x. **[VERIFIED risk, POPL'16.]**
- **Mitigation (baked into the design).** U13: types/labels are erasable (never change representation); refinements are static; dynamic islands are explicit, bounded, and checked only at their declared boundary. This is a normative spec rule (SPEC §4.3), not an aspiration.
- **Discharge.** Re-measure once Phase 2/4 programs mix hardened and permissive code; budget: boundary overhead must stay under a small constant factor on the benchmark mix, or the dynamic-island mechanism gets redesigned.

---

## 5. Sources

Provenance key: **[plan §10]** = verified during the plan's research pass and reused here; **[session]** = verified 2026-07-19 during this Phase 0 session; **[first-party]** = produced by this project.

**Empirics (LLM code generation):**

- Kharma et al., *Security and Quality in LLM-Generated Code: A Multi-Language, Multi-Model Analysis* — [arXiv 2502.01853](https://arxiv.org/abs/2502.01853) [plan §10; title/authors/findings re-verified session]
- Zhang, Kothari, *Holistic Evaluation of State-of-the-Art LLMs for Code Generation* — [arXiv 2512.18131](https://arxiv.org/abs/2512.18131) [plan §10; re-verified session]
- *Perish or Flourish? A Holistic Evaluation of LLMs for Code Generation in Functional Programming* — [arXiv 2601.02060](https://doi.org/10.48550/arxiv.2601.02060) [plan §10; re-verified session]
- [ai-coding-lang-bench](https://github.com/mame/ai-coding-lang-bench) [plan §10]
- VERA Phase -1 pilot report — [`../pilot/REPORT.md`](../pilot/REPORT.md) [first-party; recorded exit codes]

**2026 AI-first languages:**

- Turn — [arXiv 2603.08755](https://arxiv.org/abs/2603.08755) [plan §10; abstract re-verified session]
- AICore — [github.com/keaz/aicore](https://github.com/keaz/aicore) [plan §10; README re-verified session]
- Kodo — [github.com/rfunix/kodo](https://github.com/rfunix/kodo) [plan §10; README/docs re-verified session]
- Zerolang (Zero) — [zerolang.ai](https://zerolang.ai/), [github.com/vercel-labs/zerolang](https://github.com/vercel-labs/zerolang) [session — newly verified; plan named it without a link]
- Karn — [github.com/karn-lang/karn](https://github.com/karn-lang/karn) [session — newly verified; density numbers self-reported/UNVERIFIED]

**Substrate, effects, verification, security, toolchain (all [plan §10]):**

- [Unison](https://github.com/unisonweb/unison) · [Koka](https://koka-lang.github.io/koka/doc/book.html) · [Flix effects](https://doc.flix.dev/effect-system.html) · [Pony deny capabilities](https://www.ponylang.io/media/papers/fast-cheap.pdf) · [Effekt captures](https://effekt-lang.org/tour/captures) · [Scala 3 capture checking](https://www.scala-lang.org/api/3.x/docs/experimental/capture-checking/basics.html) · [Mojo vision](https://mojolang.org/docs/vision/)
- [SPARK proof of functional correctness](https://learn.adacore.com/courses/intro-to-spark/chapters/05_Proof_Of_Functional_Correctness.html) · [Dafny refinement (arXiv 1606.02022)](https://ar5iv.labs.arxiv.org/html/1606.02022) · [Flux (PLDI'23)](https://ranjitjhala.github.io/static/flux-pldi23.pdf) · ["The Prover Is the Judge" (arXiv 2607.14340)](https://arxiv.org/html/2607.14340v1)
- [CaMeL (arXiv 2503.18813)](https://arxiv.org/abs/2503.18813) · [Jif](https://www.cs.cornell.edu/jif/) · [LIO](http://www.scs.stanford.edu/%7Edm/home/papers/stefan:lio.pdf) · [E / object-capability model](http://erights.org/elib/capability/overview.html) · [SLSA v1.2](https://slsa.dev/spec/v1.2/) · [SLSA provenance](https://slsa.dev/spec/v1.1/provenance) · [PCC (Necula '97)](https://www.cs.tufts.edu/comp/150CMP/papers/necula97pcc.pdf) · [Wasmtime fuel](https://docs.wasmtime.dev/api/wasmtime/struct.Config.html)
- [Salsa/rustc queries](https://rustc-dev-guide.rust-lang.org/queries/salsa.html) · [rust-analyzer architecture](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/architecture.md) · [Cranelift aegraph](https://cfallin.org/blog/2026/04/09/aegraph/) · [MLIR progressive lowering](https://mlir.llvm.org/docs/Tutorials/Toy/Ch-5/) · [WASI Preview 2](https://github.com/WebAssembly/WASI/blob/v0.2.1/README.md) · ["Is Sound Gradual Typing Dead?" (POPL'16)](https://www2.ccs.neu.edu/racket/pubs/popl16-tfgnvf.pdf)
- [Smyth](https://uchicago-pl.github.io/smyth/) · [Hazel](https://hazel.org/) · [Hazelnut (POPL'17)](https://plv.colorado.edu/papers/hazelnut-popl17.pdf) · [JetBrains MPS](https://www.jetbrains.com/help/mps/basic-notions.html) · [GraalVM Truffle](http://lafo.ssw.uni-linz.ac.at/papers/2012_SPLASH_Truffle.pdf) · [monad-par](https://simonmar.github.io/bib/papers/monad-par.pdf) · [Adapton (PLDI'14)](http://matthewhammer.org/adapton/adapton-pldi2014.pdf) · [Perceus](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/11/perceus-tr-v4.pdf) · [Alive2](https://github.com/AliveToolkit/alive2/) · [MLGO](https://llvm.org/docs/MLGO.html) · [Microsoft Coyote](https://microsoft.github.io/coyote/)

*End of research report.*
