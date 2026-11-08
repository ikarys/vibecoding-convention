use crate::checks::support;
use crate::checks::{Check, Context};
use crate::ci;
use crate::exec;
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

        if ctx.run {
            if let Some(command) = &decl.convention.commands.coverage {
                match exec::run_command(&ctx.root, command) {
                    Ok(outcome) => {
                        if !outcome.success {
                            let tail = support::last_lines(&outcome.output, 5);
                            findings.push(Finding::fail(
                                self.id(),
                                "coverage command failed",
                                format!("coverage command failed: {command}\n{tail}"),
                                None,
                                None,
                            ));
                        }
                        if let Some(percent) = exec::last_percent(&outcome.output) {
                            ctx.metrics
                                .borrow_mut()
                                .insert("coverage".to_string(), json!(percent));
                            if percent < f64::from(coverage.threshold) {
                                findings.push(Finding::fail(
                                    self.id(),
                                    "coverage below threshold",
                                    format!(
                                        "coverage {percent:.2} % below threshold {} %",
                                        coverage.threshold
                                    ),
                                    None,
                                    None,
                                ));
                            } else {
                                findings.push(Finding::info(
                                    self.id(),
                                    "coverage ok",
                                    format!("coverage {percent:.2} %"),
                                    None,
                                ));
                            }
                        } else {
                            findings.push(Finding::warn(
                                self.id(),
                                "coverage percentage unreadable",
                                "could not read a percentage from the output",
                                None,
                                None,
                            ));
                        }
                    }
                    Err(err) => findings.push(Finding::fail(
                        self.id(),
                        "coverage command failed",
                        format!("failed to run `{command}`: {err}"),
                        None,
                        None,
                    )),
                }
            } else {
                findings.push(Finding::not_run(
                    self.id(),
                    "coverage command not declared",
                    "commands.coverage not declared",
                    None,
                ));
            }
        }

        findings
    }
}
