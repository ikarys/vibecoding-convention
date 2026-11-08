use crate::checks::Check;
use crate::finding::Finding;
use crate::git::Commit;

pub struct D3;

impl Check for D3 {
    fn id(&self) -> &'static str {
        "D3"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let Some(decl) = &ctx.decl else {
            return vec![Finding::not_run(
                "D3",
                "Commit attribution",
                "VIBECODING.md is missing; cannot determine adoption",
                None,
            )];
        };

        if !ctx.git.is_repo() {
            return vec![Finding::not_run(
                "D3",
                "Commit attribution",
                "not a git repository",
                None,
            )];
        }

        let commits = ctx.git.commits_since(&decl.convention.adopted_at);
        let total = commits.len();
        let tools = &decl.convention.tools;
        let accepted = decl.convention.attribution_trailers();
        let assisted = commits
            .iter()
            .filter(|commit| {
                commit
                    .trailers
                    .iter()
                    .any(|(key, _)| accepted.contains(&key.to_lowercase()))
            })
            .count();
        let percent = (assisted * 100).checked_div(total).unwrap_or(0);

        let mut findings = vec![Finding::info(
            "D3",
            "Commit attribution",
            format!(
                "{assisted} of {total} commits since adoption carry an AI attribution trailer ({percent} %)"
            ),
            None,
        )];

        let mut signed_off = Vec::new();
        let mut undeclared = Vec::new();
        for commit in &commits {
            if let Some(offence) = offending_reason(commit, tools) {
                signed_off.push((commit, offence));
            }
            if let Some(offence) = undeclared_tool(commit, tools, &accepted) {
                undeclared.push((commit, offence));
            }
        }

        push_offenders(
            &mut findings,
            &signed_off,
            |value| format!("Signed-off-by names a model: {value}"),
            SIGNED_OFF_FIX,
        );
        push_offenders(
            &mut findings,
            &undeclared,
            |value| format!("attribution names an undeclared tool: {value}"),
            UNDECLARED_TOOL_FIX,
        );

        findings
    }
}

const SIGNED_OFF_FIX: &str = "Only a human signs off; remove the model from Signed-off-by";
const UNDECLARED_TOOL_FIX: &str = "Declare the tool and model in convention.tools";

/// A `Signed-off-by` trailer naming a model: it certifies origin, which only a
/// human can do.
fn offending_reason(commit: &Commit, tools: &[String]) -> Option<String> {
    commit.trailers.iter().find_map(|(key, value)| {
        (key.eq_ignore_ascii_case("Signed-off-by")
            && (matches_tool(value, tools) || matches_noreply(value)))
        .then(|| value.clone())
    })
}

/// Reports an attribution trailer whose model is not declared, since the
/// declaration (D2) must match the history. Stricter than `matches_tool`,
/// which accepts the tool name alone because a Signed-off-by may omit the model.
fn undeclared_tool(commit: &Commit, tools: &[String], accepted: &[String]) -> Option<String> {
    commit.trailers.iter().find_map(|(key, value)| {
        let key = key.to_lowercase();
        // `co-authored-by` may name humans, so it cannot be judged.
        (accepted.contains(&key) && key != "co-authored-by" && !declares(value, tools))
            .then(|| value.clone())
    })
}

fn push_offenders(
    findings: &mut Vec<Finding>,
    offenders: &[(&Commit, String)],
    message: impl Fn(&str) -> String,
    fix: &str,
) {
    for (commit, value) in offenders.iter().take(10) {
        findings.push(Finding::fail(
            "D3",
            "Commit attribution",
            message(value),
            Some(commit.hash.clone()),
            Some(fix.into()),
        ));
    }
    if offenders.len() > 10 {
        findings.push(Finding::fail(
            "D3",
            "Commit attribution",
            format!("and {} more commits", offenders.len() - 10),
            None,
            Some(fix.into()),
        ));
    }
}

fn declares(value: &str, tools: &[String]) -> bool {
    let value = value.to_lowercase();
    tools.iter().any(|tool| {
        let (name, model) = tool_parts(tool);
        value.contains(&model.unwrap_or(name))
    })
}

fn matches_tool(value: &str, tools: &[String]) -> bool {
    let value = value.to_lowercase();
    tools.iter().any(|tool| {
        let (name, model) = tool_parts(tool);
        value.contains(&name) || model.map(|model| value.contains(&model)).unwrap_or(false)
    })
}

fn matches_noreply(value: &str) -> bool {
    let value = value.to_lowercase();
    value.contains("noreply@anthropic.com")
        || value.contains("noreply@openai.com")
        || value.contains("noreply@google.com")
        || (value.contains("users.noreply.github.com") && value.contains("copilot"))
}

fn tool_parts(tool: &str) -> (String, Option<String>) {
    match tool.split_once(':') {
        Some((name, model)) => (name.to_lowercase(), Some(model.to_lowercase())),
        None => (tool.to_lowercase(), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn co_authored_by_is_not_an_offence() {
        let commit = Commit {
            hash: "abc".into(),
            subject: "s".into(),
            body: String::new(),
            trailers: vec![(
                "Co-Authored-By".into(),
                "Claude <noreply@anthropic.com>".into(),
            )],
        };
        assert!(offending_reason(&commit, &[]).is_none());
    }

    #[test]
    fn detects_tool_signed_off_by() {
        let commit = Commit {
            hash: "abc".into(),
            subject: "s".into(),
            body: String::new(),
            trailers: vec![(
                "Signed-off-by".into(),
                "Claude Code:claude-fable-5-1".into(),
            )],
        };
        assert!(offending_reason(&commit, &["Claude Code:claude-fable-5-1".into()]).is_some());
    }

    #[test]
    fn assisted_by_undeclared_tool_is_reported() {
        let commit = Commit {
            hash: "abc".into(),
            subject: "s".into(),
            body: String::new(),
            trailers: vec![("Assisted-by".into(), "ninfer:qwen3.8-27b".into())],
        };
        let tools = vec!["Claude Code:claude-opus-5-5".into()];
        let accepted = vec!["assisted-by".into()];
        assert_eq!(
            undeclared_tool(&commit, &tools, &accepted),
            Some("ninfer:qwen3.8-27b".into())
        );
    }

    #[test]
    fn assisted_by_declared_tool_passes() {
        let commit = Commit {
            hash: "abc".into(),
            subject: "s".into(),
            body: String::new(),
            trailers: vec![("Assisted-by".into(), "ninfer:qwen3.8-27b".into())],
        };
        let tools = vec!["ninfer:qwen3.8-27b".into()];
        let accepted = vec!["assisted-by".into()];
        assert!(undeclared_tool(&commit, &tools, &accepted).is_none());
    }

    #[test]
    fn same_tool_with_undeclared_model_is_reported() {
        let commit = Commit {
            hash: "abc".into(),
            subject: "s".into(),
            body: String::new(),
            trailers: vec![("Assisted-by".into(), "Claude Code:claude-fable-5-1".into())],
        };
        let tools = vec!["Claude Code:claude-opus-5-5".into()];
        let accepted = vec!["assisted-by".into()];
        assert!(undeclared_tool(&commit, &tools, &accepted).is_some());
    }

    #[test]
    fn co_authored_by_is_never_judged() {
        let commit = Commit {
            hash: "abc".into(),
            subject: "s".into(),
            body: String::new(),
            trailers: vec![(
                "Co-Authored-By".into(),
                "Jane Doe <jane@example.com>".into(),
            )],
        };
        let tools = vec!["Claude Code:claude-opus-5-5".into()];
        let accepted = vec!["co-authored-by".into()];
        assert!(undeclared_tool(&commit, &tools, &accepted).is_none());
    }

    #[test]
    fn undeclared_trailer_key_is_ignored() {
        let commit = Commit {
            hash: "abc".into(),
            subject: "s".into(),
            body: String::new(),
            trailers: vec![("AI-Tool".into(), "other:model".into())],
        };
        let tools = Vec::new();
        let accepted = vec!["assisted-by".into()];
        assert!(undeclared_tool(&commit, &tools, &accepted).is_none());
    }
}
