mod common;

use std::fs;

fn where_contains(report: &vibecheck::Report, needle: &str) -> bool {
    report
        .findings
        .iter()
        .any(|f| f.where_.as_deref().is_some_and(|w| w.contains(needle)))
}

#[test]
fn gitignored_files_are_not_scanned() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    common::finalize_level2_fixture(root);
    fs::write(root.join(".gitignore"), "cache/\n").unwrap();
    fs::create_dir_all(root.join("cache")).unwrap();
    let long = "pub const A: u8 = 0;\n".repeat(700);
    fs::write(root.join("cache/long.rs"), &long).unwrap();
    // Untracked but not ignored: still scanned, so the check is not vacuous.
    fs::write(root.join("src/long.rs"), &long).unwrap();

    let report = common::helpers::run_default(root);

    assert!(where_contains(&report, "src/long.rs"));
    assert!(!where_contains(&report, "cache/"));
}
