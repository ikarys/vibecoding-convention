use crate::checks::support;
use crate::checks::{Check, Context};
use crate::ci;
use crate::ecosystem;
use crate::exec;
use crate::finding::Finding;

pub struct T1;

impl Check for T1 {
    fn id(&self) -> &'static str {
        "T1"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let mut findings = Vec::new();
        let declared = decl.convention.commands.test.clone();
        let command = match declared.clone() {
            Some(command) => command,
            None => {
                let default = ecosystem::detect(&ctx.root)
                    .into_iter()
                    .find_map(|e| ecosystem::default_test_command(e))
                    .map(|c| c.to_string());
                match default {
                    Some(command) => {
                        findings.push(Finding::warn(
                            self.id(),
                            "test command assumed",
                            format!("commands.test not declared; assuming `{command}`"),
                            None,
                            Some("declare commands.test".to_string()),
                        ));
                        command
                    }
                    None => {
                        findings.push(Finding::fail(
                            self.id(),
                            "no test command",
                            "no test command declared and no ecosystem default",
                            None,
                            Some("declare commands.test".to_string()),
                        ));
                        return findings;
                    }
                }
            }
        };

        if ci::ci_files(&ctx.root).is_empty() {
            findings.push(support::no_ci(self.id()));
        } else if ci::ci_mentions(&ctx.root, &command).is_none() {
            findings.push(support::command_not_in_ci(self.id(), &command));
        }

        if ctx.run {
            match exec::run_command(&ctx.root, &command) {
                Ok(outcome) => {
                    if !outcome.success {
                        let tail = support::last_lines(&outcome.output, 5);
                        let problem = if tail.is_empty() {
                            format!("test command failed: {command}")
                        } else {
                            format!("test command failed: {command}\n{tail}")
                        };
                        findings.push(Finding::fail(
                            self.id(),
                            "test command failed",
                            problem,
                            None,
                            Some("make the test command pass".to_string()),
                        ));
                    }
                }
                Err(err) => findings.push(Finding::fail(
                    self.id(),
                    "test command failed",
                    format!("failed to run `{command}`: {err}"),
                    None,
                    None,
                )),
            }
        }

        findings
    }
}
