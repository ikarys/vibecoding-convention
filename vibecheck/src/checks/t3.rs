use std::path::Path;

use crate::checks::support;
use crate::checks::{Check, Context};
use crate::finding::Finding;

const TEST_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "tsx", "go", "java", "rb", "php", "cs", "kt",
];
const DISABLED_PATTERNS: &[&str] = &[
    "#[ignore]",
    "@pytest.mark.skip",
    "@unittest.skip",
    "it.skip(",
    "test.skip(",
    "xit(",
    "xdescribe(",
    "t.Skip(",
    "@Disabled",
    "[Ignore]",
    "skip:",
];

pub struct T3;

impl Check for T3 {
    fn id(&self) -> &'static str {
        "T3"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(_) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let test_files = support::files_with_extension(
            &ctx.root,
            TEST_EXTENSIONS,
            &["git", "target", "node_modules"],
        )
        .into_iter()
        .filter(|path| path_contains_test_or_spec(path))
        .collect::<Vec<_>>();

        if test_files.is_empty() {
            return vec![Finding::fail(
                self.id(),
                "no test files",
                "no test files found",
                None,
                Some("add tests".to_string()),
            )];
        }

        let mut findings = vec![Finding::info(
            self.id(),
            "test files",
            format!("{} test files", test_files.len()),
            None,
        )];

        let mut unexplained = Vec::new();
        for path in &test_files {
            if let Some(text) = crate::util::read_file_if_exists(path) {
                let count = text
                    .lines()
                    .filter(|line| is_disabled(line) && !has_explanation(line))
                    .count();
                if count > 0 {
                    unexplained.push((path.clone(), count));
                }
            }
        }

        if unexplained.is_empty() {
            findings.push(Finding::info(
                self.id(),
                "disabled tests explained",
                "no unexplained disabled tests",
                None,
            ));
        } else {
            let total = unexplained.len();
            for (path, count) in unexplained.into_iter().take(10) {
                findings.push(Finding::warn(
                    self.id(),
                    "unexplained disabled tests",
                    format!("{} unexplained disabled tests", count),
                    Some(support::relative_path(&ctx.root, &path)),
                    Some("explain or remove disabled tests".to_string()),
                ));
            }
            if total > 10 {
                findings.push(Finding::warn(
                    self.id(),
                    "more unexplained disabled tests",
                    format!("{} more files have unexplained disabled tests", total - 10),
                    None,
                    None,
                ));
            }
        }

        findings
    }
}

fn path_contains_test_or_spec(path: &Path) -> bool {
    let text = path.to_string_lossy();
    text.contains("test") || text.contains("spec")
}

/// A marker that appears inside a string literal is test data, not a disabled test.
fn inside_string_literal(line: &str, marker: &str) -> bool {
    line.find(marker)
        .map(|index| line[..index].matches('"').count() % 2 == 1)
        .unwrap_or(false)
}

fn is_disabled(line: &str) -> bool {
    DISABLED_PATTERNS
        .iter()
        .any(|pattern| line.contains(pattern) && !inside_string_literal(line, pattern))
}

fn has_explanation(line: &str) -> bool {
    let re = regex::Regex::new(r"(?://|#|--)\s*[A-Za-z]{4,}").expect("static regex");
    re.is_match(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_inside_a_string_is_not_disabled() {
        assert!(is_disabled("    #[ignore]"));
        assert!(!is_disabled(
            "    fs::write(path, \"#[ignore]\n#[test]\").unwrap();"
        ));
    }
}
