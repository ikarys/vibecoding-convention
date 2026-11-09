use crate::checks::Check;
use crate::declaration::Convention;
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
                let trailers = ctx
                    .decl
                    .as_ref()
                    .map(|decl| decl.convention.attribution_trailers())
                    .unwrap_or_else(|| Convention::default().attribution_trailers());
                if !trailers
                    .iter()
                    .any(|trailer| util::case_insensitive_contains(&text, trailer))
                {
                    findings.push(Finding::fail(
                        "D8",
                        "Contribution policy",
                        format!(
                            "CONTRIBUTING.md names none of the declared attribution trailers ({})",
                            trailers.join(", ")
                        ),
                        Some(path.display().to_string()),
                        Some("say which trailer AI-assisted pull requests must carry".into()),
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
    use std::fs;

    #[test]
    fn fails_without_contributing() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Context::for_test(dir.path(), None);
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
        let ctx = Context::for_test(dir.path(), None);
        let findings = D8.run(&ctx);
        assert!(findings.is_empty());
    }

    #[test]
    fn accepts_the_declared_trailer_only() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("CONTRIBUTING.md"), "Assisted-by").unwrap();
        let ctx = Context::for_test(dir.path(), Some(decl_with_trailer("AI-Tool")));
        assert!(D8.run(&ctx).iter().any(|f| f
            .problem
            .contains("none of the declared attribution trailers")));

        fs::write(dir.path().join("CONTRIBUTING.md"), "AI-Tool: x").unwrap();
        let ctx = Context::for_test(dir.path(), Some(decl_with_trailer("AI-Tool")));
        assert!(!D8.run(&ctx).iter().any(|f| f.problem.contains("trailers")));
    }

    fn decl_with_trailer(trailer: &str) -> crate::declaration::Declaration {
        crate::declaration::Declaration {
            convention: Convention {
                attribution_trailers: vec![trailer.to_string()],
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
