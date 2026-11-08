use crate::checks::Check;
use crate::finding::Finding;
use crate::util;

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
    use crate::git::Git;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn info_when_no_files() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: None,
            decl_error: None,
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
        };
        let findings = D10.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Info));
    }

    #[test]
    fn fails_on_missing_references() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("AGENTS.md"), "hello").unwrap();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: None,
            decl_error: None,
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
        };
        let findings = D10.run(&ctx);
        assert!(
            findings
                .iter()
                .filter(|f| f.severity == crate::finding::Severity::Fail)
                .count()
                >= 2
        );
    }
}
