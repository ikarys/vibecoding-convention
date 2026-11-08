use super::*;

fn none() -> BTreeSet<String> {
    BTreeSet::new()
}

fn held(name: &str) -> BTreeSet<String> {
    [name.to_string()].into()
}

#[test]
fn read_rejects_escaping_paths() {
    let dir = tempfile::tempdir().unwrap();
    for bad in ["../x", "a/../../x", "/etc/passwd", ""] {
        assert!(
            matches!(read(dir.path(), bad), Err(EvidenceError::Unreadable(..))),
            "{bad}"
        );
    }
}

#[test]
fn read_reports_missing_and_content() {
    let dir = tempfile::tempdir().unwrap();
    let err = read(dir.path(), "audit/none.txt").unwrap_err();
    assert!(matches!(err, EvidenceError::Missing(_)));
    assert!(err.to_string().contains("audit/none.txt"));
    fs::write(dir.path().join("a.txt"), "hello").unwrap();
    assert_eq!(read(dir.path(), "./a.txt").unwrap(), "hello");
}

#[test]
fn read_rejects_directories_and_symlinks_out_of_root() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("sub")).unwrap();
    let err = read(dir.path(), "sub").unwrap_err();
    assert!(err.to_string().contains("not a regular file"));

    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("secret"), "x").unwrap();
    std::os::unix::fs::symlink(outside.path().join("secret"), dir.path().join("link")).unwrap();
    let err = read(dir.path(), "link").unwrap_err();
    assert!(err.to_string().contains("outside"));
}

#[test]
fn coverage_from_lcov() {
    let lcov = "SF:a.rs\nLF:10\nLH:9\nend_of_record\nSF:b.rs\nLF:10\nLH:5\nend_of_record\n";
    assert_eq!(coverage_percent(lcov), Some(70.0));
}

#[test]
fn coverage_from_cobertura() {
    let xml = r#"<coverage line-rate="0.875" branch-rate="0"><packages/></coverage>"#;
    assert_eq!(coverage_percent(xml), Some(87.5));
}

#[test]
fn coverage_from_llvm_cov_json() {
    let json = r#"{"data":[{"totals":{"lines":{"count":10,"percent":91.25}}}]}"#;
    assert_eq!(coverage_percent(json), Some(91.25));
}

#[test]
fn coverage_from_text_and_none() {
    assert_eq!(coverage_percent("TOTAL 100 9 91.00%\n"), Some(91.0));
    assert_eq!(coverage_percent("nothing"), None);
    assert_eq!(coverage_percent("LF:0\nLH:0\n"), None);
}

#[test]
fn last_percent_reads_last_value() {
    assert_eq!(last_percent("first 10% then 42.5%"), Some(42.5));
    assert_eq!(last_percent("no figure"), None);
}

#[test]
fn vulnerabilities_from_cargo_audit_json() {
    let one = r#"{"vulnerabilities":{"count":1,"list":[{}]},"warnings":{}}"#;
    let zero = r#"{"vulnerabilities":{"count":0,"list":[]},"warnings":{}}"#;
    assert_eq!(vulnerability_count(one), Some(1));
    assert_eq!(vulnerability_count(zero), Some(0));
}

#[test]
fn vulnerabilities_from_npm_json() {
    let total = r#"{"metadata":{"vulnerabilities":{"low":1,"total":4}}}"#;
    let parts = r#"{"metadata":{"vulnerabilities":{"low":1,"high":2,"critical":1}}}"#;
    assert_eq!(vulnerability_count(total), Some(4));
    assert_eq!(vulnerability_count(parts), Some(4));
}

#[test]
fn vulnerabilities_from_pip_audit_json() {
    let list = r#"[{"name":"a","vulns":[{"id":"X"}]},{"name":"b","vulns":[]}]"#;
    let wrapped = r#"{"dependencies":[{"name":"a","vulns":[]}]}"#;
    assert_eq!(vulnerability_count(list), Some(1));
    assert_eq!(vulnerability_count(wrapped), Some(0));
}

#[test]
fn vulnerabilities_from_text() {
    assert_eq!(
        vulnerability_count("error: 2 vulnerabilities found!"),
        Some(2)
    );
    assert_eq!(vulnerability_count("found 0 vulnerabilities"), Some(0));
    assert_eq!(vulnerability_count("No vulnerabilities found"), Some(0));
    assert_eq!(vulnerability_count("garbled"), None);
    assert_eq!(vulnerability_count(r#"{"unrelated":1}"#), None);
}

const AUDIT_BLOCK: &str = "Crate:     unsafe-libyaml
Version:   0.2.11
Warning:   unmaintained
Title:     unsafe-libyaml is unmaintained

warning: 1 allowed warning found
";

#[test]
fn audit_text_block_yields_one_name() {
    assert_eq!(audit_warnings(AUDIT_BLOCK, &none()), ["unsafe-libyaml"]);
    assert!(audit_warnings(AUDIT_BLOCK, &held("unsafe-libyaml")).is_empty());
}

const AUDIT_JSON: &str = r#"{"vulnerabilities":{"count":0,"list":[]},
  "warnings":{"unmaintained":[{"kind":"unmaintained","package":{"name":"old"},"advisory":{"id":"RUSTSEC-1","package":"old"}},
                              {"kind":"unmaintained","package":{"name":"old"},"advisory":{"id":"RUSTSEC-2","package":"old"}}],
              "yanked":[{"kind":"yanked","package":{"name":"gone","version":"1.0.0"}}]}}"#;

#[test]
fn audit_json_warnings_are_deduplicated() {
    assert_eq!(audit_warnings(AUDIT_JSON, &none()), ["old", "gone"]);
    assert_eq!(audit_warnings(AUDIT_JSON, &held("old")), ["gone"]);
}

#[test]
fn outdated_from_text_rows() {
    let table = "Name Project Latest\nserde 1.0.0 1.0.9\nfoo v1.0.0 v1.0.1\n";
    assert_eq!(outdated_count(table, &none()).0, 2);
    assert_eq!(
        outdated_count(table, &held("serde")),
        (1, vec!["foo".to_string()])
    );
}

#[test]
fn outdated_from_cargo_outdated_json() {
    let json = r#"{"crate_name":"x","dependencies":[
        {"name":"a","project":"1.0.0","latest":"1.1.0"},
        {"name":"b","project":"2.0.0","latest":"2.0.0"}]}"#;
    assert_eq!(outdated_count(json, &none()), (1, vec!["a".to_string()]));
}

#[test]
fn outdated_from_npm_json() {
    let json = r#"{"left-pad":{"current":"1.0.0","wanted":"1.0.0","latest":"2.0.0"},
                   "chalk":{"current":"4.0.0","wanted":"4.0.0","latest":"5.0.0"}}"#;
    assert_eq!(
        outdated_count(json, &held("chalk")),
        (1, vec!["left-pad".to_string()])
    );
    assert_eq!(outdated_count("[1]", &none()).0, 0);
}

#[test]
fn recognises_outdated_rows() {
    assert!(is_outdated_row("serde 1.0.0 1.0.9"));
    assert!(!is_outdated_row("Package Version Latest"));
    assert!(!is_outdated_row("serde 1.0.0"));
}
