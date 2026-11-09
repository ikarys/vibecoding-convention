use crate::checks::support;
use crate::checks::{Check, Context};
use crate::ci;
use crate::ecosystem;
use crate::finding::Finding;
use crate::util;

pub struct T7;

impl Check for T7 {
    fn id(&self) -> &'static str {
        "T7"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let mut findings = Vec::new();
        let secret_command = decl.convention.commands.secrets.clone();
        let secret_needles: Vec<String> = ecosystem::known_secret_scanners()
            .iter()
            .map(|s| s.to_string())
            .chain(secret_command.iter().cloned())
            .collect();
        let secret_needles_ref: Vec<&str> = secret_needles.iter().map(|s| s.as_str()).collect();

        let precommit_files = support::precommit_files(&ctx.root);
        let precommit_ok = precommit_files
            .iter()
            .filter_map(|path| util::read_file_if_exists(path))
            .any(|text| {
                secret_needles_ref
                    .iter()
                    .any(|needle| ci::whitespace_contains(&text, needle))
            });
        // A hook is local and optional; CI is where the outcome is enforced.
        if !precommit_ok {
            findings.push(Finding::info(
                self.id(),
                "no pre-commit secrets scanner",
                "no secrets scanner in a pre-commit hook (recommended)",
                None,
            ));
        }

        let ci_ok = ci::ci_mentions_any(&ctx.root, &secret_needles_ref).is_some();
        if !ci_ok {
            findings.push(Finding::fail(
                self.id(),
                "no CI secrets scanner",
                "no secrets scanner in CI",
                None,
                Some("add a secrets scanner to CI".to_string()),
            ));
        }

        let static_command = decl.convention.commands.static_analysis.clone();
        findings.extend(support::declared_or_known_ci(
            ctx,
            self.id(),
            "static_analysis",
            static_command.clone(),
            ecosystem::known_static_analysers(),
            "no static analysis in CI",
        ));

        let mut secret_hits = 0;
        for path in support::text_files(&ctx.root, 1024 * 1024) {
            if let Some(text) = util::read_file_if_exists(&path) {
                for (pattern, _matched) in util::scan_secrets(&text) {
                    if secret_hits >= 10 {
                        break;
                    }
                    findings.push(Finding::fail(
                        self.id(),
                        "possible secret",
                        format!("{pattern} in file"),
                        Some(support::relative_path(&ctx.root, &path)),
                        Some("remove the secret and rotate it".to_string()),
                    ));
                    secret_hits += 1;
                }
            }
            if secret_hits >= 10 {
                break;
            }
        }

        findings
    }
}
