use std::path::PathBuf;

use crate::checks::support;
use crate::checks::{Check, Context};
use crate::declaration::Declaration;
use crate::ecosystem;
use crate::evidence;
use crate::finding::Finding;
use crate::status;
use crate::util;

const MANIFESTS: &[&str] = &[
    "Cargo.toml",
    "package.json",
    "pyproject.toml",
    "go.mod",
    "pom.xml",
    "Gemfile",
    "composer.json",
];

pub struct T6;

impl Check for T6 {
    fn id(&self) -> &'static str {
        "T6"
    }

    fn level(&self) -> u8 {
        2
    }

    fn run(&self, ctx: &Context) -> Vec<Finding> {
        let Some(decl) = ctx.decl.as_ref() else {
            return support::missing_declaration(self.id());
        };

        let mut findings = Vec::new();
        let ecosystems = ecosystem::detect(&ctx.root);
        if ecosystems.is_empty() {
            findings.push(Finding::info(
                self.id(),
                "no ecosystem",
                "no ecosystem detected",
                None,
            ));
        }

        for ecosystem in ecosystems {
            let lockfiles = ecosystem::lockfiles(ecosystem);
            let has_lockfile = ecosystem::manifest_dirs(&ctx.root)
                .iter()
                .any(|dir| lockfiles.iter().any(|name| dir.join(name).is_file()));
            if !has_lockfile {
                findings.push(Finding::fail(
                    self.id(),
                    "no lockfile",
                    format!("no lockfile for {ecosystem}"),
                    None,
                    Some("commit a lockfile".to_string()),
                ));
            }
        }

        let audit = decl.convention.commands.audit.clone();
        findings.extend(support::declared_or_known_ci(
            ctx,
            self.id(),
            "audit",
            audit.clone(),
            ecosystem::known_vuln_tools(),
            "no dependency vulnerability check in CI or release checklist",
        ));
        if audit.is_none() {
            let release_docs = release_docs(&ctx.root);
            if let Some((path, tool)) = support::files_mentioning_any(
                &ctx.root,
                &release_docs,
                ecosystem::known_vuln_tools(),
            ) {
                findings.push(Finding::info(
                    self.id(),
                    "release vulnerability check",
                    format!(
                        "run at release using {tool} in {}",
                        support::relative_path(&ctx.root, &path)
                    ),
                    None,
                ));
            }
        }
        findings.extend(audit_evidence(ctx, decl));

        findings.extend(new_dependency_warnings(ctx));
        findings
    }
}

fn audit_evidence(ctx: &Context, decl: &Declaration) -> Vec<Finding> {
    let id = "T6";
    let Some(path) = decl
        .convention
        .evidence
        .as_ref()
        .and_then(|e| e.audit.as_deref())
    else {
        return vec![Finding::not_run(
            id,
            "audit evidence not declared",
            "audit evidence not declared",
            None,
        )];
    };
    let text = match support::read_evidence(ctx, id, path) {
        Ok(text) => text,
        Err(finding) => return vec![*finding],
    };
    let mut findings = vec![match evidence::vulnerability_count(&text) {
        Some(0) => Finding::info(
            id,
            "no vulnerabilities",
            format!("no vulnerabilities in {path}"),
            None,
        ),
        Some(count) => Finding::fail(
            id,
            "unresolved vulnerabilities",
            format!("{count} unresolved vulnerabilities in {path}"),
            Some(path.to_string()),
            Some("update or replace the affected dependencies".to_string()),
        ),
        None => Finding::warn(
            id,
            "vulnerability count unreadable",
            format!("could not read a vulnerability count from {path}"),
            Some(path.to_string()),
            Some(
                "write cargo-audit, npm audit or pip-audit output to the evidence file".to_string(),
            ),
        ),
    }];
    findings.extend(support::stale_evidence(ctx, id, path));
    findings
}

fn release_docs(root: &std::path::Path) -> Vec<PathBuf> {
    let mut docs = Vec::new();
    for name in ["RELEASE.md", "RELEASING.md"] {
        let path = root.join(name);
        if path.is_file() {
            docs.push(path);
        }
    }
    docs.extend(util::find_files_by_glob(root, "docs/release*.md"));
    docs
}

fn new_dependency_warnings(ctx: &Context) -> Vec<Finding> {
    let Some(decl) = ctx.decl.as_ref() else {
        return Vec::new();
    };

    let commits = if let Some((_, value)) = status::read_status_value(&ctx.root, "audit") {
        if let Some(date) = value
            .get("vibecoding")
            .and_then(|v| v.get("date"))
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        {
            ctx.git.commits_since_date(&date)
        } else if ctx.git.commit_exists(&decl.convention.adopted_at) {
            ctx.git.commits_since(&decl.convention.adopted_at)
        } else {
            Vec::new()
        }
    } else if ctx.git.commit_exists(&decl.convention.adopted_at) {
        ctx.git.commits_since(&decl.convention.adopted_at)
    } else {
        Vec::new()
    };

    let mut warnings = Vec::new();
    for commit in commits {
        let stat = ctx.git.show_stat(&commit.hash);
        if !MANIFESTS.iter().any(|manifest| stat.contains(manifest)) {
            continue;
        }
        let diff = ctx.git.show_diff(&commit.hash);
        if !diff_adds_dependency(&diff) {
            continue;
        }
        let message = format!("{} {}", commit.subject, commit.body);
        if !util::case_insensitive_contains(&message, "dependency")
            && !util::case_insensitive_contains(&message, "dep:")
            && !util::case_insensitive_contains(&message, "checked")
        {
            warnings.push(Finding::warn(
                "T6",
                "new dependency not recorded",
                "new dependency without a recorded registry check",
                Some(crate::finding::short_hash(&commit.hash).to_string()),
                Some("record the registry check in the commit message".to_string()),
            ));
        }
        if warnings.len() >= 10 {
            break;
        }
    }
    warnings
}

fn diff_adds_dependency(diff: &str) -> bool {
    let mut in_cargo_deps = false;
    for line in diff.lines() {
        if line.starts_with("+++") || line.starts_with("---") {
            continue;
        }
        let added = line.starts_with('+');
        let header = line
            .strip_prefix('+')
            .or_else(|| line.strip_prefix('-'))
            .or_else(|| line.strip_prefix(' '))
            .unwrap_or(line)
            .trim_start();
        if header.starts_with('[') {
            in_cargo_deps = header.starts_with("[dependencies]");
            continue;
        }
        if added {
            let content = line.trim_start_matches('+').trim();
            if in_cargo_deps && !content.is_empty() && !content.starts_with('[') {
                return true;
            }
            if content.contains("\"dependencies\"") || content.contains("require(") {
                return true;
            }
        }
    }
    false
}
