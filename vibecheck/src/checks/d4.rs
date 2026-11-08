use crate::checks::Check;
use crate::finding::Finding;

pub struct D4;

impl Check for D4 {
    fn id(&self) -> &'static str {
        "D4"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let Some(decl) = &ctx.decl else {
            return vec![Finding::not_run(
                "D4",
                "Human review",
                "VIBECODING.md is missing",
                None,
            )];
        };

        if decl.convention.review.trim().is_empty() {
            vec![Finding::fail(
                "D4",
                "Human review",
                "convention.review is empty",
                Some("VIBECODING.md".into()),
                Some("Describe how human review happens".into()),
            )]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Context;
    use crate::declaration::Declaration;

    #[test]
    fn fails_on_empty_review() {
        let ctx = Context::for_test(std::path::Path::new("."), Some(Declaration::default()));
        let findings = D4.run(&ctx);
        assert!(findings
            .iter()
            .any(|f| f.severity == crate::finding::Severity::Fail));
    }
}
