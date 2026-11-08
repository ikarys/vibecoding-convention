use crate::checks::support;
use crate::checks::{Check, Context};
use crate::ecosystem;
use crate::finding::Finding;

pub struct T5;

impl Check for T5 {
    fn id(&self) -> &'static str {
        "T5"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let mut findings = Vec::new();
        let lint = decl.convention.commands.lint.clone();
        let format = decl.convention.commands.format.clone();

        findings.extend(support::declared_or_known_ci(
            ctx,
            self.id(),
            "lint",
            lint.clone(),
            ecosystem::known_lint_tools(),
            "no linter in CI",
        ));
        findings.extend(support::declared_or_known_ci(
            ctx,
            self.id(),
            "format",
            format.clone(),
            ecosystem::known_lint_tools(),
            "no linter in CI",
        ));

        findings
    }
}
