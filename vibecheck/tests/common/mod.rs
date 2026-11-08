use std::fs;
use std::path::Path;
use std::process::Command;

pub fn git(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(dir)
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .expect("git command should run")
}

pub fn commit_all(dir: &Path, message: &str, trailer: &str) {
    git(dir, &["add", "-A"]);
    git(dir, &["commit", "-q", "-m", message, "--trailer", trailer]);
}

pub fn head_hash(dir: &Path) -> String {
    let output = git(dir, &["rev-parse", "HEAD"]);
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn write_compliant_project(dir: &Path) {
    fs::write(
        dir.join("README.md"),
        "See [VIBECODING.md](VIBECODING.md)\n",
    )
    .unwrap();
    fs::write(
        dir.join("VIBECODING.md"),
        r#"---
version: "0.1.1"
level: copilot
processes:
  design: pair
  implementation: copilot
  testing: none
  documentation: copilot
  review: hint
  deployment: none
convention:
  version: "0.1"
  level: 1
  adopted_at: 0000000000000000000000000000000000000000
  status: active
  tools:
    - "Claude Code:claude-fable-5-1"
  maintainers:
    - ikarys
  review: "single maintainer reads every diff before it reaches main"
  security_response: "72h"
  licence: MIT
---
# Vibecoding

## Provenance and licence

We keep copyright and safeguard the code.
"#,
    )
    .unwrap();
    fs::write(dir.join("LICENSE"), "MIT\n").unwrap();
    fs::write(dir.join("CONTRIBUTING.md"), "Use Assisted-by trailers.\n").unwrap();
    fs::write(dir.join("SECURITY.md"), "Response time: 72h\n").unwrap();
    fs::write(
        dir.join("AGENTS.md"),
        "Read VIBECODING.md. Add Assisted-by trailers.\n",
    )
    .unwrap();
}

pub fn finalize_fixture(dir: &Path) {
    git(dir, &["init", "-q"]);
    write_compliant_project(dir);
    commit_all(dir, "initial", "Assisted-by: Claude Code:claude-fable-5-1");

    let hash = head_hash(dir);
    let short = hash.get(..7).unwrap_or(&hash);
    let today = chrono::Utc::now().naive_local().date().to_string();

    let declaration = fs::read_to_string(dir.join("VIBECODING.md")).unwrap();
    let declaration = declaration.replace(
        "adopted_at: 0000000000000000000000000000000000000000",
        &format!("adopted_at: {hash}"),
    );
    fs::write(dir.join("VIBECODING.md"), declaration).unwrap();

    fs::create_dir_all(dir.join("audit")).unwrap();
    fs::write(
        dir.join("audit/status.json"),
        format!(
            r#"{{
  "schemaVersion": 1,
  "label": "vibecoding",
  "message": "level 1 · checked {today} · {short}",
  "color": "blue",
  "vibecoding": {{
    "level": 1,
    "verdict": "pass",
    "date": "{today}",
    "commit": "{short}",
    "report": null,
    "open_findings": {{ "BLOCKER": 0, "DOC-WRONG": 0, "PAINFUL": 0, "MINOR": 0 }},
    "metrics": {{}},
    "checked_by": "test",
    "read_by": null
  }}
}}"#
        ),
    )
    .unwrap();

    commit_all(dir, "finalize", "Assisted-by: Claude Code:claude-fable-5-1");
}
