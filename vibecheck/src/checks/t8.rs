use std::fs;

use crate::checks::support;
use crate::checks::{Check, Context};
use crate::finding::Finding;
use serde_json::json;

const SOURCE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "jsx", "ts", "tsx", "go", "java", "kt", "rb", "php", "cs", "c", "cc", "cpp",
    "h", "hpp", "swift",
];

pub struct T8;

impl Check for T8 {
    fn id(&self) -> &'static str {
        "T8"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let Some(maintainability) = decl.convention.maintainability.as_ref() else {
            return vec![Finding::fail(
                self.id(),
                "maintainability missing",
                "maintainability is not declared",
                None,
                Some("declare maintainability".to_string()),
            )];
        };

        let mut findings = Vec::new();
        for (name, value) in [
            ("duplication_max", maintainability.duplication_max),
            ("complexity_max", maintainability.complexity_max),
            ("function_max", maintainability.function_max),
            ("file_max", maintainability.file_max),
        ] {
            if value == 0 {
                findings.push(Finding::fail(
                    self.id(),
                    "maintainability threshold zero",
                    format!("{name} is 0"),
                    None,
                    Some("set a non-zero threshold".to_string()),
                ));
            }
        }

        let mut file_max = 0usize;
        let mut function_max = 0usize;
        let mut file_failures = 0;
        let mut function_failures = 0;

        for path in support::source_files(ctx, SOURCE_EXTENSIONS) {
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
            file_max = file_max.max(lines.len());

            if lines.len() > maintainability.file_max as usize && file_failures < 10 {
                findings.push(Finding::fail(
                    self.id(),
                    "file too long",
                    format!(
                        "file has {} lines (max {})",
                        lines.len(),
                        maintainability.file_max
                    ),
                    Some(support::relative_path(&ctx.root, &path)),
                    Some("split the file".to_string()),
                ));
                file_failures += 1;
            }

            let functions = function_lengths(&lines);
            function_max = function_max.max(functions.iter().copied().max().unwrap_or(0));
            let over = functions
                .iter()
                .filter(|len| **len > maintainability.function_max as usize)
                .count();
            if over > 0 && function_failures < 10 {
                findings.push(Finding::fail(
                    self.id(),
                    "functions too long",
                    format!(
                        "{} functions exceed max {}",
                        over, maintainability.function_max
                    ),
                    Some(support::relative_path(&ctx.root, &path)),
                    Some("split the functions".to_string()),
                ));
                function_failures += 1;
            }
        }

        ctx.metrics
            .borrow_mut()
            .insert("file_max".to_string(), json!(file_max));
        ctx.metrics
            .borrow_mut()
            .insert("function_max".to_string(), json!(function_max));

        let maintainability_command = decl.convention.commands.maintainability.clone();
        let mut maintainability_ran_ok = false;
        if ctx.run {
            if let Some(command) = &maintainability_command {
                match crate::exec::run_command(&ctx.root, command) {
                    Ok(outcome) => {
                        maintainability_ran_ok = outcome.success;
                        if !outcome.success {
                            let tail = support::last_lines(&outcome.output, 5);
                            findings.push(Finding::fail(
                                self.id(),
                                "maintainability command failed",
                                format!("maintainability command failed: {command}\n{tail}"),
                                None,
                                None,
                            ));
                        }
                    }
                    Err(err) => findings.push(Finding::fail(
                        self.id(),
                        "maintainability command failed",
                        format!("failed to run `{command}`: {err}"),
                        None,
                        None,
                    )),
                }
            }
        }

        if !maintainability_ran_ok {
            findings.push(Finding::info(
                self.id(),
                "duplication and complexity not measured",
                format!(
                    "duplication and complexity need the declared tool ({})",
                    maintainability.tool
                ),
                None,
            ));
        }

        findings
    }
}

fn function_lengths(lines: &[String]) -> Vec<usize> {
    let mut lengths = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if is_function_start(&lines[i]) {
            let start_indent = indent(&lines[i]);
            let mut length = 1;
            let mut j = i + 1;
            while j < lines.len() {
                length += 1;
                if indent(&lines[j]) <= start_indent {
                    break;
                }
                j += 1;
            }
            lengths.push(length);
            i = j;
        } else {
            i += 1;
        }
    }
    lengths
}

fn is_function_start(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("fn ")
        || trimmed.starts_with("def ")
        || trimmed.starts_with("func ")
        || trimmed.starts_with("function ")
        || trimmed.starts_with("sub ")
        || trimmed.contains("=> {")
    {
        return true;
    }
    trimmed.starts_with(|c: char| c.is_ascii_alphabetic())
        && ["public", "private", "protected"]
            .iter()
            .any(|keyword| trimmed.starts_with(keyword))
        && trimmed.contains('(')
        && trimmed.contains('{')
}

fn indent(line: &str) -> usize {
    line.chars().take_while(|c| c.is_whitespace()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_rust_function_length() {
        let lines = vec![
            "fn main() {".to_string(),
            "    let a = 1;".to_string(),
            "    let b = 2;".to_string(),
            "}".to_string(),
        ];
        assert_eq!(function_lengths(&lines), vec![4]);
    }
}
