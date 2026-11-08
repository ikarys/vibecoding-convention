//! Shared helpers for the integration tests.
#![allow(dead_code)]

use std::fs;
use std::path::Path;

use vibecheck::finding::{Finding, Severity};
use vibecheck::Options;

pub fn run_default(dir: &Path) -> vibecheck::Report {
    vibecheck::run(dir, Options::default()).unwrap()
}

pub fn has_finding<'a>(
    findings: impl IntoIterator<Item = &'a Finding>,
    id: &str,
    severity: Severity,
    needle: &str,
) -> bool {
    let findings: Vec<&Finding> = findings.into_iter().collect();
    let found = findings
        .iter()
        .any(|f| f.id == id && f.severity == severity && f.problem.contains(needle));
    if !found {
        for f in findings.iter().filter(|f| f.id == id) {
            eprintln!(
                "{id} finding: {:?} {} ({:?})",
                f.severity, f.problem, f.where_
            );
        }
    }
    found
}

pub fn has_finding_where<'a>(
    findings: impl IntoIterator<Item = &'a Finding>,
    id: &str,
    severity: Severity,
    problem_needle: &str,
    where_needle: &str,
) -> bool {
    findings.into_iter().any(|f| {
        f.id == id
            && f.severity == severity
            && f.problem.contains(problem_needle)
            && f.where_
                .as_deref()
                .is_some_and(|where_| where_.contains(where_needle))
    })
}

pub fn replace_in_file(dir: &Path, file: &str, old: &str, new: &str) {
    let path = dir.join(file);
    let text = fs::read_to_string(&path).unwrap();
    fs::write(path, text.replace(old, new)).unwrap();
}

pub fn short_hash(dir: &Path) -> String {
    super::head_hash(dir).chars().take(7).collect()
}

pub fn write_status_json(dir: &Path, level: u8, date: &str, commit: &str) {
    fs::create_dir_all(dir.join("audit")).unwrap();
    fs::write(
        dir.join("audit/status.json"),
        format!(
            "{{\n  \"vibecoding\": {{\n    \"level\": {level},\n    \"date\": \"{date}\",\n    \"commit\": \"{commit}\"\n  }}\n}}\n"
        ),
    )
    .unwrap();
}

pub fn minimal_level2(
    dir: &Path,
    coverage: &str,
    threshold: u32,
    test: &str,
    audit: &str,
    maintainability: &str,
) {
    let hash = super::head_hash(dir);
    fs::write(
        dir.join("VIBECODING.md"),
        format!(
            r#"---
version: "0.1"
level: "2"
processes: {{}}
convention:
  version: "0.1"
  level: 2
  adopted_at: {hash}
  status: active
  tools: []
  maintainers: []
  review: manual
  security_response: none
  licence: MIT
  coverage: {{ threshold: {threshold}, kind: line, tool: cov }}
  maintainability: {{ tool: m, duplication_max: 5, complexity_max: 15, function_max: 60, file_max: 600 }}
  commands:
    test: "{test}"
    coverage: {coverage}
    audit: {audit}
    maintainability: {maintainability}
  evidence: {{ coverage: audit/evidence/coverage.txt, audit: audit/evidence/audit.json }}
---
# Body
"#
        ),
    )
    .unwrap();
}

pub fn d2_declaration(hash: &str, level: u8, tools: &str, extra: &str) -> String {
    format!(
        r#"---
version: "0.1"
level: "2"
processes: {{}}
convention:
  version: "0.1"
  level: {level}
  adopted_at: {hash}
  status: active
  tools: {tools}
  maintainers: []
  review: manual
  security_response: none
  licence: MIT
{extra}
---
# Body
"#
    )
}
