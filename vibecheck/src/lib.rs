pub mod checks;
pub mod ci;
pub mod declaration;
pub mod ecosystem;
pub mod evidence;
pub mod finding;
pub mod git;
pub mod status;
pub mod util;

use std::path::Path;

pub use finding::Report;

/// Settings that change what is checked, not how: vibecheck runs no project code.
#[derive(Default, Clone, Copy)]
pub struct Options {
    pub level_override: Option<u8>,
}

pub fn run(root: &Path, opts: Options) -> anyhow::Result<Report> {
    let root = root.to_path_buf();
    let (decl, decl_error) = declaration::read_declaration(&root);
    let level = opts
        .level_override
        .or_else(|| decl.as_ref().map(|d| d.convention.level))
        .unwrap_or(1);
    let git = git::Git::new(root.clone());
    let checked_commit = git.head();
    let metrics = std::cell::RefCell::new(std::collections::BTreeMap::new());
    let ctx = checks::Context {
        root,
        decl,
        decl_error,
        git,
        level,
        metrics,
    };

    let mut findings = checks::run(&ctx);
    findings.sort_by(|a, b| {
        finding::requirement_rank(&a.id)
            .cmp(&finding::requirement_rank(&b.id))
            .then_with(|| a.severity.cmp(&b.severity))
            .then_with(|| a.problem.cmp(&b.problem))
    });
    let date = chrono::Utc::now().naive_local().date();
    let metrics = ctx.metrics.into_inner();
    let evidence = ctx
        .decl
        .as_ref()
        .map(declaration::declared_evidence)
        .unwrap_or_default();
    Ok(Report {
        level,
        findings,
        checked_commit,
        date,
        metrics,
        evidence,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_reports_missing_declaration() {
        let dir = tempfile::tempdir().unwrap();
        let report = run(dir.path(), Options::default()).unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.id == "D2" && f.severity == finding::Severity::Fail));
    }
}
