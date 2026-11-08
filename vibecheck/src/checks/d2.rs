use crate::checks::Check;
use crate::declaration;
use crate::finding::Finding;
use crate::util;

pub struct D2;

impl Check for D2 {
    fn id(&self) -> &'static str {
        "D2"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        if let Some(error) = &ctx.decl_error {
            return vec![Finding::fail(
                "D2",
                "Declaration",
                error.clone(),
                Some("VIBECODING.md".into()),
                Some("Create a valid VIBECODING.md front matter".into()),
            )];
        }

        let Some(decl) = &ctx.decl else {
            return vec![Finding::not_run(
                "D2",
                "Declaration",
                "VIBECODING.md is missing",
                Some("VIBECODING.md".into()),
            )];
        };

        let mut findings = Vec::new();
        let text = util::read_file_if_exists(&ctx.root.join("VIBECODING.md")).unwrap_or_default();
        for key in declaration::missing_required_keys(&text) {
            findings.push(Finding::fail(
                "D2",
                "Declaration",
                format!("missing required key: {key}"),
                Some("VIBECODING.md".into()),
                Some(format!("Add {key} to VIBECODING.md")),
            ));
        }

        if !(1..=3).contains(&decl.convention.level) {
            findings.push(Finding::fail(
                "D2",
                "Declaration",
                format!(
                    "convention.level must be 1..=3, found {}",
                    decl.convention.level
                ),
                Some("VIBECODING.md".into()),
                Some("Set convention.level to 1, 2, or 3".into()),
            ));
        }

        if decl.convention.level >= 2 {
            if decl.convention.coverage.is_none() {
                findings.push(Finding::fail(
                    "D2",
                    "Declaration",
                    "level 2 requires convention.coverage",
                    Some("VIBECODING.md".into()),
                    Some("Add convention.coverage".into()),
                ));
            }
            if decl.convention.freshness.is_none() {
                findings.push(Finding::fail(
                    "D2",
                    "Declaration",
                    "level 2 requires convention.freshness",
                    Some("VIBECODING.md".into()),
                    Some("Add convention.freshness".into()),
                ));
            }
            if decl.convention.maintainability.is_none() {
                findings.push(Finding::fail(
                    "D2",
                    "Declaration",
                    "level 2 requires convention.maintainability",
                    Some("VIBECODING.md".into()),
                    Some("Add convention.maintainability".into()),
                ));
            }
            let evidence = decl.convention.evidence.as_ref();
            if evidence.is_none_or(|e| e.coverage.is_none() || e.audit.is_none()) {
                findings.push(Finding::fail(
                    "D2",
                    "Declaration",
                    "level 2 requires convention.evidence.coverage and convention.evidence.audit",
                    Some("VIBECODING.md".into()),
                    Some(
                        "declare evidence.coverage and evidence.audit (files written by CI)".into(),
                    ),
                ));
            }
        }

        if decl.convention.adopted_at.is_empty() || !ctx.git.is_repo() {
            findings.push(Finding::not_run(
                "D2",
                "Declaration",
                "cannot verify adopted_at commit",
                Some("VIBECODING.md".into()),
            ));
        } else if !ctx.git.commit_exists(&decl.convention.adopted_at) {
            findings.push(Finding::fail(
                "D2",
                "Declaration",
                format!(
                    "adopted_at commit does not exist: {}",
                    decl.convention.adopted_at
                ),
                Some("VIBECODING.md".into()),
                Some("Set adopted_at to an existing commit".into()),
            ));
        }

        let commits = ctx.git.commits_since(&decl.convention.adopted_at);
        for tool in &decl.convention.tools {
            let attributed = commits.iter().any(|commit| {
                commit.trailers.iter().any(|(key, value)| {
                    (key.eq_ignore_ascii_case("Assisted-by")
                        || key.eq_ignore_ascii_case("Generated-by"))
                        && value.starts_with(tool.as_str())
                })
            });
            if !attributed {
                findings.push(Finding::warn(
                    "D2",
                    "Declaration",
                    format!("tool {tool} declared but never attributed"),
                    Some("VIBECODING.md".into()),
                    Some("Add Assisted-by or Generated-by trailers using this tool".into()),
                ));
            }
        }

        findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use crate::git::Git;
    use std::path::PathBuf;

    #[test]
    fn reports_decl_error() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: None,
            decl_error: Some("bad".into()),
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
            metrics: Default::default(),
        };
        let findings = D2.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Fail));
    }
}
