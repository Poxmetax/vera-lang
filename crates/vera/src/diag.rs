//! [P2B-DIAG] Structured, machine-readable diagnostics for the whole
//! parse → typecheck → prove pipeline (handoff task B; SPEC DP8).
//!
//! `diagnose_source` / `diagnose_program` are the documented single
//! entrypoints: they return a serializable [`DiagReport`] whose diagnostics
//! carry a pipeline `source`, a stable machine `code`, a human `message`,
//! prove-tier `status` (proved / runtime-checked / refuted), and a source
//! `span` where known. The CLI exposes it via `--diag-json`; the default CLI
//! paths (text `--prove` report, interpreter run) are unchanged.

use crate::ast::{Program, Span};
use crate::parser::{parse, ParseError};
use crate::typecheck::{check_program, TypeError};
use crate::vc::{prove_program, Discharge, Obligation};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SpanInfo {
    pub line: u32,
    pub col: u32,
}

impl From<Span> for SpanInfo {
    fn from(s: Span) -> Self {
        SpanInfo {
            line: s.line,
            col: s.col,
        }
    }
}

/// [P2E-FIX] Machine-applicable fix suggestion attached to a diagnostic
/// (SPEC DP8; handoff task E). EPHEMERAL by design: produced and
/// applied-or-discarded within one run/review cycle — never a durable
/// certificate. A durable store would need INV-2 keying (content hash +
/// toolchain/solver version): see GAP5_INV2_DESIGN_NOTE.md / GAP-D2.
#[derive(Debug, Clone, Serialize)]
pub struct FixPatch {
    /// Fix kind; this slice ships exactly one: "add-match-arms".
    pub kind: String,
    /// Always true this slice — consumers MUST NOT store/replay this patch
    /// against drifted code (durable apply requires INV-2 keys, GAP-D2).
    pub ephemeral: bool,
    /// Anchor: the `match` expression this patch targets.
    pub span: SpanInfo,
    /// Valid arm pattern stubs to add, e.g. ["None"] or ["Shape::Pt(_, _)"].
    /// Arm bodies are the consumer's choice (VERA has no `todo` construct).
    pub missing: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    /// Pipeline stage that produced this: "parse" | "typecheck" | "prove".
    pub source: String,
    /// "error" (parse/typecheck errors, refuted obligations) | "info" (proof tiers).
    pub severity: String,
    /// Stable machine code: PARSE-ERROR | TYPE-ERROR | PROVE-PROVED |
    /// PROVE-RUNTIME-CHECKED | PROVE-REFUTED | PROVE-ERROR.
    pub code: String,
    /// Human-readable one-liner.
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Prove tier: "proved" | "runtime-checked" | "refuted" (prove diagnostics only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<SpanInfo>,
    /// [P2E-FIX] Present only when a mechanical fix is computable
    /// (this slice: non-exhaustive match). Omitted from JSON when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<FixPatch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagSummary {
    /// Parse / typecheck / prove-infrastructure errors (exit 1 class).
    pub errors: usize,
    pub proved: usize,
    pub runtime_checked: usize,
    /// Refuted obligations (exit 3 class).
    pub refuted: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiagReport {
    pub tool: String,
    pub version: String,
    pub file: String,
    /// True iff there are no errors and nothing was refuted.
    pub ok: bool,
    pub summary: DiagSummary,
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagReport {
    /// Mirrors the CLI exit contract: 1 = parse/typecheck/prove error,
    /// 3 = any refuted obligation, 0 = ok.
    pub fn exit_code(&self) -> u8 {
        if self.summary.errors > 0 {
            1
        } else if self.summary.refuted > 0 {
            3
        } else {
            0
        }
    }
}

/// TypeError messages carry a "line:col: " prefix (Span Display); split it out
/// so the span is machine-readable instead of buried in the string.
fn split_span_prefix(msg: &str) -> (Option<SpanInfo>, String) {
    if let Some((head, rest)) = msg.split_once(": ") {
        if let Some((l, c)) = head.split_once(':') {
            if let (Ok(line), Ok(col)) = (l.parse::<u32>(), c.parse::<u32>()) {
                return (Some(SpanInfo { line, col }), rest.to_string());
            }
        }
    }
    (None, msg.to_string())
}

pub fn diagnostic_from_parse_error(e: &ParseError) -> Diagnostic {
    Diagnostic {
        source: "parse".into(),
        severity: "error".into(),
        code: "PARSE-ERROR".into(),
        message: e.message.clone(),
        target: None,
        kind: None,
        status: None,
        reason: None,
        span: Some(e.span.into()),
        fix: None,
    }
}

pub fn diagnostic_from_type_error(e: &TypeError) -> Diagnostic {
    let (span, message) = split_span_prefix(&e.0);
    // [P2E-FIX] a non-exhaustive match carries a machine-applicable
    // add-match-arms patch; every other TypeError has no fix payload.
    let fix = e.1.as_ref().map(|m| FixPatch {
        kind: "add-match-arms".into(),
        ephemeral: true,
        span: m.span.into(),
        missing: m.missing.clone(),
    });
    Diagnostic {
        source: "typecheck".into(),
        severity: "error".into(),
        code: "TYPE-ERROR".into(),
        message,
        target: None,
        kind: None,
        status: None,
        reason: None,
        span,
        fix,
    }
}

pub fn diagnostic_from_obligation(o: &Obligation) -> Diagnostic {
    let (severity, code, status, reason, tail) = match &o.status {
        Discharge::Proved => (
            "info",
            "PROVE-PROVED",
            "proved",
            None,
            "proved for all inputs under the stated assumptions".to_string(),
        ),
        Discharge::RuntimeChecked { reason } => (
            "info",
            "PROVE-RUNTIME-CHECKED",
            "runtime-checked",
            Some(reason.clone()),
            format!("kept as a runtime check — {reason}"),
        ),
        Discharge::Refuted { detail } => (
            "error",
            "PROVE-REFUTED",
            "refuted",
            Some(detail.clone()),
            format!("refuted — {detail}"),
        ),
    };
    Diagnostic {
        source: "prove".into(),
        severity: severity.into(),
        code: code.into(),
        message: format!("{} ({}): {}", o.target, o.kind, tail),
        target: Some(o.target.clone()),
        kind: Some(o.kind.clone()),
        status: Some(status.into()),
        reason,
        span: o.span.map(Into::into),
        fix: None,
    }
}

/// Documented entrypoint over an already-parsed program: typecheck, then
/// (optionally) discharge Phase-2 obligations. Fail-fast on the first
/// typecheck error (matching `check_program`).
pub fn diagnose_program(file: &str, program: &Program, with_prove: bool) -> DiagReport {
    let mut diagnostics = Vec::new();
    match check_program(program) {
        Ok(()) => {
            if with_prove {
                match prove_program(program) {
                    Ok(obs) => diagnostics.extend(obs.iter().map(diagnostic_from_obligation)),
                    Err(e) => diagnostics.push(Diagnostic {
                        source: "prove".into(),
                        severity: "error".into(),
                        code: "PROVE-ERROR".into(),
                        message: e.to_string(),
                        target: None,
                        kind: None,
                        status: None,
                        reason: None,
                        span: None,
                        fix: None,
                    }),
                }
            }
        }
        Err(e) => diagnostics.push(diagnostic_from_type_error(&e)),
    }
    finish(file, diagnostics)
}

/// Documented entrypoint over raw source: parse → typecheck → (optional) prove.
pub fn diagnose_source(file: &str, source: &str, with_prove: bool) -> DiagReport {
    match parse(source) {
        Ok(program) => diagnose_program(file, &program, with_prove),
        Err(e) => finish(file, vec![diagnostic_from_parse_error(&e)]),
    }
}

fn finish(file: &str, diagnostics: Vec<Diagnostic>) -> DiagReport {
    let mut summary = DiagSummary {
        errors: 0,
        proved: 0,
        runtime_checked: 0,
        refuted: 0,
    };
    for d in &diagnostics {
        match d.code.as_str() {
            "PROVE-PROVED" => summary.proved += 1,
            "PROVE-RUNTIME-CHECKED" => summary.runtime_checked += 1,
            "PROVE-REFUTED" => summary.refuted += 1,
            _ if d.severity == "error" => summary.errors += 1,
            _ => {}
        }
    }
    let ok = summary.errors == 0 && summary.refuted == 0;
    DiagReport {
        tool: "vera".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        file: file.into(),
        ok,
        summary,
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuted_obligation_is_machine_readable_error() {
        // [P2B-DIAG] refuted ⇒ severity error, status refuted, exit 3; JSON round-trips.
        let src = r#"
fn bad() -> Int
    ensures result > 0
{
    0
}
fn main(console: Console) -> Unit uses {console} {
    console.print(bad().show());
}
"#;
        let r = diagnose_source("t.vera", src, true);
        assert!(!r.ok, "{r:?}");
        assert_eq!(r.summary.refuted, 1, "{r:?}");
        assert_eq!(r.summary.errors, 0, "{r:?}");
        assert_eq!(r.exit_code(), 3);
        let d = r
            .diagnostics
            .iter()
            .find(|d| d.code == "PROVE-REFUTED")
            .expect("refuted diagnostic");
        assert_eq!(d.severity, "error");
        assert_eq!(d.status.as_deref(), Some("refuted"));
        assert!(d.span.as_ref().is_some_and(|s| s.line == 2), "{d:?}");
        let json = serde_json::to_string(&r).expect("serialize");
        let v: serde_json::Value = serde_json::from_str(&json).expect("machine-readable");
        assert_eq!(v["summary"]["refuted"], 1);
        assert_eq!(v["ok"], false);
    }

    #[test]
    fn proved_and_runtime_tiers_are_distinguishable() {
        let src = r#"
fn clamp(x: Int, lo: Int, hi: Int) -> Int
    requires lo <= hi
    ensures result >= lo
{
    if x < lo { lo } else { if x > hi { hi } else { x } }
}
fn tag() -> Str
    ensures true
{
    "t"
}
fn main(console: Console) -> Unit uses {console} {
    console.print(tag());
}
"#;
        let r = diagnose_source("t.vera", src, true);
        assert!(r.ok, "{r:?}");
        assert!(r.summary.proved >= 1, "{r:?}");
        assert!(r.summary.runtime_checked >= 1, "{r:?}");
        assert_eq!(r.exit_code(), 0);
        let statuses: Vec<_> = r.diagnostics.iter().filter_map(|d| d.status.clone()).collect();
        assert!(statuses.iter().any(|s| s == "proved"));
        assert!(statuses.iter().any(|s| s == "runtime-checked"));
    }

    #[test]
    fn type_error_carries_span_and_stable_code() {
        let src = r#"
fn first(x: Option<Int>) -> Int {
    let y: Int = x?;
    y
}
fn main(console: Console) -> Unit uses {console} {
    console.print(first(Some(1)).show());
}
"#;
        let r = diagnose_source("t.vera", src, false);
        assert!(!r.ok);
        assert_eq!(r.summary.errors, 1);
        assert_eq!(r.exit_code(), 1);
        let d = &r.diagnostics[0];
        assert_eq!(d.code, "TYPE-ERROR");
        assert_eq!(d.source, "typecheck");
        assert!(d.span.as_ref().is_some_and(|s| s.line == 3), "{d:?}");
    }

    #[test]
    fn parse_error_is_reported_with_span() {
        let r = diagnose_source("t.vera", "fn", false);
        assert!(!r.ok);
        assert_eq!(r.summary.errors, 1);
        assert_eq!(r.exit_code(), 1);
        assert_eq!(r.diagnostics[0].code, "PARSE-ERROR");
        assert!(r.diagnostics[0].span.is_some());
    }

    #[test]
    fn clean_program_without_prove_is_ok_and_empty() {
        let src = r#"
fn main(console: Console) -> Unit uses {console} {
    console.print("hi");
}
"#;
        let r = diagnose_source("t.vera", src, false);
        assert!(r.ok);
        assert!(r.diagnostics.is_empty());
        assert_eq!(r.exit_code(), 0);
    }

    #[test]
    fn fixpatch_attached_to_non_exhaustive_match() {
        // [P2E-FIX] the TYPE-ERROR carries an ephemeral add-match-arms patch
        // whose span matches the diagnostic and whose stubs are arm-ready.
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
        let r = diagnose_source("t.vera", src, false);
        assert!(!r.ok);
        assert_eq!(r.exit_code(), 1);
        let d = &r.diagnostics[0];
        assert_eq!(d.code, "TYPE-ERROR");
        let fix = d.fix.as_ref().expect("fix payload");
        assert_eq!(fix.kind, "add-match-arms");
        assert!(fix.ephemeral);
        assert_eq!(fix.missing, vec!["Light::Green".to_string()]);
        let dspan = d.span.as_ref().expect("diagnostic span");
        assert_eq!((fix.span.line, fix.span.col), (dspan.line, dspan.col));
        let v: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&r).expect("serialize")).expect("json");
        assert_eq!(v["diagnostics"][0]["fix"]["kind"], "add-match-arms");
        assert_eq!(v["diagnostics"][0]["fix"]["ephemeral"], true);
        assert_eq!(v["diagnostics"][0]["fix"]["missing"][0], "Light::Green");

        // Option scrutinee: stub list computed from the uncovered side.
        let src_opt = r#"
fn peek(x: Option<Int>) -> Int {
    match x {
        Some(v) => v,
    }
}
fn main(console: Console) -> Unit uses {console} {
    console.print(peek(Some(1)).show());
}
"#;
        let r2 = diagnose_source("t2.vera", src_opt, false);
        let f2 = r2.diagnostics[0].fix.as_ref().expect("option fix");
        assert_eq!(f2.missing, vec!["None".to_string()]);
    }

    #[test]
    fn fixpatch_omitted_on_fixless_type_error() {
        // [P2E-FIX] additive schema: diagnostics without a computable fix
        // serialize with NO "fix" key at all (omitted, not null).
        let src = r#"
fn first(x: Option<Int>) -> Int {
    let y: Int = x?;
    y
}
fn main(console: Console) -> Unit uses {console} {
    console.print(first(Some(1)).show());
}
"#;
        let r = diagnose_source("t.vera", src, false);
        assert!(!r.ok);
        assert!(r.diagnostics[0].fix.is_none());
        let json = serde_json::to_string(&r).expect("serialize");
        assert!(!json.contains("\"fix\""), "{json}");
    }
}
