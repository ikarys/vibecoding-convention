use std::fs;
mod common;

use vibecheck::finding::Severity;
use vibecheck::Options;

#[test]
fn level1_fixture_has_no_failures() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_fixture(dir.path());
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    assert!(
        !report.findings.iter().any(|f| f.severity == Severity::Fail),
        "unexpected failures: {:?}",
        report.findings
    );
}

#[test]
fn d8_fails_when_contributing_is_missing() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_fixture(dir.path());
    fs::remove_file(dir.path().join("CONTRIBUTING.md")).unwrap();
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    assert!(report
        .findings
        .iter()
        .any(|f| f.id == "D8" && f.severity == Severity::Fail));
}

#[test]
fn d3_fails_for_signed_off_commit() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_fixture(dir.path());
    fs::write(dir.path().join("note.txt"), "hello\n").unwrap();
    common::commit_all(
        dir.path(),
        "Add note",
        Some("Signed-off-by: Claude Code:claude-fable-5-1"),
    );
    let report = vibecheck::run(dir.path(), Options::default()).unwrap();
    assert!(report
        .findings
        .iter()
        .any(|f| f.id == "D3" && f.severity == Severity::Fail));
}
