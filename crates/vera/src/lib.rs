//! VERA Phase 1–2 — reference front-end + interpreter + thin Z3 VC slice.
//!
//! MVP subset per `docs/spec/SPEC.md` §3 + CONF-P1 store round-trip / edit txs.
//! Phase 2 slice: `--prove` discharges requires/ensures / `{x:Int|pred}` via Z3 SMT-LIB2.

pub mod ast;
pub mod diag;
pub mod interp;
pub mod label;
pub mod lexer;
pub mod mcp;
pub mod parser;
pub mod render;
pub mod smt;
pub mod store;
pub mod typecheck;
pub mod vc;

pub use diag::{diagnose_program, diagnose_source, DiagReport, Diagnostic};
pub use interp::{Console, Interpreter, Trap};
pub use label::{Atom, Label};
pub use parser::{parse, ParseError};
pub use render::render_program;
pub use store::{CodebaseStore, DurableCertStore, EditOp, EditTransaction, StoreError};
pub use typecheck::{check_program, TypeError};
pub use vc::{format_report, prove_program, Discharge, Obligation, ProvedSet, VcError};

/// Canonical lowercase name of the language implemented by this crate.
pub fn language_name() -> &'static str {
    "vera"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    fn examples_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("examples")
    }

    #[test]
    fn round_trip_all_examples() {
        let dir = examples_dir();
        let mut files: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("vera"))
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no examples found in {}", dir.display());
        for path in files {
            let src = fs::read_to_string(&path).unwrap();
            CodebaseStore::round_trip_ok(&src).unwrap_or_else(|e| {
                panic!("round-trip failed for {}: {e}", path.display());
            });
        }
    }

    #[test]
    fn edit_tx_rejects_stale_base() {
        let src = fs::read_to_string(examples_dir().join("hello.vera")).unwrap();
        let prog = parse(&src).unwrap();
        check_program(&prog).unwrap();
        let mut store = CodebaseStore::new();
        let entries = store.load_program(&prog);
        let main_hash = entries
            .iter()
            .find(|e| e.name == "main")
            .unwrap()
            .content_hash
            .clone();

        let mut base = HashMap::new();
        base.insert("main".into(), "deadbeefdeadbeef".into());
        let err = store
            .apply(EditTransaction { base, ops: vec![] })
            .unwrap_err();
        assert!(matches!(err, StoreError::StaleBase { .. }), "{err}");

        // Fresh base + no-op succeeds.
        let mut base_ok = HashMap::new();
        base_ok.insert("main".into(), main_hash);
        store
            .apply(EditTransaction {
                base: base_ok,
                ops: vec![],
            })
            .unwrap();
    }

    #[test]
    fn edit_tx_replace_requires_typecheck() {
        let src = fs::read_to_string(examples_dir().join("hello.vera")).unwrap();
        let prog = parse(&src).unwrap();
        let mut store = CodebaseStore::new();
        store.load_program(&prog);
        let err = store
            .apply(EditTransaction {
                base: HashMap::new(),
                ops: vec![EditOp::ReplaceFn {
                    name: "main".into(),
                    source: "fn main(console: Console) -> Unit uses {console} { 1 }\n".into(),
                }],
            })
            .unwrap_err();
        // body Int != Unit
        assert!(matches!(err, StoreError::Type(_)), "{err}");
    }

    #[test]
    fn language_name_returns_vera() {
        assert_eq!(language_name(), "vera");
    }

    #[test]
    fn gitattributes_pins_eol() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(".gitattributes");
        let text =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("missing {}: {e}", path.display()));
        let lines: Vec<&str> = text.lines().map(str::trim).collect();
        for pin in ["*.rs text eol=lf", "*.md text eol=lf", "*.vera text eol=lf"] {
            assert!(lines.contains(&pin), "missing pin line {pin:?}");
        }
    }

    #[test]
    fn blame_ignore_revs_well_formed() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(".git-blame-ignore-revs");
        let text =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("missing {}: {e}", path.display()));
        let mut hash_lines = 0;
        for line in text.lines().map(str::trim) {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            assert!(
                line.len() == 40 && line.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')),
                "malformed hash line {line:?}"
            );
            hash_lines += 1;
        }
        assert!(hash_lines > 0, "no hash lines in {}", path.display());
    }
}
