use crate::checks::Check;
use crate::finding::Finding;
use crate::util;
use regex::Regex;

pub struct D10;

impl Check for D10 {
    fn id(&self) -> &'static str {
        "D10"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let declared: Vec<String> = ctx
            .decl
            .as_ref()
            .map(|decl| decl.convention.agent_instruction_files.clone())
            .unwrap_or_default();
        let files = agent_files(&ctx.root, &declared);
        let trailers: Vec<String> = ctx
            .decl
            .as_ref()
            .map(|decl| decl.convention.attribution_trailers())
            .unwrap_or_else(|| {
                crate::declaration::DEFAULT_ATTRIBUTION_TRAILERS
                    .iter()
                    .map(|t| t.to_lowercase())
                    .collect()
            });
        if files.is_empty() {
            return vec![Finding::info(
                "D10",
                "Agent instructions",
                "no agent instruction file",
                None,
            )];
        }

        let test_command = ctx
            .decl
            .as_ref()
            .and_then(|decl| decl.convention.commands.test.clone());
        let level_regex = Regex::new(r"(?i)\blevel\s+(\d+)\b").expect("static regex");

        let mut findings = Vec::new();
        for file in files {
            let text = util::read_file_if_exists(&file).unwrap_or_default();
            let display = file.display().to_string();

            if !util::case_insensitive_contains(&text, "VIBECODING.md") {
                findings.push(Finding::fail(
                    "D10",
                    "Agent instructions",
                    "missing VIBECODING.md reference",
                    Some(display.clone()),
                    Some("Reference VIBECODING.md".into()),
                ));
            }
            if !trailers
                .iter()
                .any(|trailer| util::case_insensitive_contains(&text, trailer))
            {
                findings.push(Finding::fail(
                    "D10",
                    "Agent instructions",
                    "no attribution trailer named",
                    Some(display.clone()),
                    Some("Tell the agent which trailer to put on AI-written commits".into()),
                ));
            }
            if let Some(command) = &test_command {
                if !text.contains(command.as_str()) {
                    findings.push(Finding::fail(
                        "D10",
                        "Agent instructions",
                        format!("missing declared test command: {command}"),
                        Some(display.clone()),
                        Some("Include the declared test command".into()),
                    ));
                }
            }

            if let Some(decl) = &ctx.decl {
                let declared = decl.convention.level.to_string();
                let named: Vec<&str> = level_regex
                    .captures_iter(&text)
                    .filter_map(|c| c.get(1).map(|m| m.as_str()))
                    .collect();
                match named.first() {
                    None => findings.push(Finding::fail(
                        "D10",
                        "Agent instructions",
                        "convention level not named",
                        Some(display.clone()),
                        Some("Name the declared convention level".into()),
                    )),
                    Some(found) if !named.contains(&declared.as_str()) => {
                        findings.push(Finding::fail(
                            "D10",
                            "Agent instructions",
                            format!("names level {found}, declared level {declared}"),
                            Some(display.clone()),
                            Some("Match the level declared in VIBECODING.md".into()),
                        ))
                    }
                    Some(_) => {}
                }
            }

            for (pattern, match_) in util::scan_secrets(&text) {
                findings.push(Finding::fail(
                    "D10",
                    "Agent instructions",
                    format!("secret pattern {pattern} found: {match_}"),
                    Some(display.clone()),
                    Some("Remove the secret".into()),
                ));
            }
        }

        findings
    }
}

fn agent_files(root: &std::path::Path, declared: &[String]) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();

    for name in declared {
        let path = root.join(name);
        if path.is_file() {
            files.push(path);
        }
    }

    for name in ["AGENTS.md", "CLAUDE.md", ".cursorrules", "GEMINI.md"] {
        if let Some(path) = util::find_file_case_insensitive(root, name) {
            files.push(path);
        }
    }

    let copilot = root.join(".github/copilot-instructions.md");
    if copilot.is_file() {
        files.push(copilot);
    }

    files.extend(util::find_files_by_glob(root, ".cursor/rules/*.md"));
    files.sort();
    files.dedup();
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use std::fs;

    const VIBECODING_SAMPLE: &str = r#"---
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
  adopted_at: 03c8df2
  status: active
  tools:
    - "Claude Code:claude-fable-5-1"
  attribution_trailers: [Assisted-by, Generated-by]
  coverage: { threshold: 80, kind: line, tool: cargo-llvm-cov }
  maintainability: { tool: none, duplication_max: 5, complexity_max: 15, function_max: 80, file_max: 400, markers_max: 0 }
  freshness: { max_outdated: 3, max_lag_months: 6 }
  evidence: { coverage: audit/evidence/coverage.txt, audit: audit/evidence/audit.json }
  commands:
    test: "cargo test --manifest-path vibecheck/Cargo.toml"
    lint: "cargo clippy --manifest-path vibecheck/Cargo.toml --all-targets -- -D warnings"
    format: "cargo fmt --manifest-path vibecheck/Cargo.toml --check"
    coverage: "cargo llvm-cov --manifest-path vibecheck/Cargo.toml --summary-only"
    audit: "cargo audit --file vibecheck/Cargo.lock"
    static_analysis: "cargo clippy --manifest-path vibecheck/Cargo.toml --all-targets -- -D warnings"
    secrets: "gitleaks detect --no-banner"
    outdated: "cargo outdated --manifest-path vibecheck/Cargo.toml --exit-code 1"
  maintainers: [ikarys]
  review: "single maintainer reads every diff before it reaches main"
  security_response: "none"
  licence: MIT
---
# Vibecoding

## Provenance and licence

We keep copyright and safeguard the code.
"#;

    fn declaration_with_level(
        root: &std::path::Path,
        level: u8,
    ) -> crate::declaration::Declaration {
        let text = VIBECODING_SAMPLE.replace("level: 2", &format!("level: {level}"));
        fs::write(root.join("VIBECODING.md"), text).unwrap();
        let (decl, err) = crate::declaration::read_declaration(root);
        assert!(err.is_none(), "{err:?}");
        let decl = decl.unwrap();
        assert_eq!(decl.convention.level, level);
        decl
    }

    #[test]
    fn info_when_no_files() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Context::for_test(dir.path(), None);
        let findings = D10.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Info));
    }

    #[test]
    fn fails_on_missing_references() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("AGENTS.md"), "hello").unwrap();
        let ctx = Context::for_test(dir.path(), None);
        let findings = D10.run(&ctx);
        assert!(
            findings
                .iter()
                .filter(|f| f.severity == crate::finding::Severity::Fail)
                .count()
                >= 2
        );
    }

    #[test]
    fn fails_when_agent_file_names_wrong_level() {
        let dir = tempfile::tempdir().unwrap();
        let decl = declaration_with_level(dir.path(), 2);
        fs::write(
            dir.path().join("AGENTS.md"),
            "Convention level 1. Follow VIBECODING.md. Add Assisted-by trailers. cargo test --manifest-path vibecheck/Cargo.toml\n",
        )
        .unwrap();
        let ctx = Context::for_test(dir.path(), Some(decl));
        let findings = D10.run(&ctx);
        assert!(findings.iter().any(|f| {
            f.severity == crate::finding::Severity::Fail && f.problem.contains("declared level 2")
        }));
    }

    #[test]
    fn passes_when_agent_file_names_declared_level() {
        let dir = tempfile::tempdir().unwrap();
        let decl = declaration_with_level(dir.path(), 2);
        fs::write(
            dir.path().join("AGENTS.md"),
            "Convention level 2. Follow VIBECODING.md. Add Assisted-by trailers. cargo test --manifest-path vibecheck/Cargo.toml\n",
        )
        .unwrap();
        let ctx = Context::for_test(dir.path(), Some(decl));
        let findings = D10.run(&ctx);
        assert!(!findings.iter().any(|f| {
            f.severity == crate::finding::Severity::Fail && f.problem.contains("level")
        }));
    }
}
