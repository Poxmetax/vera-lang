//! CLI: `vera <file.vera> [--hash-only] [--dump-ast] [--round-trip] [--prove]`
//!
//! Run from `vera-lang/` via `cargo run -p vera -- examples/hello.vera`.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use vera::{check_program, format_report, parse, prove_program, CodebaseStore, Discharge, Interpreter};

fn usage() {
    eprintln!(
        "usage: vera <file.vera> [--hash-only] [--dump-ast] [--round-trip] [--prove]"
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

    let mut interp = Interpreter::new(&program);
    match interp.run_main() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("trap: {e}");
            ExitCode::from(2)
        }
    }
}
