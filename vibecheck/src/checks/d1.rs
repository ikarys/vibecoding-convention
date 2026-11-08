use crate::checks::Check;
use crate::finding::Finding;
use crate::util;

pub struct D1;

impl Check for D1 {
    fn id(&self) -> &'static str {
        "D1"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let readme = util::find_file_case_insensitive(&ctx.root, "README.md");
        match readme {
            None => vec![Finding::fail(
                "D1",
                "README discloses AI assistance",
                "README.md not found",
                Some("README.md".into()),
                Some("Create README.md and link VIBECODING.md".into()),
            )],
            Some(path) => {
                let text = util::read_file_if_exists(&path).unwrap_or_default();
                if util::case_insensitive_contains(&text, "VIBECODING.md") {
                    Vec::new()
                } else {
                    vec![Finding::fail(
                        "D1",
                        "README discloses AI assistance",
                        "README.md does not mention VIBECODING.md",
                        Some(path.display().to_string()),
                        Some("Add a link or badge to VIBECODING.md".into()),
                    )]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use crate::git::Git;
    use std::fs;
    use std::path::PathBuf;

    fn ctx(root: &std::path::Path) -> Context {
        Context {
            root: root.to_path_buf(),
            decl: None,
            decl_error: None,
            git: Git::new(PathBuf::from(root)),
            level: 1,
            run: false,
            metrics: Default::default(),
        }
    }

    #[test]
    fn fails_without_readme() {
        let dir = tempfile::tempdir().unwrap();
        let findings = D1.run(&ctx(dir.path()));
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Fail));
    }

    #[test]
    fn passes_with_readme_link() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("README.md"), "See VIBECODING.md").unwrap();
        let findings = D1.run(&ctx(dir.path()));
        assert!(findings.is_empty());
    }
}
