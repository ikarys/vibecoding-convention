//! A5: a release tag must not leave blocking audit findings open and must
//! name a declared maintainer as approver.

use std::path::Path;

use crate::audit;
use crate::checks::support;
use crate::checks::{Check, Context};
use crate::finding::Finding;

pub struct A5;

impl Check for A5 {
    fn id(&self) -> &'static str {
        "A5"
    }

    fn level(&self) -> u8 {
        3
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration("A5");
        };
        if !ctx.git.is_repo() {
            return vec![Finding::not_run(
                "A5",
                "Release rule",
                "not a git repository",
                None,
            )];
        }
        let Some((tag, _)) = ctx.git.latest_tag() else {
            return vec![Finding::not_run(
                "A5",
                "Release rule",
                "no release tag yet",
                None,
            )];
        };

        let Some(dir) = support::audit_dir(decl) else {
            return vec![Finding::fail(
                "A5",
                "Release rule",
                "audit_dir must be a relative path inside the repository",
                None,
                None,
            )];
        };

        let mut findings = open_blocking_findings(ctx, &tag, &dir);
        if findings.is_empty() {
            findings.push(Finding::info(
                "A5",
                "Release rule",
                format!("no open BLOCKER or DOC-WRONG at {tag}"),
                None,
            ));
        }

        let maintainers: Vec<String> = decl
            .convention
            .maintainers
            .iter()
            .map(|maintainer| maintainer.trim().to_string())
            .filter(|maintainer| !maintainer.is_empty())
            .collect();
        if maintainers.is_empty() {
            findings.push(Finding::fail(
                "A5",
                "Release rule",
                "no maintainer declared",
                None,
                None,
            ));
            return findings;
        }

        let version = tag.strip_prefix(['v', 'V']).unwrap_or(&tag);
        let mut approver_text = ctx.git.tag_message(&tag);
        for name in ["CHANGELOG.md", "CHANGES.md", "RELEASES.md", "HISTORY.md"] {
            if let Some(text) = ctx.git.show_at(&tag, name) {
                approver_text.push('\n');
                approver_text.push_str(&notes_section(&text, version));
            }
        }

        let lower_text = approver_text.to_lowercase();
        let approved = maintainers
            .iter()
            .find(|maintainer| lower_text.contains(&maintainer.to_lowercase()));
        match approved {
            Some(name) => findings.push(Finding::info(
                "A5",
                "Release rule",
                format!("release {tag} approved by {name}"),
                None,
            )),
            None => findings.push(Finding::fail(
                "A5",
                "Release rule",
                format!("release {tag} names no declared maintainer as approver"),
                None,
                Some(
                    "name the approving maintainer in the tag message or the release notes"
                        .to_string(),
                ),
            )),
        }
        findings
    }
}

/// One FAIL per report at `tag` and per blocking severity left open there.
fn open_blocking_findings(ctx: &Context, tag: &str, dir: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for path in ctx.git.files_at(tag, dir) {
        if !is_report_path(&path) {
            continue;
        }
        let Some(text) = ctx.git.show_at(tag, &path) else {
            continue;
        };
        let report = audit::parse_report(&text);
        for severity in ["BLOCKER", "DOC-WRONG"] {
            let Some(count) = report.open.get(severity) else {
                continue;
            };
            findings.push(Finding::fail(
                "A5",
                "Release rule",
                format!("release {tag} has {count} open {severity}"),
                Some(path.clone()),
                Some(
                    "decide the finding (fixed, documented, won't fix) before releasing"
                        .to_string(),
                ),
            ));
        }
    }
    findings
}

fn is_report_path(path: &str) -> bool {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(audit::is_report_name)
        .unwrap_or(false)
}

fn notes_section(text: &str, version: &str) -> String {
    let mut section = String::new();
    let mut in_section = false;
    let mut section_depth = 0usize;
    for line in text.lines() {
        if let Some(depth) = heading_depth(line) {
            if in_section && depth <= section_depth {
                break;
            }
            if !in_section && line.contains(version) {
                in_section = true;
                section_depth = depth;
            }
        }
        if in_section {
            section.push_str(line);
            section.push('\n');
        }
    }
    section
}

fn heading_depth(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let depth = trimmed
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if depth > 0 {
        Some(depth)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notes_section_finds_version_section() {
        let text = "# Changelog\n## 0.1.0\n- fix\n## 0.0.9\n- old\n";
        assert_eq!(notes_section(text, "0.1.0"), "## 0.1.0\n- fix\n");
    }

    #[test]
    fn notes_section_absent_version_is_empty() {
        let text = "# Changelog\n## 0.2.0\n- new\n";
        assert_eq!(notes_section(text, "0.1.0"), "");
    }
}
