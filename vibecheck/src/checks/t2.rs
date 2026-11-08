use crate::checks::support;
use crate::checks::{Check, Context};
use crate::ci;
use crate::declaration::Declaration;
use crate::evidence;
use crate::finding::Finding;
use serde_json::json;

pub struct T2;

impl Check for T2 {
    fn id(&self) -> &'static str {
        "T2"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let Some(coverage) = decl.convention.coverage.as_ref() else {
            return vec![Finding::fail(
                self.id(),
                "coverage missing",
                "coverage is not declared",
                None,
                Some("declare coverage".to_string()),
            )];
        };

        let mut findings = Vec::new();
        if coverage.threshold == 0 {
            findings.push(Finding::fail(
                self.id(),
                "coverage threshold zero",
                "coverage threshold is 0",
                None,
                Some("set a non-zero coverage threshold".to_string()),
            ));
        }
        if coverage.threshold < 80 {
            findings.push(Finding::info(
                self.id(),
                "coverage below recommendation",
                "below the recommended 80 %; the declaration must explain it",
                None,
            ));
        }

        let mut in_ci = ci::ci_mentions(&ctx.root, &coverage.tool).is_some();
        if let Some(command) = &decl.convention.commands.coverage {
            if ci::ci_mentions(&ctx.root, command).is_some() {
                in_ci = true;
            }
        }
        if !in_ci {
            findings.push(Finding::warn(
                self.id(),
                "coverage not in CI",
                "coverage not run in CI",
                None,
                Some("run coverage in CI".to_string()),
            ));
        }

        findings.extend(coverage_evidence(ctx, decl, coverage.threshold));
        findings
    }
}

fn coverage_evidence(ctx: &Context, decl: &Declaration, threshold: u32) -> Vec<Finding> {
    let id = "T2";
    let Some(path) = decl
        .convention
        .evidence
        .as_ref()
        .and_then(|e| e.coverage.as_deref())
    else {
        return vec![Finding::not_run(
            id,
            "coverage evidence not declared",
            "coverage evidence not declared",
            None,
        )];
    };
    let text = match support::read_evidence(ctx, id, path) {
        Ok(text) => text,
        Err(finding) => return vec![*finding],
    };
    let Some(percent) = evidence::coverage_percent(&text) else {
        return vec![Finding::fail(
            id,
            "coverage figure missing",
            format!("no coverage figure in {path}"),
            Some(path.to_string()),
            Some("have CI write the coverage summary to the evidence file".to_string()),
        )];
    };
    ctx.metrics
        .borrow_mut()
        .insert("coverage".to_string(), json!(percent));
    let mut findings = vec![if percent < f64::from(threshold) {
        Finding::fail(
            id,
            "coverage below threshold",
            format!("coverage {percent:.2} % below threshold {threshold} % ({path})"),
            Some(path.to_string()),
            None,
        )
    } else {
        Finding::info(
            id,
            "coverage ok",
            format!("coverage {percent:.2} % ({path})"),
            None,
        )
    }];
    findings.extend(support::stale_evidence(ctx, id, path));
    findings
}
