//! Pull-request mode on a branch that breaks each range rule once, and on a clean one.
mod common;

use std::fs;
use std::path::Path;

use vibecheck::finding::{Report, Severity};

const TRAILER: &str = "Assisted-by: Claude Code:claude-fable-5-1";

fn branch_off(dir: &Path) -> String {
    common::finalize_fixture(dir);
    let base = common::head_hash(dir);
    common::git(dir, &["checkout", "-q", "-b", "feature"]);
    base
}

fn assert_finding(report: &Report, id: &str, severity: Severity, needle: &str) {
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.id == id && f.severity == severity && f.problem.contains(needle)),
        "missing {id} {severity:?} containing {needle:?} in {:?}",
        report.findings
    );
}

#[test]
fn dirty_branch_gets_one_finding_per_rule() {
    let dir = tempfile::tempdir().unwrap();
    let base = branch_off(dir.path());

    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    common::commit_all(dir.path(), "add serde", None);

    fs::create_dir_all(dir.path().join("tests")).unwrap();
    fs::write(dir.path().join("tests/x.rs"), "#[test]\nfn test_a() {}\n").unwrap();
    common::commit_all(dir.path(), "add a test", Some(TRAILER));
    fs::remove_file(dir.path().join("tests/x.rs")).unwrap();
    common::commit_all(dir.path(), "drop test", Some(TRAILER));

    fs::create_dir_all(dir.path().join("src")).unwrap();
    // Assembled at run time so the repository's own secrets scan never sees a key-shaped literal.
    let fake_key = format!("AKIA{}", "IOSFODNN7EXAMPLE");
    fs::write(
        dir.path().join("src/k.rs"),
        format!("let key = \"{fake_key}\";\n// TODO later\n"),
    )
    .unwrap();
    common::commit_all(dir.path(), "add key", Some(TRAILER));

    let report = vibecheck::run_review(dir.path(), &base).unwrap();
    assert_finding(&report, "D3", Severity::Warn, "no attribution trailer");
    assert_finding(&report, "D6", Severity::Warn, "without saying why");
    assert_finding(
        &report,
        "T6",
        Severity::Warn,
        "without a recorded registry check",
    );
    assert_finding(&report, "T7", Severity::Fail, "possible secret added");
    assert_finding(&report, "T8", Severity::Warn, "marker added in src/k.rs");
    assert!(
        !report
            .findings
            .iter()
            .any(|f| f.problem.contains(&fake_key)),
        "the secret itself must not be printed"
    );
}

#[test]
fn clean_branch_reports_nothing() {
    let dir = tempfile::tempdir().unwrap();
    let base = branch_off(dir.path());
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(dir.path().join("src/clean.rs"), "fn clean() {}\n").unwrap();
    common::commit_all(dir.path(), "add clean code", Some(TRAILER));

    let report = vibecheck::run_review(dir.path(), &base).unwrap();
    assert!(
        !report.has_fail() && !report.has_warn(),
        "{:?}",
        report.findings
    );
    assert_finding(
        &report,
        "PR",
        Severity::Info,
        "1 commits reviewed, nothing to report",
    );
}

#[test]
fn invalid_and_unknown_bases_are_refused() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_fixture(dir.path());

    assert!(vibecheck::run_review(dir.path(), "-x").is_err());
    assert!(vibecheck::run_review(dir.path(), "a..b").is_err());

    let report = vibecheck::run_review(dir.path(), "nonexistent").unwrap();
    assert_eq!(report.findings.len(), 1);
    assert_eq!(report.findings[0].id, "PR");
    assert_eq!(report.findings[0].severity, Severity::NotRun);
    assert!(report.findings[0].problem.contains("fetch it first"));
}
