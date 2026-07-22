//! [GAP-D2-MCP] Thin MCP persistence seam over [`DurableCertStore`].
//!
//! The store-facing primitive a future MCP compiler-service (SPEC DP8 /
//! CONF-P3) calls to read and write INV-2 durable proof certificates. This is
//! NOT the MCP server: no JSON-RPC, no protocol loop, no request routing, no
//! tool schemas -- those stay CONF-P3 (`mcp/README.md`). It reuses the landed
//! GAP-D2 store + GAP-D2-LOCK primitives verbatim, adding only the thin,
//! INV-2-keyed, lock-correct boundary an MCP tool binds to.
//!
//! Discipline inherited unchanged from the CLI write session
//! (`main.rs::run_prove_cache`, [`DurableCertStore::try_write_lock`]):
//!   * writes take the store's exclusive advisory lock ONCE, held across the
//!     whole session (open -> persist* -> drop); contention or lock failure is
//!     fail-closed -- nothing is written, the caller re-proves, never a forged
//!     HIT, never a torn write;
//!   * reads are lock-free (atomic-rename visibility) -- the same reader model
//!     as the CLI skip path; a reader never blocks and never blocks a writer;
//!   * exact INV-2 keying only: any component mismatch, or an EMPTY / corrupt /
//!     future-version store, is a MISS (`None`) -- re-prove, never a stale
//!     verdict.
//!
//! Scope: proof-certificate (`Discharge`) verdicts only. A durable FixPatch
//! write channel is NOT part of this slice (that is AGT-1, parked); live
//! FixPatch emissions stay `ephemeral: true` untouched (`diag.rs`).

use crate::store::{DurableCertStore, StoreWriteLock};
use crate::vc::{Discharge, ProofCacheKey};
use std::io;
use std::path::Path;

/// [GAP-D2-MCP] Lock-free exact-key read: what an MCP `prove`-class tool calls
/// to serve a durable verdict without running the solver. Returns the stored
/// [`Discharge`] on an exact INV-2 key match; `None` on any component mismatch
/// or an EMPTY / corrupt / future-version store (fail-closed toward
/// re-proving). Takes NO lock -- a reader never contends with a concurrent
/// write session (atomic-rename visibility; the CLI skip-path reader model).
/// The only error path is directory creation inside [`DurableCertStore::open`].
pub fn mcp_get_cert(dir: &Path, key: &ProofCacheKey) -> io::Result<Option<Discharge>> {
    let store = DurableCertStore::open(dir)?;
    Ok(store.get_cert(key).cloned())
}

/// [GAP-D2-MCP] One MCP durable-write session over [`DurableCertStore`].
///
/// Acquires the GAP-D2-LOCK exclusive advisory lock EXACTLY ONCE, before the
/// store is opened, and holds it for the session's lifetime -- every
/// [`persist_cert`](McpWriteSession::persist_cert) runs under the same lock,
/// mirroring `run_prove_cache`'s "one writer across the whole session"
/// guarantee (never a per-write reacquire, so it can never self-conflict with
/// its own `open` -- the two-handle same-process conflict the lock probe
/// proved). The OS releases the lock when the guard drops (or the process
/// dies): no stale-lock wedge.
#[derive(Debug)]
pub struct McpWriteSession {
    /// RAII lock guard -- dropping it releases the OS advisory lock.
    _lock: StoreWriteLock,
    store: DurableCertStore,
}

impl McpWriteSession {
    /// Open a durable-write session under `dir`. `Ok(None)` means the store's
    /// write lock is held by another writer -- the caller must skip
    /// persistence and re-prove (fail-closed; a lock contention can never
    /// forge a HIT). `Err` is a real I/O failure (lock or directory), also
    /// fail-closed: nothing is written.
    pub fn open(dir: &Path) -> io::Result<Option<Self>> {
        // Lock BEFORE open, exactly once (the run_prove_cache discipline);
        // never reacquired per persist, so it cannot self-conflict.
        let Some(lock) = DurableCertStore::try_write_lock(dir)? else {
            return Ok(None);
        };
        let store = DurableCertStore::open(dir)?;
        Ok(Some(Self { _lock: lock, store }))
    }

    /// Upsert one INV-2 verdict under the held lock, persisted write-through
    /// (temp + fsync + atomic rename -- never a torn store). Idempotent per
    /// key: a second persist under the same key replaces the verdict.
    pub fn persist_cert(&mut self, key: ProofCacheKey, verdict: Discharge) -> io::Result<()> {
        self.store.put_cert(key, verdict)
    }
}

#[cfg(test)]
mod tests {
    use super::{mcp_get_cert, McpWriteSession};
    use crate::diag::diagnose_source;
    use crate::parser::parse;
    use crate::store::CodebaseStore;
    use crate::vc::{Discharge, ProofCacheKey, ToolchainId};
    use std::fs;
    use std::path::PathBuf;

    /// Fresh per-test scratch dir (stale copy from a prior run removed).
    fn tdir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("vera_gapd2mcp_{}_{}", name, std::process::id()));
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

    /// Two REAL, distinct definition hashes (the store's own `hash_def`).
    fn two_hashes() -> (String, String) {
        let p = parse("fn mcp_a() -> Int { 1 }\nfn mcp_b() -> Int { 2 }").expect("parse");
        let h1 = CodebaseStore::hash_def(&p.functions[0]);
        let h2 = CodebaseStore::hash_def(&p.functions[1]);
        assert_ne!(h1, h2, "distinct defs must hash differently");
        (h1, h2)
    }

    #[test]
    fn gapd2mcp_persist_then_get_hit_across_instances_and_def_drift_miss() {
        // Durability across the seam: a verdict persisted by one write session
        // is an exact-key HIT for a FRESH lock-free read; a different
        // definition hash is a MISS (INV-2).
        let dir = tdir("hit");
        let (h1, h2) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        {
            let mut s = McpWriteSession::open(&dir).expect("io").expect("lock free");
            s.persist_cert(k.clone(), Discharge::Proved)
                .expect("persist");
        }
        assert_eq!(
            mcp_get_cert(&dir, &k).expect("read"),
            Some(Discharge::Proved),
            "persisted verdict must HIT a fresh reader"
        );
        assert_eq!(
            mcp_get_cert(&dir, &key(&h2, "prove/ensures[0]", "z3-4.16.0")).expect("read"),
            None,
            "definition drift must MISS"
        );
    }

    #[test]
    fn gapd2mcp_toolchain_bump_is_miss() {
        // INV-2 end-to-end on the seam: a solver bump or a vera-version bump
        // never serves the stored verdict.
        let dir = tdir("bump");
        let (h1, _) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        {
            let mut s = McpWriteSession::open(&dir).expect("io").expect("lock");
            s.persist_cert(k.clone(), Discharge::Proved)
                .expect("persist");
        }
        assert!(
            mcp_get_cert(&dir, &k).expect("read").is_some(),
            "exact toolchain must HIT"
        );
        assert_eq!(
            mcp_get_cert(&dir, &key(&h1, "prove/ensures[0]", "z3-4.17.0")).expect("read"),
            None,
            "solver bump must MISS"
        );
        let vera_bump = ProofCacheKey {
            toolchain: ToolchainId {
                vera_version: "9.9.9-test".into(),
                solver_id: "z3-4.16.0".into(),
            },
            ..k
        };
        assert_eq!(
            mcp_get_cert(&dir, &vera_bump).expect("read"),
            None,
            "vera-version bump must MISS"
        );
    }

    #[test]
    fn gapd2mcp_empty_and_corrupt_store_is_miss_never_panic() {
        // Fail-closed read: a missing store, then a corrupt file, both MISS
        // (None) and never panic.
        let dir = tdir("empty");
        let (h1, _) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        assert_eq!(
            mcp_get_cert(&dir, &k).expect("read empty"),
            None,
            "missing store = MISS"
        );
        fs::write(dir.join("certs.json"), "not json {{{{").expect("write garbage");
        assert_eq!(
            mcp_get_cert(&dir, &k).expect("read corrupt"),
            None,
            "corrupt store = MISS (fail-closed)"
        );
    }

    #[test]
    fn gapd2mcp_write_session_contended_refuses_fail_closed() {
        // Fail-closed write: while one session holds the lock, a second
        // McpWriteSession::open returns Ok(None) (refuse), and the refused
        // caller writes nothing -- store bytes stay byte-identical. Releasing
        // the holder lets a fresh session re-acquire.
        let dir = tdir("contend");
        let (h1, _) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        let mut held = McpWriteSession::open(&dir).expect("io").expect("lock");
        held.persist_cert(k.clone(), Discharge::Proved)
            .expect("persist");
        let before = fs::read(dir.join("certs.json")).expect("read bytes");
        assert!(
            McpWriteSession::open(&dir).expect("io").is_none(),
            "second write session must contend while the first holds the lock"
        );
        let after = fs::read(dir.join("certs.json")).expect("read bytes");
        assert_eq!(before, after, "refused writer must not touch the store");
        drop(held);
        assert!(
            McpWriteSession::open(&dir).expect("io").is_some(),
            "lock must be re-acquirable after the holder drops"
        );
    }

    #[test]
    fn gapd2mcp_reader_is_lockfree_during_write_session() {
        // Operator ask: mcp_get_cert takes NO lock -- it reads successfully
        // WHILE a McpWriteSession holds the exclusive write lock in the same
        // process (mirrors LOCK's reader-unlocked invariant). The contending
        // second session below proves the lock genuinely is held.
        let dir = tdir("reader");
        let (h1, _) = two_hashes();
        let k = key(&h1, "prove/ensures[0]", "z3-4.16.0");
        {
            let mut seed = McpWriteSession::open(&dir).expect("io").expect("lock");
            seed.persist_cert(k.clone(), Discharge::Proved)
                .expect("persist");
        }
        // Hold the write lock for the rest of the test.
        let mut held = McpWriteSession::open(&dir).expect("io").expect("lock");
        // Reader path is lock-free: it reads the seeded verdict despite the
        // held write lock.
        assert_eq!(
            mcp_get_cert(&dir, &k).expect("lock-free read"),
            Some(Discharge::Proved),
            "reader must see the cert while the write lock is held"
        );
        // Proof the lock really is exclusive for writers.
        assert!(
            McpWriteSession::open(&dir).expect("io").is_none(),
            "a second write session must contend while the lock is held"
        );
        // The holder can still persist under its own lock.
        held.persist_cert(
            key(&h1, "prove/return_refine", "z3-4.16.0"),
            Discharge::Proved,
        )
        .expect("persist under held lock");
    }

    #[test]
    fn gapd2mcp_live_fixpatch_stays_ephemeral() {
        // Regression guard for the handoff's "ephemeral live FixPatch
        // unchanged": this slice adds only a cert persistence seam -- there is
        // NO durable FixPatch write channel here (that is AGT-1, parked), so
        // the live diagnostic FixPatch contract (diag.rs) is untouched and
        // still emits ephemeral: true. Proven through the real public API.
        let src = r#"
enum Light {
    Red,
    Green,
}
fn light_label(l: Light) -> Str {
    match l {
        Light::Red => "red",
    }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(light_label(Light::Red));
}
"#;
        let report = diagnose_source("t.vera", src, false);
        let fix = report
            .diagnostics
            .iter()
            .find_map(|d| d.fix.as_ref())
            .expect("non-exhaustive match must carry a FixPatch");
        assert!(fix.ephemeral, "live FixPatch must stay ephemeral: true");
        assert_eq!(fix.kind, "add-match-arms");
    }
}
