use std::fs;
use std::path::Path;
use std::process::Command;

use common::finalize_level2_fixture;

mod common;

fn run_vibecheck(dir: &Path, args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_vibecheck"))
        .arg(dir)
        .args(args)
        .env_remove(vibecheck::exec::NESTED_MARKER)
        .output()
        .expect("vibecheck should run");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    format!("{stdout}{stderr}")
}

fn has_fail(output: &str, id: &str) -> bool {
    output.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with(id) && trimmed.contains("FAIL")
    })
}

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    finalize_level2_fixture(dir.path());
    dir
}

#[test]
fn level2_fixture_has_no_t9_fail() {
    let dir = fixture();
    let output = run_vibecheck(dir.path(), &["--level", "2"]);
    assert!(!has_fail(&output, "T9"));
}

#[test]
fn missing_update_mechanism_fails() {
    let dir = fixture();
    fs::remove_file(dir.path().join(".github/dependabot.yml")).unwrap();
    let declaration = fs::read_to_string(dir.path().join("VIBECODING.md")).unwrap();
    let declaration = declaration.replace("    outdated: \"echo none\"\n", "");
    fs::write(dir.path().join("VIBECODING.md"), declaration).unwrap();

    let output = run_vibecheck(dir.path(), &["--level", "2"]);
    assert!(has_fail(&output, "T9"));
    assert!(output.contains("update mechanism"));
}

#[test]
fn outdated_count_respects_held_back() {
    let dir = fixture();
    let declaration = fs::read_to_string(dir.path().join("VIBECODING.md")).unwrap();
    // YAML double quotes turn `\n` into `\n`, which printf then prints as a newline.
    let command =
        "printf 'serde 1.0.0 1.0.9\\nfoo 1.0.0 1.0.9\\nbar 1.0.0 1.0.9\\nbaz 1.0.0 1.0.9\\n'";
    let declaration = declaration.replace(
        "outdated: \"echo none\"",
        &format!("outdated: \"{command}\""),
    );
    fs::write(dir.path().join("VIBECODING.md"), declaration).unwrap();

    let output = run_vibecheck(dir.path(), &["--level", "2", "--run"]);
    assert!(has_fail(&output, "T9"));
    assert!(output.contains("outdated dependencies"));

    let declaration = fs::read_to_string(dir.path().join("VIBECODING.md")).unwrap();
    let declaration = format!("{declaration}\n## Dependencies held back\n\n- serde\n");
    fs::write(dir.path().join("VIBECODING.md"), declaration).unwrap();

    let output = run_vibecheck(dir.path(), &["--level", "2", "--run"]);
    assert!(!has_fail(&output, "T9"));
}

#[test]
fn leftover_markers_fail() {
    let dir = fixture();
    fs::write(
        dir.path().join("src/leftover.rs"),
        "// TODO: implement\ntodo!()\n",
    )
    .unwrap();

    let output = run_vibecheck(dir.path(), &["--level", "2"]);
    assert!(has_fail(&output, "T8"));
    assert!(output.contains("leftover"));
    assert!(output.contains("leftover.rs"));
}

#[test]
fn marker_budget_allows_markers() {
    let dir = fixture();
    fs::write(
        dir.path().join("src/leftover.rs"),
        "// TODO: implement\ntodo!()\n",
    )
    .unwrap();
    let declaration = fs::read_to_string(dir.path().join("VIBECODING.md")).unwrap();
    let declaration = declaration.replace("markers_max: 0", "markers_max: 5");
    fs::write(dir.path().join("VIBECODING.md"), declaration).unwrap();

    let output = run_vibecheck(dir.path(), &["--level", "2"]);
    assert!(!has_fail(&output, "T8"));
    assert!(output.contains("leftover markers"));
}
