//! Content-addressed definition store + typed edit transactions (Phase 1 / U16 / CONF-P1).
//! Hash: BLAKE3 of canonical JSON (spans excluded); first 16 hex chars.

use crate::ast::{FnDecl, Program};
use crate::parser::{parse, ParseError};
use crate::render::render_program;
use crate::typecheck::{check_program, TypeError};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct DefEntry {
    pub name: String,
    pub content_hash: String,
}

#[derive(Debug, Clone, Default)]
pub struct CodebaseStore {
    /// Committed program — always typechecks when `Some` (store invariant).
    program: Option<Program>,
    by_name: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("parse: {0}")]
    Parse(#[from] ParseError),
    #[error("type: {0}")]
    Type(#[from] TypeError),
    #[error("stale base: expected {name}#{expected}, have {actual}")]
    StaleBase {
        name: String,
        expected: String,
        actual: String,
    },
    #[error("unknown definition {0}")]
    Unknown(String),
    #[error("duplicate definition {0}")]
    Duplicate(String),
    #[error("empty store")]
    Empty,
}

/// Typed transaction on the semantic graph (U16).
#[derive(Debug, Clone)]
pub enum EditOp {
    /// Replace an existing function by name (AST-level; source is re-parsed as a program fragment).
    ReplaceFn { name: String, source: String },
    /// Insert a new function from source (single `fn` declaration).
    InsertFn { source: String },
    /// Delete a function by name.
    DeleteFn { name: String },
}

#[derive(Debug, Clone)]
pub struct EditTransaction {
    /// Expected content hashes at base (name → hash). Missing keys are not checked.
    pub base: HashMap<String, String>,
    pub ops: Vec<EditOp>,
}

impl CodebaseStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hash_def(fn_decl: &FnDecl) -> String {
        let value = serde_json::to_value(fn_decl).expect("FnDecl serializable");
        let canonical = serde_json::to_string(&value).expect("Value serializable");
        let hash = blake3::hash(canonical.as_bytes());
        hash.to_hex()[..16].to_string()
    }

    pub fn hash_program(program: &Program) -> String {
        let value = serde_json::to_value(program).expect("Program serializable");
        let canonical = serde_json::to_string(&value).expect("Value serializable");
        let hash = blake3::hash(canonical.as_bytes());
        hash.to_hex()[..16].to_string()
    }

    pub fn load_program(&mut self, program: &Program) -> Vec<DefEntry> {
        let mut out = Vec::new();
        let mut map = HashMap::new();
        for f in &program.functions {
            let h = Self::hash_def(f);
            map.insert(f.name.clone(), h.clone());
            out.push(DefEntry {
                name: f.name.clone(),
                content_hash: h,
            });
        }
        self.by_name = map;
        self.program = Some(program.clone());
        out
    }

    pub fn program(&self) -> Option<&Program> {
        self.program.as_ref()
    }

    pub fn summary(&self) -> Vec<serde_json::Value> {
        self.by_name
            .iter()
            .map(|(name, hash)| {
                serde_json::json!({
                    "name": name,
                    "hash": hash,
                })
            })
            .collect()
    }

    /// CONF-P1: parse → hash → render → parse → hash equality.
    pub fn round_trip_ok(source: &str) -> Result<(String, String), StoreError> {
        let p1 = parse(source)?;
        check_program(&p1)?;
        let h1 = Self::hash_program(&p1);
        let rendered = render_program(&p1);
        let p2 = parse(&rendered)?;
        check_program(&p2)?;
        let h2 = Self::hash_program(&p2);
        if h1 != h2 {
            return Err(StoreError::Type(TypeError(format!(
                "round-trip hash mismatch: {h1} != {h2}\n--- rendered ---\n{rendered}"
            ))));
        }
        Ok((h1, rendered))
    }

    /// Apply a typed edit transaction. On any failure the store is unchanged.
    pub fn apply(&mut self, tx: EditTransaction) -> Result<Vec<DefEntry>, StoreError> {
        let mut prog = self.program.clone().ok_or(StoreError::Empty)?;

        // Stale-base check (U16).
        for (name, expected) in &tx.base {
            let actual = self
                .by_name
                .get(name)
                .cloned()
                .unwrap_or_else(|| "<missing>".into());
            if &actual != expected {
                return Err(StoreError::StaleBase {
                    name: name.clone(),
                    expected: expected.clone(),
                    actual,
                });
            }
        }

        for op in &tx.ops {
            match op {
                EditOp::DeleteFn { name } => {
                    if !prog.functions.iter().any(|f| &f.name == name) {
                        return Err(StoreError::Unknown(name.clone()));
                    }
                    prog.functions.retain(|f| &f.name != name);
                }
                EditOp::InsertFn { source } => {
                    let frag = parse(source)?;
                    if frag.functions.len() != 1 {
                        return Err(StoreError::Type(TypeError(
                            "InsertFn expects exactly one fn in source".into(),
                        )));
                    }
                    let f = frag.functions.into_iter().next().unwrap();
                    if prog.functions.iter().any(|x| x.name == f.name) {
                        return Err(StoreError::Duplicate(f.name));
                    }
                    prog.functions.push(f);
                }
                EditOp::ReplaceFn { name, source } => {
                    let frag = parse(source)?;
                    if frag.functions.len() != 1 {
                        return Err(StoreError::Type(TypeError(
                            "ReplaceFn expects exactly one fn in source".into(),
                        )));
                    }
                    let f = frag.functions.into_iter().next().unwrap();
                    if &f.name != name {
                        return Err(StoreError::Type(TypeError(format!(
                            "ReplaceFn name mismatch: expected {name}, got {}",
                            f.name
                        ))));
                    }
                    let Some(slot) = prog.functions.iter_mut().find(|x| &x.name == name) else {
                        return Err(StoreError::Unknown(name.clone()));
                    };
                    *slot = f;
                }
            }
        }

        // Committed codebase always typechecks.
        check_program(&prog)?;
        Ok(self.load_program(&prog))
    }
}

#[allow(dead_code)]
fn _assert_serializable<T: Serialize>(_: &T) {}
