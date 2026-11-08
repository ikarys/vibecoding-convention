mod common;

use common::{commit_all, finalize_fixture, git};

#[test]
fn compliant_level_one_project_has_no_failures() {
    let dir = tempfile::tempdir().unwrap();
    finalize_fixture(dir.path());

    let report = vibecheck::run(dir.path(), None).unwrap();
    let fails: Vec<_> = report
        .findings
        .iter()
        .filter(|finding| finding.severity == vibecheck::finding::Severity::Fail)
        .collect();

    assert!(
        fails.is_empty(),
        "expected no failures, got: {:?}",
        fails
            .iter()
            .map(|finding| (finding.id.as_str(), finding.problem.as_str()))
            .collect::<Vec<_>>()
    );
}

#[test]
fn missing_contributing_is_d8_fail() {
    let dir = tempfile::tempdir().unwrap();
    finalize_fixture(dir.path());
    std::fs::remove_file(dir.path().join("CONTRIBUTING.md")).unwrap();
    git(dir.path(), &["add", "-A"]);
    git(
        dir.path(),
        &[
            "commit",
            "-q",
            "-m",
            "remove contributing",
            "--trailer",
            "Assisted-by: Claude Code:claude-fable-5-1",
        ],
    );

    let report = vibecheck::run(dir.path(), None).unwrap();
    assert!(report.findings.iter().any(
        |finding| finding.id == "D8" && finding.severity == vibecheck::finding::Severity::Fail
    ));
}

#[test]
fn co_authored_by_noreply_is_d3_fail() {
    let dir = tempfile::tempdir().unwrap();
    finalize_fixture(dir.path());
    std::fs::write(dir.path().join("note.txt"), "x\n").unwrap();
    commit_all(
        dir.path(),
        "add note",
        "Signed-off-by: Claude Code:claude-fable-5-1",
    );

    let report = vibecheck::run(dir.path(), None).unwrap();
    assert!(report.findings.iter().any(
        |finding| finding.id == "D3" && finding.severity == vibecheck::finding::Severity::Fail
    ));
}
