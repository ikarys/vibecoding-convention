//! CONVENTION.md, docs/requirements.md and convention.yaml must describe the same
//! requirements; these tests fail when an id, a level, a check kind or the
//! AI-specific list differs, or when vibecheck lacks a declared mechanical check.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
struct Convention {
    requirements: Vec<Requirement>,
    // Part B has its own key in the YAML; the tests treat both lists as one.
    contributions: Vec<Requirement>,
    ai_specific: Vec<String>,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
struct Requirement {
    id: String,
    level: Option<u8>,
    check: Option<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate root has no parent")
        .to_path_buf()
}

fn read(relative: &str) -> String {
    fs::read_to_string(repo_root().join(relative)).unwrap_or_else(|e| panic!("{relative}: {e}"))
}

fn yaml() -> (BTreeMap<String, Requirement>, BTreeSet<String>) {
    let mut parsed: Convention =
        serde_yaml_ng::from_str(&read("convention.yaml")).expect("convention.yaml parses");
    parsed.requirements.append(&mut parsed.contributions);
    let by_id = parsed
        .requirements
        .into_iter()
        .map(|r| (r.id.clone(), r))
        .collect();
    (by_id, parsed.ai_specific.into_iter().collect())
}

/// Part A rows carry a level and an `M`, `J` or `M+J` cell; Part B rows carry neither.
fn convention_md() -> (BTreeMap<String, Requirement>, BTreeSet<String>) {
    let text = read("CONVENTION.md");
    let part_a = Regex::new(r"^\| ([DTA]\d+) \| (\d) \| .* \| ([^|]+) \|$").unwrap();
    let part_b = Regex::new(r"^\| (C\d+) \| ").unwrap();
    let mut by_id = BTreeMap::new();
    for line in text.lines() {
        if let Some(c) = part_a.captures(line) {
            let check = match c[3].trim() {
                "M" => "mechanical",
                "J" => "judgement",
                "M+J" => "both",
                other => panic!("CONVENTION.md: unknown check kind {other:?} for {}", &c[1]),
            };
            let req = Requirement {
                id: c[1].to_string(),
                level: Some(c[2].parse().unwrap()),
                check: Some(check.to_string()),
            };
            by_id.insert(req.id.clone(), req);
        } else if let Some(c) = part_b.captures(line) {
            let req = Requirement {
                id: c[1].to_string(),
                ..Default::default()
            };
            by_id.insert(req.id.clone(), req);
        }
    }
    let parity = Regex::new(r"exist only because AI is involved: ([^.]+)\.").unwrap();
    let sentence = parity
        .captures(&text)
        .expect("CONVENTION.md has the parity sentence");
    let ids = Regex::new(r"[DTA]\d+")
        .unwrap()
        .find_iter(&sentence[1])
        .map(|m| m.as_str().to_string())
        .collect();
    (by_id, ids)
}

/// Requirements are `**ID. Title.**` paragraphs under `### Level N` or `## Part B`.
fn requirements_md() -> BTreeMap<String, Requirement> {
    let heading = Regex::new(r"^\*\*([DTAC]\d+)\. ").unwrap();
    let level = Regex::new(r"^### Level (\d):").unwrap();
    let mut current: Option<u8> = None;
    let mut by_id = BTreeMap::new();
    for line in read("docs/requirements.md").lines() {
        if let Some(c) = level.captures(line) {
            current = Some(c[1].parse().unwrap());
        } else if line.starts_with("## ") {
            current = None;
        } else if let Some(c) = heading.captures(line) {
            let req = Requirement {
                id: c[1].to_string(),
                level: current,
                check: None,
            };
            by_id.insert(req.id.clone(), req);
        }
    }
    by_id
}

fn ids(map: &BTreeMap<String, Requirement>) -> BTreeSet<&String> {
    map.keys().collect()
}

#[test]
fn same_ids_everywhere() {
    let (yaml, _) = yaml();
    let (md, _) = convention_md();
    let docs = requirements_md();
    assert_eq!(ids(&yaml), ids(&md), "convention.yaml vs CONVENTION.md");
    assert_eq!(
        ids(&yaml),
        ids(&docs),
        "convention.yaml vs docs/requirements.md"
    );
}

#[test]
fn same_levels_and_check_kinds() {
    let (yaml, _) = yaml();
    let (md, _) = convention_md();
    let docs = requirements_md();
    for (id, req) in &yaml {
        let in_md = &md[id];
        assert_eq!(
            req.level, in_md.level,
            "{id}: level, convention.yaml vs CONVENTION.md"
        );
        assert_eq!(
            req.check, in_md.check,
            "{id}: check kind, convention.yaml vs CONVENTION.md"
        );
        assert_eq!(
            req.level, docs[id].level,
            "{id}: level, convention.yaml vs docs/requirements.md section"
        );
        assert_eq!(
            req.level.is_none(),
            id.starts_with('C'),
            "{id}: only Part B has no level"
        );
    }
}

#[test]
fn ai_specific_matches_parity_sentence() {
    let (_, in_yaml) = yaml();
    let (_, in_md) = convention_md();
    assert_eq!(
        in_yaml, in_md,
        "ai_specific vs the CONVENTION.md parity sentence"
    );
}

// Every purely mechanical requirement has a check at every level. The M half
// of the level-3 "both" requirements (A2, A6, A8) is still the agents' work.
// The reverse direction only asks that each check is a known requirement at the
// right level: vibecheck also supports judgement checks such as T3 informatively.
#[test]
fn vibecheck_covers_mechanical_requirements() {
    let (yaml, _) = yaml();
    let implemented: BTreeMap<&str, u8> = vibecheck::checks::all()
        .iter()
        .map(|c| (c.id(), c.level()))
        .collect();
    for (id, req) in &yaml {
        let mechanical = req.check.as_deref() == Some("mechanical")
            || (req.check.as_deref() == Some("both") && req.level.is_some_and(|l| l <= 2));
        if mechanical {
            assert!(
                implemented.contains_key(id.as_str()),
                "{id}: no check in vibecheck"
            );
        }
    }
    for (id, level) in implemented {
        let req = yaml
            .get(id)
            .unwrap_or_else(|| panic!("{id}: not in convention.yaml"));
        assert_eq!(
            req.level,
            Some(level),
            "{id}: level, vibecheck vs convention.yaml"
        );
    }
}
