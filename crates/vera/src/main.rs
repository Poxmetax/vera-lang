//! CLI: `vera <file.vera> [--hash-only] [--dump-ast] [--round-trip] [--prove]`
//!
//! Run from `vera-lang/` via `cargo run -p vera -- examples/hello.vera`.

use std::env;
use std::fs;
use std::path::PathBuf;
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
    // [SOFT-EXIT-HELP] document CLI exit codes (trap=2, refute=3)
    eprintln!(
        "exit codes: 0 ok | 1 usage/parse/type/prove-err | 2 runtime trap | 3 any REFUTED (--prove)"
    );
}

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        usage();
        return ExitCode::from(1);
    }
    let mut hash_only = false;
    let mut dump_ast = false;
    let mut round_trip = false;
    let mut prove = false;
    let mut prove_run = false;
    let mut diag_json = false;
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
        _ => true,
    });
    if args.len() != 1 {
        usage();
        return ExitCode::from(1);
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
        match prove_program(&program) {
            Ok(obs) => {
                print!("{}", format_report(&path.display().to_string(), &obs));
                if obs.iter().any(|o| matches!(o.status, Discharge::Refuted { .. })) {
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
                if obs.iter().any(|o| matches!(o.status, Discharge::Refuted { .. })) {
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
        println!("{}", serde_json::to_string_pretty(&store.summary()).unwrap());
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
                eprintln!("[P2D-ELIDE] elided {} runtime check(s)", interp.elided_checks);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("trap: {e}");
            ExitCode::from(2)
        }
    }
}
