use std::fs;

use crate::checks::support;
use crate::checks::{Check, Context};
use crate::declaration::Declaration;
use crate::finding::Finding;
use serde_json::json;

pub(crate) const SOURCE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "jsx", "ts", "tsx", "go", "java", "kt", "rb", "php", "cs", "c", "cc", "cpp",
    "h", "hpp", "swift",
];

const MARKER_PATTERNS: &[&str] = &[
    "TODO",
    "FIXME",
    "XXX",
    "HACK",
    "PLACEHOLDER",
    "placeholder",
    "implement me",
    "not implemented",
    "unimplemented!(",
    "todo!(",
    "NotImplementedError",
    "raise NotImplemented",
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
        let mut marker_count = 0usize;
        let mut marker_locations = Vec::new();

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

            for (index, line) in lines.iter().enumerate() {
                if is_marker_line(line) {
                    marker_count += 1;
                    if marker_locations.len() < 10 {
                        marker_locations.push(format!(
                            "{}:{}",
                            support::relative_path(&ctx.root, &path),
                            index + 1
                        ));
                    }
                }
            }
        }

        ctx.metrics
            .borrow_mut()
            .insert("file_max".to_string(), json!(file_max));
        ctx.metrics
            .borrow_mut()
            .insert("function_max".to_string(), json!(function_max));
        ctx.metrics
            .borrow_mut()
            .insert("markers".to_string(), json!(marker_count));

        if marker_count > maintainability.markers_max as usize {
            findings.push(Finding::fail(
                self.id(),
                "leftover markers",
                format!(
                    "{} leftover markers (max {})",
                    marker_count, maintainability.markers_max
                ),
                Some(marker_locations.join(", ")),
                Some("remove leftover markers".to_string()),
            ));
        } else {
            findings.push(Finding::info(
                self.id(),
                "leftover markers",
                format!("{marker_count} leftover markers"),
                None,
            ));
        }

        findings.extend(maintainability_evidence(ctx, decl, &maintainability.tool));

        if let Some(command) = decl.convention.commands.doc_lint.clone() {
            if crate::ci::ci_mentions(&ctx.root, &command).is_none() {
                findings.push(Finding::fail(
                    self.id(),
                    "doc_lint not in CI",
                    format!("doc_lint command not in CI: {command}"),
                    None,
                    Some("add the doc_lint command to CI".to_string()),
                ));
            }
        } else {
            findings.push(Finding::info(
                self.id(),
                "doc_lint",
                "no public API documentation lint declared",
                None,
            ));
        }

        findings
    }
}

fn maintainability_evidence(ctx: &Context, decl: &Declaration, tool: &str) -> Vec<Finding> {
    let id = "T8";
    let declared = decl
        .convention
        .evidence
        .as_ref()
        .and_then(|e| e.maintainability.as_deref());
    match declared.map(|path| (path, support::read_evidence(ctx, id, path))) {
        Some((path, Ok(_))) => vec![Finding::info(
            id,
            "maintainability evidence",
            format!("maintainability evidence present ({path})"),
            None,
        )],
        Some((_, Err(finding))) => vec![*finding],
        None => vec![Finding::info(
            id,
            "duplication and complexity not measured",
            format!("duplication and complexity need the declared tool ({tool})"),
            None,
        )],
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

fn is_marker_line(line: &str) -> bool {
    if marker_occurrences(line) > 2 {
        return false;
    }
    if has_marker_outside_string(line) {
        return true;
    }
    is_commented_code(line)
}

fn marker_occurrences(line: &str) -> usize {
    MARKER_PATTERNS
        .iter()
        .filter(|pattern| line.contains(**pattern))
        .count()
}

pub(crate) fn has_marker_outside_string(line: &str) -> bool {
    support::contains_outside_string(line, MARKER_PATTERNS)
}

fn is_commented_code(line: &str) -> bool {
    let trimmed = line.trim_start();
    // A block-comment continuation is `* text`; a bare `*` starts a deref.
    let Some(prefix) = ["//", "#", "--", "/*", "* "]
        .iter()
        .find(|prefix| trimmed.starts_with(**prefix))
    else {
        return false;
    };

    let content = trimmed[prefix.len()..].trim();
    if !content.ends_with([';', '{', '}', ')']) {
        return false;
    }
    if !content.contains('(') && !content.contains('=') {
        return false;
    }
    if content.len() > 1
        && content.starts_with(|c: char| c.is_ascii_alphabetic())
        && content.as_bytes()[1] == b' '
        && !content.contains('(')
    {
        return false;
    }
    true
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

    #[test]
    fn detects_leftover_markers() {
        assert!(is_marker_line("// TODO: implement"));
        assert!(is_marker_line("todo!()"));
        assert!(is_marker_line("// foo();"));
        assert!(is_marker_line("// let x = 1;"));
        assert!(!is_marker_line("// the counter is reset here"));
        assert!(is_marker_line(" * foo();"));
        assert!(!is_marker_line("*count += 1;"));
    }

    #[test]
    fn ignores_markers_inside_strings() {
        let marker = ["TO", "D"].concat();
        let line = format!("let s = \"{marker} in a string\";");
        assert!(!is_marker_line(&line));
    }
}
