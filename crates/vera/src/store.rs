//! Content-addressed definition store + typed edit transactions (Phase 1 / U16 / CONF-P1).
//! Hash: BLAKE3 of canonical JSON (spans excluded); first 16 hex chars.

use crate::ast::{FnDecl, Program, Type};
use crate::diag::FixPatch;
use crate::parser::{parse, ParseError};
use crate::render::render_program;
use crate::typecheck::{check_program, TypeError};
use crate::vc::{Discharge, Obligation, ProofCacheKey, ToolchainId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
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
            return Err(StoreError::Type(TypeError(
                format!("round-trip hash mismatch: {h1} != {h2}\n--- rendered ---\n{rendered}"),
                None,
            )));
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
                            None,
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
                            None,
                        )));
                    }
                    let f = frag.functions.into_iter().next().unwrap();
                    if &f.name != name {
                        return Err(StoreError::Type(TypeError(
                            format!("ReplaceFn name mismatch: expected {name}, got {}", f.name),
                            None,
                        )));
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

// ---------------------------------------------------------------------------
// [GAP-D2] Thin durable INV-2 store (SPEC INV-2 / 6.4). The first durable
// consumer of `ProofCacheKey`: proof-certificate verdicts and FixPatch
// records persisted as ONE JSON file with atomic replace. Honesty rules,
// all test-pinned:
//   * a lookup whose key differs in ANY component (definition hash, query
//     kind, vera version, solver id) is a MISS -- re-prove / re-emit, never
//     a stale hit;
//   * a FixPatch is returned ONLY when the caller's CURRENT target content
//     hash equals the key's definition hash (the stale-patch hazard from the
//     design note; in store-world the definition IS the patch target);
//   * a missing, corrupt, or wrong-format-version file loads as EMPTY
//     (fail-closed toward re-proving) -- never an error-hit, never a panic;
//   * a persisted FixPatch carries `ephemeral: false` -- the durable
//     contract (key + target hash) exists exactly here; live diagnostics
//     stay `ephemeral: true` (P2E contract untouched).
// NOT a cert platform: no CLI wiring (`--prove` never touches this store
// this slice), no MCP persistence, no RepairPlan planner, no eviction.
// ---------------------------------------------------------------------------

const GAPD2_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CertEntry {
    key: ProofCacheKey,
    verdict: Discharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FixEntry {
    key: ProofCacheKey,
    patch: FixPatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DurableData {
    version: u32,
    certs: Vec<CertEntry>,
    fixes: Vec<FixEntry>,
}

impl Default for DurableData {
    fn default() -> Self {
        Self {
            version: GAPD2_FORMAT_VERSION,
            certs: Vec::new(),
            fixes: Vec::new(),
        }
    }
}

/// [GAP-D2] Durable proof-certificate / FixPatch store keyed by
/// [`ProofCacheKey`]. One JSON file (`certs.json`) under a caller-chosen
/// directory; every mutation persists write-through via temp file + fsync +
/// atomic rename. Lookup is exact-key equality -- the INV-2 rule that a
/// toolchain or definition change can never serve a stale verdict.
#[derive(Debug)]
pub struct DurableCertStore {
    path: PathBuf,
    data: DurableData,
}

impl DurableCertStore {
    /// Open (or start) the store under `dir`. Directory creation is the only
    /// error path; an unreadable, corrupt, or wrong-version store file loads
    /// as EMPTY -- fail-closed toward re-proving, never a stale hit.
    pub fn open(dir: &Path) -> io::Result<Self> {
        fs::create_dir_all(dir)?;
        let path = dir.join("certs.json");
        let data = match fs::read_to_string(&path) {
            Ok(text) => match serde_json::from_str::<DurableData>(&text) {
                Ok(d) if d.version == GAPD2_FORMAT_VERSION => d,
                _ => DurableData::default(),
            },
            Err(_) => DurableData::default(),
        };
        Ok(Self { path, data })
    }

    pub fn cert_count(&self) -> usize {
        self.data.certs.len()
    }

    pub fn fix_count(&self) -> usize {
        self.data.fixes.len()
    }

    /// Upsert a proof verdict under its INV-2 key and persist.
    pub fn put_cert(&mut self, key: ProofCacheKey, verdict: Discharge) -> io::Result<()> {
        if let Some(e) = self.data.certs.iter_mut().find(|e| e.key == key) {
            e.verdict = verdict;
        } else {
            self.data.certs.push(CertEntry { key, verdict });
        }
        self.persist()
    }

    /// Exact-key lookup. Any component mismatch (definition hash, query
    /// kind, vera version, solver id) is a MISS (`None`).
    pub fn get_cert(&self, key: &ProofCacheKey) -> Option<&Discharge> {
        self.data
            .certs
            .iter()
            .find(|e| &e.key == key)
            .map(|e| &e.verdict)
    }

    /// Upsert a FixPatch under its INV-2 key and persist. The stored copy is
    /// marked `ephemeral: false`: the durable contract (key + target content
    /// hash) exists exactly here, per the P2E honest limit.
    pub fn put_fixpatch(&mut self, key: ProofCacheKey, mut patch: FixPatch) -> io::Result<()> {
        patch.ephemeral = false;
        if let Some(e) = self.data.fixes.iter_mut().find(|e| e.key == key) {
            e.patch = patch;
        } else {
            self.data.fixes.push(FixEntry { key, patch });
        }
        self.persist()
    }

    /// FixPatch lookup: HIT only when the key matches exactly AND the
    /// caller's CURRENT target content hash equals the key's definition
    /// hash. Belt-and-suspenders: even a caller replaying an OLD key object
    /// cannot hit against drifted code, because the current hash is compared
    /// too (the stale-patch hazard the design note names).
    pub fn get_fixpatch(
        &self,
        key: &ProofCacheKey,
        current_target_hash: &str,
    ) -> Option<&FixPatch> {
        if current_target_hash != key.definition_hash {
            return None;
        }
        self.data
            .fixes
            .iter()
            .find(|e| &e.key == key)
            .map(|e| &e.patch)
    }

    /// [GAP-D2-EVICTION] Explicit eviction/GC (opt-in
    /// `--prove-cache-prune`): remove every cert and FixPatch entry whose
    /// toolchain can no longer produce an exact-key HIT under the CURRENT
    /// toolchain -- a different `vera_version`, or a solver-keyed entry
    /// whose `solver_id` differs from the current solver. Solver-free
    /// entries (`solver_id: "none"`) survive a solver-scoped prune for the
    /// same reason they survive a solver bump (design-note rule 2); they
    /// are pruned only on `vera_version` drift. Fail-closed by
    /// construction: a pruned entry could only have MISSed under the
    /// current toolchain (exact-key equality), so removal can never turn a
    /// would-be HIT into a MISS for the running toolchain, and a future
    /// toolchain downgrade simply re-proves (MISS -> fresh -- the always-
    /// safe direction). Returns `(removed, kept)`; persists only when
    /// something was removed (a no-op prune never rewrites the file, and
    /// in particular never "heals" a corrupt store loaded as EMPTY).
    pub fn prune_stale_toolchain(&mut self, current_solver_id: &str) -> io::Result<(usize, usize)> {
        let current = ToolchainId::current(current_solver_id);
        let live = |tc: &ToolchainId| {
            tc.vera_version == current.vera_version
                && (tc.solver_id == "none" || tc.solver_id == current.solver_id)
        };
        let before = self.data.certs.len() + self.data.fixes.len();
        self.data.certs.retain(|e| live(&e.key.toolchain));
        self.data.fixes.retain(|e| live(&e.key.toolchain));
        let kept = self.data.certs.len() + self.data.fixes.len();
        let removed = before - kept;
        if removed > 0 {
            self.persist()?;
        }
        Ok((removed, kept))
    }

    /// [GAP-D2-LOCK] Exclusive multi-writer lock for one store WRITE session
    /// (open -> mutate -> persist), taken on a sibling `certs.lock` file via
    /// the OS advisory file lock (`File::try_lock`: LockFileEx / flock).
    /// Never blocks: contention returns `Ok(None)` and the caller must skip
    /// its whole cache pass (fail-closed -- re-proving is always safer than
    /// racing writers; a lock failure can never forge a HIT). The lock file
    /// is never the store file, so lock state can never corrupt
    /// `certs.json`; it is opened without truncation so taking a handle
    /// never disturbs a lock another process holds. The OS releases the
    /// lock when the guard's handle closes (drop) or the process dies -- a
    /// crashed writer cannot wedge future writers behind a stale lock.
    pub fn try_write_lock(dir: &Path) -> io::Result<Option<StoreWriteLock>> {
        fs::create_dir_all(dir)?;
        let file = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(dir.join("certs.lock"))?;
        match file.try_lock() {
            Ok(()) => Ok(Some(StoreWriteLock { _file: file })),
            Err(fs::TryLockError::WouldBlock) => Ok(None),
            Err(fs::TryLockError::Error(e)) => Err(e),
        }
    }

    /// Write-through persistence: serialize to a sibling temp file, fsync,
    /// then atomically rename over `certs.json` (replace-existing on all
    /// supported platforms) -- a crash mid-write can never leave a torn
    /// store file.
    fn persist(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.data).map_err(io::Error::other)?;
        let tmp = self.path.with_extension("json.tmp");
        {
            let mut f = fs::File::create(&tmp)?;
            f.write_all(json.as_bytes())?;
            f.sync_all()?;
        }
        fs::rename(&tmp, &self.path)
    }
}

/// [GAP-D2-LOCK] RAII guard for the store write lock: holding it means this
/// process owns the exclusive advisory lock on the store's `certs.lock`.
/// Dropping it closes the handle, which releases the OS lock.
#[derive(Debug)]
pub struct StoreWriteLock {
    _file: fs::File,
}

/// [GAP-D2-CLI] Result of one re-prove-and-compare cache pass.
#[derive(Debug, Default)]
pub struct ProveCacheReport {
    /// Cached verdict present and equal to the fresh one.
    pub matches: usize,
    /// Human-readable "target: cached X vs fresh Y" lines.
    pub mismatches: Vec<String>,
    /// Fresh Proved/Refuted verdicts persisted (miss, or mismatch overwrite).
    pub persisted: usize,
    /// Fn-level obligations not cacheable this pass (fresh RuntimeChecked on
    /// a miss, or a duplicate fn name).
    pub uncacheable: usize,
}

/// [GAP-D2-SOLVER-SKIP] Single source of truth for the INV-2 `query_kind`
/// string of a fn-level obligation (shared by the reconcile pass and the
/// skip engine -- a key-format drift between the two would silently turn
/// every HIT into a miss, or worse, alias distinct obligations).
fn fnlevel_query_kind(kind: &str, ensures_index: Option<usize>) -> Option<String> {
    match (kind, ensures_index) {
        ("ensures", Some(i)) => Some(format!("prove/ensures[{i}]")),
        ("return_refine", _) => Some("prove/return_refine".to_string()),
        _ => None,
    }
}

/// [GAP-D2-CLI] Re-prove-and-compare prove-cache wiring (shape B;
/// shape B, 2026-07-21): Z3 has ALREADY run -- `obligations` are the fresh
/// results. This pass never changes prove output or exit codes; it only
/// reconciles the durable store with fresh truth and reports. Rules:
///   * fn-level obligations only (`ensures` / `return_refine`) -- a
///     call-site obligation depends on BOTH caller and callee, so a single
///     definition-hash key would be unsound; excluded by design;
///   * duplicate fn names are never cached (ambiguous identity -- the same
///     guard `ProvedSet` uses);
///   * only Proved / Refuted persist: a fresh RuntimeChecked is transient
///     (timeout / unsupported fragment) -- it is neither stored nor allowed
///     to destroy a stored certificate (certificate kept + mismatch
///     reported);
///   * on a Proved/Refuted mismatch the FRESH verdict overwrites the store
///     (the solver just ran under this exact toolchain).
pub fn reconcile_prove_cache(
    store: &mut DurableCertStore,
    program: &Program,
    obligations: &[Obligation],
    solver_id: &str,
) -> io::Result<ProveCacheReport> {
    let mut decl_count: HashMap<&str, usize> = HashMap::new();
    for f in &program.functions {
        *decl_count.entry(f.name.as_str()).or_insert(0) += 1;
    }
    let mut hashes: HashMap<&str, String> = HashMap::new();
    for f in &program.functions {
        if decl_count.get(f.name.as_str()) == Some(&1) {
            hashes.insert(f.name.as_str(), CodebaseStore::hash_def(f));
        }
    }
    let mut report = ProveCacheReport::default();
    for o in obligations {
        // Call-site obligations carry no fn identity -- excluded by design.
        let Some(fn_name) = &o.fn_name else { continue };
        let Some(query_kind) = fnlevel_query_kind(o.kind.as_str(), o.ensures_index) else {
            continue;
        };
        let Some(def_hash) = hashes.get(fn_name.as_str()) else {
            report.uncacheable += 1; // duplicate fn name
            continue;
        };
        let key = ProofCacheKey {
            definition_hash: def_hash.clone(),
            query_kind,
            toolchain: ToolchainId::current(solver_id),
        };
        let fresh_stable = matches!(o.status, Discharge::Proved | Discharge::Refuted { .. });
        match store.get_cert(&key) {
            Some(cached) if cached == &o.status => report.matches += 1,
            Some(cached) => {
                report.mismatches.push(format!(
                    "{}: cached {:?} vs fresh {:?}",
                    o.target, cached, o.status
                ));
                if fresh_stable {
                    store.put_cert(key, o.status.clone())?;
                    report.persisted += 1;
                }
            }
            None if fresh_stable => {
                store.put_cert(key, o.status.clone())?;
                report.persisted += 1;
            }
            None => report.uncacheable += 1,
        }
    }
    Ok(report)
}

/// [GAP-D2-SOLVER-SKIP] Result of one cache-skip prove pass.
#[derive(Debug, Default)]
pub struct SkipProveOutcome {
    /// Every obligation, in exactly `prove_program` order (the report and
    /// the exit code read this).
    pub obligations: Vec<Obligation>,
    /// The subset that ran fresh this pass (fn-level misses plus every
    /// call-site obligation) -- the input for the post-report reconcile
    /// pass. Skipped HITs are excluded so its counters stay honest (a
    /// verdict copied FROM the store must not count as a "match").
    pub fresh: Vec<Obligation>,
    /// Fn-level obligations served from an exact INV-2 HIT (Z3 not run).
    pub skipped: usize,
}

/// [GAP-D2-SOLVER-SKIP] Prove with Z3 skipped on sound INV-2 cache HITs
/// (opt-in `--prove-cache-skip`; the flagless and compare
/// paths stay the default and are untouched). READ-ONLY on the store --
/// persisting fresh verdicts stays in `reconcile_prove_cache`, which the
/// CLI runs AFTER the report (shape-B write discipline unchanged).
///
/// Soundness contract (full argument: GAP_D2_SOLVER_SKIP_SLICE.md):
///   * skip granularity is the WHOLE fn, fail-closed: Z3 is skipped for a
///     declaration only when EVERY fn-level obligation it would produce
///     (enumerated from the CURRENT AST: one per `ensures` clause, plus
///     `return_refine` when the return type carries a predicate) has a
///     stored Proved/Refuted verdict under its exact INV-2 key
///     (definition hash + query kind + vera version + solver id);
///   * anything else proves the whole fn fresh, exactly as `prove_program`
///     would: any single miss, a duplicate fn name (ambiguous identity),
///     or a stored verdict that is not Proved/Refuted (poison shape --
///     `reconcile_prove_cache` never persists RuntimeChecked);
///   * cached Proved/Refuted are stable facts of (definition, toolchain):
///     the SMT query is a pure function of the hashed FnDecl, and a fresh
///     re-run under the same solver could only time out INTO
///     RuntimeChecked -- never flip Proved<->Refuted. Trusting the store
///     beyond that is the explicit, documented semantic of the skip flag;
///     compare mode remains the tamper/nondeterminism canary;
///   * call-site obligations are NEVER cached or skipped (two-sided
///     dependency, the GAP-D2-CLI exclusion) -- they run fresh every pass.
pub fn prove_program_skip_cache(
    store: &DurableCertStore,
    program: &Program,
    solver_id: &str,
) -> SkipProveOutcome {
    let mut decl_count: HashMap<&str, usize> = HashMap::new();
    for f in &program.functions {
        *decl_count.entry(f.name.as_str()).or_insert(0) += 1;
    }
    let mut out = SkipProveOutcome::default();
    for f in &program.functions {
        // Fn-level obligation descriptors, in prove_fn emission order.
        let mut descs: Vec<(&str, Option<usize>, String)> = Vec::new();
        for (i, _) in f.ensures.iter().enumerate() {
            descs.push(("ensures", Some(i), crate::vc::ensures_target(&f.name, i)));
        }
        if let Type::Refine {
            name,
            pred: Some(_),
        } = &f.ret
        {
            descs.push((
                "return_refine",
                None,
                crate::vc::return_refine_target(&f.name, name),
            ));
        }
        if descs.is_empty() {
            continue; // no fn-level obligations either way
        }
        // All-or-nothing HIT check (fail-closed toward proving fresh).
        let hits = (|| {
            if decl_count.get(f.name.as_str()) != Some(&1) {
                return None; // duplicate fn name -- never skipped
            }
            let def_hash = CodebaseStore::hash_def(f);
            let mut acc = Vec::new();
            for (kind, idx, target) in &descs {
                let key = ProofCacheKey {
                    definition_hash: def_hash.clone(),
                    query_kind: fnlevel_query_kind(kind, *idx)?,
                    toolchain: ToolchainId::current(solver_id),
                };
                match store.get_cert(&key) {
                    Some(v) if matches!(v, Discharge::Proved | Discharge::Refuted { .. }) => {
                        acc.push((target.clone(), *kind, *idx, v.clone()));
                    }
                    _ => return None, // miss or poison shape -- whole fn fresh
                }
            }
            Some(acc)
        })();
        match hits {
            Some(entries) => {
                out.skipped += entries.len();
                for (target, kind, ensures_index, status) in entries {
                    out.obligations.push(Obligation {
                        target,
                        kind: kind.to_string(),
                        status,
                        span: Some(f.span),
                        fn_name: Some(f.name.clone()),
                        ensures_index,
                    });
                }
            }
            None => {
                let fresh = crate::vc::prove_fn_obligations(f);
                out.fresh.extend(fresh.iter().cloned());
                out.obligations.extend(fresh);
            }
        }
    }
    let calls = crate::vc::prove_call_obligations(program);
    out.fresh.extend(calls.iter().cloned());
    out.obligations.extend(calls);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diag::SpanInfo;
    use crate::vc::ToolchainId;

    /// Fresh per-test scratch dir (stale copy from a prior run removed).
    fn tdir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("vera_gapd2_{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).expect("test dir");
        d
    }

    fn key(def: &str, kind: &str, solver: &str) -> ProofCacheKey {
        ProofCacheKey {
            definition_hash: def.into(),
            query_kind: kind.into(),
            toolchain: ToolchainId::current(solver),
        }
    }

    /// Two REAL definition hashes (the store's own `hash_def`), guaranteed
    /// distinct -- the corpus for definition-drift tests.
    fn two_hashes() -> (String, String) {
        let p = parse("fn gapd2_a() -> Int { 1 }\nfn gapd2_b() -> Int { 2 }").expect("parse");
        let h1 = CodebaseStore::hash_def(&p.functions[0]);
        let h2 = CodebaseStore::hash_def(&p.functions[1]);
        assert_ne!(h1, h2, "distinct defs must hash differently");
        (h1, h2)
    }

    #[test]
    fn gapd2_cert_hit_across_instances_and_def_drift_miss() {
        // [GAP-D2] durability: a verdict put by one instance is a HIT for a
        // FRESH instance on the same dir (exact key); a different definition
        // hash is a MISS (INV-2).
        let dir = tdir("cert_roundtrip");
        let (h1, h2) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        {
            let mut a = DurableCertStore::open(&dir).expect("open a");
            a.put_cert(k.clone(), Discharge::Proved).expect("put");
        }
        let b = DurableCertStore::open(&dir).expect("open b");
        assert_eq!(b.cert_count(), 1);
        assert_eq!(b.get_cert(&k), Some(&Discharge::Proved));
        assert_eq!(b.get_cert(&key(&h2, "prove/ensures[0]", "z3-4.16.0")), None);
    }

    #[test]
    fn gapd2_toolchain_bump_is_miss() {
        // [GAP-D2] INV-2: a solver bump or a vera-version bump is a MISS --
        // never a stale verdict across upgrades.
        let dir = tdir("toolchain_bump");
        let (h1, _) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(k.clone(), Discharge::Proved).expect("put");
        assert!(s.get_cert(&k).is_some(), "exact toolchain must hit");
        assert_eq!(
            s.get_cert(&key(&h1, "prove/ensures[0]", "z3-4.17.0")),
            None,
            "solver bump must miss"
        );
        let vera_bump = ProofCacheKey {
            toolchain: ToolchainId {
                vera_version: "9.9.9-test".into(),
                solver_id: "z3-4.16.0".into(),
            },
            ..k
        };
        assert_eq!(s.get_cert(&vera_bump), None, "vera-version bump must miss");
    }

    #[test]
    fn gapd2_solver_free_kind_keys_none_and_survives_solver_bump() {
        // [GAP-D2] design-note rule 2: solver-free query kinds key
        // solver_id "none", so a Z3 upgrade does not invalidate them --
        // while prove kinds keyed to the solver DO miss after the bump.
        let dir = tdir("solver_free");
        let (h1, _) = two_hashes();
        let tc = key(&h1, "typecheck", "none");
        let pv = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(tc.clone(), Discharge::Proved).expect("put tc");
        s.put_cert(pv, Discharge::Proved).expect("put pv");
        // "after the solver upgrade": typecheck callers still key "none".
        assert!(s.get_cert(&tc).is_some(), "solver-free kind survives");
        assert_eq!(
            s.get_cert(&key(&h1, "prove/ensures[0]", "z3-4.17.0")),
            None,
            "solver-keyed prove verdict misses after bump"
        );
    }

    #[test]
    fn gapd2_fixpatch_target_hash_gate_and_ephemeral_flip() {
        // [GAP-D2] the P2E durable contract: a persisted FixPatch is keyed
        // AND gated on the current target content hash; the stored copy is
        // ephemeral: false (durable), and drifted code can never hit.
        let dir = tdir("fixpatch_gate");
        let (h1, h2) = two_hashes();
        let k = key(&h1, "fixpatch", "none");
        let patch = FixPatch {
            kind: "add-match-arms".into(),
            ephemeral: true, // live-diagnostic default; store must flip it
            span: SpanInfo { line: 11, col: 5 },
            missing: vec!["Signal::Sell(_)".into(), "Signal::Hold".into()],
        };
        {
            let mut s = DurableCertStore::open(&dir).expect("open");
            s.put_fixpatch(k.clone(), patch).expect("put fix");
        }
        let s = DurableCertStore::open(&dir).expect("reopen");
        assert_eq!(s.fix_count(), 1);
        let hit = s
            .get_fixpatch(&k, &h1)
            .expect("matching target hash must hit");
        assert!(!hit.ephemeral, "persisted patch carries ephemeral: false");
        assert_eq!(hit.missing.len(), 2);
        // Drifted target (current hash != key's definition hash) -> MISS,
        // even though the stored key object itself still matches.
        assert!(
            s.get_fixpatch(&k, &h2).is_none(),
            "drifted target must miss"
        );
        // A rebuilt key from the drifted definition also misses (no entry).
        assert!(
            s.get_fixpatch(&key(&h2, "fixpatch", "none"), &h2).is_none(),
            "rekeyed drifted lookup must miss"
        );
    }

    #[test]
    fn gapd2_corrupt_missing_or_wrong_version_is_empty_never_panic() {
        // [GAP-D2] fail-closed loading: no file, garbage bytes, or a future
        // format version all load as EMPTY (re-prove), never a stale hit,
        // never a panic.
        let dir = tdir("corrupt");
        let s = DurableCertStore::open(&dir).expect("open fresh");
        assert_eq!(
            (s.cert_count(), s.fix_count()),
            (0, 0),
            "missing file = empty"
        );

        fs::write(dir.join("certs.json"), "not json {{{{").expect("write garbage");
        let s = DurableCertStore::open(&dir).expect("open corrupt");
        assert_eq!(
            (s.cert_count(), s.fix_count()),
            (0, 0),
            "corrupt file = empty"
        );

        let future = serde_json::json!({
            "version": 999,
            "certs": [{
                "key": {
                    "definition_hash": "abc",
                    "query_kind": "prove/ensures[0]",
                    "toolchain": { "vera_version": "0.1.0", "solver_id": "z3-4.16.0" }
                },
                "verdict": "Proved"
            }],
            "fixes": []
        });
        fs::write(dir.join("certs.json"), future.to_string()).expect("write future");
        let s = DurableCertStore::open(&dir).expect("open future-version");
        assert_eq!(
            (s.cert_count(), s.fix_count()),
            (0, 0),
            "unknown format version = empty (fail-closed)"
        );
    }

    #[test]
    fn gapd2_upsert_replaces_same_key_and_file_stays_valid_json() {
        // [GAP-D2] idempotent re-prove: a second put under the SAME key
        // replaces the verdict (one entry, second wins), and the atomic
        // rename-over-existing path leaves a parseable store file.
        let dir = tdir("upsert");
        let (h1, _) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(k.clone(), Discharge::Proved).expect("put 1");
        s.put_cert(
            k.clone(),
            Discharge::Refuted {
                detail: "counterexample".into(),
            },
        )
        .expect("put 2 (rename over existing)");
        assert_eq!(s.cert_count(), 1, "upsert must not duplicate");
        assert!(matches!(s.get_cert(&k), Some(Discharge::Refuted { .. })));
        let text = fs::read_to_string(dir.join("certs.json")).expect("read");
        let v: serde_json::Value = serde_json::from_str(&text).expect("valid json on disk");
        assert_eq!(v["version"], GAPD2_FORMAT_VERSION);
    }

    /// [GAP-D2-CLI] Fabricated fn-level obligation (pub fields) -- lets the
    /// edge tests drive `reconcile_prove_cache` deterministically, without
    /// depending on solver behavior.
    fn fab_refine_ob(fn_name: &str, status: Discharge) -> Obligation {
        Obligation {
            target: format!("{fn_name} return refine"),
            kind: "return_refine".into(),
            status,
            span: None,
            fn_name: Some(fn_name.into()),
            ensures_index: None,
        }
    }

    #[test]
    fn gapd2cli_real_prove_persists_then_matches() {
        // [GAP-D2-CLI] end-to-end shape B on a REAL prove: first pass
        // persists the fn-level Proved verdict, second pass matches it.
        // Z3 runs BOTH times (re-prove-and-compare -- no solver skip).
        let dir = tdir("cli_roundtrip");
        let src = r#"
fn clamp_cli(x: Int, lo: Int, hi: Int) -> {r: Int | r >= lo && r <= hi}
    requires lo <= hi
{
    if x < lo { lo } else { if x > hi { hi } else { x } }
}
fn main(console: Console) -> Unit uses {console} {
    console.print("x");
}
"#;
        let prog = parse(src).expect("parse");
        let obs = crate::vc::prove_program(&prog).expect("prove 1");
        assert!(
            obs.iter().any(|o| matches!(o.status, Discharge::Proved)),
            "fixture must prove"
        );
        let mut s = DurableCertStore::open(&dir).expect("open");
        let r1 = reconcile_prove_cache(&mut s, &prog, &obs, "z3-4.16.0").expect("pass 1");
        assert_eq!(
            (r1.matches, r1.persisted, r1.uncacheable),
            (0, 1, 0),
            "mismatches: {:?}",
            r1.mismatches
        );
        assert!(r1.mismatches.is_empty());
        let obs2 = crate::vc::prove_program(&prog).expect("prove 2");
        let r2 = reconcile_prove_cache(&mut s, &prog, &obs2, "z3-4.16.0").expect("pass 2");
        assert_eq!((r2.matches, r2.persisted, r2.uncacheable), (1, 0, 0));
        assert!(r2.mismatches.is_empty());
    }

    #[test]
    fn gapd2cli_poisoned_cache_mismatch_fresh_wins() {
        // [GAP-D2-CLI] a cached verdict that disagrees with the fresh solver
        // run is REPORTED and the fresh Proved/Refuted verdict overwrites it
        // (the solver just ran under this exact toolchain).
        let dir = tdir("cli_poison");
        let prog = parse("fn f_poison(x: Int) -> Int { x }\n").expect("parse");
        let h = CodebaseStore::hash_def(&prog.functions[0]);
        let k = key(&h, "prove/return_refine", "z3-4.16.0");
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(
            k.clone(),
            Discharge::Refuted {
                detail: "sat".into(),
            },
        )
        .expect("seed");
        let fresh = vec![fab_refine_ob("f_poison", Discharge::Proved)];
        let r = reconcile_prove_cache(&mut s, &prog, &fresh, "z3-4.16.0").expect("pass");
        assert_eq!(r.matches, 0);
        assert_eq!(r.mismatches.len(), 1);
        assert!(r.mismatches[0].contains("f_poison"), "{:?}", r.mismatches);
        assert_eq!(r.persisted, 1, "fresh must overwrite");
        assert_eq!(s.get_cert(&k), Some(&Discharge::Proved));
    }

    #[test]
    fn gapd2cli_runtime_checked_not_persisted_and_keeps_certificate() {
        // [GAP-D2-CLI] a fresh RuntimeChecked is transient: it is neither
        // persisted on a miss nor allowed to destroy a stored certificate
        // on a mismatch (kept + reported).
        let dir = tdir("cli_rtc");
        let prog = parse("fn f_rtc(x: Int) -> Int { x }\n").expect("parse");
        let h = CodebaseStore::hash_def(&prog.functions[0]);
        let k = key(&h, "prove/return_refine", "z3-4.16.0");
        let rtc = Discharge::RuntimeChecked {
            reason: "unknown".into(),
        };
        let mut s = DurableCertStore::open(&dir).expect("open");
        let r = reconcile_prove_cache(
            &mut s,
            &prog,
            &[fab_refine_ob("f_rtc", rtc.clone())],
            "z3-4.16.0",
        )
        .expect("miss pass");
        assert_eq!((r.matches, r.persisted, r.uncacheable), (0, 0, 1));
        assert_eq!(s.cert_count(), 0, "RuntimeChecked never persisted");
        s.put_cert(k.clone(), Discharge::Proved).expect("seed");
        let r = reconcile_prove_cache(&mut s, &prog, &[fab_refine_ob("f_rtc", rtc)], "z3-4.16.0")
            .expect("mismatch pass");
        assert_eq!(r.mismatches.len(), 1);
        assert_eq!(r.persisted, 0);
        assert_eq!(
            s.get_cert(&k),
            Some(&Discharge::Proved),
            "transient noise must not destroy a certificate"
        );
    }

    #[test]
    fn gapd2cli_solver_bump_is_fresh_key_not_match() {
        // [GAP-D2-CLI] INV-2 end-to-end: a bumped solver id never matches
        // the old certificate -- it persists under a NEW key.
        let dir = tdir("cli_bump");
        let prog = parse("fn f_bump(x: Int) -> Int { x }\n").expect("parse");
        let mut s = DurableCertStore::open(&dir).expect("open");
        let fresh = || vec![fab_refine_ob("f_bump", Discharge::Proved)];
        let r = reconcile_prove_cache(&mut s, &prog, &fresh(), "z3-4.16.0").expect("old");
        assert_eq!(r.persisted, 1);
        let r = reconcile_prove_cache(&mut s, &prog, &fresh(), "z3-4.17.0").expect("bumped");
        assert_eq!(
            (r.matches, r.persisted),
            (0, 1),
            "bump must not match old key"
        );
        assert_eq!(s.cert_count(), 2, "two toolchains, two certificates");
    }

    #[test]
    fn gapd2cli_duplicate_fn_names_never_cached() {
        // [GAP-D2-CLI] duplicate declarations share a name-keyed identity --
        // ambiguous, so the cache skips them wholesale (the ProvedSet guard,
        // same reason: prove_program also runs on unchecked Programs).
        let dir = tdir("cli_dup");
        let prog = parse("fn f_dup(x: Int) -> Int { x }\nfn f_dup(y: Int) -> Int { y }\n")
            .expect("parse (GAP-1 rejects at check, not at parse)");
        let mut s = DurableCertStore::open(&dir).expect("open");
        let fresh = vec![
            fab_refine_ob("f_dup", Discharge::Proved),
            fab_refine_ob("f_dup", Discharge::Proved),
        ];
        let r = reconcile_prove_cache(&mut s, &prog, &fresh, "z3-4.16.0").expect("pass");
        assert_eq!((r.matches, r.persisted, r.uncacheable), (0, 0, 2));
        assert_eq!(s.cert_count(), 0, "ambiguous identity must never be cached");
    }

    /// [GAP-D2-SOLVER-SKIP] Shared fixture: 3 fn-level obligations
    /// (ensures[0], ensures[1], return_refine -- all Proved with real Z3)
    /// plus a call-site obligation from main (never cacheable/skippable).
    fn skip_fixture() -> Program {
        parse(
            r#"
fn skip_clamp(x: Int, lo: Int, hi: Int) -> {r: Int | r >= lo && r <= hi}
    requires lo <= hi
    ensures result >= lo
    ensures result <= hi
{
    if x < lo { lo } else { if x > hi { hi } else { x } }
}
fn main(console: Console) -> Unit uses {console} {
    let _a: Int = skip_clamp(5, 0, 10);
    console.print("ok");
}
"#,
        )
        .expect("parse")
    }

    #[test]
    fn gapd2skip_cold_cache_equals_fresh_and_skips_nothing() {
        // [GAP-D2-SOLVER-SKIP] empty store: the skip engine must produce
        // byte-identical results to prove_program (targets, kinds, statuses,
        // order -- pinned via format_report equality) and skip nothing.
        let dir = tdir("skip_cold");
        let prog = skip_fixture();
        let fresh = crate::vc::prove_program(&prog).expect("prove");
        let s = DurableCertStore::open(&dir).expect("open");
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!(o.skipped, 0);
        assert_eq!(o.fresh.len(), o.obligations.len(), "everything ran fresh");
        assert_eq!(
            crate::vc::format_report("t", &fresh),
            crate::vc::format_report("t", &o.obligations),
            "cold-cache skip output must be byte-identical to prove_program"
        );
    }

    #[test]
    fn gapd2skip_warm_cache_skips_fn_level_and_report_is_identical() {
        // [GAP-D2-SOLVER-SKIP] warmed store (via the landed reconcile pass):
        // all 3 fn-level obligations are served from cache, only call-site
        // obligations run fresh, and the printed report stays byte-identical
        // to the fresh one (the A5 stdout-shape pin).
        let dir = tdir("skip_warm");
        let prog = skip_fixture();
        let fresh = crate::vc::prove_program(&prog).expect("prove");
        let mut s = DurableCertStore::open(&dir).expect("open");
        let r = reconcile_prove_cache(&mut s, &prog, &fresh, "z3-4.16.0").expect("warm");
        assert_eq!(
            r.persisted, 3,
            "fixture persists ensures[0..1] + return_refine"
        );
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!(o.skipped, 3);
        assert!(
            o.fresh.iter().all(|x| x.fn_name.is_none()),
            "only call-site obligations may run fresh on a full HIT: {:?}",
            o.fresh.iter().map(|x| &x.target).collect::<Vec<_>>()
        );
        assert_eq!(
            crate::vc::format_report("t", &fresh),
            crate::vc::format_report("t", &o.obligations),
            "warm-cache skip output must be byte-identical to prove_program"
        );
    }

    #[test]
    fn gapd2skip_poisoned_hit_is_served_without_running_z3() {
        // [GAP-D2-SOLVER-SKIP] the trust semantics of the flag, pinned: a
        // seeded verdict under the exact key is served verbatim on a full
        // HIT -- Z3 verifiably did NOT run, because a fresh prove of this
        // fn would REFUTE (r >= 0 with unconstrained x), yet the skip pass
        // returns the seeded Proved. Compare mode remains the canary that
        // catches exactly this poison (gapd2cli_poisoned_cache_mismatch_
        // fresh_wins). Deterministic: the full-HIT path never spawns Z3.
        let dir = tdir("skip_poison");
        let prog = parse("fn skip_lone(x: Int) -> {r: Int | r >= 0} { x }\n").expect("parse");
        let h = CodebaseStore::hash_def(&prog.functions[0]);
        let k = key(&h, "prove/return_refine", "z3-4.16.0");
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(k, Discharge::Proved).expect("seed");
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!((o.skipped, o.fresh.len()), (1, 0));
        assert_eq!(o.obligations.len(), 1);
        assert_eq!(
            o.obligations[0].status,
            Discharge::Proved,
            "served from store"
        );
        assert_eq!(
            o.obligations[0].target, "skip_lone return refine {r: Int | …}",
            "reconstructed target must match the fresh-path shape"
        );
        assert_eq!(o.obligations[0].fn_name.as_deref(), Some("skip_lone"));
    }

    #[test]
    fn gapd2skip_partial_hit_or_poison_shape_proves_whole_fn_fresh() {
        // [GAP-D2-SOLVER-SKIP] fail-closed granularity: (a) a fn with TWO
        // fn-level obligations and only ONE cached proves BOTH fresh
        // (all-or-nothing); (b) a stored RuntimeChecked (a shape reconcile
        // never persists -- poison) is not a HIT either.
        let dir = tdir("skip_partial");
        let src = r#"
fn skip_two(x: Int) -> {r: Int | r == x}
    ensures result == x
{
    x
}
fn skip_rtc(y: Int) -> {r: Int | r == y} {
    y
}
"#;
        let prog = parse(src).expect("parse");
        let h_two = CodebaseStore::hash_def(&prog.functions[0]);
        let h_rtc = CodebaseStore::hash_def(&prog.functions[1]);
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(
            key(&h_two, "prove/ensures[0]", "z3-4.16.0"),
            Discharge::Proved,
        )
        .expect("seed partial");
        s.put_cert(
            key(&h_rtc, "prove/return_refine", "z3-4.16.0"),
            Discharge::RuntimeChecked {
                reason: "poison".into(),
            },
        )
        .expect("seed rtc");
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!(o.skipped, 0, "no fn may skip: partial hit + poison shape");
        assert_eq!(o.fresh.len(), o.obligations.len());
        assert_eq!(
            o.obligations.len(),
            3,
            "ensures + refine (skip_two), refine (skip_rtc)"
        );
        assert!(
            o.obligations
                .iter()
                .all(|x| matches!(x.status, Discharge::Proved)),
            "{:?}",
            o.obligations
                .iter()
                .map(|x| (&x.target, &x.status))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn gapd2skip_duplicate_fn_names_never_skip() {
        // [GAP-D2-SOLVER-SKIP] ambiguous identity: seeded certs under BOTH
        // duplicate declarations' hashes must not enable a skip -- the same
        // guard reconcile and ProvedSet use (prove_program also runs on
        // unchecked Programs, where GAP-1's front door has not fired).
        let dir = tdir("skip_dup");
        let prog = parse(
            "fn f_skipdup(x: Int) -> {r: Int | r == x} { x }\nfn f_skipdup(y: Int) -> {r: Int | r == y} { y }\n",
        )
        .expect("parse (GAP-1 rejects at check, not at parse)");
        let mut s = DurableCertStore::open(&dir).expect("open");
        for f in &prog.functions {
            let h = CodebaseStore::hash_def(f);
            s.put_cert(
                key(&h, "prove/return_refine", "z3-4.16.0"),
                Discharge::Proved,
            )
            .expect("seed");
        }
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!(o.skipped, 0, "duplicate fn names must never be skipped");
        assert_eq!(o.obligations.len(), 2);
        assert_eq!(o.fresh.len(), 2);
    }

    #[test]
    fn gapd2skip_solver_bump_never_hits() {
        // [GAP-D2-SOLVER-SKIP] INV-2 end-to-end on the skip path: a verdict
        // stored under z3-4.16.0 must not be served under a bumped id --
        // the fn proves fresh instead (never a stale-toolchain skip).
        let dir = tdir("skip_bump");
        let prog = parse("fn skip_bump_fn(x: Int) -> {r: Int | r == x} { x }\n").expect("parse");
        let h = CodebaseStore::hash_def(&prog.functions[0]);
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(
            key(&h, "prove/return_refine", "z3-4.16.0"),
            Discharge::Proved,
        )
        .expect("seed");
        let o = prove_program_skip_cache(&s, &prog, "z3-4.17.0");
        assert_eq!(o.skipped, 0, "bumped solver id must miss");
        assert_eq!(o.fresh.len(), 1);
        assert!(
            matches!(o.obligations[0].status, Discharge::Proved),
            "proved fresh"
        );
    }

    #[test]
    fn gapd2skip_mixed_hit_and_miss_compose_in_order() {
        // [GAP-D2-SOLVER-SKIP] mixed run in ONE pass: fn1 fully cached
        // (skipped) + fn2 uncached (fresh) must preserve prove_program order
        // and stay byte-identical across the fn boundary. Per-fn structure
        // makes this correct by construction; this pins the last ordering
        // hole that all-hit + all-miss alone did not exercise.
        let dir = tdir("skip_mixed");
        let src = r#"
fn skip_hit(a: Int) -> {r: Int | r == a} { a }
fn skip_miss(b: Int) -> {r: Int | r == b} { b }
"#;
        let prog = parse(src).expect("parse");
        let fresh = crate::vc::prove_program(&prog).expect("prove");
        let mut s = DurableCertStore::open(&dir).expect("open");
        let h_hit = CodebaseStore::hash_def(&prog.functions[0]);
        s.put_cert(
            key(&h_hit, "prove/return_refine", "z3-4.16.0"),
            Discharge::Proved,
        )
        .expect("seed only skip_hit");
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!(o.skipped, 1, "only skip_hit is served from cache");
        assert!(
            o.fresh
                .iter()
                .any(|x| x.fn_name.as_deref() == Some("skip_miss")),
            "skip_miss must prove fresh: {:?}",
            o.fresh.iter().map(|x| &x.target).collect::<Vec<_>>()
        );
        assert_eq!(
            crate::vc::format_report("t", &fresh),
            crate::vc::format_report("t", &o.obligations),
            "mixed hit/miss skip output must be byte-identical to prove_program"
        );
    }

    #[test]
    fn gapd2skip_cached_refuted_is_served_and_reaches_obligations() {
        // [GAP-D2-SOLVER-SKIP] a cached Refuted is served verbatim on a full
        // HIT (Z3 not run) and lands in outcome.obligations with Refuted
        // status -- the store-level half of the CLI's exit-3 mapping
        // (main.rs scans obligations for Refuted -> ExitCode::from(3)).
        // Pins invariant 6 against a silent refactor.
        let dir = tdir("skip_refuted");
        let prog = parse("fn skip_ref(x: Int) -> {r: Int | r >= 1} { x }\n").expect("parse");
        let h = CodebaseStore::hash_def(&prog.functions[0]);
        let k = key(&h, "prove/return_refine", "z3-4.16.0");
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(
            k,
            Discharge::Refuted {
                detail: "seed".into(),
            },
        )
        .expect("seed refuted");
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!(
            (o.skipped, o.fresh.len()),
            (1, 0),
            "served from HIT, nothing fresh"
        );
        assert_eq!(o.obligations.len(), 1);
        assert!(
            matches!(o.obligations[0].status, Discharge::Refuted { .. }),
            "cached Refuted must reach obligations verbatim (feeds CLI exit 3)"
        );
    }

    #[test]
    fn gapd2evict_prune_all_stale_empties_store_and_persists() {
        // [GAP-D2-EVICTION] "empty after prune": a store holding ONLY
        // entries a current-toolchain lookup could never HIT (bumped
        // solver, bumped vera version -- certs AND fixes) prunes to empty,
        // the removal is persisted (a fresh instance agrees), and the
        // on-disk file stays valid version-1 JSON (no format change).
        let dir = tdir("evict_all_stale");
        let (h1, h2) = two_hashes();
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(key(&h1, "prove/ensures[0]", "z3-4.15.0"), Discharge::Proved)
            .expect("stale solver");
        let old_vera_cert = ProofCacheKey {
            toolchain: ToolchainId {
                vera_version: "0.0.0-old".into(),
                solver_id: "z3-4.16.0".into(),
            },
            ..key(&h2, "prove/return_refine", "z3-4.16.0")
        };
        s.put_cert(old_vera_cert, Discharge::Proved)
            .expect("stale vera");
        let old_vera_fix = ProofCacheKey {
            toolchain: ToolchainId {
                vera_version: "0.0.0-old".into(),
                solver_id: "none".into(),
            },
            ..key(&h1, "fixpatch", "none")
        };
        s.put_fixpatch(
            old_vera_fix,
            FixPatch {
                kind: "add-match-arms".into(),
                ephemeral: true,
                span: SpanInfo { line: 1, col: 1 },
                missing: vec![],
            },
        )
        .expect("stale fix (solver-free dies only on vera drift)");
        let (removed, kept) = s.prune_stale_toolchain("z3-4.16.0").expect("prune");
        assert_eq!((removed, kept), (3, 0));
        assert_eq!((s.cert_count(), s.fix_count()), (0, 0), "empty after prune");
        let re = DurableCertStore::open(&dir).expect("reopen");
        assert_eq!(
            (re.cert_count(), re.fix_count()),
            (0, 0),
            "prune must persist"
        );
        let text = fs::read_to_string(dir.join("certs.json")).expect("read");
        let v: serde_json::Value = serde_json::from_str(&text).expect("valid json on disk");
        assert_eq!(
            v["version"], GAPD2_FORMAT_VERSION,
            "format version unchanged"
        );
    }

    #[test]
    fn gapd2evict_survivor_hits_solver_free_survives_wrong_key_still_misses() {
        // [GAP-D2-EVICTION] the fail-closed contract from the other side:
        // prune removes ONLY entries that could never HIT -- a
        // current-toolchain cert still HITs afterwards, a solver-free
        // ("none") entry survives a solver-scoped prune (design-note rule
        // 2, same reason it survives a solver bump), and pruning never
        // CREATES a hit (wrong def hash / bumped solver still MISS).
        let dir = tdir("evict_survivors");
        let (h1, h2) = two_hashes();
        let cur = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        let free = key(&h1, "typecheck", "none");
        let stale = key(&h2, "prove/ensures[0]", "z3-4.15.0");
        let live_fix = key(&h1, "fixpatch", "none");
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(cur.clone(), Discharge::Proved).expect("cur");
        s.put_cert(free.clone(), Discharge::Proved).expect("free");
        s.put_cert(stale.clone(), Discharge::Proved).expect("stale");
        s.put_fixpatch(
            live_fix.clone(),
            FixPatch {
                kind: "add-match-arms".into(),
                ephemeral: true,
                span: SpanInfo { line: 2, col: 3 },
                missing: vec!["Signal::Hold".into()],
            },
        )
        .expect("live fix (current vera + solver-free)");
        let (removed, kept) = s.prune_stale_toolchain("z3-4.16.0").expect("prune");
        assert_eq!((removed, kept), (1, 3));
        assert_eq!(
            s.get_cert(&cur),
            Some(&Discharge::Proved),
            "survivor must still HIT"
        );
        assert_eq!(
            s.get_cert(&free),
            Some(&Discharge::Proved),
            "solver-free entry survives (rule 2)"
        );
        assert!(
            s.get_fixpatch(&live_fix, &h1).is_some(),
            "surviving FixPatch must still HIT through the target-hash gate after prune"
        );
        assert_eq!(s.get_cert(&stale), None, "pruned entry is gone");
        assert_eq!(
            s.get_cert(&key(&h2, "prove/ensures[0]", "z3-4.16.0")),
            None,
            "wrong def hash still MISS after prune"
        );
        assert_eq!(
            s.get_cert(&key(&h1, "prove/ensures[0]", "z3-4.17.0")),
            None,
            "bumped solver still MISS after prune"
        );
    }

    #[test]
    fn gapd2evict_noop_prune_leaves_file_bytes_untouched() {
        // [GAP-D2-EVICTION] a prune that removes nothing must not rewrite
        // the store file: on-disk bytes compare equal before/after (the
        // no-op stays read-only on disk).
        let dir = tdir("evict_noop");
        let (h1, _) = two_hashes();
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(key(&h1, "prove/ensures[0]", "z3-4.16.0"), Discharge::Proved)
            .expect("cur");
        let before = fs::read(dir.join("certs.json")).expect("read before");
        let (removed, kept) = s.prune_stale_toolchain("z3-4.16.0").expect("prune");
        assert_eq!((removed, kept), (0, 1));
        let after = fs::read(dir.join("certs.json")).expect("read after");
        assert_eq!(before, after, "no-op prune must not rewrite the file");
    }

    #[test]
    fn gapd2evict_corrupt_store_still_empty_and_prune_never_heals_it() {
        // [GAP-D2-EVICTION] corrupt bytes still load EMPTY (the GAP-D2
        // fail-closed rule is unchanged by eviction), pruning the empty
        // result is a (0, 0) no-op, and -- because a no-op never persists
        // -- the corrupt file is left on disk as evidence, not silently
        // overwritten with an empty valid store.
        let dir = tdir("evict_corrupt");
        fs::write(dir.join("certs.json"), "not json {{{{").expect("write garbage");
        let mut s = DurableCertStore::open(&dir).expect("open corrupt");
        assert_eq!((s.cert_count(), s.fix_count()), (0, 0), "corrupt = EMPTY");
        let (removed, kept) = s.prune_stale_toolchain("z3-4.16.0").expect("prune");
        assert_eq!((removed, kept), (0, 0));
        let s2 = DurableCertStore::open(&dir).expect("reopen");
        assert_eq!((s2.cert_count(), s2.fix_count()), (0, 0), "still EMPTY");
        assert_eq!(
            fs::read_to_string(dir.join("certs.json")).expect("read"),
            "not json {{{{",
            "no-op prune must not overwrite the corrupt file"
        );
    }

    #[test]
    fn gapd2evict_persist_failure_leaves_disk_unchanged() {
        // [GAP-D2-EVICTION] the "on-disk store unchanged" claim of the CLI
        // prune-failure note, pinned: a persist failure mid-prune (temp-file
        // creation blocked by a DIRECTORY squatting on certs.json.tmp)
        // returns Err and leaves the on-disk store byte-identical -- a
        // fresh instance still sees every pre-prune entry. The in-memory
        // divergence dies with the instance, and the dropped entries were
        // unservable under the current toolchain anyway (MISS-safe).
        let dir = tdir("evict_persist_fail");
        let (h1, h2) = two_hashes();
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(key(&h1, "prove/ensures[0]", "z3-4.16.0"), Discharge::Proved)
            .expect("live");
        s.put_cert(key(&h2, "prove/ensures[0]", "z3-4.15.0"), Discharge::Proved)
            .expect("stale");
        let before = fs::read(dir.join("certs.json")).expect("read before");
        fs::create_dir_all(dir.join("certs.json.tmp")).expect("blocker dir");
        let res = s.prune_stale_toolchain("z3-4.16.0");
        assert!(res.is_err(), "blocked temp file must surface as Err");
        let after = fs::read(dir.join("certs.json")).expect("read after");
        assert_eq!(
            before, after,
            "failed prune must leave on-disk bytes unchanged"
        );
        let re = DurableCertStore::open(&dir).expect("reopen");
        assert_eq!(
            re.cert_count(),
            2,
            "fresh instance still sees every pre-prune entry"
        );
    }

    #[test]
    fn gapd2evict_prune_composes_with_reconcile_and_skip() {
        // [GAP-D2-EVICTION] end-to-end with the real consumers: warm the
        // store via the landed reconcile pass (current toolchain), seed one
        // stale-solver cert, prune -- the stale generation goes, the warm
        // generation stays, and the skip engine still serves all 3 fn-level
        // obligations from the survivors, byte-identical report included.
        let dir = tdir("evict_compose");
        let prog = skip_fixture();
        let fresh = crate::vc::prove_program(&prog).expect("prove");
        let mut s = DurableCertStore::open(&dir).expect("open");
        let r = reconcile_prove_cache(&mut s, &prog, &fresh, "z3-4.16.0").expect("warm");
        assert_eq!(
            r.persisted, 3,
            "fixture persists ensures[0..1] + return_refine"
        );
        let (h1, _) = two_hashes();
        s.put_cert(key(&h1, "prove/ensures[0]", "z3-4.15.0"), Discharge::Proved)
            .expect("seed stale generation");
        assert_eq!(s.cert_count(), 4);
        let (removed, kept) = s.prune_stale_toolchain("z3-4.16.0").expect("prune");
        assert_eq!((removed, kept), (1, 3));
        let o = prove_program_skip_cache(&s, &prog, "z3-4.16.0");
        assert_eq!(o.skipped, 3, "survivors must still serve the skip path");
        assert_eq!(
            crate::vc::format_report("t", &fresh),
            crate::vc::format_report("t", &o.obligations),
            "post-prune skip output must stay byte-identical to prove_program"
        );
    }

    #[test]
    fn gapd2lock_acquire_contend_release_reacquire() {
        // [GAP-D2-LOCK] lock boundaries: first acquire wins; a second
        // handle in the same process contends (fail-closed None, no panic,
        // no block); dropping the guard releases the OS lock for the next
        // writer.
        let dir = tdir("lock_roundtrip");
        let a = DurableCertStore::try_write_lock(&dir)
            .expect("lock io")
            .expect("first acquire");
        assert!(
            DurableCertStore::try_write_lock(&dir)
                .expect("lock io")
                .is_none(),
            "held lock must contend, not be re-acquired"
        );
        drop(a);
        assert!(
            DurableCertStore::try_write_lock(&dir)
                .expect("lock io")
                .is_some(),
            "dropping the guard must release the lock"
        );
    }

    #[test]
    fn gapd2lock_contended_writer_fails_closed_store_untouched() {
        // [GAP-D2-LOCK] the CLI contract: a contended would-be writer gets
        // None and skips its whole cache pass -- the store file it would
        // have written stays byte-identical, and the lock file is a
        // sibling, never the store file itself.
        let dir = tdir("lock_contend");
        let (h1, _) = two_hashes();
        let mut s = DurableCertStore::open(&dir).expect("open");
        s.put_cert(key(&h1, "prove/ensures[0]", "z3-4.16.0"), Discharge::Proved)
            .expect("seed");
        let bytes_before = fs::read(dir.join("certs.json")).expect("read store");
        let _held = DurableCertStore::try_write_lock(&dir)
            .expect("lock io")
            .expect("acquire");
        assert!(
            DurableCertStore::try_write_lock(&dir)
                .expect("lock io")
                .is_none(),
            "second writer must fail closed"
        );
        let bytes_after = fs::read(dir.join("certs.json")).expect("read store");
        assert_eq!(bytes_before, bytes_after, "loser must not touch the store");
    }

    #[test]
    fn gapd2lock_error_path_releases_lock_on_drop() {
        // [GAP-D2-LOCK] unlock on the error path: a write session that
        // fails mid-way drops its guard, so the lock must not leak past
        // the error -- the next writer acquires cleanly.
        fn write_session_that_fails(dir: &Path) -> io::Result<()> {
            let _guard = DurableCertStore::try_write_lock(dir)?
                .ok_or_else(|| io::Error::other("contended"))?;
            Err(io::Error::other("simulated mid-session failure"))
            // _guard drops here, releasing the lock despite the Err.
        }
        let dir = tdir("lock_error_release");
        assert!(write_session_that_fails(&dir).is_err());
        assert!(
            DurableCertStore::try_write_lock(&dir)
                .expect("lock io")
                .is_some(),
            "lock must be free after the failed session dropped its guard"
        );
    }

    #[test]
    fn gapd2lock_no_torn_store_under_thread_contention() {
        // [GAP-D2-LOCK] the property the lock exists for: two writers race
        // the same dir; every session that acquires the lock does a full
        // open -> upsert -> persist under it, losers fail closed (skip).
        // Serialized read-modify-write means NO lost update -- the final
        // store must hold exactly one entry per winning session and parse
        // as one valid v1 store (never torn).
        let dir = tdir("lock_torn");
        let (h1, _) = two_hashes();
        let handles: Vec<_> = (0..2)
            .map(|t| {
                let dir = dir.clone();
                let h = h1.clone();
                std::thread::spawn(move || {
                    let mut wins = 0usize;
                    for i in 0..25 {
                        // Contended (`None`) is the fail-closed skip path.
                        if let Some(_guard) =
                            DurableCertStore::try_write_lock(&dir).expect("lock io")
                        {
                            let mut s = DurableCertStore::open(&dir).expect("open");
                            s.put_cert(
                                key(&h, &format!("prove/ensures[{}]", t * 100 + i), "z3-4.16.0"),
                                Discharge::Proved,
                            )
                            .expect("put under lock");
                            wins += 1;
                        }
                    }
                    wins
                })
            })
            .collect();
        let wins: usize = handles.into_iter().map(|h| h.join().expect("join")).sum();
        assert!(wins > 0, "at least one session must acquire");
        let s = DurableCertStore::open(&dir).expect("reopen");
        assert_eq!(
            s.cert_count(),
            wins,
            "every locked upsert persisted -- no lost update, no torn file"
        );
    }
}
