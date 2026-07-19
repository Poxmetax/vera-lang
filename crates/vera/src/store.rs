//! Content-addressed definition store (Phase 1).
//! Hash: BLAKE3 of canonical JSON (spans excluded); first 16 hex chars.

use crate::ast::{FnDecl, Program};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct DefEntry {
    pub name: String,
    pub content_hash: String,
}

#[derive(Debug, Default)]
pub struct CodebaseStore {
    by_name: Vec<DefEntry>,
}

impl CodebaseStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hash_def(fn_decl: &FnDecl) -> String {
        // Canonical JSON: serde tag + sorted keys via Value round-trip.
        let value = serde_json::to_value(fn_decl).expect("FnDecl serializable");
        let canonical = serde_json::to_string(&value).expect("Value serializable");
        let hash = blake3::hash(canonical.as_bytes());
        hash.to_hex()[..16].to_string()
    }

    pub fn load_program(&mut self, program: &Program) -> Vec<DefEntry> {
        let mut out = Vec::new();
        for f in &program.functions {
            let entry = DefEntry {
                name: f.name.clone(),
                content_hash: Self::hash_def(f),
            };
            out.push(entry.clone());
            self.by_name.push(entry);
        }
        out
    }

    pub fn summary(&self) -> Vec<serde_json::Value> {
        self.by_name
            .iter()
            .map(|e| {
                serde_json::json!({
                    "name": e.name,
                    "hash": e.content_hash,
                })
            })
            .collect()
    }
}

/// Marker so unused Serialize import stays meaningful for INV-2 hashing discipline.
#[allow(dead_code)]
fn _assert_serializable<T: Serialize>(_: &T) {}
