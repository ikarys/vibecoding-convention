use chrono::NaiveDate;
use serde_json::Value;

use crate::checks::Check;
use crate::finding::Finding;
use crate::status;
use crate::util;

pub struct D5;

impl Check for D5 {
    fn id(&self) -> &'static str {
        "D5"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let Some(decl) = &ctx.decl else {
            return vec![Finding::not_run(
                "D5",
                "Dated check",
                "VIBECODING.md is missing",
                None,
            )];
        };

        let audit_dir = if decl.convention.audit_dir.is_empty() {
            "audit"
        } else {
            &decl.convention.audit_dir
        };
        let (path, value) = match status::read_status_value(&ctx.root, audit_dir) {
            Some(value) => value,
            None => {
                return vec![Finding::fail(
                    "D5",
                    "Dated check",
                    "audit/status.json is missing or unparsable",
                    Some(path_display(ctx.root.join(audit_dir).join("status.json"))),
                    Some("Run vibecheck --write-status".into()),
                )];
            }
        };

        let mut findings = Vec::new();
        let vibecoding = value.get("vibecoding").cloned().unwrap_or(Value::Null);

        let status_level = vibecoding.get("level").and_then(|v| v.as_u64());
        if status_level != Some(u64::from(decl.convention.level)) {
            findings.push(Finding::fail(
                "D5",
                "Dated check",
                format!(
                    "status level {} does not match convention.level {}",
                    status_level
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "missing".into()),
                    decl.convention.level
                ),
                Some(path.display().to_string()),
                Some("Update audit/status.json level".into()),
            ));
        }

        let commit = vibecoding
            .get("commit")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        if commit.is_empty() {
            findings.push(Finding::fail(
                "D5",
                "Dated check",
                "status commit is missing",
                Some(path.display().to_string()),
                Some("Set vibecoding.commit".into()),
            ));
        } else if ctx.git.is_repo() {
            if !ctx.git.commit_exists(&commit) {
                findings.push(Finding::fail(
                    "D5",
                    "Dated check",
                    format!("status commit does not exist: {commit}"),
                    Some(path.display().to_string()),
                    Some("Set vibecoding.commit to an existing commit".into()),
                ));
            }
        } else {
            findings.push(Finding::not_run(
                "D5",
                "Dated check",
                "not a git repository; cannot verify status commit",
                Some(path.display().to_string()),
            ));
        }

        let date_string = vibecoding
            .get("date")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        if let Ok(date) = NaiveDate::parse_from_str(date_string, "%Y-%m-%d") {
            let interval = decl
                .convention
                .check_interval
                .map(|value| value.as_duration())
                .unwrap_or_else(|| chrono::Duration::days(180));
            let now = chrono::Utc::now().naive_local().date();
            let mut stale = date < now - interval;

            if let Some((_, tag_date)) = ctx.git.latest_tag() {
                let tag_date = tag_date.naive_utc().date();
                stale |= date < tag_date;
            }

            if stale {
                let archived = decl.convention.status.eq_ignore_ascii_case("archived");
                if archived {
                    findings.push(Finding::info(
                        "D5",
                        "Dated check",
                        "status.json is stale",
                        Some(path.display().to_string()),
                    ));
                } else {
                    findings.push(Finding::warn(
                        "D5",
                        "Dated check",
                        "status.json is stale",
                        Some(path.display().to_string()),
                        Some("Run vibecheck --write-status".into()),
                    ));
                }
            }
        } else {
            findings.push(Finding::fail(
                "D5",
                "Dated check",
                format!("invalid status date: {date_string}"),
                Some(path.display().to_string()),
                Some("Set vibecoding.date to YYYY-MM-DD".into()),
            ));
        }

        let readme = util::find_file_case_insensitive(&ctx.root, "README.md");
        let readme_text = readme
            .as_ref()
            .and_then(|path| util::read_file_if_exists(path))
            .unwrap_or_default();
        if !util::case_insensitive_contains(&readme_text, "status.json") {
            findings.push(Finding::info(
                "D5",
                "Dated check",
                "badge not found; the D1 link is enough",
                readme.map(|path| path.display().to_string()),
            ));
        }

        findings
    }
}

fn path_display(path: std::path::PathBuf) -> String {
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use crate::declaration::Declaration;
    use crate::git::Git;
    use std::path::PathBuf;

    #[test]
    fn fails_without_status() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: Some(Declaration::default()),
            decl_error: None,
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
            metrics: Default::default(),
        };
        let findings = D5.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Fail));
    }

    #[test]
    fn path_display_works() {
        assert_eq!(path_display(PathBuf::from("a/b")), "a/b");
    }
}
