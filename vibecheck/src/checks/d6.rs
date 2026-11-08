use chrono::NaiveDate;

use crate::checks::Check;
use crate::finding::Finding;
use crate::status;

pub struct D6;

impl Check for D6 {
    fn id(&self) -> &'static str {
        "D6"
    }

    fn level(&self) -> u8 {
        1
    }

    fn run(&self, ctx: &crate::checks::Context) -> Vec<Finding> {
        let Some(decl) = &ctx.decl else {
            return vec![Finding::not_run(
                "D6",
                "Test changes explained",
                "VIBECODING.md is missing",
                None,
            )];
        };

        if !ctx.git.is_repo() {
            return vec![Finding::not_run(
                "D6",
                "Test changes explained",
                "not a git repository",
                None,
            )];
        }

        let audit_dir = if decl.convention.audit_dir.is_empty() {
            "audit"
        } else {
            &decl.convention.audit_dir
        };
        let status_date = status::read_status_value(&ctx.root, audit_dir).and_then(|(_, value)| {
            value
                .get("vibecoding")
                .and_then(|v| v.get("date"))
                .and_then(|v| v.as_str())
                .and_then(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d").ok())
        });

        let commits = if let Some(date) = status_date {
            ctx.git.commits_since_date(&date)
        } else if decl.convention.adopted_at.is_empty() {
            return vec![Finding::not_run(
                "D6",
                "Test changes explained",
                "adopted_at is missing",
                None,
            )];
        } else {
            ctx.git.commits_since(&decl.convention.adopted_at)
        };

        let mut offending = Vec::new();
        for commit in &commits {
            let stat = ctx.git.show_stat(&commit.hash);
            let diff = ctx.git.show_diff(&commit.hash);
            if is_test_change(&stat, &diff) && !has_explanation(&commit.subject, &commit.body) {
                offending.push(commit);
            }
        }

        let mut findings = Vec::new();
        for commit in offending.iter().take(10) {
            findings.push(Finding::warn(
                "D6",
                "Test changes explained",
                "test change without explanation",
                Some(commit.hash.clone()),
                Some("Explain why tests changed in the commit subject or body".into()),
            ));
        }
        if offending.len() > 10 {
            findings.push(Finding::warn(
                "D6",
                "Test changes explained",
                format!("and {} more commits", offending.len() - 10),
                None,
                Some("Explain why tests changed in the commit subject or body".into()),
            ));
        }

        findings
    }
}

fn is_test_change(stat: &str, diff: &str) -> bool {
    for line in stat.lines() {
        let line = line.trim();
        if let Some(pipe) = line.find('|') {
            let path = line[..pipe].trim();
            let rest = line[pipe + 1..].trim();
            if let Some(bars) = rest.split_whitespace().last() {
                let lower = path.to_lowercase();
                if !path.is_empty()
                    && bars.contains('-')
                    && !bars.contains('+')
                    && (lower.contains("test") || lower.contains("spec"))
                {
                    return true;
                }
            }
        }
    }

    for line in diff.lines() {
        if line.starts_with("---") {
            continue;
        }
        if let Some(content) = line.strip_prefix('-') {
            if contains_any(
                content,
                &[
                    "#[test]",
                    "fn test_",
                    "it(",
                    "test(",
                    "describe(",
                    "def test_",
                    "assert",
                ],
            ) {
                return true;
            }
        }
        if let Some(content) = line.strip_prefix('+') {
            if contains_any(
                content,
                &["#[ignore]", ".skip(", "@skip", "xit(", "xdescribe("],
            ) {
                return true;
            }
        }
    }

    false
}

fn has_explanation(subject: &str, body: &str) -> bool {
    let text = format!("{subject}\n{body}").to_lowercase();
    [
        "why",
        "because",
        "reason",
        "replaced",
        "moved",
        "obsolete",
        "flaky",
        "duplicate",
        "renamed",
        "refactor",
    ]
    .iter()
    .any(|word| text.contains(word))
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_removed_test_line() {
        assert!(is_test_change("", "-fn test_old() {}"));
    }

    #[test]
    fn detects_added_skip_line() {
        assert!(is_test_change("", "+it.skip('x')"));
    }

    #[test]
    fn explanation_words_work() {
        assert!(has_explanation("fix flaky test", ""));
        assert!(!has_explanation("update", ""));
    }
}
