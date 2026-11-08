use std::collections::BTreeSet;

use crate::checks::support;
use crate::checks::{Check, Context};
use crate::finding::Finding;
use serde_json::json;

const UPDATE_BOT_FILES: &[&str] = &[
    ".github/dependabot.yml",
    ".github/dependabot.yaml",
    "renovate.json",
    "renovate.json5",
    ".renovaterc",
    ".renovaterc.json",
    ".gitlab/renovate.json",
];

pub struct T9;

impl Check for T9 {
    fn id(&self) -> &'static str {
        "T9"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let Some(freshness) = decl.convention.freshness.as_ref() else {
            return vec![Finding::fail(
                self.id(),
                "freshness",
                "freshness is not declared",
                None,
                Some("declare convention.freshness".to_string()),
            )];
        };

        let mut findings = Vec::new();
        if let Some(file) = UPDATE_BOT_FILES
            .iter()
            .find(|file| ctx.root.join(*file).is_file())
        {
            findings.push(Finding::info(
                self.id(),
                "update bot",
                format!("update bot: {file}"),
                None,
            ));
        } else if decl.convention.commands.outdated.is_some() {
            findings.push(Finding::info(
                self.id(),
                "update cadence",
                "update cadence by command",
                None,
            ));
        } else {
            findings.push(Finding::fail(
                self.id(),
                "update mechanism",
                "no dependency update mechanism (bot configuration or commands.outdated)",
                None,
                Some("add a bot configuration or commands.outdated".to_string()),
            ));
        }

        let held_back = held_back_names(&decl.body);
        let outdated_command = decl.convention.commands.outdated.clone();

        if ctx.run && outdated_command.is_some() {
            if let Some(command) = &outdated_command {
                match crate::exec::run_command(&ctx.root, command) {
                    Ok(outcome) => {
                        let lower = outcome.output.to_lowercase();
                        if lower.contains("not found") || lower.contains("no such") {
                            findings.push(Finding::not_run(
                                self.id(),
                                "outdated",
                                "outdated command unavailable",
                                None,
                            ));
                        } else {
                            let (count, names) = count_outdated(&outcome.output, &held_back);
                            ctx.metrics
                                .borrow_mut()
                                .insert("outdated".to_string(), json!(count));
                            if count > freshness.max_outdated as usize {
                                findings.push(Finding::fail(
                                    self.id(),
                                    "outdated",
                                    format!(
                                        "{count} outdated dependencies (max {})",
                                        freshness.max_outdated
                                    ),
                                    Some(names.join(", ")),
                                    Some("update dependencies or declare exceptions".to_string()),
                                ));
                            } else {
                                findings.push(Finding::info(
                                    self.id(),
                                    "outdated",
                                    format!("{count} outdated dependencies"),
                                    None,
                                ));
                            }
                        }
                    }
                    Err(_) => findings.push(Finding::not_run(
                        self.id(),
                        "outdated",
                        "outdated command unavailable",
                        None,
                    )),
                }
            }
        } else {
            findings.push(Finding::info(
                self.id(),
                "lag",
                "lag not measured (run with --run and declare commands.outdated)",
                None,
            ));
        }

        if ctx.run {
            if let Some(command) = &decl.convention.commands.audit {
                if let Ok(outcome) = crate::exec::run_command(&ctx.root, command) {
                    check_audit(self.id(), &outcome.output, &held_back, &mut findings);
                }
            }
        }

        findings
    }
}

fn held_back_names(body: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut in_section = false;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            in_section = trimmed.to_lowercase().contains("held back");
            continue;
        }
        if in_section && trimmed.starts_with("- ") {
            // Names are usually written in backticks; the first word is the name.
            if let Some(name) = trimmed[2..].split_whitespace().next() {
                names.insert(name.trim_matches('`').to_string());
            }
        }
    }

    names
}

fn count_outdated(output: &str, held_back: &BTreeSet<String>) -> (usize, Vec<String>) {
    let mut count = 0usize;
    let mut names = Vec::new();

    for line in output.lines() {
        if !is_outdated_row(line) {
            continue;
        }
        if let Some(name) = line.split_whitespace().next() {
            if !held_back.contains(name) {
                count += 1;
                if names.len() < 10 {
                    names.push(name.to_string());
                }
            }
        }
    }

    (count, names)
}

fn is_outdated_row(line: &str) -> bool {
    let mut parts = line.split_whitespace();
    let first = parts.next();
    let second = parts.next();
    let third = parts.next();

    matches!(first, Some(name) if !name.is_empty()) && has_version(second) && has_version(third)
}

fn has_version(token: Option<&str>) -> bool {
    let Some(token) = token else {
        return false;
    };
    token
        .as_bytes()
        .windows(3)
        .any(|window| window[0].is_ascii_digit() && window[1] == b'.' && window[2].is_ascii_digit())
}

fn check_audit(id: &str, output: &str, held_back: &BTreeSet<String>, findings: &mut Vec<Finding>) {
    // cargo-audit, npm audit and pip-audit print one block per advisory; the
    // crate name comes on its own labelled line before the status lines, so
    // the last name seen is the one the warning belongs to.
    let mut current_name: Option<String> = None;
    let mut reported: BTreeSet<String> = BTreeSet::new();

    for line in output.lines() {
        if let Some(name) = name_after_label(line) {
            current_name = Some(name);
        }
        let lower = line.to_lowercase();
        if !(lower.contains("unmaintained") || lower.contains("yanked")) {
            continue;
        }
        let name = current_name
            .clone()
            .unwrap_or_else(|| line.trim().chars().take(80).collect());
        if held_back.contains(&name) || !reported.insert(name.clone()) {
            continue;
        }
        findings.push(Finding::fail(
            id,
            "audit",
            format!("unmaintained dependency without a declared exception: {name}"),
            None,
            None,
        ));
        if reported.len() >= 10 {
            break;
        }
    }
}

fn name_after_label(line: &str) -> Option<String> {
    for label in ["Crate:", "Package:", "name:"] {
        if let Some(index) = line.find(label) {
            let rest = line[index + label.len()..].trim();
            if let Some(name) = rest.split_whitespace().next() {
                return Some(name.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_outdated_rows() {
        assert!(is_outdated_row("serde 1.0.0 1.0.9"));
        assert!(is_outdated_row("  foo v1.0.0 v1.0.1"));
        assert!(!is_outdated_row("Package Version Latest"));
        assert!(!is_outdated_row("serde 1.0.0"));
    }

    const AUDIT_BLOCK: &str = "Crate:     unsafe-libyaml
Version:   0.2.11
Warning:   unmaintained
Title:     unsafe-libyaml is unmaintained
Date:      2024-03-25

warning: 1 allowed warning found
";

    #[test]
    fn audit_block_yields_one_finding_per_crate() {
        let mut findings = Vec::new();
        check_audit("T9", AUDIT_BLOCK, &BTreeSet::new(), &mut findings);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].problem.ends_with("unsafe-libyaml"));
    }

    #[test]
    fn audit_respects_held_back() {
        let mut findings = Vec::new();
        let held: BTreeSet<String> = ["unsafe-libyaml".to_string()].into_iter().collect();
        check_audit("T9", AUDIT_BLOCK, &held, &mut findings);
        assert!(findings.is_empty());
    }

    #[test]
    fn parses_held_back_names() {
        let body = "\n## Dependencies held back\n\n- serde\n- foo bar\n";
        let names = held_back_names(body);
        assert!(names.contains("serde"));
        assert!(names.contains("foo"));
    }
}
