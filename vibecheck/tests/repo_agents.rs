use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate root has no parent")
        .to_path_buf()
}

fn skill_names() -> BTreeSet<String> {
    let skills_dir = repo_root().join("skills");
    fs::read_dir(&skills_dir)
        .expect("skills directory exists")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect()
}

fn skill_files() -> BTreeSet<PathBuf> {
    let mut files = BTreeSet::new();
    for name in skill_names() {
        let path = repo_root().join("skills").join(&name).join("SKILL.md");
        if path.is_file() {
            files.insert(path);
        }
    }
    files
}

fn agent_ids(harness: &str) -> BTreeSet<String> {
    let dir = repo_root().join("agents").join(harness);
    fs::read_dir(&dir)
        .expect("agent directory exists")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter_map(|file_name| match harness {
            "claude-code" => {
                if file_name == "README.md" || !file_name.ends_with(".md") {
                    return None;
                }
                front_matter_value(&dir.join(&file_name), "name")
            }
            "fennec" if file_name.ends_with(".agent.md") => {
                front_matter_value(&dir.join(&file_name), "id")
            }
            _ => None,
        })
        .collect()
}

/// Claude Code names an agent by `name:`; Fennec by `id:`, its `name:` being
/// a display label.
fn front_matter_value(path: &Path, key: &str) -> Option<String> {
    let content = fs::read_to_string(path).expect("agent file is readable");
    let prefix = format!("{key}:");
    let mut in_front_matter = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if in_front_matter {
                return None;
            }
            in_front_matter = true;
            continue;
        }
        if in_front_matter {
            if let Some(value) = trimmed.strip_prefix(prefix.as_str()) {
                let value = value.trim();
                let value = value
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
                    .unwrap_or(value);
                return Some(value.to_string());
            }
        }
    }
    None
}

fn agent_references() -> BTreeSet<(String, PathBuf)> {
    let skill_names = skill_names();
    let re = Regex::new(r"`(vibecoding-[a-z-]+)`").expect("agent reference regex is valid");
    let mut references = BTreeSet::new();
    for path in skill_files() {
        let content = fs::read_to_string(&path).expect("skill file is readable");
        for capture in re.captures_iter(&content) {
            let name = capture[1].to_string();
            if !skill_names.contains(&name) {
                references.insert((name, path.clone()));
            }
        }
    }
    references
}

fn format_missing(missing: &BTreeSet<(String, PathBuf)>) -> String {
    missing
        .iter()
        .map(|(name, path)| format!("{} in {}", name, path.display()))
        .collect::<Vec<_>>()
        .join(", ")
}

#[test]
fn skills_name_only_shipped_agents() {
    let references = agent_references();
    for harness in ["claude-code", "fennec"] {
        let agents = agent_ids(harness);
        let missing: BTreeSet<(String, PathBuf)> = references
            .iter()
            .filter(|(name, _)| !agents.contains(name))
            .cloned()
            .collect();
        assert!(
            missing.is_empty(),
            "harness {} references agents that are not shipped: {}",
            harness,
            format_missing(&missing)
        );
    }
}

#[test]
fn harnesses_ship_the_same_agents() {
    assert_eq!(agent_ids("claude-code"), agent_ids("fennec"));
}

#[test]
fn not_vacuous_success() {
    assert!(
        !agent_references().is_empty(),
        "no agent references found in skills"
    );
    assert!(
        !agent_ids("claude-code").is_empty(),
        "no claude-code agents found"
    );
    assert!(!agent_ids("fennec").is_empty(), "no fennec agents found");
}
