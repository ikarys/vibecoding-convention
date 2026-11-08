use crate::checks::Check;
use crate::finding::Finding;
use crate::util;

pub struct D7;

impl Check for D7 {
    fn id(&self) -> &'static str {
        "D7"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let Some(decl) = &ctx.decl else {
            return vec![Finding::not_run(
                "D7",
                "Provenance",
                "VIBECODING.md is missing",
                None,
            )];
        };

        let mut findings = Vec::new();

        if decl.convention.licence.trim().is_empty() {
            findings.push(Finding::fail(
                "D7",
                "Provenance",
                "convention.licence is not set",
                Some("VIBECODING.md".into()),
                Some("Set convention.licence".into()),
            ));
        }

        let has_licence_file = !util::find_files_by_glob(&ctx.root, "LICENSE*").is_empty()
            || !util::find_files_by_glob(&ctx.root, "COPYING*").is_empty();
        if !has_licence_file {
            findings.push(Finding::fail(
                "D7",
                "Provenance",
                "no LICENSE* or COPYING* file found",
                Some(ctx.root.display().to_string()),
                Some("Add a LICENSE or COPYING file".into()),
            ));
        }

        let body = &decl.body;
        let has_heading = body.lines().any(|line| {
            line.trim_start().starts_with('#')
                && util::case_insensitive_contains(line, "Provenance")
        });
        let mentions_copyright = util::case_insensitive_contains(body, "copyright");
        let mentions_safeguard = util::case_insensitive_contains(body, "safeguard");

        if !has_heading || !mentions_copyright || !mentions_safeguard {
            findings.push(Finding::fail(
                "D7",
                "Provenance",
                "VIBECODING.md body must have a Provenance heading and mention copyright and safeguard",
                Some("VIBECODING.md".into()),
                Some("Add a Provenance section explaining copyright and safeguards".into()),
            ));
        }

        findings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use crate::declaration::Declaration;
    use std::fs;

    #[test]
    fn fails_without_licence_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut decl = Declaration::default();
        decl.convention.licence = "MIT".into();
        let ctx = Context::for_test(dir.path(), Some(decl));
        let findings = D7.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Fail));
    }

    #[test]
    fn passes_with_licence_file_and_body() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("LICENSE"), "MIT").unwrap();
        let mut decl = Declaration::default();
        decl.convention.licence = "MIT".into();
        decl.body = "## Provenance\ncopyright and safeguard".into();
        let ctx = Context::for_test(dir.path(), Some(decl));
        let findings = D7.run(&ctx);
        assert!(findings.is_empty());
    }
}
