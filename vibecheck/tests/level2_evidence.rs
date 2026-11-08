//! Evidence-driven checks: coverage (T2), declaration (D2), maintainability
//! (T8), path hardening, staleness and the status file.
mod common;

use std::fs;
use std::process::Command;

use common::helpers::*;
use vibecheck::finding::Severity;

const COVERAGE: &str = "audit/evidence/coverage.txt";

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_level2_fixture(dir.path());
    dir
}

fn write_evidence(dir: &tempfile::TempDir, file: &str, content: &str) {
    fs::write(dir.path().join(file), content).unwrap();
}

fn coverage_metric(report: &vibecheck::Report) -> Option<f64> {
    report.metrics.get("coverage").and_then(|v| v.as_f64())
}

#[test]
fn t2_passes_with_llvm_cov_summary_text() {
    let dir = fixture();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Info,
        "coverage 91.00 %"
    ));
    assert_eq!(coverage_metric(&report), Some(91.0));
    assert!(!report.has_fail(), "{:?}", report.findings);
}

#[test]
fn t2_reads_lcov() {
    let dir = fixture();
    write_evidence(&dir, COVERAGE, "SF:a.rs\nLF:20\nLH:18\nend_of_record\n");
    let report = run_default(dir.path());
    assert_eq!(coverage_metric(&report), Some(90.0));
}

#[test]
fn t2_reads_cobertura() {
    let dir = fixture();
    write_evidence(
        &dir,
        COVERAGE,
        "<?xml version=\"1.0\"?><coverage line-rate=\"0.85\" branch-rate=\"0\"></coverage>\n",
    );
    let report = run_default(dir.path());
    assert!((coverage_metric(&report).unwrap() - 85.0).abs() < 1e-9);
}

#[test]
fn t2_reads_llvm_cov_json() {
    let dir = fixture();
    write_evidence(
        &dir,
        COVERAGE,
        r#"{"data":[{"totals":{"lines":{"percent":88.5}}}],"type":"llvm.coverage.json.export"}"#,
    );
    let report = run_default(dir.path());
    assert_eq!(coverage_metric(&report), Some(88.5));
}

#[test]
fn t2_fails_below_threshold() {
    let dir = fixture();
    write_evidence(&dir, COVERAGE, "TOTAL 100 58 42.00%\n");
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Fail,
        "below threshold"
    ));
    assert_eq!(coverage_metric(&report), Some(42.0));
}

#[test]
fn t2_fails_when_declared_file_is_missing() {
    let dir = fixture();
    fs::remove_file(dir.path().join(COVERAGE)).unwrap();
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Fail,
        "declared evidence missing: audit/evidence/coverage.txt"
    ));
}

#[test]
fn t2_fails_when_file_has_no_figure() {
    let dir = fixture();
    write_evidence(&dir, COVERAGE, "nothing to see\n");
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Fail,
        "no coverage figure in audit/evidence/coverage.txt"
    ));
}

#[test]
fn t2_is_not_run_when_evidence_is_not_declared() {
    let dir = fixture();
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "coverage: audit/evidence/coverage.txt, ",
        "",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::NotRun,
        "coverage evidence not declared"
    ));
}

#[test]
fn d2_fails_at_level_2_without_evidence() {
    let dir = fixture();
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "  evidence: { coverage: audit/evidence/coverage.txt, audit: audit/evidence/audit.json }\n",
        "",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "D2",
        Severity::Fail,
        "evidence.coverage and convention.evidence.audit"
    ));
}

#[test]
fn evidence_path_escaping_the_root_is_rejected() {
    let dir = fixture();
    let outside = dir.path().parent().unwrap().join("outside-evidence.txt");
    fs::write(&outside, "TOTAL 99.00%\n").unwrap();
    for bad in ["../outside-evidence.txt", "/etc/hostname"] {
        replace_in_file(
            dir.path(),
            "VIBECODING.md",
            "coverage: audit/evidence/coverage.txt",
            &format!("coverage: {bad}"),
        );
        let report = run_default(dir.path());
        assert!(
            has_finding(report.findings.iter(), "T2", Severity::Fail, "unreadable"),
            "{bad}"
        );
        assert_eq!(coverage_metric(&report), None);
        replace_in_file(
            dir.path(),
            "VIBECODING.md",
            &format!("coverage: {bad}"),
            "coverage: audit/evidence/coverage.txt",
        );
    }
    fs::remove_file(outside).unwrap();
}

#[test]
fn t8_reports_maintainability_evidence() {
    let dir = fixture();
    replace_in_file(
        dir.path(),
        "VIBECODING.md",
        "audit: audit/evidence/audit.json }",
        "audit: audit/evidence/audit.json, maintainability: audit/evidence/m.json }",
    );
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T8",
        Severity::Fail,
        "declared evidence missing"
    ));
    write_evidence(&dir, "audit/evidence/m.json", "{}");
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T8",
        Severity::Info,
        "maintainability evidence present (audit/evidence/m.json)"
    ));
    assert!(!has_finding(
        report.findings.iter(),
        "T8",
        Severity::Info,
        "not measured"
    ));
}

#[test]
fn stale_evidence_is_warned_about_after_a_release() {
    let dir = fixture();
    let status = Command::new("git")
        .args(["tag", "-a", "v9.9.9", "-m", "release"])
        .current_dir(dir.path())
        .env("GIT_COMMITTER_DATE", "2099-01-01T00:00:00+00:00")
        .env("GIT_COMMITTER_NAME", "Vibecheck Test")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .status()
        .unwrap();
    assert!(status.success());
    let report = run_default(dir.path());
    assert!(has_finding(
        report.findings.iter(),
        "T2",
        Severity::Warn,
        "evidence older than release v9.9.9"
    ));
}

#[test]
fn status_json_carries_evidence_and_coverage_metric() {
    let dir = fixture();
    let report = run_default(dir.path());
    vibecheck::status::write_status(dir.path(), &report).unwrap();
    let text = fs::read_to_string(dir.path().join("audit/status.json")).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    let vibecoding = &value["vibecoding"];
    assert_eq!(vibecoding["evidence"]["coverage"], COVERAGE);
    assert_eq!(vibecoding["evidence"]["audit"], "audit/evidence/audit.json");
    assert_eq!(vibecoding["metrics"]["coverage"], 91.0);
}
