//! A4: the audit index must list every report and name real commits.

use std::path::Path;

use crate::audit;
use crate::checks::support;
use crate::checks::{Check, Context};
use crate::evidence;
use crate::finding::{Finding, Severity};
use crate::status;

pub struct A4;

impl Check for A4 {
    fn id(&self) -> &'static str {
        "A4"
    }

    fn level(&self) -> u8 {
        3
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration("A4");
        };
        let Some(dir) = support::audit_dir(decl) else {
            return vec![Finding::fail(
                "A4",
                "Audit index",
                "audit_dir must be a relative path inside the repository",
                None,
                None,
            )];
        };

        let index_path = format!("{dir}/README.md");
        let Ok(index_text) = evidence::read(&ctx.root, &index_path) else {
            return vec![Finding::fail(
                "A4",
                "Audit index",
                "audit index missing",
                Some(index_path),
                Some(format!(
                    "create {dir}/README.md from templates/audit/README.md"
                )),
            )];
        };
        let rows = audit::parse_index(&index_text);
        let report_paths = list_report_paths(&ctx.root, &dir);
        let mut findings = Vec::new();

        findings.extend(
            report_paths
                .iter()
                .filter_map(|path| report_header_finding(ctx, &dir, &rows, path)),
        );

        if ctx.git.is_repo() {
            for row in &rows {
                if !ctx.git.commit_exists(&row.commit) {
                    findings.push(Finding::fail(
                        "A4",
                        "Audit index",
                        format!("index names a commit that does not exist: {}", row.commit),
                        Some(row.commit.clone()),
                        None,
                    ));
                }
            }
            for path in &report_paths {
                rewrite_findings(ctx, path, &mut findings);
            }
        }

        if let Some((_, value)) = status::read_status_value(&ctx.root, &dir) {
            if let Some(report) = value
                .get("vibecoding")
                .and_then(|vibecoding| vibecoding.get("report"))
                .and_then(|report| report.as_str())
            {
                let normalized = report.trim_start_matches("./");
                if !report_paths.iter().any(|path| path == normalized) {
                    findings.push(Finding::fail(
                        "A4",
                        "Audit index",
                        format!("status.json names a report that is not in {dir}"),
                        Some(report.to_string()),
                        None,
                    ));
                }
            }
        }

        let has_fail = findings
            .iter()
            .any(|finding| finding.severity == Severity::Fail);
        if report_paths.is_empty() && rows.is_empty() {
            findings.push(Finding::info(
                "A4",
                "Audit index",
                "no audit report yet",
                None,
            ));
        } else if !report_paths.is_empty() && !has_fail {
            findings.push(Finding::info(
                "A4",
                "Audit index",
                format!("{} reports indexed", report_paths.len()),
                None,
            ));
        }
        findings
    }
}

fn list_report_paths(root: &Path, dir: &str) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root.join(dir)) {
        for entry in entries.flatten() {
            let is_file = entry
                .file_type()
                .map(|file_type| file_type.is_file())
                .unwrap_or(false);
            if !is_file {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if audit::is_report_name(&name) {
                names.push(name);
            }
        }
    }
    names.sort();
    names
        .into_iter()
        .map(|name| format!("{dir}/{name}"))
        .collect()
}

fn report_header_finding(
    ctx: &Context,
    dir: &str,
    rows: &[audit::IndexRow],
    path: &str,
) -> Option<Finding> {
    let text = match support::read_evidence(ctx, "A4", path) {
        Ok(text) => text,
        Err(finding) => return Some(*finding),
    };
    let report = audit::parse_report(&text);
    let Some(header) = report.header else {
        return Some(Finding::fail(
            "A4",
            "Audit index",
            "report without a header",
            Some(path.to_string()),
            Some(
                "start the report with a header: date, commit, kind (see templates/audit/)"
                    .to_string(),
            ),
        ));
    };

    let row = rows.iter().find(|row| {
        row.date == header.date
            && audit::same_commit(&row.commit, &header.commit)
            && (row.kind.trim().is_empty()
                || row.kind.trim().eq_ignore_ascii_case(header.kind.trim()))
    });
    match row {
        None => Some(Finding::fail(
            "A4",
            "Audit index",
            "report not in the index",
            Some(path.to_string()),
            Some(format!("add its row to {dir}/README.md")),
        )),
        Some(row) => {
            if !row
                .read_by
                .trim()
                .eq_ignore_ascii_case(header.read_by.trim())
            {
                Some(Finding::fail(
                    "A4",
                    "Audit index",
                    format!(
                        "index row disagrees with the report header: read_by is {} in the index but {} in the report",
                        row.read_by, header.read_by
                    ),
                    Some(path.to_string()),
                    None,
                ))
            } else {
                None
            }
        }
    }
}

fn rewrite_findings(ctx: &Context, path: &str, findings: &mut Vec<Finding>) {
    let history = ctx.git.numstat_history(path);
    if history.len() < 2 {
        return;
    }
    // The last entry is the commit that created the report; only later
    // commits that removed lines are rewrites.
    for (hash, _added, deleted) in history.iter().take(history.len() - 1) {
        if *deleted > 0 {
            let short_hash = &hash[..hash.len().min(7)];
            findings.push(Finding::warn(
                "A4",
                "Audit index",
                "report rewritten after the fact",
                Some(format!("{path} ({short_hash})")),
                Some(
                    "append decisions instead of editing; if the edit corrected an error, say so in the commit"
                        .to_string(),
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::declaration::{Convention, Declaration};
    use std::path::Path;

    fn declaration_with_audit_dir(audit_dir: &str) -> Declaration {
        Declaration {
            convention: Convention {
                level: 3,
                audit_dir: audit_dir.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn write_file(root: &Path, relative: &str, content: &str) {
        let path = root.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }

    fn matching_index() -> &'static str {
        "| Date | Commit | Kind | Tool / model | Read by | Verdict | Open findings |\n\
         |---|---|---|---|---|---|---|\n\
         | 2026-11-08 | abc1234 | architecture | Claude Code:claude-fable-5-1 | ikarys | usable | 0 |\n"
    }

    fn matching_report() -> &'static str {
        "---\n\
         date: 2026-11-08\n\
         commit: abc1234\n\
         kind: architecture\n\
         read_by: ikarys\n\
         ---\n\
         - Severity: MINOR\n"
    }

    #[test]
    fn index_missing_is_fail() {
        let root = tempfile::tempdir().unwrap();
        let ctx = Context::for_test(root.path(), Some(declaration_with_audit_dir("audit")));
        let findings = A4.run(&ctx);
        assert!(findings.iter().any(|finding| finding.id == "A4"
            && finding.severity == Severity::Fail
            && finding.problem.contains("audit index missing")));
    }

    #[test]
    fn matching_report_and_index_has_no_fail() {
        let root = tempfile::tempdir().unwrap();
        write_file(root.path(), "audit/README.md", matching_index());
        write_file(
            root.path(),
            "audit/2026-11-08-architecture.md",
            matching_report(),
        );
        let ctx = Context::for_test(root.path(), Some(declaration_with_audit_dir("audit")));
        let findings = A4.run(&ctx);
        assert!(!findings
            .iter()
            .any(|finding| finding.severity == Severity::Fail));
        assert!(findings
            .iter()
            .any(|finding| finding.severity == Severity::Info
                && finding.problem.contains("1 reports indexed")));
    }

    #[test]
    fn unlisted_report_is_fail() {
        let root = tempfile::tempdir().unwrap();
        write_file(root.path(), "audit/README.md", matching_index());
        write_file(
            root.path(),
            "audit/2026-11-09-security.md",
            &matching_report().replace("architecture", "security"),
        );
        let ctx = Context::for_test(root.path(), Some(declaration_with_audit_dir("audit")));
        let findings = A4.run(&ctx);
        assert!(findings.iter().any(|finding| finding.id == "A4"
            && finding.severity == Severity::Fail
            && finding.problem.contains("not in the index")));
    }

    #[test]
    fn non_relative_audit_dir_is_fail() {
        let root = tempfile::tempdir().unwrap();
        let ctx = Context::for_test(root.path(), Some(declaration_with_audit_dir("../x")));
        let findings = A4.run(&ctx);
        assert_eq!(findings.len(), 1);
        assert!(findings[0].problem.contains("relative path"));
    }
}
