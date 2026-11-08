#![allow(dead_code)]
pub mod helpers;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "Vibecheck Test")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Vibecheck Test")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .output()
        .expect("git command should run");
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

pub fn commit_all(dir: &Path, message: &str, trailer: Option<&str>) {
    git(dir, &["add", "-A"]);
    let full_message = match trailer {
        Some(trailer) => format!("{message}\n\n{trailer}"),
        None => message.to_string(),
    };
    git(dir, &["commit", "-m", &full_message]);
}

pub fn head_hash(dir: &Path) -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .expect("git rev-parse should run");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn write_compliant_project(dir: &Path) {
    fs::write(
        dir.join("README.md"),
        "See [VIBECODING.md](VIBECODING.md).\n",
    )
    .unwrap();
    fs::write(dir.join("VIBECODING.md"), level1_declaration()).unwrap();
    fs::write(dir.join("LICENSE"), "MIT License\n").unwrap();
    fs::write(dir.join("CONTRIBUTING.md"), "Use Assisted-by trailers.\n").unwrap();
    fs::write(
        dir.join("SECURITY.md"),
        "Report security issues within 72h.\n",
    )
    .unwrap();
    fs::write(
        dir.join("AGENTS.md"),
        "Convention level 1. Follow VIBECODING.md. Add Assisted-by trailers.\n",
    )
    .unwrap();
}

pub fn finalize_fixture(dir: &Path) {
    git(dir, &["init"]);
    write_compliant_project(dir);
    commit_all(
        dir,
        "Initial vibecoding project",
        Some("Assisted-by: Claude Code:claude-fable-5-1"),
    );
    let hash = head_hash(dir);
    let declaration =
        level1_declaration().replacen("0000000000000000000000000000000000000000", &hash, 1);
    fs::write(dir.join("VIBECODING.md"), declaration).unwrap();
    write_status(dir, 1, &hash);
    commit_all(
        dir,
        "Record adopted commit",
        Some("Assisted-by: Claude Code:claude-fable-5-1"),
    );
}

pub fn write_level2_project(dir: &Path) {
    write_compliant_project(dir);
    fs::write(dir.join("VIBECODING.md"), level2_declaration()).unwrap();
    fs::write(
        dir.join("AGENTS.md"),
        "Convention level 2. Follow VIBECODING.md. Add Assisted-by trailers. Run true.\n",
    )
    .unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::write(dir.join("Cargo.lock"), "# lockfile 3\n").unwrap();
    fs::create_dir_all(dir.join(".github/workflows")).unwrap();
    fs::write(
        dir.join(".github/workflows/ci.yml"),
        "name: ci\non: push\njobs:\n  test:\n    steps:\n      - run: true\n      - run: echo lint\n      - run: echo format\n      - run: echo audit\n      - run: echo static\n      - run: gitleaks detect\n      - run: echo lines 91% # cargo-llvm-cov stand-in\n",
    )
    .unwrap();
    fs::create_dir_all(dir.join(".github")).unwrap();
    fs::write(
        dir.join(".github/dependabot.yml"),
        "version: 2\nupdates: []\n",
    )
    .unwrap();
    fs::write(
        dir.join(".pre-commit-config.yaml"),
        "repos:\n  - repo: gitleaks\n",
    )
    .unwrap();
    fs::create_dir_all(dir.join("tests")).unwrap();
    fs::write(
        dir.join("tests/smoke.rs"),
        "#[test]\nfn smoke() {\n    assert_eq!(1, 1);\n}\n",
    )
    .unwrap();
    fs::create_dir_all(dir.join("tests/e2e")).unwrap();
    fs::write(dir.join("tests/e2e/README.md"), "End-to-end tests.\n").unwrap();
    fs::create_dir_all(dir.join("audit/evidence")).unwrap();
    fs::write(
        dir.join("audit/evidence/coverage.txt"),
        "Filename   Lines  Missed  Cover
TOTAL      100    9       91.00%
",
    )
    .unwrap();
    fs::write(
        dir.join("audit/evidence/audit.json"),
        "{\"vulnerabilities\":{\"count\":0,\"list\":[]},\"warnings\":{}}
",
    )
    .unwrap();
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("src/lib.rs"), "pub fn one() -> u32 {\n    1\n}\n").unwrap();
}

pub fn finalize_level2_fixture(dir: &Path) {
    git(dir, &["init"]);
    write_level2_project(dir);
    commit_all(
        dir,
        "Initial level 2 project",
        Some("Assisted-by: Claude Code:claude-fable-5-1"),
    );
    let hash = head_hash(dir);
    let declaration =
        level2_declaration().replacen("0000000000000000000000000000000000000000", &hash, 1);
    fs::write(dir.join("VIBECODING.md"), declaration).unwrap();
    write_status(dir, 2, &hash);
    commit_all(
        dir,
        "Record adopted commit",
        Some("Assisted-by: Claude Code:claude-fable-5-1"),
    );
}

fn write_status(dir: &Path, level: u8, hash: &str) {
    let date = chrono::Utc::now().naive_local().date().to_string();
    let short = &hash[..7.min(hash.len())];
    fs::create_dir_all(dir.join("audit")).unwrap();
    fs::write(
        dir.join("audit/status.json"),
        format!(
            "{{\n  \"vibecoding\": {{\n    \"level\": {level},\n    \"date\": \"{date}\",\n    \"commit\": \"{short}\"\n  }}\n}}\n"
        ),
    )
    .unwrap();
}

fn level1_declaration() -> String {
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
"#
    .to_string()
}

fn level2_declaration() -> String {
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
  level: 2
  adopted_at: 0000000000000000000000000000000000000000
  status: active
  tools:
    - "Claude Code:claude-fable-5-1"
  maintainers:
    - ikarys
  review: "single maintainer reads every diff before it reaches main"
  security_response: "72h"
  licence: MIT
  coverage: { threshold: 80, kind: line, tool: cargo-llvm-cov }
  maintainability: { tool: rust-code-analysis, duplication_max: 5, complexity_max: 15, function_max: 60, file_max: 600, markers_max: 0 }
  freshness: { max_outdated: 3, max_lag_months: 6 }
  commands:
    test: "true"
    lint: "echo lint"
    format: "echo format"
    audit: "echo audit"
    static_analysis: "echo static"
    secrets: "echo secrets"
    coverage: "echo lines 91%"
  evidence: { coverage: audit/evidence/coverage.txt, audit: audit/evidence/audit.json }
---
# Vibecoding

## Provenance and licence

We keep copyright and safeguard the code.
"#
    .to_string()
}
