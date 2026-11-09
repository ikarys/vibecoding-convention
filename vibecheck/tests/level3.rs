mod common;

use common::helpers::*;
use common::{commit_all, finalize_level2_fixture, git, head_hash};

use std::fs;
use std::path::Path;

use vibecheck::finding::Severity;
use vibecheck::Options;

fn fixture_with_report(dir: &Path) {
    finalize_level2_fixture(dir);
    let commit = head_hash(dir);
    let report = format!(
        "---\n\
         date: 2026-11-08\n\
         commit: {commit}\n\
         kind: architecture\n\
         read_by: ikarys\n\
         ---\n\
         - Severity: MINOR\n\
         - Severity: PAINFUL\n\
         \n\
         ## Decisions\n\
         \n\
         | 1 | fixed | {commit} |\n\
         | 2 | documented | {commit} |\n"
    );
    fs::write(dir.join("audit/2026-11-08-architecture.md"), report).unwrap();
    let index = format!(
        "| Date | Commit | Kind | Tool / model | Read by | Verdict | Open findings |\n\
         |---|---|---|---|---|---|---|\n\
         | 2026-11-08 | {commit} | architecture | Claude Code:claude-fable-5-1 | ikarys | usable | 0 |\n"
    );
    fs::write(dir.join("audit/README.md"), index).unwrap();
    commit_all(dir, "add audit report", None);
}

fn run_level3(dir: &Path) -> vibecheck::Report {
    vibecheck::run(
        dir,
        Options {
            level_override: Some(3),
        },
    )
    .unwrap()
}

#[test]
fn a4_passes_when_index_matches_reports() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    let report = run_level3(dir.path());
    assert!(!has_finding(&report.findings, "A4", Severity::Fail, ""));
    assert!(has_finding(
        &report.findings,
        "A4",
        Severity::Info,
        "1 reports indexed"
    ));
}

#[test]
fn a4_fails_for_unlisted_report() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    let commit = head_hash(dir.path());
    let report = format!(
        "---\n\
         date: 2026-11-09\n\
         commit: {commit}\n\
         kind: security\n\
         read_by: ikarys\n\
         ---\n\
         - Severity: MINOR\n"
    );
    fs::write(dir.path().join("audit/2026-11-09-security.md"), report).unwrap();
    commit_all(dir.path(), "add unlisted report", None);
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A4",
        Severity::Fail,
        "not in the index"
    ));
}

#[test]
fn a4_fails_for_unknown_commit() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    let index = "| Date | Commit | Kind | Tool / model | Read by | Verdict | Open findings |\n\
                 |---|---|---|---|---|---|---|\n\
                 | 2026-11-08 | deadbeef0 | architecture | Claude Code:claude-fable-5-1 | ikarys | usable | 0 |\n";
    fs::write(dir.path().join("audit/README.md"), index).unwrap();
    commit_all(dir.path(), "break index commit", None);
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A4",
        Severity::Fail,
        "does not exist"
    ));
}

#[test]
fn a4_warns_when_report_rewritten() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    let path = dir.path().join("audit/2026-11-08-architecture.md");
    let text = fs::read_to_string(&path).unwrap();
    let rewritten = text.replace("- Severity: PAINFUL\n", "");
    fs::write(&path, rewritten).unwrap();
    commit_all(dir.path(), "rewrite report", None);
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A4",
        Severity::Warn,
        "rewritten"
    ));
}

#[test]
fn a5_not_run_without_tag() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A5",
        Severity::NotRun,
        "no release tag"
    ));
}

#[test]
fn a5_passes_with_approved_tag() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    git(
        dir.path(),
        &[
            "tag",
            "-a",
            "v0.1.0",
            "-m",
            "Release 0.1.0 approved by ikarys",
        ],
    );
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A5",
        Severity::Info,
        "approved by ikarys"
    ));
    assert!(!has_finding(&report.findings, "A5", Severity::Fail, ""));
}

#[test]
fn a5_fails_on_open_blocker_at_tag() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    let commit = head_hash(dir.path());
    let report = format!(
        "---\n\
         date: 2026-11-10\n\
         commit: {commit}\n\
         kind: architecture\n\
         read_by: ikarys\n\
         ---\n\
         - Severity: BLOCKER\n\
         \n\
         ## Decisions\n\
         \n\
         | 1 | open |  |\n"
    );
    fs::write(dir.path().join("audit/2026-11-10-architecture.md"), report).unwrap();
    commit_all(dir.path(), "add open blocker report", None);
    git(
        dir.path(),
        &[
            "tag",
            "-a",
            "v0.2.0",
            "-m",
            "Release 0.2.0 approved by ikarys",
        ],
    );
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A5",
        Severity::Fail,
        "open BLOCKER"
    ));
}

#[test]
fn a5_fails_without_approver() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    git(dir.path(), &["tag", "-a", "v0.3.0", "-m", "Release 0.3.0"]);
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A5",
        Severity::Fail,
        "no declared maintainer"
    ));
}

#[test]
fn a7_info_when_not_declared() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A7",
        Severity::Info,
        "not measured"
    ));
}

#[test]
fn a7_compares_to_threshold() {
    let dir = tempfile::tempdir().unwrap();
    fixture_with_report(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "evidence: { coverage: audit/evidence/coverage.txt, audit: audit/evidence/audit.json }",
        "evidence: { coverage: audit/evidence/coverage.txt, audit: audit/evidence/audit.json, mutation: audit/evidence/mutation.json }\n  mutation: { tool: cargo-mutants, threshold: 80 }",
    );
    fs::write(
        dir.path().join("audit/evidence/mutation.json"),
        r#"{"caught": 90, "missed": 10}"#,
    )
    .unwrap();
    commit_all(dir.path(), "add mutation evidence", None);
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A7",
        Severity::Info,
        "mutation score 90.00 %"
    ));
    assert_eq!(
        report.metrics.get("mutation"),
        Some(&serde_json::json!(90.0))
    );

    fs::write(
        dir.path().join("audit/evidence/mutation.json"),
        r#"{"caught": 5, "missed": 15}"#,
    )
    .unwrap();
    commit_all(dir.path(), "lower mutation score", None);
    let report = run_level3(dir.path());
    assert!(has_finding(
        &report.findings,
        "A7",
        Severity::Fail,
        "below threshold"
    ));
}
