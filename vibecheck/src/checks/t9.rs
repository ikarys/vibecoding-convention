use std::collections::BTreeSet;

use crate::checks::support;
use crate::checks::{Check, Context};
use crate::evidence;
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

        let held_back = held_back_names(&decl.body);
        let evidence = decl.convention.evidence.as_ref();
        let outdated_path = evidence.and_then(|e| e.outdated.as_deref());
        let bot = UPDATE_BOT_FILES
            .iter()
            .find(|file| ctx.root.join(*file).is_file());

        let mut findings = vec![update_mechanism(bot.copied(), outdated_path)];
        match outdated_path {
            Some(path) => findings.extend(outdated_evidence(
                ctx,
                path,
                freshness.max_outdated as usize,
                &held_back,
            )),
            None => {
                if bot.is_some() {
                    findings.push(Finding::info(
                        self.id(),
                        "lag",
                        "lag not measured: outdated evidence not declared",
                        None,
                    ));
                }
            }
        }
        if let Some(path) = evidence.and_then(|e| e.audit.as_deref()) {
            if let Ok(text) = evidence::read(&ctx.root, path) {
                findings.extend(unmaintained_findings(&text, &held_back));
            }
        }

        findings
    }
}

fn update_mechanism(bot: Option<&str>, outdated_path: Option<&str>) -> Finding {
    let id = "T9";
    match (bot, outdated_path) {
        (Some(file), _) => Finding::info(id, "update bot", format!("update bot: {file}"), None),
        (None, Some(_)) => Finding::info(id, "update cadence", "update cadence by evidence", None),
        (None, None) => Finding::fail(
            id,
            "update mechanism",
            "no dependency update mechanism (bot configuration or evidence.outdated)",
            None,
            Some("add a bot configuration or evidence.outdated".to_string()),
        ),
    }
}

fn outdated_evidence(
    ctx: &Context,
    path: &str,
    max_outdated: usize,
    held_back: &BTreeSet<String>,
) -> Vec<Finding> {
    let id = "T9";
    let text = match support::read_evidence(ctx, id, path) {
        Ok(text) => text,
        Err(finding) => return vec![*finding],
    };
    let (count, names) = evidence::outdated_count(&text, held_back);
    ctx.metrics
        .borrow_mut()
        .insert("outdated".to_string(), json!(count));
    let mut findings = vec![if count > max_outdated {
        Finding::fail(
            id,
            "outdated",
            format!("{count} outdated dependencies (max {max_outdated})"),
            Some(names.join(", ")),
            Some("update dependencies or declare exceptions".to_string()),
        )
    } else {
        Finding::info(
            id,
            "outdated",
            format!("{count} outdated dependencies ({path})"),
            None,
        )
    }];
    findings.extend(support::stale_evidence(ctx, id, path));
    findings
}

fn unmaintained_findings(text: &str, held_back: &BTreeSet<String>) -> Vec<Finding> {
    evidence::audit_warnings(text, held_back)
        .into_iter()
        .map(|name| {
            Finding::fail(
                "T9",
                "audit",
                format!("unmaintained dependency without a declared exception: {name}"),
                None,
                None,
            )
        })
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_held_back_names() {
        let body = "\n## Dependencies held back\n\n- serde\n- foo bar\n";
        let names = held_back_names(body);
        assert!(names.contains("serde"));
        assert!(names.contains("foo"));
    }
}
