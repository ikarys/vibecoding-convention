use crate::checks::Check;
use crate::finding::Finding;
use crate::util;

pub struct D9;

impl Check for D9 {
    fn id(&self) -> &'static str {
        "D9"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let Some(decl) = &ctx.decl else {
            return vec![Finding::not_run(
                "D9",
                "Status",
                "VIBECODING.md is missing",
                None,
            )];
        };

        let mut findings = Vec::new();
        let status = decl.convention.status.to_lowercase();
        if !["active", "maintenance", "archived"].contains(&status.as_str()) {
            findings.push(Finding::fail(
                "D9",
                "Status",
                format!(
                    "convention.status must be active, maintenance, or archived; found {}",
                    decl.convention.status
                ),
                Some("VIBECODING.md".into()),
                Some("Set convention.status".into()),
            ));
        }

        let security = decl.convention.security_response.trim();
        if security.is_empty() {
            findings.push(Finding::fail(
                "D9",
                "Status",
                "convention.security_response is not set",
                Some("VIBECODING.md".into()),
                Some("Set convention.security_response".into()),
            ));
        } else if !security.eq_ignore_ascii_case("none") {
            let security_file = util::find_file_case_insensitive(&ctx.root, "SECURITY.md");
            match security_file {
                None => findings.push(Finding::fail(
                    "D9",
                    "Status",
                    "SECURITY.md not found",
                    Some("SECURITY.md".into()),
                    Some("Create SECURITY.md".into()),
                )),
                Some(path) => {
                    let text = util::read_file_if_exists(&path).unwrap_or_default();
                    if !util::case_insensitive_contains(&text, security) {
                        findings.push(Finding::fail(
                            "D9",
                            "Status",
                            format!(
                                "SECURITY.md does not contain security_response value: {security}"
                            ),
                            Some(path.display().to_string()),
                            Some("Document the same security response in SECURITY.md".into()),
                        ));
                    }
                }
            }
        }

        findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use crate::declaration::Declaration;
    use crate::git::Git;
    use std::path::PathBuf;

    #[test]
    fn fails_on_bad_status() {
        let dir = tempfile::tempdir().unwrap();
        let mut decl = Declaration::default();
        decl.convention.status = "weird".into();
        decl.convention.security_response = "none".into();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: Some(decl),
            decl_error: None,
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
            run: false,
            metrics: Default::default(),
        };
        let findings = D9.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Fail));
    }

    #[test]
    fn passes_with_none_security() {
        let dir = tempfile::tempdir().unwrap();
        let mut decl = Declaration::default();
        decl.convention.status = "active".into();
        decl.convention.security_response = "none".into();
        let ctx = Context {
            root: dir.path().to_path_buf(),
            decl: Some(decl),
            decl_error: None,
            git: Git::new(PathBuf::from(dir.path())),
            level: 1,
            run: false,
            metrics: Default::default(),
        };
        let findings = D9.run(&ctx);
        assert!(findings.is_empty());
    }
}
