#![allow(unused_imports)]
mod common;

use std::fs;
use std::path::Path;

use common::helpers::*;
use vibecheck::finding::{Finding, Severity};
use vibecheck::Options;

#[test]
fn t6_fails_when_audit_command_not_in_ci() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "audit: \"cargo audit\"",
        "audit: \"cargo audit --locked\"",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Fail,
        "CI"
    ));
}

#[test]
fn t6_informs_when_release_doc_mentions_audit_tool() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "audit: \"cargo audit\"",
        "audit: null",
    );
    fs::write(
        dir.path().join(".github/workflows/ci.yml"),
        "name: ci\non: push\njobs:\n  test:\n    steps:\n      - run: cargo test\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("RELEASE.md"),
        "Run cargo audit before release.\n",
    )
    .unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Info,
        "release"
    ));
}

#[test]
fn t6_warns_for_new_dependency_without_registry_check() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    common::commit_all(dir.path(), "add serde", None);
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Warn,
        "registry"
    ));
}

#[test]
fn t6_does_not_warn_for_new_dependency_with_registry_check() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    common::commit_all(
        dir.path(),
        "add serde, dependency checked on crates.io",
        None,
    );
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "T6" && f.severity == Severity::Warn));
}

#[test]
fn t6_fails_when_audit_command_fails_with_run() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    minimal_level2(dir.path(), "null", 80, "true", "\"exit 2\"", "null");
    let report = run_with(dir.path(), true);
    assert!(has_finding(
        report.findings.iter(),
        "T6",
        Severity::Fail,
        "exit"
    ));
}

#[test]
fn t3_warns_for_unexplained_ignore() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::write(
        dir.path().join("tests/smoke.rs"),
        "#[ignore]\n#[test]\nfn smoke() {\n    assert_eq!(1, 1);\n}\n",
    )
    .unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T3",
        Severity::Warn,
        "disabled"
    ));
}

#[test]
fn t3_does_not_warn_for_explained_ignore() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::write(
        dir.path().join("tests/smoke.rs"),
        "#[ignore] // flaky on CI, see issue 12\n#[test]\nfn smoke() {\n    assert_eq!(1, 1);\n}\n",
    )
    .unwrap();
    let report = run_default(dir.path());
    assert!(!report
        .findings
        .iter()
        .any(|f| f.id == "T3" && f.severity == Severity::Warn));
}

#[test]
fn t3_fails_when_no_test_files_exist() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::remove_file(dir.path().join("tests/smoke.rs")).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T3",
        Severity::Fail,
        "test files"
    ));
}

#[test]
fn t8_fails_for_long_python_function() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let body: String = (0..89).map(|i| format!("    x{i} = {i};\n")).collect();
    fs::write(dir.path().join("src/big.py"), format!("def big():\n{body}")).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding_where(
        report.findings.iter(),
        "T8",
        Severity::Fail,
        "functions",
        "big.py"
    ));
}

#[test]
fn t8_does_not_report_unmeasured_when_maintainability_command_succeeds() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    minimal_level2(dir.path(), "null", 80, "true", "null", "\"true\"");
    let report = run_with(dir.path(), true);
    assert!(!report.findings.iter().any(|f| f.id == "T8"
        && f.severity == Severity::Info
        && f.problem.contains("not measured")));
}

#[test]
fn t8_fails_when_file_max_is_zero() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    replace_in_file(dir.path(), "VIBECODING.md", "file_max: 600", "file_max: 0");
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T8",
        Severity::Fail,
        "file_max"
    ));
}
