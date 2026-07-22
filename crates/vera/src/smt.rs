//! Z3 backend via SMT-LIB2 subprocess (Phase 2 thin slice).
//!
//! Preferred path on Windows: spawn `z3.exe` rather than linking the `z3` crate
//! (MSVC/DLL discoverability is fragile). See PHASE2_VC_SLICE_REPORT.md.

use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmtError {
    #[error("z3 not found (set VERA_Z3 or add z3 to PATH); looked: {0}")]
    NotFound(String),
    #[error("z3 spawn failed: {0}")]
    Spawn(String),
    #[error("z3 I/O: {0}")]
    Io(String),
    #[error("z3 returned unexpected output: {0:?}")]
    BadOutput(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatResult {
    Sat,
    Unsat,
    Unknown,
}

/// Locate `z3` / `z3.exe`. Order: `VERA_Z3`, PATH, then sibling-tree Z3 unpack.
/// [GAP-D2-CLI] Parse `z3 --version` output ("Z3 version 4.16.0 - 64 bit")
/// into an INV-2 solver id ("z3-4.16.0"). `None` on anything unrecognized --
/// callers treat that as cache-disabled (fail-closed), never a guessed id.
pub fn solver_id_from_version_output(out: &str) -> Option<String> {
    let rest = out.trim().strip_prefix("Z3 version ")?;
    let ver = rest.split_whitespace().next()?;
    if ver.is_empty() || !ver.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return None;
    }
    Some(format!("z3-{ver}"))
}

/// [GAP-D2-CLI] Discover the current solver id by running `--version` on the
/// same binary `find_z3` resolves. `None` on any failure (missing binary,
/// bad exit, unparseable output).
pub fn discover_solver_id() -> Option<String> {
    let z3 = find_z3().ok()?;
    let out = Command::new(&z3).arg("--version").output().ok()?;
    if !out.status.success() {
        return None;
    }
    solver_id_from_version_output(&String::from_utf8_lossy(&out.stdout))
}

pub fn find_z3() -> Result<PathBuf, SmtError> {
    if let Ok(p) = env::var("VERA_Z3") {
        let pb = PathBuf::from(&p);
        if pb.is_file() {
            return Ok(pb);
        }
        return Err(SmtError::NotFound(p));
    }
    if let Ok(out) = Command::new("z3").arg("--version").output() {
        if out.status.success() {
            return Ok(PathBuf::from("z3"));
        }
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest
            .join("../../..")
            .join("z3-4.16.0-x64-win")
            .join("bin")
            .join("z3.exe"),
        manifest
            .join("../..")
            .join("z3-4.16.0-x64-win")
            .join("bin")
            .join("z3.exe"),
    ];
    let mut looked = Vec::new();
    for c in &candidates {
        looked.push(c.display().to_string());
        if let Ok(canon) = c.canonicalize() {
            if canon.is_file() {
                return Ok(canon);
            }
        } else if c.is_file() {
            return Ok(c.clone());
        }
    }
    Err(SmtError::NotFound(looked.join("; ")))
}

/// Run a full SMT-LIB2 script; return the first sat/unsat/unknown line from stdout.
pub fn check_smtlib(script: &str) -> Result<SatResult, SmtError> {
    let z3 = find_z3()?;
    check_smtlib_with(&z3, script)
}

pub fn check_smtlib_with(z3: &Path, script: &str) -> Result<SatResult, SmtError> {
    let mut child = Command::new(z3)
        .arg("-in")
        .arg("-T:5")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| SmtError::Spawn(e.to_string()))?;
    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| SmtError::Io("no stdin".into()))?;
        stdin
            .write_all(script.as_bytes())
            .map_err(|e| SmtError::Io(e.to_string()))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| SmtError::Io(e.to_string()))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    for line in stdout.lines().chain(stderr.lines()) {
        let t = line.trim();
        if t == "sat" {
            return Ok(SatResult::Sat);
        }
        if t == "unsat" {
            return Ok(SatResult::Unsat);
        }
        if t == "unknown" {
            return Ok(SatResult::Unknown);
        }
    }
    Err(SmtError::BadOutput(format!(
        "stdout={stdout:?} stderr={stderr:?}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z3_proves_trivial_unsat() {
        let r = check_smtlib("(set-logic QF_LIA)\n(assert false)\n(check-sat)\n")
            .expect("z3 available");
        assert_eq!(r, SatResult::Unsat);
    }

    #[test]
    fn gapd2cli_solver_id_parse_is_fail_closed() {
        // [GAP-D2-CLI] the INV-2 solver id comes from `z3 --version` output;
        // anything unrecognized is None (cache disabled), never a guess.
        assert_eq!(
            solver_id_from_version_output("Z3 version 4.16.0 - 64 bit"),
            Some("z3-4.16.0".into())
        );
        assert_eq!(
            solver_id_from_version_output("Z3 version 4.17.1\n"),
            Some("z3-4.17.1".into())
        );
        assert_eq!(solver_id_from_version_output("z3 4.16.0"), None);
        assert_eq!(solver_id_from_version_output("Z3 version "), None);
        assert_eq!(solver_id_from_version_output("Z3 version abc"), None);
        assert_eq!(solver_id_from_version_output(""), None);
    }
}
