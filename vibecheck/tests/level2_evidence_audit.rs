//! Evidence-driven checks for dependencies: vulnerabilities (T6) and
//! freshness (T9).
mod common;

use std::fs;

use common::helpers::*;
use vibecheck::finding::Severity;

const AUDIT: &str = "audit/evidence/audit.json";

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    dir
}

fn write(dir: &tempfile::TempDir, file: &str, content: &str) {
    fs::write(dir.path().join(file), content).unwrap();
}

fn declare_outdated(dir: &tempfile::TempDir) {
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "audit: audit/evidence/audit.json }",
        "audit: audit/evidence/audit.json, outdated: audit/evidence/outdated.txt }",
    );
}

fn hold_back(dir: &tempfile::TempDir, name: &str) {
    let path = dir.path().join("VIBECODING.md");
    let text = fs::read_to_string(&path).unwrap();
    fs::write(
        path,
        format!("{text}\n## Dependencies held back\n\n- {name}\n"),
    )
    .unwrap();
}

const UNMAINTAINED: &str = r#"{"vulnerabilities":{"count":0,"list":[]},
  "warnings":{"unmaintained":[{"kind":"unmaintained","package":{"name":"oldcrate"},
  "advisory":{"id":"RUSTSEC-2024-0001","package":"oldcrate"}}]}}"#;

#[test]
fn t6_fails_on_vulnerabilities_in_cargo_audit_json() {
    let dir = fixture();
    write(
        &dir,
        AUDIT,
        r#"{"vulnerabilities":{"count":1,"list":[{}]},"warnings":{}}"#,
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Fail,
        "1 unresolved vulnerabilities in audit/evidence/audit.json"
    ));
}

#[test]
fn t6_passes_with_zero_vulnerabilities() {
    let dir = fixture();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Info,
        "no vulnerabilities"
    ));
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "T6" && f.severity != Severity::Info));
}

#[test]
fn t6_warns_when_count_is_unreadable() {
    let dir = fixture();
    write(&dir, AUDIT, "audit tool crashed\n");
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Warn,
        "could not read a vulnerability count from audit/evidence/audit.json"
    ));
}

#[test]
fn t6_fails_when_audit_file_is_missing() {
    let dir = fixture();
    fs::remove_file(dir.path().join(AUDIT)).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Fail,
        "declared evidence missing"
    ));
}

#[test]
fn t9_fails_for_unmaintained_crate_in_audit_json() {
    let dir = fixture();
    write(&dir, AUDIT, UNMAINTAINED);
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T9",
        Severity::Fail,
        "oldcrate"
    ));
}

#[test]
fn t9_respects_held_back_unmaintained_crate() {
    let dir = fixture();
    write(&dir, AUDIT, UNMAINTAINED);
    hold_back(&dir, "oldcrate");
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "T9" && f.severity == Severity::Fail));
}

#[test]
fn t9_counts_outdated_from_text_and_respects_held_back() {
    let dir = fixture();
    declare_outdated(&dir);
    write(
        &dir,
        "audit/evidence/outdated.txt",
        "Name Project Latest\nserde 1.0.0 1.0.9\nfoo 1.0.0 1.0.9\nbar 1.0.0 1.0.9\nbaz 1.0.0 1.0.9\n",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T9",
        Severity::Fail,
        "4 outdated dependencies (max 3)"
    ));
    assert_eq!(report.metrics["outdated"], 4);

    hold_back(&dir, "serde");
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "T9" && f.severity == Severity::Fail));
    assert_eq!(report.metrics["outdated"], 3);
}

#[test]
fn t9_counts_outdated_from_npm_json() {
    let dir = fixture();
    declare_outdated(&dir);
    write(
        &dir,
        "audit/evidence/outdated.txt",
        r#"{"a":{"latest":"2.0.0"},"b":{"latest":"2.0.0"},"c":{"latest":"2.0.0"},"d":{"latest":"2.0.0"}}"#,
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T9",
        Severity::Fail,
        "4 outdated dependencies"
    ));
}

#[test]
fn t9_fails_when_outdated_file_is_missing() {
    let dir = fixture();
    declare_outdated(&dir);
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T9",
        Severity::Fail,
        "declared evidence missing"
    ));
}

#[test]
fn t9_needs_a_bot_or_outdated_evidence() {
    let dir = fixture();
    fs::remove_file(dir.path().join(".github/dependabot.yml")).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T9",
        Severity::Fail,
        "no dependency update mechanism"
    ));

    declare_outdated(&dir);
    write(&dir, "audit/evidence/outdated.txt", "");
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "T9" && f.severity == Severity::Fail));
}

#[test]
fn t9_freshness_is_optional_with_an_update_bot() {
    let dir = fixture();
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "  freshness: { max_outdated: 3 }\n",
        "",
    );
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| (f.id == "T9" || f.id == "D2") && f.severity == Severity::Fail));
}

#[test]
fn t9_outdated_evidence_needs_a_declared_limit() {
    let dir = fixture();
    declare_outdated(&dir);
    write(&dir, "audit/evidence/outdated.txt", "");
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "  freshness: { max_outdated: 3 }\n",
        "",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T9",
        Severity::Fail,
        "without freshness.max_outdated"
    ));
}

#[test]
fn t9_notes_that_outdated_evidence_is_not_declared() {
    let dir = fixture();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T9",
        Severity::Info,
        "outdated evidence not declared"
    ));
}
