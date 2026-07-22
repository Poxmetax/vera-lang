# VERA Language Specification — v0.1 (whitepaper draft)

**Version:** 0.1 (Phase 0 deliverable) · **Date:** 2026-07-19 · **Status:** design specification, pre-implementation
**Companion document:** [`../research/RESEARCH_REPORT.md`](../research/RESEARCH_REPORT.md) (evidence and prior-art grounding for every design decision here)
**First-party evidence:** [`../pilot/REPORT.md`](../pilot/REPORT.md) (Phase -1 pilot, verdict PASS, 2026-07-19)

**Normativity and epistemic labels.** Sections marked *(normative)* bind later phases; sections marked *(informative)* explain. Claims carry the shared convention: **[VERIFIED]** (source- or pilot-backed), **[DESIGN CHOICE]** (a decision, defensible but not empirically forced), **[UNVERIFIED/OPEN]** (must be demonstrated before it may be asserted; each such item maps to a risk in RESEARCH_REPORT §4).

**Global caveat.** All surface syntax in this document is **v0.1 pseudo-syntax, subject to Phase 1 implementation feedback**. Semantics and invariants are the stable part; concrete tokens may change.

---

## Table of contents

- [1. Thesis, design principles, and integration invariants](#1-thesis-design-principles-and-integration-invariants)
- [2. Surface syntax (informative sketch)](#2-surface-syntax-informative-sketch)
- [3. Grammar — MVP core subset (normative for Phase 1)](#3-grammar--mvp-core-subset-normative-for-phase-1)
- [4. Static semantics](#4-static-semantics)
- [5. Dynamic semantics](#5-dynamic-semantics)
- [6. The content-addressed model](#6-the-content-addressed-model)
- [7. Agentic primitives](#7-agentic-primitives)
- [8. Security model (normative)](#8-security-model-normative)
- [9. Standard library sketch](#9-standard-library-sketch)
- [10. Conformance and acceptance milestones](#10-conformance-and-acceptance-milestones)
- [11. Sources](#11-sources)

---

## 1. Thesis, design principles, and integration invariants

### 1.1 Thesis *(normative intent)*

> **Familiar, low-ceremony surface; strict, machine-verified substrate; a prover — not tests — as the ground-truth oracle; and a tight structured edit→verify→auto-repair loop so an AI author reaches "green" fast.** Easy to write, impossible to write wrong *silently*.

The defect classes that the empirical studies show LLMs ship silently (unvalidated input, injection, crypto misuse, hard-coded secrets, unhandled error paths, bounds/overflow errors — RESEARCH_REPORT §1.2) are made unrepresentable or statically caught, while the surface stays inside the fluency distribution of the languages LLMs already write best. The Phase -1 pilot supports the thesis direction: 6/6 buckets caught pre-ship at mean +2.8 logical lines of ceremony **[VERIFIED, first-party]**, with the honest caveat that 2 of 6 catches were contract/property-time rather than compile-time — the gap this spec's §4.4 exists to close.

### 1.2 Design principles *(normative)*

- **DP1 — Familiar surface.** Syntax stays close to the Python/Ruby/Rust mainstream; ceremony is budgeted (every annotation must pay for itself in caught bugs). Braces with a canonical formatter; one canonical rendering (§6.3). **[DESIGN CHOICE**, motivated by arXiv 2502.01853 / 2601.02060.**]**
- **DP2 — Strict substrate.** Static types everywhere, no null, ADTs + exhaustive match, `Option`/`Result` as the only absence/failure story, checked arithmetic by default. **[VERIFIED** as the bug-class killer — pilot buckets 2–5 were caught by exactly this discipline.**]**
- **DP3 — Progressive hardening, erasable types.** Code starts permissive and tightens (types → labels → contracts → proofs) *without changing runtime representation*. Adding a type or label never inserts a runtime wrapper; hardening adds checking, not tax. Pervasive typed/untyped contract boundaries are forbidden (the POPL'16 35x–104x trap). Dynamic islands, when they arrive (post-MVP), are explicit, bounded, and checked only at their declared boundary. *(→ §4.3)*
- **DP4 — One label.** Effects, capabilities, and taint/secrecy are ONE annotation (the unified label lattice, §4.2). A capability is the token that authorizes an effect; taint is data-provenance in the same set. There is no separate effect system, capability system, and taint system to keep coherent. *(→ RESEARCH_REPORT §3.2; risk R2)*
- **DP5 — Dangerous things are never silent.** Every effect/capability a function needs is visible in its signature (`uses {...}`). No ambient authority: authority reaches code only through parameters (capability handles), including in `main`. **[VERIFIED** pattern: Effekt/Scala captures, WASI P2, E/ocap.**]**
- **DP6 — The prover is the judge.** Contracts (`requires`/`ensures`) and refinements are runtime-checked by default and SMT-proved when possible. Contracts are **never mandatory to compile** — mandatory specs would trigger the ceremony-reversion failure mode. A proof, when it exists, replaces the runtime check (INV-1-compatible: elision is proof-gated).
- **DP7 — Deterministic by default.** All nondeterminism (time, randomness, I/O, scheduling) is an effect behind a capability. Same program + same inputs + same fuel ⇒ same observable behavior, including trap points. *(→ §5.1)*
- **DP8 — Machine-readable everything.** Diagnostics are structured JSON with machine-applicable `FixPatch`/`RepairPlan`; the compiler is a service (MCP) as well as a CLI; the codebase is a queryable store, not text files. *(→ §6, Phase 3)*
- **DP9 — Smartness is a dividend of verification, never a bolt-on.** Synthesis, parallelization, incremental execution, and optimization are enabled *because* purity/equivalence/authority are already proven. Anything that cannot be proof-gated is advisory-only or an explicit effect.

### 1.3 Integration invariants *(normative — spec-level rules binding all phases)*

These three rules are what make DP9 sound as the smart layer grows (plan §7). They are conformance requirements, not guidance:

- **INV-1 (correctness is never speculative).** Only correctness-preserving transformations may be automatic. Anything that could change observable behavior is either an explicit effect (visible in the label) or gated by the prover (translation validation for rewrites; guarded, deoptimizable speculation; advisory-only ML).
- **INV-2 (no stale results).** Every cache, memo table, incremental result, and proof certificate is keyed by content hash **plus solver/model/toolchain version**. One keying scheme is shared by the compiler query engine, the runtime memoizer, and the proof cache. A cache not keyed this way is a conformance violation.
- **INV-3 (deterministic by default).** Implicit parallelism/reordering is restricted to computations the checker proves pure/commutative; all other nondeterminism is an explicit effect/capability. This resolves the one real tension in the plan's integration analysis (auto-parallelism vs. determinism).

---

## 2. Surface syntax (informative sketch)

> **Status: v0.1 pseudo-syntax.** This section shows the *full* intended surface (including post-MVP forms: labels, `infer`, actors, policies) so the design is judged as a whole. The MVP grammar in §3 is the strict subset Phase 1 implements. Concrete tokens are subject to Phase 1 feedback; the semantics are the stable part.

### 2.1 General shape

- Braces delimit blocks; blocks are expressions (last expression is the value). Statements end with `;`.
- `//` line comments. One canonical auto-format (§6.3): formatting is not author-controlled, so diffs are semantic.
- Names: `lower_snake` for values/functions, `UpperCamel` for types/variants — the case distinction is load-bearing in patterns (§3).
- No null. Absence is `Option<T>`; failure is `Result<T, E>`; `?` propagates `Err`/`None` to the caller (Rust-familiar).

### 2.2 Declarations, types, labels, contracts

```text
// Structs and enums (ADTs). Construction is call-style with named args for
// structs, positional for enum variants. No brace-literal ambiguity.
struct User { id: Int, name: Str }
enum  Verdict { Approved, Rejected(Str) }

let u = User(id: 7, name: "ada");
let v = Verdict::Rejected("insufficient data");

// Functions: types on parameters, inferred locals, `uses` = the label
// (capabilities/effects the body may exercise), contracts inline.
fn apply_discount(price: Int, pct: Int) -> Int
    requires price >= 0
    requires pct >= 0 && pct <= 100
    ensures  result >= 0 && result <= price
{
    price - (price * pct) / 100
}

// Refinement form: the same property, moved INTO the type, so the checker
// (Phase 2: SMT) discharges call sites statically. One form in MVP: {x: Int | pred}.
fn apply_discount2(price: {p: Int | p >= 0}, pct: {d: Int | 0 <= d && d <= 100})
    -> {r: Int | r >= 0}
{
    price - (price * pct) / 100
}

// Value labels (post-MVP surface; Phase 2 semantics): `T^{...}` carries
// capability captures and data provenance in ONE set.
let raw: Str^{net, untrusted} = net.get("/users/7")?;
```

### 2.3 Worked examples

**E1 — `fetch_and_store` (the plan's flagship example; labels + contracts + quarantine).**

```text
fn fetch_and_store(id: UserId, db: Db, net: Net) -> Result<Unit, Error>
    uses {net, db}                             // the label: what this fn may do
    requires id.value > 0                      // runtime-checked; SMT-proved when possible
    ensures  result is Ok implies db.has(id)   // postcondition
{
    let raw: Str^{net, untrusted} = net.get("/users/" ++ id.show())?;  // data is tainted
    let user: User^{untrusted}   = parse_user(raw)?;   // quarantined parser: uses {} — no tools
    let vetted: User             = validate(user)?;     // explicit endorse point (audited)
    db.insert(id, vetted)?;                             // db.insert accepts only untainted rows
    Ok(unit)
}
```

The injection defense is structural: `db.insert`'s parameter bound excludes `untrusted`, so passing unvetted data is a *type error*, not a code-review finding (pilot bucket 2's static catch, natively).

**E2 — `clamp` with a typed hole (verifier-guided synthesis, S1).**

```text
fn clamp(x: Int, lo: Int, hi: Int) -> {r: Int | r >= lo && r <= hi}
    requires lo <= hi
= ?body
// The author states intent (signature + refinement + precondition) and leaves
// `?body`. The synthesizer proposes terms; only candidates that typecheck AND
// satisfy the refinement are offered (best-of-N gated by the verifier, S11).
```

A hand-written body, for comparison (this is also MVP-legal minus the refinement):

```text
fn clamp(x: Int, lo: Int, hi: Int) -> Int
    requires lo <= hi
    ensures  result >= lo && result <= hi
{
    if x < lo { lo } else { if x > hi { hi } else { x } }
}
```

**E3 — Safe indexing (pilot bucket 6, moved into the type).**

```text
fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int {
    xs.get_unchecked(i)    // legal ONLY because the refinement proves the bound
}
// Call sites must prove their index is in range (SMT, Phase 2) or take the
// total path: xs.get(i) -> Option<Int>.  `len(xs)` inside a refinement is a
// measure (Flux/LiquidHaskell-style) — REQ-REFINE-2 in §4.4.
```

**E4 — Agentic: LLM inference as a typed primitive with a confidence gate (Phase 3).**

```text
enum Sentiment { Positive, Negative, Neutral }

fn triage(llm: Llm, ticket: Str^{untrusted}) -> Result<Sentiment^{untrusted}, TriageError>
    uses {llm}
{
    // Compiler derives the JSON Schema from `Sentiment`; the runtime validates
    // the model's output against it BEFORE binding (Turn-style Cognitive Type
    // Safety). The result value is ALWAYS labeled `untrusted`: schema-valid is
    // not the same as true or safe (VERA's extension beyond Turn).
    let r = infer<Sentiment>(llm, prompt: "Classify: " ++ ticket)?;
    // r : Inferred<Sentiment>  with  r.value : Sentiment^{untrusted},
    //                                r.confidence : Float
    if r.confidence >= 0.8 {
        Ok(r.value)                        // still untrusted; endorsement is separate
    } else {
        Err(TriageError::NeedsHuman)       // confidence operator gates control flow
    }
}
```

**E5 — CaMeL-style policy: untrusted data blocked from a privileged sink (Phase 3).**

```text
quarantine fn parse_iban(raw: Str^{untrusted}) -> Result<Iban^{untrusted}, ParseErr> {
    // `quarantine` forces uses {} — this function can call no tools and
    // perform no effects; it can only compute. (The quarantined-parser half
    // of CaMeL, as a checked language construct.)
    ...
}

policy payments_require_trust {
    deny pay.transfer(to, amount) when untrusted in label(to);
}

fn handle_email(pay: Payments, mail: Mail, ui: Ui) -> Result<Unit, Error>
    uses {pay, mail, ui}
{
    let msg: Str^{untrusted} = mail.read_latest()?;    // tool output: untrusted by construction
    let to = parse_iban(msg)?;                          // Iban^{untrusted}
    // pay.transfer(to: to, amount: 100)?               // ✗ REJECTED: policy + label check
    let confirmed: Iban = ui.confirm_recipient(to)?;    // human-in-loop endorsement (audited)
    pay.transfer(to: confirmed, amount: 100)?;          // ✓ endorsed data may reach the sink
    Ok(unit)
}
```

**E6 — Secrets: non-loggable, non-serializable, capability-born (pilot bucket 4, both vectors).**

```text
fn connect(env: Env, net: Net) -> Result<Db, Error>
    uses {env, net}
{
    // The ONLY constructors for Secret take a capability (env, keystore).
    // `Secret::literal("hunter2")` does not exist in production profiles —
    // closing the hardcode-provenance hole the pilot flagged (caveat 2).
    let key: Secret<Str> = env.secret("DB_KEY")?;
    // console.print(key)     // ✗ type error: Secret<T> has no render to Str
    // json.encode(key)       // ✗ type error: Secret<T> is not serializable
    net.connect("db.internal", auth: key)    // ✓ a sink typed to accept Secret
}
```

**E7 — MVP flavor (exactly what Phase 1 runs; grammar of §3).**

```text
enum Grade { Pass, Fail }

fn grade(score: Int) -> Grade
    requires score >= 0 && score <= 100
{
    if score >= 50 { Grade::Pass } else { Grade::Fail }
}

fn main(console: Console) -> Unit
    uses {console}
{
    let g = grade(72);
    match g {
        Grade::Pass => console.print("pass"),
        Grade::Fail => console.print("fail"),
    }
}
```

---

## 3. Grammar — MVP core subset *(normative for Phase 1)*

**Scope [per plan §9, verbatim]:** Int, Bool, Str, List; user structs + enums (ADTs); Option, Result<T,E>; let, fn/lambda, application, if, exhaustive match; ONE explicit effect (Console/IO) as a capability; requires/ensures on functions; ONE refinement form `{x: Int | pred}`. Everything else in §2 is **post-MVP** (see §3.3).

### 3.1 EBNF

Notation: ISO-style EBNF. `{X}` = zero or more, `[X]` = optional, `|` = choice, terminals in double quotes, `(* ... *)` comments.

```ebnf
(* ============ VERA MVP core, v0.1 ============ *)

program        = { declaration } ;

declaration    = struct_decl | enum_decl | fn_decl ;

(* ---- nominal types (monomorphic in MVP; user type params are post-MVP) ---- *)
struct_decl    = "struct" type_ident "{" [ field { "," field } [ "," ] ] "}" ;
field          = ident ":" type ;

enum_decl      = "enum" type_ident "{" variant { "," variant } [ "," ] "}" ;
variant        = type_ident [ "(" type { "," type } ")" ] ;

(* ---- functions ---- *)
fn_decl        = "fn" ident "(" [ params ] ")" "->" type
                 [ uses_clause ] { contract } ( block | "=" hole ) ;
params         = param { "," param } ;
param          = ident ":" type ;
uses_clause    = "uses" "{" [ cap_atom { "," cap_atom } ] "}" ;
cap_atom       = "console" ;               (* MVP: exactly one capability atom *)
contract       = "requires" expr
               | "ensures" expr ;          (* in "ensures", the contextual name
                                              `result` denotes the return value *)

(* ---- types ---- *)
type           = "Int" | "Bool" | "Str" | "Unit"
               | "List" "<" type ">"
               | "Option" "<" type ">"
               | "Result" "<" type "," type ">"
               | "Console"                  (* the capability type *)
               | type_ident                 (* user struct/enum *)
               | fn_type
               | refinement ;
fn_type        = "fn" "(" [ type { "," type } ] ")" "->" type ;
refinement     = "{" ident ":" "Int" "|" expr "}" ;
                 (* MVP: refinements over Int only; the predicate must be a
                    pure Bool expression over the bound name, the function's
                    parameters, and integer literals *)

(* ---- statements and expressions ---- *)
block          = "{" { stmt } [ expr ] "}" ;
stmt           = "let" ident [ ":" type ] "=" expr ";"
               | expr ";" ;

expr           = if_expr | match_expr | lambda | or_expr ;

if_expr        = "if" expr block "else" ( block | if_expr ) ;
                 (* `if` is an expression; `else` is mandatory *)

match_expr     = "match" expr "{" arm { "," arm } [ "," ] "}" ;
arm            = pattern "=>" expr ;
pattern        = "_"
               | literal
               | ident                              (* lowercase: binder *)
               | path [ "(" pattern { "," pattern } ")" ] ;  (* constructor *)

lambda         = "fn" "(" [ lparams ] ")" [ "->" type ] block ;
lparams        = ident [ ":" type ] { "," ident [ ":" type ] } ;
                 (* lambda parameter types may be omitted when inferable *)

or_expr        = and_expr { "||" and_expr } ;
and_expr       = cmp_expr { "&&" cmp_expr } ;
cmp_expr       = add_expr [ rel_op add_expr ] ;
rel_op         = "==" | "!=" | "<" | "<=" | ">" | ">=" ;
add_expr       = mul_expr { ( "+" | "-" | "++" ) mul_expr } ;
                 (* ++ = Str/List concatenation *)
mul_expr       = unary { ( "*" | "/" | "%" ) unary } ;
unary          = [ "-" | "!" ] postfix ;
postfix        = primary { "(" [ args ] ")"        (* application *)
                         | "." ident               (* field access / method *)
                         | "?" } ;                 (* Err/None propagation *)
args           = arg { "," arg } ;
arg            = [ ident ":" ] expr ;              (* named args: struct ctors only *)

primary        = literal | path | "(" expr ")" | list_lit | hole ;
path           = ident
               | type_ident [ "::" type_ident ] ;  (* Type, Type::Variant *)
list_lit       = "[" [ expr { "," expr } ] "]" ;
literal        = int_lit | str_lit | "true" | "false" | "unit" ;
hole           = "?" ident ;                       (* single lexer token: no space *)

(* ---- lexical ---- *)
ident          = lower_letter { letter | digit | "_" } ;
type_ident     = upper_letter { letter | digit } ;
int_lit        = digit { digit | "_" } ;
str_lit        = '"' { string_char | escape } '"' ;
comment        = "//" { any_char_except_newline } ;
```

### 3.2 Disambiguation and lexical rules *(normative)*

1. **Holes vs. propagation.** `?ident` with no intervening whitespace lexes as a single HOLE token; a bare postfix `?` is Err/None propagation. (`x?` = propagate; `?body` = hole.)
2. **Case is load-bearing.** In patterns, a lowercase `ident` is a *binder*; a capitalized `path` is a *constructor*. This removes the classic "misspelled constructor silently becomes a catch-all binder" bug.
3. **Prelude constructors.** `Some`, `None`, `Ok`, `Err`, `True`-free: booleans are `true`/`false` literals; `Some(x)`, `None`, `Ok(x)`, `Err(e)` are ordinary enum variants of prelude `Option`/`Result`, usable without qualification.
4. **Named vs. positional arguments.** Named args (`User(id: 7, name: "ada")`) are required for struct construction, forbidden elsewhere (MVP). Enum variants take positional args. This is a typing rule, not a grammar rule.
5. **Keywords (reserved in MVP):** `fn let if else match struct enum uses requires ensures true false unit`. **Reserved for post-MVP** (may not be used as identifiers): `actor policy quarantine infer spawn dyn secret endorse declassify invariant type trait impl use`.
6. **No user generics in MVP.** `List`/`Option`/`Result` are built-in generic; user structs/enums are monomorphic until Phase 2+ (plan §9: "generics beyond List" excluded from MVP).
7. **Entry point.** An MVP program's entry is `fn main(console: Console) -> Unit uses {console}` (or `uses {}` for a pure program). The runtime constructs the `Console` handle; user code cannot (§5.3).

### 3.3 Post-MVP extensions *(informative, one paragraph each — normative form lands with their phase)*

- **Value labels `T^{...}`** — the `^`-set on types (§4.2) activates in Phase 2; in MVP the only label surface is the `uses` clause. The token `^` is reserved.
- **Float and numeric widths** — MVP has `Int` only (§5.2); `Float`, sized ints, and `wrapping`/`saturating` variants arrive Phase 2.
- **Generics, traits, `impl`** — Phase 2+.
- **Mutation and ownership** — the MVP core is immutable (`let` only); controlled mutation, ownership/borrowing for hot paths: Phase 4.
- **`invariant` on types, `old()` in postconditions** — with the full contract layer, Phase 2.
- **`dyn` islands** — explicit bounded dynamic regions, Phase 2+ (DP3 constraints apply).
- **Agentic forms** — `infer`, `Inferred<T>`, `actor`/`spawn`/mailboxes, `quarantine`, `policy`, schema absorption: Phase 3 (§7).
- **Effect handlers** — algebraic-effect-style handlers for async/generators/exceptions-as-libraries: Phase 3+ design work; MVP has exactly one effect and no handler syntax.

---

## 4. Static semantics

### 4.1 Type system base *(normative)*

- **Hindley–Milner inference** over a rank-1 polymorphic core (built-in generics only in MVP). Locals never need annotations; function signatures require full parameter/return types **[DESIGN CHOICE:** signatures are the machine-readable interface agents and the store index on; inference hides inside bodies only**]**.
- **Algebraic data types** (structs, enums) with **exhaustive `match`**: a non-exhaustive match is a compile error, with the missing constructors named in the diagnostic (and a `FixPatch` adding the arms, Phase 2+).
- **No null, no exceptions in the core.** Absence = `Option`, failure = `Result`, propagation = `?`. `unwrap()` exists (pilot caveat 3) but is lint-flagged and forbidden in `deny`-profile modules (§8, SEC2 spec-quality gates).
- **Nominal typing** for structs/enums; structural refinements layer on top (§4.4).

### 4.2 The unified label lattice *(normative design; Phase 2 implementation)*

**The one new type-system concept in VERA** (plan §8 complexity guard). Everything else in this spec is borrowed machinery.

**Atoms.** A label is a finite set of atoms, `A = A_auth ∪ A_data`:

- **Authority atoms** (`A_auth`): capability names — MVP: `console`; Phase 2+: `net`, `fs`, `db`, `env`, `clock`, `rand`, `llm`, user-declared capabilities. On a *function*, the `uses` set is the authority it may exercise; on a *value* (closure, handle), the capture set is the authority it embodies.
- **Data atoms** (`A_data`): provenance/secrecy marks — `untrusted` (integrity: content may be attacker-influenced) and `secret` (confidentiality: content must not reach public sinks). Fixed set in v0.1 **[DESIGN CHOICE:** a two-atom data vocabulary keeps the lattice small; user-defined data atoms are a post-v1 question**]**.

**Lattice.** Labels are ordered by set inclusion: `L1 ⊑ L2 iff L1 ⊆ L2`; join = ∪, meet = ∩, ⊥ = ∅ (pure computation, trusted public data), ⊤ = all atoms. "Lower is better" uniformly: fewer capabilities needed, less taint, less secrecy.

**Subsumption.** A labeled type is covariant in its label upper bound:

```text
(SUB-LABEL)      T^{L1}  <:  T^{L2}      iff  L1 ⊆ L2
```

A value may flow anywhere that *permits at least* its label. A parameter's declared type is an **upper bound** on what it accepts. This one rule does double duty:

- *Integrity sinks:* `db.insert(row: User)` — plain `User` bounds the label at ∅-data, so `User^{untrusted}` is rejected (injection, pilot bucket 2).
- *Confidentiality sinks:* `console.print(s: Str)` bounds at ∅-data, so `Str^{secret}` is rejected (leak, pilot bucket 4). A sink that legitimately handles secrets says so: `net.connect(auth: Secret<Str>)`.

**Propagation.** Data atoms propagate through computation by join; authority atoms propagate into *closures* (capture) and accumulate in the `uses` requirement of callers:

```text
(TAINT-PROP)   Γ ⊢ e1 : Int^{L1}    Γ ⊢ e2 : Int^{L2}
               ────────────────────────────────────────
               Γ ⊢ e1 + e2 : Int^{ data(L1) ∪ data(L2) }

(APP)          Γ ⊢ f : fn(P1..Pn) -> R uses C_f       Γ ⊢ a_i : T_i^{L_i}
               T_i^{L_i} <: P_i   (each argument within its bound)
               C_f ⊆ C_ctx        (caller's context provides the authority)
               ────────────────────────────────────────
               Γ ; C_ctx ⊢ f(a_1 .. a_n) : R

(CAPTURE)      a lambda's type carries the union of the authority atoms of
               everything it closes over: capturing a Net handle yields
               fn(...) -> ...  ^{net}
```

**Inference stance [DESIGN CHOICE, risk R2].** Labels are *inferred* within module bodies and *explicit* at module boundaries and sinks. The Phase 2 ergonomics gate (RESEARCH_REPORT §4, R2): on the example corpus, no label annotations should be needed except at boundaries/sinks, and inferred label sets must stay human-readable (small). If inference ergonomics fail, the documented fallback is splitting authority and data atoms into two simpler checkers.

**Declassification/endorsement.** Removing an atom is never implicit. `endorse(v, reason)` strips `untrusted`; `declassify(v, reason)` strips `secret`. Both require a dedicated capability atom in the caller's `uses` (`endorse` / `declassify`), are audited (§8 audit log), and are the designated human-in-loop points (E5).

**Implicit flows. [UNVERIFIED/OPEN]** v0.1 tracks *explicit* data flow only. Branch-on-secret/branch-on-untrusted leakage (implicit flows) is not yet controlled; the reference designs are LIO's floating context label and Jif's program-counter label. Phase 2/3 must pick one for effect-bearing contexts (a candidate rule: a context that has observed `untrusted` data in its branch condition may not exercise privileged capabilities without a policy check). Until then VERA must not claim full IFC — only explicit-flow taint safety.

### 4.3 Erasability *(normative — the U13 rule)*

- Types, labels, refinements, and *proved* contracts are **erasable**: for a well-typed program, erasing them yields a program with identical runtime representation and identical observable behavior (except that unproved contracts compile to boundary checks, which are visible as deterministic traps on violation).
- Adding an annotation never changes a value's representation, inserts a wrapper, or adds a crossing cost. There are **no pervasive typed↔untyped contract boundaries** (the POPL'16 trap, RESEARCH_REPORT §4 R5).
- Dynamic islands (post-MVP `dyn` blocks) are the only places representation may differ, and values crossing an island boundary are checked exactly once, at the boundary, with the check visible in the label.

### 4.4 Contracts and refinements *(normative; contains the Phase 2 conformance gate)*

**Layering.** Two mechanisms, one below the other:

1. **Refinement types** `{x: Int | pred}` — for the ubiquitous, decidable properties (ranges, bounds, non-zero divisors, non-negative sizes). Mostly *inferred* (Flux-style), checked by SMT over decidable fragments (v0.1 scope: quantifier-free linear integer arithmetic, QF_LIA, plus measures like `len`). This is the cheap layer an LLM should almost never have to write by hand.
2. **Contracts** `requires`/`ensures` (later `invariant`) — for arbitrary functional properties. **Runtime-checked by default; SMT-proved when possible; never mandatory** (DP6). When the prover discharges an obligation, the runtime check is elided (proof-gated, INV-1). Contract expressions must be pure (`uses {}` — enforced).
3. **Contracts double as oracles** (U9): every `ensures` auto-generates property tests and fuzz targets; the same spec text yields both formal and empirical checking.

**Obligation flow.** Each call site of a refined/contracted function yields verification conditions. Discharge order: (1) trivial/syntactic, (2) refinement SMT (decidable fragment, multi-prover Z3/CVC5 — U7), (3) contract SMT (may time out → stays a runtime check), (4) runtime check. The diagnostic always states *which* tier handled each obligation — an agent can see "proved" vs "will check at runtime" per call site (DP8).

**REQ-REFINE — the Phase 2 conformance gate (discharges pilot caveat 1; risk R1).** The pilot could not demonstrate static value-range/bounds checking (no SMT installed); its buckets 1 and 6 were caught at contract/property time only. Phase 2 is **required** to demonstrate, with zero execution of the program under test:

- **REQ-REFINE-1 (bucket 1, input validation static).** Given `fn apply_discount(price: {p: Int | p >= 0}, pct: {d: Int | 0 <= d && d <= 100}) -> {r: Int | r >= 0}`, the call `apply_discount(100, 150)` is rejected at compile time (refinement violation at the call site), and a body that could return a negative result is rejected at definition time.
- **REQ-REFINE-2 (bucket 6, bounds static).** Given `fn nth(xs: List<Int>, i: {k: Int | 0 <= k && k < len(xs)}) -> Int`, a call with a provably out-of-range index (e.g. literal `-1`, or `len(xs)` itself) is rejected at compile time; `len` is available in refinements as a measure. An index the prover cannot bound forces the caller through the total API (`get -> Option`) or an explicit runtime-checked assertion.
- **Status marker.** Until both hold on the Phase 2 checker, every claim of "static bounds/range checking" in VERA material must carry **[UNVERIFIED/OPEN]**. The runtime-contract fallback (demonstrated by the pilot **[VERIFIED, first-party]**) remains sound regardless.

### 4.5 Exhaustiveness, totality, purity *(normative)*

- `match` exhaustiveness is required (§4.1). `if` without `else` is not an expression form (grammar).
- MVP functions are total up to deterministic traps (§5.2) and general recursion; termination checking is **not** claimed (optional totality checking for critical code is a Phase 3+ knob, per plan SEC3).
- A function with `uses {}` is pure: no effects, no capability access. Purity is what the smart layer keys on (INV-3, S7/S8) — it is a *theorem from the label*, not an attribute the author asserts.

---

## 5. Dynamic semantics

### 5.1 Evaluation model *(normative)*

- **Strict call-by-value**, left-to-right evaluation of arguments, sequential statement order. Blocks evaluate to their final expression, `Unit` otherwise.
- **Deterministic by default (DP7/INV-3):** the core language has *no* observable nondeterminism. Time, randomness, I/O, scheduling, and LLM inference are effects behind capability atoms; a program whose label is `{}` is a pure function of its inputs. Same program + inputs + fuel ⇒ identical observable behavior including trap points. Record/replay (Phase 3, U10) records only capability-boundary events.
- **Traps.** Contract violation, refinement runtime-check failure, checked-arithmetic overflow, division by zero, fuel exhaustion, and policy denial (Phase 3) are **deterministic traps**: they abort the program (MVP) with a structured diagnostic (JSON: kind, definition hash, source span, message). Traps are not catchable in MVP **[DESIGN CHOICE:** failure-is-a-value (`Result`) is the recoverable path; catchable traps would reintroduce hidden control flow**]**.

### 5.2 Numbers and checked arithmetic *(normative)*

- MVP `Int` is a 64-bit signed integer. `+ - * /` **trap deterministically on overflow** and `/ %` on zero divisor (plan SEC5: checked arithmetic by default). `wrapping_*` / `saturating_*` variants arrive Phase 2 as explicit opt-ins.
- **[DESIGN CHOICE]** Arbitrary-precision `Int` (Python-style) was considered and rejected for v0.1: it hides a performance cliff and diverges from the plan's checked-arithmetic posture; refinements (§4.4) are the designated mechanism for *statically* excluding overflow where it matters.

### 5.3 Capabilities at runtime *(normative)*

- A capability is an **unforgeable runtime handle** paired with its authority atom. User code cannot mint one: handles enter a program only through `main`'s parameters (injected by the runtime), through explicit grant from code that already holds them, or (Phase 3+) through attenuation of a broader handle. There is no global `Console::new()`, no ambient `os.system` — DP5.
- MVP ships exactly one: `Console` (`.print(s: Str) -> Unit`, `.read_line() -> Str`; the latter's result is labeled `untrusted` from Phase 2 on).
- Phase 3 adds **attenuation** (derive a weaker facet: `fs.subdir("/tmp")`), **revocation** (revocable forwarders/membranes), per plan SEC4 (E/ocap lineage). The WASI Preview 2 component model is the sandbox-level enforcement of the same handles (U5): the type system and the runtime tell one story.

### 5.4 `Secret<T>` *(normative)*

- `Secret<T>` is a library type over the `secret` data atom: statically, it is `T^{secret}` plus API discipline; §4.2's sink rule blocks it from print/serialize/log sinks at compile time.
- Runtime behavior: `display`/`debug` render as `"<redacted>"`; serialization hooks are absent by construction; comparison is constant-time where the platform allows (Phase 2+, SEC5); the payload is **zeroized on drop** — guaranteed in the Phase 4 native runtime; best-effort in the Phase 1 interpreter (host GC may copy) **[honest limitation, stated here so it is never laundered into a guarantee]**.
- **Construction is capability-gated** (closes pilot caveat 2's provenance hole): the only constructors are `env.secret(name)`, `keystore.load(id)`, and similar capability-bearing sources. A literal constructor exists only in the `dev` profile and is a deny-level lint in production profiles.

### 5.5 Fuel metering *(normative; Phase 3 full enforcement)*

- Execution is **fuel-metered** (Wasmtime-style, SEC3): every evaluation step consumes fuel from a per-execution budget set by the embedder/CLI. Exhaustion is a deterministic trap (same point every run — fuel is part of the semantics, unlike wall-clock).
- Memory is capped by a resource limiter (Phase 3). Wall-clock/epoch interruption exists only as a non-deterministic *backstop* outside observable semantics (ops emergency stop), per plan SEC3.

---

## 6. The content-addressed model *(normative design; Phase 1 implementation)*

### 6.1 Definitions are hashes

- The unit of code is the **definition** (function, type, constant). Each definition is stored under a cryptographic hash of its **canonicalized AST**: parse → resolve every reference to the referent's hash → alpha-normalize binders → canonical serialization → hash. Comments and formatting are metadata, not hash input.
- **Names are metadata:** a namespace maps names → hashes. Renames edit the namespace only — zero code churn, instantly propagated, semantically impossible to "miss a call site."
- Hash function: decided in Phase 1 **[open implementation decision:** Unison precedent is a 512-bit SHA-3 family hash; BLAKE3 is the performance candidate; the spec requires collision resistance and stability across toolchain versions**]**.
- The store is **append-only**: a definition, once hashed, is immortal; "editing" creates a new definition and rebinds the name. History, diffs, and rollback are free consequences.

### 6.2 Edits are typed transactions (the never-broken invariant)

- Agents and tools do not edit text; they submit **edit transactions**: `{expected_base: [hashes], new_definitions: [...], rebinds: [name -> hash]}`.
- The store **typechecks the transaction before commit**. On success: atomic namespace update. On failure: a structured conflict/diagnostic (with `FixPatch` where computable) and **no state change**. Stale `expected_base` hashes are rejected (optimistic concurrency — same idea Zerolang ships as graph-hash guards).
- **Invariant (normative): the committed codebase always typechecks.** There is no "broken build" state an agent can wedge the project into mid-refactor. Combined with the structural edit API (S2), syntactically invalid states are unrepresentable and semantically invalid states are uncommittable.

### 6.3 Dual projection

- The AST is the source of truth; **surface text is a projection**. Two canonical projections render from the same definition: the **human syntax** (§2/§3) and a **token-dense LLM projection** (U14; Karn-motivated) for context-window economy.
- Round-trip law (normative): `parse(render_h(d)) ≡ d ≡ parse(render_llm(d))` for every definition `d`. The formatter is total and deterministic — there are no formatting choices to encode in the AST, so diffs are always semantic.

### 6.4 Caching, provenance, and metadata

- Every derived result — typecheck verdict, inferred labels, proof certificate, test result, compiled artifact — is cached keyed by `(definition hash, query kind, toolchain version, solver/model version)` (**INV-2, normative**). An edit re-verifies only the affected dependency cone (U1 query engine).
- **Provenance metadata** (`authored_by`, `confidence`, `reviewed_by` — Kodo lineage) attaches to definitions *in the store*, not as source text: forging a review requires forging store metadata, not typing an annotation. Transitive confidence and `min_confidence` deploy gates read this metadata. **[Honest limit:** like Kodo, the root of trust is external — store-level signing (SEC6/sigstore) is the Phase 4-5 answer; until then provenance is tamper-evident within the store, not cryptographically bound to identities.**]**
- Proof certificates are content-addressed like everything else (plan §8): security re-verification after an edit touches only the changed cone.

---

## 7. Agentic primitives *(normative design; Phase 3 implementation)*

### 7.1 `infer` — LLM inference as a typed primitive

- `infer<T>(llm, prompt: ...) -> Result<Inferred<T>, InferError>`, where `T` is a struct/enum. The compiler derives a JSON Schema from `T`; the runtime sends it with the request, **validates the model output against the schema before binding**, and retries per a bounded, declared policy (Turn's Cognitive Type Safety).
- `Inferred<T>` carries `value: T^{untrusted}` and `confidence: Float`. **The `untrusted` atom is not optional** (VERA's extension beyond Turn): schema validity proves *shape*, never *truth or safety*; only explicit endorsement (§4.2) removes the atom. Requires the `llm` capability atom.

```text
(INFER)   Γ ⊢ m : Llm     Γ ⊢ p : Str     schema(T) derivable     llm ∈ C_ctx
          ───────────────────────────────────────────────────────
          Γ ; C_ctx ⊢ infer<T>(m, prompt: p) : Result<Inferred<T>, InferError>
          with  value-label(result.value) ⊇ {untrusted}
```

### 7.2 `confidence`

Model certainty is data, not vibes: `r.confidence` is an ordinary `Float` computed by the inference layer (provider-reported or calibration-derived — Phase 3 defines the sourcing), so gating is ordinary control flow (`if r.confidence >= 0.8 {...}`) and thresholds are ordinary reviewable constants. It composes with store-level confidence metadata (§6.4) but is a distinct concept: *this inference's* certainty vs. *this definition's* trust.

### 7.3 Actors

- `actor` declarations with `spawn`, typed mailboxes, `send` (fire-and-forget) and `ask` (request/response). Each actor has an **isolated heap, context, memory, and mailbox** (Turn/Erlang/Pony lineage); message passing is the only interaction; messages are values (no shared mutable state) — data-race freedom by construction.
- Scheduling is an effect: production scheduling is nondeterministic *behind the capability*, and the **test scheduler is deterministic and systematic** (Coyote-style, U11) — interleavings are explorable and replayable.

### 7.4 Quarantine and the planner split (CaMeL as language semantics)

- `quarantine fn` forces `uses {}` **and** forbids capability-typed parameters: a quarantined function can only compute. It is the designated place to parse/interpret untrusted content (the quarantined-LLM half of CaMeL).
- The **privileged planner** pattern is then a type discipline, not a convention: privileged code holds capabilities but consumes untrusted data only through quarantined parsers and endorsement gates. Control-flow-from-trusted-query-only is enforced at v0.1 strength by explicit-flow taint (§4.2) plus policy gates (§7.5); full implicit-flow control is the §4.2 OPEN item.

### 7.5 Policy gates

- `policy` declarations attach predicates over argument labels/values to capability operations: `deny pay.transfer(to, amount) when untrusted in label(to);`
- Policies are checked at **every effect call** (the runtime gate the capability already passes through), statically discharged where labels prove them, dynamically otherwise. Denial is a deterministic trap (or `Err` at declared fallible gates) with a structured diagnostic naming the policy.
- Every capability crossing (allowed or denied) can emit a **tamper-evident, content-addressed audit record** (plan §8's "capability audit log for free").

### 7.6 Schema absorption

`use schema::openapi("api.json")` (later `graphql`, `mcp`) synthesizes typed, capability-scoped bindings at compile time (Turn precedent). Absorbed endpoints are ordinary functions with honest labels (`uses {net}`; results `^{untrusted}`).

---

## 8. Security model *(normative)*

Threat model: AI-agent-authored code, prompt-injected data, spec-gaming authors, compromised dependencies, resource-exhaustion — on top of classical memory/type safety (which the substrate provides by construction: Perceus + no raw pointers + checked arithmetic). Guiding rule (plan §8): **security rides the same substrate** — labels on existing types, policies on existing effect gates, certificates from existing proofs. Exactly one new type-system concept (the label lattice, §4.2).

- **SEC1 — Untrusted-data flow / prompt injection [CRITICAL].** All external input (tool output, `infer` results, `read_line`, network) is born `untrusted`. Privileged sinks bound their parameters at trusted; quarantined parsers compute without capabilities; policies gate effect calls; endorsement is explicit, capability-bearing, audited (§4.2, §7.4–7.5). *Phase 3 acceptance: a CaMeL-style AgentDojo-class injection is blocked by the label/policy check (plan §9).*
- **SEC2 — Verifier trust and spec-gaming [HIGH].** (a) Proof-carrying code: each proved definition ships a machine-checkable certificate; a small independent checker (TCB: logic + machine semantics, not the prover) re-validates it (Necula precedent — checking ≪ proving). (b) Spec-quality gates: mutation testing of specs (a spec no mutant violates is vacuous → flagged), required negative/security properties for sink-adjacent code, `unwrap`-free deny profiles, and human review for security-critical specs. *Phase 4.*
- **SEC3 — Resource exhaustion [MEDIUM].** Deterministic fuel + memory limits (§5.5); epoch wall-clock backstop outside observable semantics; optional totality checking for critical code. *Phase 3; acceptance: fuel halts an infinite loop deterministically (plan §9 Phase 4 criterion).*
- **SEC4 — Capability hygiene [MEDIUM].** Attenuation, revocable forwarders, membranes, POLA defaults, rights amplification only via explicit patterns — all as operations on the existing capture sets (E/ocap lineage). *Phase 3.*
- **SEC5 — Secrets, arithmetic, side channels [MEDIUM/MICRO].** `Secret<T>` (§5.4) with capability-gated construction; checked arithmetic default (§5.2); constant-time primitives and **reuse of verified crypto** (HACL*/libsodium class — never roll our own) as audited library + checker, not a language guarantee; timing side channels explicitly *out of scope* for the proof claims (SPARK's honest caveat, inherited).
- **SEC6 — Supply chain [HIGH].** Content addressing proves *what*; SLSA-style signed provenance + sigstore transparency logs prove *where from*; **capability-scoped dependencies** bound *what it may do* (a package's authority = its declared label — Flix's motivation made enforceable); reproducible builds fall out of determinism; cargo-vet-style audit records ride the store. *Phase 4–5.*

**Complexity guard (normative).** Net new type-system concepts: **one** (the label). Everything else is a runtime knob (fuel), a library type (`Secret<T>`), tooling (PCC checker, provenance, audit log), or a policy on an existing boundary. Rejected as over-complex for v1 (plan §8): full dependent types for security, dynamic per-value declassification without authority, language-level constant-time *guarantees*.

---

## 9. Standard library sketch *(informative, MVP-focused)*

The pilot's 64-logical-line hand-rolled substrate (`Option/Result`, `Tainted/Trusted`, `Secret`) is the empirical seed of this stdlib **[VERIFIED, first-party]** — with one structural difference: `Tainted`/`Trusted` wrapper types **do not exist** in VERA, because the label lattice does that job natively (§4.2). `Secret<T>` remains a real library type (over the `secret` atom, §5.4).

**Phase 1 ships (MVP):**

- `prelude` — `Option<T>` (`map`, `unwrap_or`, `is_some`...), `Result<T,E>` (`map`, `map_err`, `unwrap_or`, lint-flagged `unwrap`), `Bool`/`Int`/`Str` intrinsics (`show`, `parse_int -> Option<Int>`, `++`).
- `list` — persistent `List<T>`: `len`, `get -> Option<T>` (total), `head`/`tail -> Option`, `append`/`++`, `map`, `filter`, `fold`. No partial index operator in MVP; `get_unchecked` arrives only with refinements (§4.4, E3).
- `console` — the `Console` capability (`print`, `read_line`).
- `contract` — the runtime-check machinery behind `requires`/`ensures` and its structured trap diagnostics.

**Phase 2 adds:** label intrinsics (`endorse`/`declassify` + their capability atoms), `Secret<T>`, `Float`, sized/`wrapping`/`saturating` arithmetic, `Map<K,V>`, measures (`len` in refinements), property-test/fuzz harness from contracts (U9).

**Phase 3 adds:** `Llm`/`Inferred<T>`/`infer`, actors + mailboxes + deterministic test scheduler, `policy` runtime, capability suite (`Net`, `Fs`, `Env`, `Clock`, `Rand` — each an atom + handle + WASI mapping), schema absorption, record/replay.

**Phase 4-5 adds:** verified-crypto bindings, constant-time primitives, PCC checker tooling, provenance/signing, package manager surface.

---

## 10. Conformance and acceptance milestones *(normative)*

Progression is gated on working artifacts, not calendar (plan §9). Restated as conformance milestones:

- **CONF-P1 (front-end + store).** `.vera` hello-world and small programs run under the tree-walking interpreter; the content-addressed store round-trips definitions (parse → hash → render → parse = identity); invalid syntax is unrepresentable through the structural edit API; edit transactions reject stale bases with structured conflicts.
- **CONF-P2 (verification core).** The type + label checker passes its soundness suite (well-typed programs don't get stuck; ill-labeled flows rejected — including the E1/E5 injection shape and the E6 leak shape); **REQ-REFINE-1 and REQ-REFINE-2 demonstrated** (§4.4 — pilot buckets 1 and 6 caught statically, zero execution); ≥1 contract SMT-proved end-to-end with its runtime check elided; JSON diagnostics with `FixPatch` emitted; label-inference ergonomics gate passed or fallback invoked (R2).
- **CONF-P3 (agentic layer).** `infer` validates output against a derived schema and labels it `untrusted`; a CaMeL-style policy blocks an AgentDojo-class injection (SEC1 acceptance); the MCP compiler-service answers typecheck/prove; fuel metering halts a runaway loop deterministically; deterministic test scheduler reproduces an actor interleaving bug.
- **CONF-P4 (native + performance).** Native and WASM/WASI binaries run; ≥1 optimization ships translation-validated (Alive2-style); Perceus RC + FBIP measured; PCC certificate checked by the independent checker.
- **CONF-P5 (ecosystem).** Package install + semantic diff + LSP hover work against the store.

**Non-goals (v1) — verbatim from plan §9:**

- Not a general-purpose Python/Rust replacement in year 1.
- Not targeting embedded/no-std; GPU/accelerators are a Phase-4 stretch, not v1.
- Not a full interactive theorem prover — only SMT-automatable properties.
- Not source-compatible with any existing language.

**MVP subset (restated from plan §9, the contract for §3):** Int, Bool, Str, List; user structs + enums; Option, Result; let, fn/lambda, application, if, exhaustive match; ONE effect (Console/IO) as a capability; requires/ensures; ONE refinement form. Excluded from MVP: actor runtime, metaprogramming, JIT, ownership, generics beyond List.

**License (plan §9):** permissive open source, Apache-2.0 planned (Turn precedent); research prototype.

---

## 11. Sources

This spec cites the same verified source base as the research report; see [`../research/RESEARCH_REPORT.md` §5](../research/RESEARCH_REPORT.md#5-sources) for the full annotated list (provenance-keyed: plan §10 pre-verified / session-verified / first-party). Load-bearing citations for this document:

- Pilot (first-party evidence, PASS + caveats): [`../pilot/REPORT.md`](../pilot/REPORT.md)
- Unified-mechanism sources: [Effekt captures](https://effekt-lang.org/tour/captures), [Scala 3 capture checking](https://www.scala-lang.org/api/3.x/docs/experimental/capture-checking/basics.html), [Jif](https://www.cs.cornell.edu/jif/), [LIO](http://www.scs.stanford.edu/%7Edm/home/papers/stefan:lio.pdf), [E/ocap](http://erights.org/elib/capability/overview.html), [CaMeL (arXiv 2503.18813)](https://arxiv.org/abs/2503.18813)
- Verification layer: [Flux (PLDI'23)](https://ranjitjhala.github.io/static/flux-pldi23.pdf), [SPARK](https://learn.adacore.com/courses/intro-to-spark/chapters/05_Proof_Of_Functional_Correctness.html), [Dafny (arXiv 1606.02022)](https://ar5iv.labs.arxiv.org/html/1606.02022), ["The Prover Is the Judge" (arXiv 2607.14340)](https://arxiv.org/html/2607.14340v1), [PCC (Necula '97)](https://www.cs.tufts.edu/comp/150CMP/papers/necula97pcc.pdf)
- Substrate and runtime: [Unison](https://github.com/unisonweb/unison), [Salsa/rustc queries](https://rustc-dev-guide.rust-lang.org/queries/salsa.html), [WASI Preview 2](https://github.com/WebAssembly/WASI/blob/v0.2.1/README.md), [Wasmtime fuel](https://docs.wasmtime.dev/api/wasmtime/struct.Config.html), [Perceus](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/11/perceus-tr-v4.pdf), [Coyote](https://microsoft.github.io/coyote/), [Turn (arXiv 2603.08755)](https://arxiv.org/abs/2603.08755)
- Trap to avoid: ["Is Sound Gradual Typing Dead?" (POPL'16)](https://www2.ccs.neu.edu/racket/pubs/popl16-tfgnvf.pdf)

*End of specification v0.1.*
