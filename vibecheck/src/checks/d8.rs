use crate::checks::Check;
use crate::finding::Finding;
use crate::util;

pub struct D8;

impl Check for D8 {
    fn id(&self) -> &'static str {
        "D8"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let mut findings = Vec::new();

        let contributing = util::find_file_case_insensitive(&ctx.root, "CONTRIBUTING.md");
        match contributing {
            None => findings.push(Finding::fail(
                "D8",
                "Contribution policy",
                "CONTRIBUTING.md not found",
                Some("CONTRIBUTING.md".into()),
                Some("Create CONTRIBUTING.md".into()),
            )),
            Some(path) => {
                let text = util::read_file_if_exists(&path).unwrap_or_default();
                if !util::case_insensitive_contains(&text, "Assisted-by") {
                    findings.push(Finding::fail(
                        "D8",
                        "Contribution policy",
                        "CONTRIBUTING.md does not mention Assisted-by",
                        Some(path.display().to_string()),
                        Some("Document the Assisted-by trailer requirement".into()),
                    ));
                }
            }
        }

        let has_pr_template = ctx.root.join(".github/PULL_REQUEST_TEMPLATE.md").is_file()
            || ctx.root.join(".github/pull_request_template.md").is_file()
            || !util::find_files_by_glob(&ctx.root, ".gitlab/merge_request_templates/*").is_empty();

        if !has_pr_template {
            findings.push(Finding::warn(
                "D8",
                "Contribution policy",
                "no pull request or merge request template found",
                Some(".github".into()),
                Some("Add a PR/MR template".into()),
            ));
        }

        findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use crate::git::Git;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn fails_without_contributing() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: None,
            decl_error: None,
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
            run: false,
            metrics: Default::default(),
        };
        let findings = D8.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Fail));
    }

    #[test]
    fn passes_with_contributing() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("CONTRIBUTING.md"), "Assisted-by").unwrap();
        fs::create_dir_all(dir.path().join(".github")).unwrap();
        fs::write(dir.path().join(".github/PULL_REQUEST_TEMPLATE.md"), "x").unwrap();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: None,
            decl_error: None,
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
            run: false,
            metrics: Default::default(),
        };
        let findings = D8.run(&ctx);
        assert!(findings.is_empty());
    }
}
