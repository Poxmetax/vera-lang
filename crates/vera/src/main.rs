//! CLI: `vera <file.vera> [--hash-only] [--dump-ast] [--round-trip] [--prove]`
//!
//! Run from `vera-lang/` via `cargo run -p vera -- examples/hello.vera`.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use vera::{
    check_program, format_report, parse, prove_program, CodebaseStore, Discharge, Interpreter,
    ProvedSet,
};

fn usage() {
    // [SOFT-PROVE-HELP] clearer Phase-2 flag description for operators / Fable handoff
    eprintln!(
        "usage: vera <file.vera> [--hash-only] [--dump-ast] [--round-trip] [--prove] [--diag-json]"
    );
    eprintln!(
        "  --prove   Phase 2 VC slice: discharge Int/bool/ite requires·ensures·{{x:Int|pred}} via Z3"
    );
    eprintln!(
        "            prints [PROVED] / [RUNTIME-CHECKED] / [REFUTED] (exit 3 if any REFUTED)"
    );
    // [P2B-DIAG] machine-readable diagnostics mode
    eprintln!(
        "  --diag-json  structured JSON diagnostics (parse+typecheck; with --prove also obligations); does not run the program"
    );
    // [P2D-ELIDE] proof-gated run mode
    eprintln!(
        "  --prove-run  prove first, then run with proof-gated check elision (INV-1); any REFUTED -> exit 3, no run. --prove alone never runs and takes precedence"
    );
    // [GAP-D2-CLI] durable prove-cache reconcile (shape B: re-prove-and-compare)
    eprintln!(
        "  --prove-cache <dir>  with --prove: reconcile the durable INV-2 cert store (Z3 still runs; compare + persist; never changes prove results)"
    );
    // [GAP-D2-SOLVER-SKIP] Opt-in: serve exact INV-2 HITs without re-running Z3
    eprintln!(
        "  --prove-cache-skip   with --prove --prove-cache <dir>: skip Z3 for fn-level obligations on an exact INV-2 HIT (fail-closed; call sites always prove fresh; trusts the store -- compare mode is the tamper canary)"
    );
    // [GAP-D2-EVICTION] Opt-in eviction/GC: explicit stale-toolchain prune
    eprintln!(
        "  --prove-cache-prune  with --prove --prove-cache <dir>: prune store entries whose toolchain can no longer HIT under the current one (stale vera/solver generations; solver-free \"none\" entries survive; current-toolchain entries never touched)"
    );
    // [SOFT-EXIT-HELP] document CLI exit codes (trap=2, refute=3)
    eprintln!(
        "exit codes: 0 ok | 1 usage/parse/type/prove-err | 2 runtime trap | 3 any REFUTED (--prove)"
    );
}

/// [GAP-D2-CLI] Re-prove-and-compare cache pass (shape B):
/// Z3 has already run; reconcile the durable INV-2 store with the fresh
/// obligations and report on stderr. Fail-closed at every step -- no solver
/// id, unopenable store, or persist error disables/aborts the cache pass
/// without ever touching prove results.
/// [GAP-D2-EVICTION] With `prune` (opt-in `--prove-cache-prune`), the
/// stale-toolchain prune runs AFTER the reconcile pass -- same write
/// discipline (report first, store maintenance after), same fail-closed
/// gates (no solver id / unopenable store disable it with the notes above).
fn run_prove_cache(
    dir: &Path,
    program: &vera::ast::Program,
    obs: &[vera::Obligation],
    prune: bool,
) {
    let Some(solver_id) = vera::smt::discover_solver_id() else {
        eprintln!("[GAP-D2-CLI] solver id not discovered -- cache disabled (fail-closed)");
        return;
    };
    // [GAP-D2-LOCK] One writer at a time: the whole cache-write session
    // (open -> reconcile -> prune) runs under the store's exclusive advisory
    // lock. Contention or lock failure skips the pass entirely (fail-closed)
    // -- prove results are already printed and unaffected; the next run
    // simply re-reconciles. Held by RAII guard until run_prove_cache exits.
    let _lock = match vera::store::DurableCertStore::try_write_lock(dir) {
        Ok(Some(guard)) => guard,
        Ok(None) => {
            eprintln!(
                "[GAP-D2-LOCK] cache lock held by another writer -- cache pass skipped (fail-closed)"
            );
            return;
        }
        Err(e) => {
            eprintln!("[GAP-D2-LOCK] cache lock failed: {e} -- cache pass skipped (fail-closed)");
            return;
        }
    };
    let mut store = match vera::store::DurableCertStore::open(dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[GAP-D2-CLI] cache open failed: {e} -- cache disabled");
            return;
        }
    };
    match vera::store::reconcile_prove_cache(&mut store, program, obs, &solver_id) {
        Ok(r) => {
            for m in &r.mismatches {
                eprintln!("[GAP-D2-CLI] cache MISMATCH: {m}");
            }
            eprintln!(
                "[GAP-D2-CLI] cache ({solver_id}): {} match, {} MISMATCH, {} persisted, {} not cacheable",
                r.matches,
                r.mismatches.len(),
                r.persisted,
                r.uncacheable
            );
        }
        Err(e) => eprintln!("[GAP-D2-CLI] cache persist failed: {e} -- prove results unaffected"),
    }
    // [GAP-D2-EVICTION] Explicit prune of entries that can never HIT under
    // the discovered toolchain (exact-key equality): dropping is always
    // MISS-safe, solver-free "none" entries survive (design-note rule 2),
    // and a persist failure leaves the on-disk store as it was (temp +
    // atomic rename).
    if prune {
        match store.prune_stale_toolchain(&solver_id) {
            Ok((removed, kept)) => eprintln!(
                "[GAP-D2-EVICTION] cache prune ({solver_id}): {removed} stale-toolchain entr(ies) removed, {kept} kept"
            ),
            Err(e) => {
                eprintln!("[GAP-D2-EVICTION] cache prune failed: {e} -- on-disk store unchanged")
            }
        }
    }
}

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        usage();
        return ExitCode::from(1);
    }
    // [GAP-D2-CLI] valued flag: extract `--prove-cache <dir>` before the
    // boolean-flag retain pass.
    let mut prove_cache_dir: Option<PathBuf> = None;
    if let Some(i) = args.iter().position(|a| a == "--prove-cache") {
        if i + 1 >= args.len() {
            eprintln!("error: --prove-cache requires a directory argument");
            usage();
            return ExitCode::from(1);
        }
        let dir = args.remove(i + 1);
        args.remove(i);
        prove_cache_dir = Some(PathBuf::from(dir));
    }
    let mut hash_only = false;
    let mut dump_ast = false;
    let mut round_trip = false;
    let mut prove = false;
    let mut prove_run = false;
    let mut diag_json = false;
    // [GAP-D2-SOLVER-SKIP] opt-in Z3 skip on exact INV-2 cache HITs.
    let mut cache_skip = false;
    // [GAP-D2-EVICTION] opt-in explicit stale-toolchain prune of the store.
    let mut cache_prune = false;
    args.retain(|a| match a.as_str() {
        "--hash-only" => {
            hash_only = true;
            false
        }
        "--dump-ast" => {
            dump_ast = true;
            false
        }
        "--round-trip" => {
            round_trip = true;
            false
        }
        "--prove" => {
            prove = true;
            false
        }
        "--prove-run" => {
            prove_run = true;
            false
        }
        "--diag-json" => {
            diag_json = true;
            false
        }
        "--prove-cache-skip" => {
            cache_skip = true;
            false
        }
        "--prove-cache-prune" => {
            cache_prune = true;
            false
        }
        _ => true,
    });
    if args.len() != 1 {
        usage();
        return ExitCode::from(1);
    }
    // [GAP-D2-CLI] cache reconcile is scoped to --prove only; the elision
    // path (--prove-run) stays cache-free by design (execution gating must
    // never depend on a cache file).
    if prove_cache_dir.is_some() && !prove {
        eprintln!("[GAP-D2-CLI] --prove-cache has effect only with --prove (ignored)");
    }
    // [GAP-D2-SOLVER-SKIP] skip is meaningful only on the --prove report
    // path with a cache directory; anywhere else it is ignored with a note
    // (--prove-run stays cache-free by design, and --prove takes precedence
    // over --prove-run, so the skip path can never reach elision or a run).
    if cache_skip && !(prove && prove_cache_dir.is_some()) {
        eprintln!(
            "[GAP-D2-SOLVER-SKIP] --prove-cache-skip has effect only with --prove --prove-cache <dir> (ignored)"
        );
    }
    // [GAP-D2-EVICTION] prune is store maintenance on the --prove report
    // path with a cache directory -- same gating as the skip flag (it can
    // never reach elision or a program run).
    if cache_prune && !(prove && prove_cache_dir.is_some()) {
        eprintln!(
            "[GAP-D2-EVICTION] --prove-cache-prune has effect only with --prove --prove-cache <dir> (ignored)"
        );
    }
    let path = PathBuf::from(&args[0]);
    let source = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: read {}: {e}", path.display());
            return ExitCode::from(1);
        }
    };

    if round_trip {
        match CodebaseStore::round_trip_ok(&source) {
            Ok((h, _)) => {
                println!("round-trip OK  program#{h}");
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("round-trip FAIL: {e}");
                return ExitCode::from(1);
            }
        }
    }

    // [P2B-DIAG] machine-readable pipeline diagnostics; does not execute the program.
    if diag_json {
        let report = vera::diag::diagnose_source(&path.display().to_string(), &source, prove);
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("DiagReport serializable")
        );
        return ExitCode::from(report.exit_code());
    }

    let program = match parse(&source) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(1);
        }
    };
    if let Err(e) = check_program(&program) {
        eprintln!("error: {e}");
        return ExitCode::from(1);
    }

    if prove {
        // [SOFT-Z3-PATH] discovery transparency — print resolved binary once (non-fatal if missing)
        match vera::smt::find_z3() {
            Ok(p) => eprintln!("[SOFT-Z3-PATH] using Z3: {}", p.display()),
            Err(e) => eprintln!("[SOFT-Z3-PATH] not resolved: {e}"),
        }
        // [GAP-D2-SOLVER-SKIP] Opt-in skip path: fn-level
        // obligations with an exact INV-2 HIT are served from the store
        // WITHOUT running Z3; misses, duplicate names, poison-shaped
        // entries and every call-site obligation prove fresh. Fail-closed:
        // no discovered solver id / unopenable store -> full fresh prove
        // (the unchanged path below). Store READS happen here (inherent to
        // skipping); WRITES stay in the post-report reconcile pass
        // (shape-B discipline: the report is never affected by persist).
        if cache_skip {
            if let Some(dir) = &prove_cache_dir {
                let ready = match vera::smt::discover_solver_id() {
                    Some(id) => match vera::store::DurableCertStore::open(dir) {
                        Ok(store) => Some((id, store)),
                        Err(e) => {
                            eprintln!(
                                "[GAP-D2-SOLVER-SKIP] cache open failed: {e} -- skip disabled, proving fresh"
                            );
                            None
                        }
                    },
                    None => {
                        eprintln!(
                            "[GAP-D2-SOLVER-SKIP] solver id not discovered -- skip disabled, proving fresh"
                        );
                        None
                    }
                };
                if let Some((solver_id, store)) = ready {
                    let outcome =
                        vera::store::prove_program_skip_cache(&store, &program, &solver_id);
                    print!(
                        "{}",
                        format_report(&path.display().to_string(), &outcome.obligations)
                    );
                    eprintln!(
                        "[GAP-D2-SOLVER-SKIP] cache skip ({solver_id}): {} fn-level obligation(s) served from INV-2 HIT (Z3 not run for those); {} obligation(s) proved fresh",
                        outcome.skipped,
                        outcome.fresh.len()
                    );
                    run_prove_cache(dir, &program, &outcome.fresh, cache_prune);
                    if outcome
                        .obligations
                        .iter()
                        .any(|o| matches!(o.status, Discharge::Refuted { .. }))
                    {
                        return ExitCode::from(3);
                    }
                    return ExitCode::SUCCESS;
                }
            }
        }
        match prove_program(&program) {
            Ok(obs) => {
                print!("{}", format_report(&path.display().to_string(), &obs));
                // [GAP-D2-CLI] optional durable-store reconcile AFTER the
                // report: prove output and exit codes are never affected.
                if let Some(dir) = &prove_cache_dir {
                    run_prove_cache(dir, &program, &obs, cache_prune);
                }
                if obs
                    .iter()
                    .any(|o| matches!(o.status, Discharge::Refuted { .. }))
                {
                    return ExitCode::from(3);
                }
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("prove error: {e}");
                return ExitCode::from(1);
            }
        }
    }

    // [P2D-ELIDE] opt-in run-after-prove (SPEC DP6 / INV-1): prove first, then
    // run with proved fn-level checks elided. Any REFUTED -> report + exit 3
    // without running. The default run path (no flag) elides nothing.
    let mut proved_set: Option<ProvedSet> = None;
    if prove_run {
        match vera::smt::find_z3() {
            Ok(p) => eprintln!("[SOFT-Z3-PATH] using Z3: {}", p.display()),
            Err(e) => eprintln!("[SOFT-Z3-PATH] not resolved: {e}"),
        }
        match prove_program(&program) {
            Ok(obs) => {
                print!("{}", format_report(&path.display().to_string(), &obs));
                if obs
                    .iter()
                    .any(|o| matches!(o.status, Discharge::Refuted { .. }))
                {
                    eprintln!("[P2D-ELIDE] refuted obligation(s) -- not running");
                    return ExitCode::from(3);
                }
                let ps = ProvedSet::build(&program, &obs);
                eprintln!(
                    "[P2D-ELIDE] proof-gated elision armed: {} fn-level obligation(s)",
                    ps.len()
                );
                proved_set = Some(ps);
            }
            Err(e) => {
                eprintln!("prove error: {e}");
                return ExitCode::from(1);
            }
        }
    }

    let mut store = CodebaseStore::new();
    let entries = store.load_program(&program);
    println!("content-addressed definitions:");
    for e in &entries {
        println!("  {}  #{}", e.name, e.content_hash);
    }
    if dump_ast {
        println!(
            "{}",
            serde_json::to_string_pretty(&store.summary()).unwrap()
        );
    }
    if hash_only {
        return ExitCode::SUCCESS;
    }

    let mut interp = match proved_set {
        Some(ps) => Interpreter::with_proved(&program, ps),
        None => Interpreter::new(&program),
    };
    match interp.run_main() {
        Ok(_) => {
            if prove_run {
                eprintln!(
                    "[P2D-ELIDE] elided {} runtime check(s)",
                    interp.elided_checks
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("trap: {e}");
            ExitCode::from(2)
        }
    }
}
