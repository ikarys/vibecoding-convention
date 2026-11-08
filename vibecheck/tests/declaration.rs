use std::fs;

use vibecheck::declaration::{parse_interval, read_declaration, Declaration};

#[test]
fn read_declaration_errors_without_front_matter() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("VIBECODING.md"), "no front matter here\n").unwrap();
    let (decl, err) = read_declaration(dir.path());
    assert!(decl.is_none());
    assert!(err.is_some_and(|e| e.contains("front matter")));
}

#[test]
fn read_declaration_errors_on_invalid_yaml() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("VIBECODING.md"),
        "---\nlevel: [1\n---\nbody\n",
    )
    .unwrap();
    let (decl, err) = read_declaration(dir.path());
    assert!(decl.is_none());
    assert!(err.is_some_and(|e| e.contains("YAML")));
}

#[test]
fn parse_interval_accepts_supported_units() {
    assert_eq!(parse_interval("90d").unwrap(), chrono::Duration::days(90));
    assert_eq!(parse_interval("2y").unwrap(), chrono::Duration::days(730));
    assert_eq!(parse_interval("6m").unwrap(), chrono::Duration::days(180));
    assert_eq!(parse_interval("").unwrap(), chrono::Duration::days(180));
}

#[test]
fn parse_interval_rejects_bogus_unit() {
    assert!(parse_interval("bogus").is_err());
}

#[test]
fn attribution_trailers_default_and_declared() {
    let mut decl = Declaration::default();
    let default = decl.convention.attribution_trailers();
    assert!(default.contains(&"assisted-by".to_string()));
    assert!(default.contains(&"generated-by".to_string()));

    decl.convention.attribution_trailers = vec!["Custom-Tool".into()];
    assert_eq!(
        decl.convention.attribution_trailers(),
        vec!["custom-tool".to_string()]
    );
}
