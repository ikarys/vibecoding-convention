//! A7: the declared mutation evidence must contain a score above threshold.

use crate::checks::support;
use crate::checks::{Check, Context};
use crate::evidence;
use crate::finding::Finding;
use serde_json::json;

pub struct A7;

impl Check for A7 {
    fn id(&self) -> &'static str {
        "A7"
    }

    fn level(&self) -> u8 {
        3
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration("A7");
        };
        let Some(mutation) = decl.convention.mutation.as_ref() else {
            return vec![Finding::info(
                "A7",
                "Mutation score",
                "mutation score not measured (recommended)",
                None,
            )];
        };
        if mutation.threshold == 0 {
            return vec![Finding::fail(
                "A7",
                "Mutation score",
                "mutation threshold is 0",
                None,
                None,
            )];
        }
        let Some(path) = decl
            .convention
            .evidence
            .as_ref()
            .and_then(|evidence| evidence.mutation.as_ref())
        else {
            return vec![Finding::not_run(
                "A7",
                "Mutation score",
                "mutation evidence not declared",
                None,
            )];
        };

        let text = match support::read_evidence(ctx, "A7", path) {
            Ok(text) => text,
            Err(finding) => return vec![*finding],
        };
        let Some(score) = evidence::mutation_score(&text) else {
            return vec![Finding::fail(
                "A7",
                "Mutation score",
                format!("no mutation score in {path}"),
                Some(path.to_string()),
                Some("have CI write the tool's summary or JSON to the evidence file".to_string()),
            )];
        };

        ctx.metrics
            .borrow_mut()
            .insert("mutation".to_string(), json!(score));

        let mut findings = if score < mutation.threshold as f64 {
            vec![Finding::fail(
                "A7",
                "Mutation score",
                format!(
                    "mutation score {score:.2} % below threshold {} % ({path})",
                    mutation.threshold
                ),
                Some(path.to_string()),
                None,
            )]
        } else {
            vec![Finding::info(
                "A7",
                "Mutation score",
                format!("mutation score {score:.2} % ({path})"),
                None,
            )]
        };
        findings.extend(support::stale_evidence(ctx, "A7", path));
        findings
    }
}
