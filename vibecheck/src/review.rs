//! Pull-request mode: only what HEAD adds beyond a base revision is read.
//! The full-repository checks do not run here; the five range checks reuse
//! the same helpers so a finding means the same thing in both modes.
use std::collections::BTreeMap;
use std::path::Path;

use crate::checks::{d3, d6, t6, t8};
use crate::declaration::{self, Convention};
use crate::finding::{requirement_rank, Finding, Report};
use crate::git::{Commit, Git};
use crate::util;

/// A hostile pull request can carry a huge diff; past this size the rest is not read.
const MAX_DIFF_BYTES: usize = 16 * 1024 * 1024;
/// Same cap per check as the full run, so one bad branch cannot flood the output.
const MAX_PER_CHECK: usize = 10;

/// Review the commits `base..HEAD` and the diff `base...HEAD` of the repository at `root`.
///
/// Returns an error when `base` is not a plausible revision (empty, option-like,
/// whitespace, a range). A base that does not resolve yields a single NOT RUN finding.
pub fn run_review(root: &Path, base: &str) -> anyhow::Result<Report> {
    if !valid_base(base) {
        anyhow::bail!("invalid base revision");
    }
    let (decl, _) = declaration::read_declaration(root);
    let git = Git::new(root.to_path_buf());
    let level = decl.as_ref().map(|d| d.convention.level).unwrap_or(1);
    let date = chrono::Utc::now().naive_local().date();
    let report = |findings| Report {
        level,
        findings,
        checked_commit: git.head(),
        date,
        metrics: BTreeMap::new(),
        evidence: BTreeMap::new(),
    };

    if !git.commit_exists(base) {
        return Ok(report(vec![Finding::not_run(
            "PR",
            "base not found",
            format!("base revision {base} not found; fetch it first (fetch-depth: 0)"),
            None,
        )]));
    }

    let commits = git.commits_in_range(base);
    let mut diff = git.range_diff(base);
    let mut findings = Vec::new();
    if diff.len() > MAX_DIFF_BYTES {
        let cut = (0..=MAX_DIFF_BYTES)
            .rev()
            .find(|&i| diff.is_char_boundary(i))
            .unwrap_or(0);
        diff.truncate(cut);
        findings.push(Finding::warn(
            "PR",
            "diff truncated",
            "diff larger than 16 MiB; only the first 16 MiB were reviewed",
            None,
            None,
        ));
    }

    let tools = decl
        .as_ref()
        .map(|d| d.convention.tools.clone())
        .unwrap_or_default();
    let accepted = decl
        .as_ref()
        .map(|d| d.convention.attribution_trailers())
        .unwrap_or_else(|| Convention::default().attribution_trailers());

    let mut d3_findings = Vec::new();
    let mut d6_findings = Vec::new();
    let mut t6_findings = Vec::new();
    for commit in &commits {
        let short: String = commit.hash.chars().take(7).collect();
        if let Some(value) = d3::offending_reason(commit, &tools) {
            d3_findings.push(Finding::fail(
                "D3",
                "Signed-off-by names a model",
                format!("Signed-off-by names a model ({value})"),
                Some(short.clone()),
                Some(d3::SIGNED_OFF_FIX.to_string()),
            ));
        } else if let Some(value) = d3::undeclared_tool(commit, &tools, &accepted) {
            d3_findings.push(Finding::fail(
                "D3",
                "undeclared tool or model",
                format!("attribution names an undeclared tool or model ({value})"),
                Some(short.clone()),
                Some(d3::UNDECLARED_TOOL_FIX.to_string()),
            ));
        } else if !has_accepted_trailer(commit, &accepted) {
            d3_findings.push(Finding::warn(
                "D3",
                "no attribution trailer",
                format!(
                    "commit {short} carries no attribution trailer (fine only if no AI wrote it)"
                ),
                Some(short.clone()),
                Some(
                    "add the project's trailer, for example Assisted-by: <Tool>:<model>"
                        .to_string(),
                ),
            ));
        }

        let stat = git.show_stat(&commit.hash);
        let commit_diff = git.show_diff(&commit.hash);
        if d6::is_test_change(&stat, &commit_diff)
            && !d6::has_explanation(&commit.subject, &commit.body)
        {
            d6_findings.push(Finding::warn(
                "D6",
                "test change without a reason",
                format!("commit {short} removes, skips or weakens a test without saying why"),
                Some(short.clone()),
                Some("state the reason in the commit message".to_string()),
            ));
        }

        let names_manifest = t6::MANIFESTS.iter().any(|m| stat.contains(m));
        if names_manifest
            && t6::diff_adds_dependency(&commit_diff)
            && !registry_check_recorded(commit)
        {
            t6_findings.push(Finding::warn(
                "T6",
                "new dependency not recorded",
                format!("commit {short} adds a dependency without a recorded registry check"),
                Some(short),
                Some(
                    "record the registry check (name, version, why this package) in the commit message"
                        .to_string(),
                ),
            ));
        }
    }

    let added = added_lines(&diff);
    let t7_findings = added.iter().flat_map(|(path, content)| {
        util::scan_secrets(content)
            .into_iter()
            .map(move |(pattern, _)| {
                Finding::fail(
                    "T7",
                    "secret in the diff",
                    format!("possible secret added ({pattern})"),
                    Some(path.clone()),
                    Some("remove it and rotate the credential".to_string()),
                )
            })
    });
    let t8_findings = added
        .iter()
        .filter(|(path, content)| is_source(path) && t8::has_marker_outside_string(content))
        .map(|(path, _)| {
            Finding::warn(
                "T8",
                "leftover marker",
                format!("marker added in {path}"),
                Some(path.clone()),
                Some("finish it or remove it before merge".to_string()),
            )
        });

    findings.extend(d3_findings.into_iter().take(MAX_PER_CHECK));
    findings.extend(d6_findings.into_iter().take(MAX_PER_CHECK));
    findings.extend(t6_findings.into_iter().take(MAX_PER_CHECK));
    findings.extend(t7_findings.take(MAX_PER_CHECK));
    findings.extend(t8_findings.take(MAX_PER_CHECK));

    if findings.is_empty() {
        findings.push(Finding::info(
            "PR",
            "clean",
            format!("{} commits reviewed, nothing to report", commits.len()),
            None,
        ));
    }
    findings.sort_by(|a, b| {
        requirement_rank(&a.id)
            .cmp(&requirement_rank(&b.id))
            .then_with(|| a.severity.cmp(&b.severity))
            .then_with(|| a.problem.cmp(&b.problem))
    });
    Ok(report(findings))
}

/// A leading `-` would be read by git as an option, `..` as a range.
fn valid_base(base: &str) -> bool {
    !base.is_empty()
        && !base.starts_with('-')
        && !base.contains(char::is_whitespace)
        && !base.contains("..")
}

fn has_accepted_trailer(commit: &Commit, accepted: &[String]) -> bool {
    commit
        .trailers
        .iter()
        .any(|(key, _)| accepted.contains(&key.to_lowercase()))
}

fn registry_check_recorded(commit: &Commit) -> bool {
    let message = format!("{}\n{}", commit.subject, commit.body);
    ["dependency", "dep:", "checked"]
        .iter()
        .any(|needle| util::case_insensitive_contains(&message, needle))
}

fn is_source(path: &str) -> bool {
    path.rsplit_once('.')
        .is_some_and(|(_, ext)| t8::SOURCE_EXTENSIONS.contains(&ext))
}

/// Added lines of a unified diff with the path of the file they belong to.
fn added_lines(diff: &str) -> Vec<(String, String)> {
    let mut path = None;
    let mut lines = Vec::new();
    for line in diff.lines() {
        if let Some(p) = line.strip_prefix("+++ b/") {
            path = Some(p.to_string());
        } else if line.starts_with("+++ ") {
            path = None;
        } else if let Some(content) = line.strip_prefix('+') {
            if let Some(p) = &path {
                lines.push((p.clone(), content.to_string()));
            }
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_validation_rejects_hostile_revisions() {
        assert!(valid_base("origin/main"));
        assert!(valid_base("abc1234"));
        assert!(!valid_base(""));
        assert!(!valid_base("-x"));
        assert!(!valid_base("a b"));
        assert!(!valid_base("a..b"));
    }

    #[test]
    fn added_lines_tracks_the_current_path_and_skips_removals() {
        let diff = "--- a/src/a.rs\n+++ b/src/a.rs\n@@ -1 +1 @@\n-let old = 1;\n+let new = 2;\n--- a/src/b.rs\n+++ /dev/null\n@@ -1 +0,0 @@\n-gone\n--- /dev/null\n+++ b/src/c.rs\n@@ -0,0 +1 @@\n+let y = 2;\n";
        assert_eq!(
            added_lines(diff),
            vec![
                ("src/a.rs".to_string(), "let new = 2;".to_string()),
                ("src/c.rs".to_string(), "let y = 2;".to_string())
            ]
        );
    }

    #[test]
    fn source_detection_uses_the_extension() {
        assert!(is_source("src/k.rs"));
        assert!(!is_source("README.md"));
        assert!(!is_source("Makefile"));
    }
}
