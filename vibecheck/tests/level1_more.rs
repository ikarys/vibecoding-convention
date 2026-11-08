#![allow(unused_imports)]
mod common;

use std::fs;
use std::path::Path;

use common::helpers::*;
use vibecheck::finding::{Finding, Severity};
use vibecheck::Options;

#[test]
fn d2_fails_for_each_missing_level3_key() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let hash = common::head_hash(dir.path());
    let extra = "  coverage: { threshold: 80, kind: line, tool: cov }\n  maintainability: { tool: m, duplication_max: 5, complexity_max: 15, function_max: 60, file_max: 600 }";
    fs::write(
        dir.path().join("VIBECODING.md"),
        d2_declaration(&hash, 3, "[]", extra),
    )
    .unwrap();
    let report = run_default(dir.path());
    for key in [
        "audit_dir",
        "audit_cadence",
        "blind_test",
        "check_interval",
        "agent_permissions",
    ] {
        assert!(
            has_finding(report.findings.iter(), "D2", Severity::Fail, key),
            "missing {key}"
        );
    }
}

#[test]
fn d2_fails_when_level2_missing_coverage_and_maintainability() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let hash = common::head_hash(dir.path());
    fs::write(
        dir.path().join("VIBECODING.md"),
        d2_declaration(&hash, 2, "[]", ""),
    )
    .unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D2",
        Severity::Fail,
        "coverage"
    ));
    assert!(has_finding(
        report.findings.iter(),
        "D2",
        Severity::Fail,
        "maintainability"
    ));
}

#[test]
fn d2_fails_when_adopted_at_commit_missing() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let extra = "  coverage: { threshold: 80, kind: line, tool: cov }\n  maintainability: { tool: m, duplication_max: 5, complexity_max: 15, function_max: 60, file_max: 600 }";
    fs::write(
        dir.path().join("VIBECODING.md"),
        d2_declaration("deadbeef", 2, "[]", extra),
    )
    .unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D2",
        Severity::Fail,
        "commit"
    ));
}

#[test]
fn d2_warns_when_declared_tool_never_attributed() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let hash = common::head_hash(dir.path());
    let extra = "  coverage: { threshold: 80, kind: line, tool: cov }\n  maintainability: { tool: m, duplication_max: 5, complexity_max: 15, function_max: 60, file_max: 600 }";
    fs::write(
        dir.path().join("VIBECODING.md"),
        d2_declaration(&hash, 2, "[\"NeverTool\"]", extra),
    )
    .unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D2",
        Severity::Warn,
        "attributed"
    ));
}

#[test]
fn d5_fails_when_status_level_mismatches_declaration() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let today = chrono::Utc::now().naive_local().date().to_string();
    write_status_json(dir.path(), 1, &today, &short_hash(dir.path()));
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D5",
        Severity::Fail,
        "level"
    ));
}

#[test]
fn d5_warns_when_status_is_old() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let old = (chrono::Utc::now().naive_local().date() - chrono::Duration::days(365)).to_string();
    write_status_json(dir.path(), 2, &old, &short_hash(dir.path()));
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D5",
        Severity::Warn,
        "stale"
    ));
}

#[test]
fn d5_informs_when_archived_status_is_old() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "status: active",
        "status: archived",
    );
    let old = (chrono::Utc::now().naive_local().date() - chrono::Duration::days(365)).to_string();
    write_status_json(dir.path(), 2, &old, &short_hash(dir.path()));
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D5",
        Severity::Info,
        "stale"
    ));
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "D5" && f.severity == Severity::Warn));
}

#[test]
fn d5_warns_when_status_is_older_than_tag() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let yesterday =
        (chrono::Utc::now().naive_local().date() - chrono::Duration::days(1)).to_string();
    write_status_json(dir.path(), 2, &yesterday, &short_hash(dir.path()));
    common::git(dir.path(), &["tag", "-a", "v0.1.0", "-m", "v0.1.0"]);
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D5",
        Severity::Warn,
        "stale"
    ));
}

#[test]
fn d5_fails_when_status_json_invalid() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::write(dir.path().join("audit/status.json"), "not json").unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D5",
        Severity::Fail,
        "status.json"
    ));
}

#[test]
fn d6_warns_when_test_removed_without_explanation() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::remove_file(dir.path().join("tests/smoke.rs")).unwrap();
    common::commit_all(dir.path(), "remove", None);
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D6",
        Severity::Warn,
        "explanation"
    ));
}

#[test]
fn d6_does_not_warn_when_test_removal_is_explained() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::remove_file(dir.path().join("tests/smoke.rs")).unwrap();
    common::commit_all(dir.path(), "remove obsolete test because duplicate", None);
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "D6" && f.severity == Severity::Warn));
}

#[test]
fn d6_warns_when_ignore_added_without_reason() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::write(
        dir.path().join("tests/smoke.rs"),
        "#[ignore]\n#[test]\nfn smoke() {\n    assert_eq!(1, 1);\n}\n",
    )
    .unwrap();
    common::commit_all(dir.path(), "add ignore", None);
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D6",
        Severity::Warn,
        "explanation"
    ));
}

#[test]
fn d9_passes_when_security_response_is_none() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "D9" && f.severity == Severity::Fail));
}

#[test]
fn d9_fails_when_security_file_missing_value() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "security_response: \"72h\"",
        "security_response: \"48h\"",
    );
    fs::write(dir.path().join("SECURITY.md"), "Report issues quickly.\n").unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D9",
        Severity::Fail,
        "SECURITY.md"
    ));
}

#[test]
fn d9_fails_when_status_is_paused() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "status: active",
        "status: paused",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D9",
        Severity::Fail,
        "status"
    ));
}
