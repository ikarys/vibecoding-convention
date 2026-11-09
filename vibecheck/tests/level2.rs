mod common;

use std::fs;

use vibecheck::finding::Severity;
use vibecheck::Options;

#[test]
fn level2_fixture_has_no_failures_without_run() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    assert!(
        !report.findings.iter().any(|f| f.severity == Severity::Fail),
        "unexpected failures: {:?}",
        report.findings
    );
}

#[test]
fn t6_fails_when_cargo_lock_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::remove_file(dir.path().join("Cargo.lock")).unwrap();
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    assert!(report
        .findings
        .iter()
        .any(|f| f.id == "T6" && f.severity == Severity::Fail));
}

#[test]
fn t8_fails_for_large_file() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let big: String = (0..700).map(|i| format!("let x{i} = {i};\n")).collect();
    fs::write(dir.path().join("src/big.rs"), big).unwrap();
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    assert!(report.findings.iter().any(|f| f.id == "T8"
        && f.severity == Severity::Fail
        && f.where_.as_deref().is_some_and(|w| w.contains("big.rs"))));
}

#[test]
fn t7_notes_a_missing_precommit_hook_without_failing() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    fs::remove_file(dir.path().join(".pre-commit-config.yaml")).unwrap();
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    let t7: Vec<_> = report.findings.iter().filter(|f| f.id == "T7").collect();
    assert!(t7
        .iter()
        .any(|f| f.severity == Severity::Info && f.problem.contains("pre-commit")));
    assert!(t7.iter().all(|f| f.severity != Severity::Fail));
}

#[test]
fn t7_fails_when_ci_has_no_secrets_scanner() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    let ci = dir.path().join(".github/workflows/ci.yml");
    let text = fs::read_to_string(&ci)
        .unwrap()
        .replace("gitleaks detect", "true");
    fs::write(&ci, text).unwrap();
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    assert!(report.findings.iter().any(|f| f.id == "T7"
        && f.severity == Severity::Fail
        && f.problem.contains("no secrets scanner in CI")));
}
