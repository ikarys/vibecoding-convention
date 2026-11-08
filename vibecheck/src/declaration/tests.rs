use super::*;

#[test]
fn parses_intervals() {
    assert_eq!(parse_interval("90d").unwrap(), Duration::days(90));
    assert_eq!(parse_interval("6m").unwrap(), Duration::days(180));
    assert_eq!(parse_interval("1y").unwrap(), Duration::days(365));
    assert!(parse_interval("6x").is_err());
}

#[test]
fn splits_front_matter() {
    let text = "---\nlevel: 1\n---\n# Body\n";
    let (yaml, body) = split_front_matter(text).unwrap();
    assert_eq!(yaml, "level: 1");
    assert_eq!(body, "# Body");
}

#[test]
fn reports_missing_required_keys() {
    let text = "---\nversion: \"0.1\"\n---\n";
    let missing = missing_required_keys(text);
    assert!(missing.contains(&"level".to_string()));
    assert!(missing.contains(&"convention".to_string()));
}
