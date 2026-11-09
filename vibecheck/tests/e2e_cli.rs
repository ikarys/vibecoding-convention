//! End-to-end: the built binary on an empty directory and on a compliant one.
mod common;

use std::process::Command;

fn vibecheck(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_vibecheck"))
        .args(args)
        .output()
        .expect("binary runs")
}

#[test]
fn empty_directory_fails_with_exit_code_one() {
    let dir = tempfile::tempdir().unwrap();
    let out = vibecheck(&[dir.path().to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1));
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("D2") && text.contains("FAIL"), "{text}");
}

#[test]
fn compliant_level1_project_passes_and_json_is_valid() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_fixture(dir.path());
    let out = vibecheck(&[dir.path().to_str().unwrap(), "--format", "json"]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).expect("valid json");
    assert_eq!(value["level"], 1);
}

#[test]
fn write_status_creates_the_badge_file() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_fixture(dir.path());
    std::fs::remove_file(dir.path().join("audit/status.json")).unwrap();
    let out = vibecheck(&[dir.path().to_str().unwrap(), "--write-status"]);
    assert!(out.status.success() || out.status.code() == Some(1));
    let status = std::fs::read_to_string(dir.path().join("audit/status.json")).unwrap();
    assert!(status.contains("\"label\": \"vibecoding\""));
}

#[test]
fn pr_review_refuses_write_status_with_exit_code_two() {
    let dir = tempfile::tempdir().unwrap();
    common::finalize_fixture(dir.path());
    let base = common::head_hash(dir.path());
    let out = vibecheck(&[
        dir.path().to_str().unwrap(),
        "--pr-review",
        &base,
        "--write-status",
    ]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("does not apply"));
}
