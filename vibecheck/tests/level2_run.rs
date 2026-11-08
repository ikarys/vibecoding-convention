#![allow(unused_imports)]
mod common;

use std::fs;
use std::path::Path;

use common::helpers::*;
use vibecheck::finding::{Finding, Severity};
use vibecheck::Options;

#[test]
fn t2_run_reports_good_coverage_metric() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    minimal_level2(
        dir.path(),
        r#""echo TOTAL lines 91.5%""#,
        80,
        "true",
        "null",
        "null",
    );
    let report = run_with(dir.path(), true);
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Info,
        "coverage 91.5"
    ));
    assert!(report
        .metrics
        .get("coverage")
        .is_some_and(|v| v.as_f64() == Some(91.5)));
}

#[test]
fn t2_run_fails_below_threshold() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    minimal_level2(
        dir.path(),
        r#""echo TOTAL lines 42%""#,
        80,
        "true",
        "null",
        "null",
    );
    let report = run_with(dir.path(), true);
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Fail,
        "below threshold"
    ));
}

#[test]
fn t2_run_warns_when_percentage_unreadable() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    minimal_level2(
        dir.path(),
        r#""echo nothing here""#,
        80,
        "true",
        "null",
        "null",
    );
    let report = run_with(dir.path(), true);
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Warn,
        "percentage"
    ));
}

#[test]
fn t2_fails_when_threshold_is_zero() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(dir.path(), "VIBECODING.md", "threshold: 80", "threshold: 0");
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Fail,
        "threshold"
    ));
}

#[test]
fn t2_informs_when_threshold_below_recommendation() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "threshold: 80",
        "threshold: 50",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Info,
        "recommended"
    ));
}

#[test]
fn t2_warns_when_coverage_tool_not_in_ci() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "coverage: { threshold: 80, kind: line, tool: cargo-llvm-cov }",
        "coverage: { threshold: 80, kind: line, tool: cov-tool }",
    );
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "coverage: \"cargo llvm-cov --summary-only\"",
        "coverage: \"cov-tool\"",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Warn,
        "CI"
    ));
}

#[test]
fn t4_warns_without_end_to_end_suite() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::remove_dir_all(dir.path().join("tests/e2e")).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T4",
        Severity::Warn,
        "end-to-end"
    ));
}

#[test]
fn t4_informs_when_cypress_directory_exists() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::create_dir_all(dir.path().join("cypress")).unwrap();
    fs::write(dir.path().join("cypress/README.md"), "cypress suite\n").unwrap();
    let report = run_default(dir.path());
    assert!(has_finding_where(
        report.findings.iter(),
        "T4",
        Severity::Info,
        "end-to-end",
        "cypress"
    ));
}

#[test]
fn t1_warns_when_assuming_cargo_test() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "test: \"cargo test\"",
        "test: null",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T1",
        Severity::Warn,
        "assuming"
    ));
}

#[test]
fn t1_fails_when_no_test_command_and_no_manifest() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "test: \"cargo test\"",
        "test: null",
    );
    fs::remove_file(dir.path().join("Cargo.toml")).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T1",
        Severity::Fail,
        "test command"
    ));
}

#[test]
fn t1_fails_when_no_ci_configuration() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::remove_dir_all(dir.path().join(".github")).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T1",
        Severity::Fail,
        "CI"
    ));
}

#[test]
fn t1_has_no_failures_when_true_command_is_in_ci() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    minimal_level2(dir.path(), "null", 80, "true", "null", "null");
    fs::create_dir_all(dir.path().join(".github/workflows")).unwrap();
    fs::write(
        dir.path().join(".github/workflows/ci.yml"),
        "name: ci\non: push\njobs:\n  test:\n    steps:\n      - run: true\n",
    )
    .unwrap();
    let report = run_with(dir.path(), true);
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "T1" && f.severity == Severity::Fail));
}
