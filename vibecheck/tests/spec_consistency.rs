//! CONVENTION.md, its French translation, docs/requirements.md, convention.yaml
//! and the two site pages must describe the same requirements; these tests fail
//! when an id, a level, a check kind, the core list or the AI-specific list
//! differs, when a site link points to no heading, or when vibecheck lacks a
//! declared mechanical check.

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
    core: Vec<String>,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
struct Requirement {
    id: String,
    level: Option<u8>,
    check: Option<String>,
}

struct Yaml {
    by_id: BTreeMap<String, Requirement>,
    ai_specific: BTreeSet<String>,
    core: BTreeSet<String>,
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

fn yaml() -> Yaml {
    let mut parsed: Convention =
        serde_yaml_ng::from_str(&read("convention.yaml")).expect("convention.yaml parses");
    parsed.requirements.append(&mut parsed.contributions);
    Yaml {
        by_id: parsed
            .requirements
            .into_iter()
            .map(|r| (r.id.clone(), r))
            .collect(),
        ai_specific: parsed.ai_specific.into_iter().collect(),
        core: parsed.core.into_iter().collect(),
    }
}

/// Part A rows carry a level and an `M`, `J` or `M+J` cell; Part B rows carry
/// neither. A bold id marks a core requirement. The French text keeps the ids
/// and the check letters, so the same parser reads both.
fn convention_md(path: &str) -> (BTreeMap<String, Requirement>, BTreeSet<String>) {
    let text = read(path);
    let part_a = Regex::new(r"^\| (\**)([DTA]\d+)\** \| (\d) \| .* \| ([^|]+) \|$").unwrap();
    let part_b = Regex::new(r"^\| (C\d+) \| ").unwrap();
    let mut by_id = BTreeMap::new();
    let mut core = BTreeSet::new();
    for line in text.lines() {
        if let Some(c) = part_a.captures(line) {
            let check = match c[4].trim() {
                "M" => "mechanical",
                "J" => "judgement",
                "M+J" => "both",
                other => panic!("{path}: unknown check kind {other:?} for {}", &c[2]),
            };
            if !c[1].is_empty() {
                core.insert(c[2].to_string());
            }
            let req = Requirement {
                id: c[2].to_string(),
                level: Some(c[3].parse().unwrap()),
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
    (by_id, core)
}

/// The parity argument lives in the rationale page, not in the one-page text.
fn parity_ids() -> BTreeSet<String> {
    let rationale = read("docs/rationale.md");
    let parity = Regex::new(r"exist only because AI is involved: ([^.]+)\.").unwrap();
    let sentence = parity
        .captures(&rationale)
        .expect("docs/rationale.md has the parity sentence");
    Regex::new(r"[DTA]\d+")
        .unwrap()
        .find_iter(&sentence[1])
        .map(|m| m.as_str().to_string())
        .collect()
}

struct RequirementsMd {
    by_id: BTreeMap<String, Requirement>,
    /// Core ids with the level of the section whose sentence names them.
    core: BTreeMap<String, u8>,
    /// The anchor kramdown and GitHub derive from each `#### ID. Title` heading.
    anchors: BTreeMap<String, String>,
}

/// Requirements are `#### ID. Title` headings under `### Level N` or `## Part B`;
/// each level section opens with a "The core of this level" sentence.
fn requirements_md() -> RequirementsMd {
    let heading = Regex::new(r"^#### ([DTAC]\d+)\. (.*)$").unwrap();
    let level = Regex::new(r"^### Level (\d):").unwrap();
    let core_sentence = Regex::new(r"^The core of this level: ([^.]+)\.").unwrap();
    let id = Regex::new(r"[DTA]\d+").unwrap();
    let mut current: Option<u8> = None;
    let mut out = RequirementsMd {
        by_id: BTreeMap::new(),
        core: BTreeMap::new(),
        anchors: BTreeMap::new(),
    };
    for line in read("docs/requirements.md").lines() {
        if let Some(c) = level.captures(line) {
            current = Some(c[1].parse().unwrap());
        } else if line.starts_with("## ") {
            current = None;
        } else if let Some(c) = core_sentence.captures(line) {
            let level = current.expect("core sentence outside a level section");
            for m in id.find_iter(&c[1]) {
                out.core.insert(m.as_str().to_string(), level);
            }
        } else if let Some(c) = heading.captures(line) {
            out.anchors.insert(c[1].to_string(), slug(&c[0][5..]));
            let req = Requirement {
                id: c[1].to_string(),
                level: current,
                check: None,
            };
            out.by_id.insert(req.id.clone(), req);
        }
    }
    out
}

/// kramdown and GitHub agree on this for ASCII headings: lowercase, drop
/// everything but letters, digits, spaces and hyphens, spaces to hyphens.
fn slug(heading: &str) -> String {
    heading
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

struct SiteLink {
    level: u8,
    anchor: String,
    core: bool,
}

/// The level cards nest 3 > 2 > 1, each listing its own requirements before the
/// inner card opens, so the last `level level-N` class seen is the current level.
fn site_page(path: &str) -> BTreeMap<String, SiteLink> {
    let level = Regex::new(r#"class="level level-(\d)""#).unwrap();
    let link = Regex::new(r#"^\s*<li( class="core")?><a href=".*requirements\.html#([a-z0-9-]+)' \| relative_url }}">([DTA]\d+)</a>"#).unwrap();
    let mut current = 0;
    let mut links = BTreeMap::new();
    for line in read(path).lines() {
        if let Some(c) = level.captures(line) {
            current = c[1].parse().unwrap();
        } else if let Some(c) = link.captures(line) {
            assert_ne!(current, 0, "{path}: {} listed outside a level card", &c[3]);
            links.insert(
                c[3].to_string(),
                SiteLink {
                    level: current,
                    anchor: c[2].to_string(),
                    core: c.get(1).is_some(),
                },
            );
        }
    }
    links
}

fn ids(map: &BTreeMap<String, Requirement>) -> BTreeSet<&String> {
    map.keys().collect()
}

#[test]
fn same_ids_everywhere() {
    let yaml = yaml();
    let (md, _) = convention_md("CONVENTION.md");
    let (fr, _) = convention_md("CONVENTION.fr.md");
    let docs = requirements_md();
    assert_eq!(
        ids(&yaml.by_id),
        ids(&md),
        "convention.yaml vs CONVENTION.md"
    );
    assert_eq!(
        ids(&yaml.by_id),
        ids(&fr),
        "convention.yaml vs CONVENTION.fr.md"
    );
    assert_eq!(
        ids(&yaml.by_id),
        ids(&docs.by_id),
        "convention.yaml vs docs/requirements.md"
    );
}

#[test]
fn same_levels_and_check_kinds() {
    let yaml = yaml();
    let (md, _) = convention_md("CONVENTION.md");
    let (fr, _) = convention_md("CONVENTION.fr.md");
    let docs = requirements_md();
    for (id, req) in &yaml.by_id {
        for (name, text) in [("CONVENTION.md", &md), ("CONVENTION.fr.md", &fr)] {
            let in_md = &text[id];
            assert_eq!(
                req.level, in_md.level,
                "{id}: level, convention.yaml vs {name}"
            );
            assert_eq!(
                req.check, in_md.check,
                "{id}: check kind, convention.yaml vs {name}"
            );
        }
        assert_eq!(
            req.level, docs.by_id[id].level,
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
fn same_core_everywhere() {
    let yaml = yaml();
    let (_, md) = convention_md("CONVENTION.md");
    let (_, fr) = convention_md("CONVENTION.fr.md");
    let docs = requirements_md();
    assert_eq!(
        yaml.core, md,
        "core, convention.yaml vs bold ids in CONVENTION.md"
    );
    assert_eq!(
        yaml.core, fr,
        "core, convention.yaml vs bold ids in CONVENTION.fr.md"
    );
    assert_eq!(
        yaml.core,
        docs.core.keys().cloned().collect(),
        "core, convention.yaml vs the core sentences of docs/requirements.md"
    );
    for (id, level) in &docs.core {
        assert_eq!(
            yaml.by_id[id].level,
            Some(*level),
            "{id}: named as core under the wrong level in docs/requirements.md"
        );
    }
}

// The translation names the date of the English text it was made from.
#[test]
fn french_text_follows_the_latest_english_text() {
    let english = Regex::new(r"Last changed (\d{4}-\d{2}-\d{2})\.").unwrap();
    let french = Regex::new(r"modifié le (\d{4}-\d{2}-\d{2})").unwrap();
    let en = read("CONVENTION.md");
    let fr = read("CONVENTION.fr.md");
    let en_date = &english
        .captures(&en)
        .expect("CONVENTION.md has a Last changed date")[1];
    let fr_date = &french
        .captures(&fr)
        .expect("CONVENTION.fr.md names the date it translates")[1];
    assert_eq!(
        en_date, fr_date,
        "CONVENTION.fr.md translates an older CONVENTION.md"
    );
}

#[test]
fn ai_specific_matches_parity_sentence() {
    assert_eq!(
        yaml().ai_specific,
        parity_ids(),
        "ai_specific vs the docs/rationale.md parity sentence"
    );
}

// Each site page lists every Part A requirement once, inside the card of its
// level, marks the core ones, and links to the heading that exists in
// docs/requirements.md.
#[test]
fn site_pages_list_every_requirement_in_its_level() {
    let yaml = yaml();
    let docs = requirements_md();
    let part_a: BTreeSet<&String> = yaml
        .by_id
        .values()
        .filter(|r| r.level.is_some())
        .map(|r| &r.id)
        .collect();
    for path in ["index.html", "fr/index.html"] {
        let links = site_page(path);
        assert_eq!(links.keys().collect::<BTreeSet<_>>(), part_a, "{path}: ids");
        for (id, link) in &links {
            assert_eq!(
                yaml.by_id[id].level,
                Some(link.level),
                "{path}: {id} listed in the wrong level card"
            );
            assert_eq!(
                docs.anchors[id], link.anchor,
                "{path}: {id} links to an anchor docs/requirements.md does not have"
            );
            assert_eq!(
                yaml.core.contains(id),
                link.core,
                "{path}: {id} core marking differs from convention.yaml"
            );
        }
    }
}

// Every purely mechanical requirement has a check at every level. The M half
// of the level-3 "both" requirements (A2, A6, A8) is still the agents' work.
// The reverse direction only asks that each check is a known requirement at the
// right level: vibecheck also supports judgement checks such as T3 informatively.
#[test]
fn vibecheck_covers_mechanical_requirements() {
    let yaml = yaml().by_id;
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
