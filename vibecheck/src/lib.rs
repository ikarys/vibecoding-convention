pub mod checks;
pub mod declaration;
pub mod finding;
pub mod git;
pub mod status;
pub mod util;

use std::path::Path;

pub use finding::Report;

pub fn run(root: &Path, level_override: Option<u8>) -> anyhow::Result<Report> {
    let root = root.to_path_buf();
    let (decl, decl_error) = declaration::read_declaration(&root);
    let level = level_override
        .or_else(|| decl.as_ref().map(|d| d.convention.level))
        .unwrap_or(1);
    let git = git::Git::new(root.clone());
    let checked_commit = git.head();
    let ctx = checks::Context {
        root,
        decl,
        decl_error,
        git,
        level,
    };

    let mut findings = checks::run(&ctx);
    findings.sort_by(|a, b| {
        finding::requirement_rank(&a.id)
            .cmp(&finding::requirement_rank(&b.id))
            .then_with(|| a.severity.cmp(&b.severity))
            .then_with(|| a.problem.cmp(&b.problem))
    });

    let date = chrono::Utc::now().naive_local().date();

    Ok(Report {
        level,
        findings,
        checked_commit,
        date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_reports_missing_declaration() {
        let dir = tempfile::tempdir().unwrap();
        let report = run(dir.path(), None).unwrap();
        assert!(report
            .findings
            .iter()
            .any(|f| f.id == "D2" && f.severity == finding::Severity::Fail));
    }
}
