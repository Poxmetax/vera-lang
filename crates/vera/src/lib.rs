//! VERA Phase 1–2 — reference front-end + interpreter + thin Z3 VC slice.
//!
//! MVP subset per `docs/spec/SPEC.md` §3 + CONF-P1 store round-trip / edit txs.
//! Phase 2 slice: `--prove` discharges requires/ensures / `{x:Int|pred}` via Z3 SMT-LIB2.

pub mod ast;
pub mod diag;
pub mod interp;
pub mod lexer;
pub mod parser;
pub mod render;
pub mod smt;
pub mod store;
pub mod typecheck;
pub mod vc;

pub use diag::{diagnose_program, diagnose_source, DiagReport, Diagnostic};
pub use interp::{Console, Interpreter, Trap};
pub use parser::{parse, ParseError};
pub use render::render_program;
pub use store::{CodebaseStore, EditOp, EditTransaction, StoreError};
pub use typecheck::{check_program, TypeError};
pub use vc::{format_report, prove_program, Discharge, Obligation, VcError};

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
            .apply(EditTransaction {
                base,
                ops: vec![],
            })
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
}
