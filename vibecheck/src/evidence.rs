//! Readers for the evidence files a project's CI writes. vibecheck never runs
//! the project's tools; it parses what they left behind.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;

use crate::git::Git;

/// Evidence files larger than this are refused: the declaration is hostile
/// input and a path may point at an unbounded file.
const MAX_EVIDENCE_BYTES: u64 = 16 * 1024 * 1024;

/// Most names reported per finding family, to keep the report readable.
const MAX_NAMES: usize = 10;

/// Why an evidence file could not be read.
#[derive(Debug)]
pub enum EvidenceError {
    /// The declared file does not exist.
    Missing(PathBuf),
    /// The file exists but may not or cannot be read; the string is the reason.
    Unreadable(PathBuf, String),
}

impl fmt::Display for EvidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvidenceError::Missing(path) => {
                write!(f, "declared evidence missing: {}", path.display())
            }
            EvidenceError::Unreadable(path, why) => {
                write!(
                    f,
                    "declared evidence unreadable: {} ({why})",
                    path.display()
                )
            }
        }
    }
}

/// Reads an evidence file named relative to the repository root. Absolute
/// paths, `..` components and symlinks that leave the root are refused.
pub fn read(root: &Path, relative: &str) -> Result<String, EvidenceError> {
    let rel = PathBuf::from(relative);
    let unreadable = |why: &str| EvidenceError::Unreadable(rel.clone(), why.to_string());
    if relative.is_empty() {
        return Err(unreadable("empty path"));
    }
    let escapes = Path::new(relative)
        .components()
        .any(|c| !matches!(c, Component::Normal(_) | Component::CurDir));
    if escapes {
        return Err(unreadable("path must stay inside the repository"));
    }
    let full = root.join(relative);
    if !full.exists() {
        return Err(EvidenceError::Missing(rel));
    }
    let real = full
        .canonicalize()
        .map_err(|e| unreadable(&e.to_string()))?;
    let real_root = root
        .canonicalize()
        .map_err(|e| unreadable(&e.to_string()))?;
    if !real.starts_with(&real_root) {
        return Err(unreadable("path resolves outside the repository"));
    }
    let meta = fs::metadata(&real).map_err(|e| unreadable(&e.to_string()))?;
    if !meta.is_file() {
        return Err(unreadable("not a regular file"));
    }
    if meta.len() > MAX_EVIDENCE_BYTES {
        return Err(unreadable("file too large"));
    }
    let bytes = fs::read(&real).map_err(|e| unreadable(&e.to_string()))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// Last percentage in a text, such as the TOTAL line of a coverage summary.
pub fn last_percent(text: &str) -> Option<f64> {
    let re = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*%").ok()?;
    re.captures_iter(text)
        .last()
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
}

/// Line coverage from lcov, Cobertura XML, cargo-llvm-cov JSON or, failing
/// those, the last percentage in the text.
pub fn coverage_percent(text: &str) -> Option<f64> {
    lcov_percent(text)
        .or_else(|| cobertura_percent(text))
        .or_else(|| llvm_cov_json_percent(text))
        .or_else(|| last_percent(text))
}

fn lcov_percent(text: &str) -> Option<f64> {
    let (mut found, mut hit) = (0u64, 0u64);
    for line in text.lines() {
        if let Some(n) = line.trim().strip_prefix("LF:") {
            found += n.trim().parse::<u64>().ok()?;
        } else if let Some(n) = line.trim().strip_prefix("LH:") {
            hit += n.trim().parse::<u64>().ok()?;
        }
    }
    (found > 0).then(|| hit as f64 / found as f64 * 100.0)
}

fn cobertura_percent(text: &str) -> Option<f64> {
    let re = regex::Regex::new(r#"line-rate="([0-9.]+)""#).ok()?;
    let rate: f64 = re.captures(text)?.get(1)?.as_str().parse().ok()?;
    Some(rate * 100.0)
}

fn llvm_cov_json_percent(text: &str) -> Option<f64> {
    let value: Value = serde_json::from_str(text).ok()?;
    value
        .pointer("/data/0/totals/lines/percent")
        .and_then(Value::as_f64)
}

/// Number of unresolved vulnerabilities reported by cargo-audit, npm/pnpm
/// audit or pip-audit (JSON), or by a text summary. `None` when no count can
/// be read.
pub fn vulnerability_count(text: &str) -> Option<usize> {
    if let Ok(value) = serde_json::from_str::<Value>(text) {
        return json_vulnerability_count(&value);
    }
    let re = regex::Regex::new(r"(\d+) vulnerabilit").ok()?;
    if let Some(caps) = re.captures(text) {
        return caps.get(1)?.as_str().parse().ok();
    }
    text.to_lowercase()
        .contains("no vulnerabilities")
        .then_some(0)
}

fn json_vulnerability_count(value: &Value) -> Option<usize> {
    if let Some(count) = value
        .pointer("/vulnerabilities/count")
        .and_then(Value::as_u64)
    {
        return Some(count as usize);
    }
    if let Some(meta) = value.pointer("/metadata/vulnerabilities") {
        if let Some(total) = meta.get("total").and_then(Value::as_u64) {
            return Some(total as usize);
        }
        let sum: u64 = ["info", "low", "moderate", "high", "critical"]
            .iter()
            .filter_map(|key| meta.get(key).and_then(Value::as_u64))
            .sum();
        return Some(sum as usize);
    }
    let deps = value
        .as_array()
        .or_else(|| value.get("dependencies").and_then(Value::as_array))?;
    let affected = deps
        .iter()
        .filter(|dep| {
            dep.get("vulns")
                .and_then(Value::as_array)
                .is_some_and(|vulns| !vulns.is_empty())
        })
        .count();
    Some(affected)
}

/// Names of unmaintained or yanked dependencies in an audit output (cargo-audit
/// JSON or text), without the held-back ones, one entry per name.
pub fn audit_warnings(text: &str, held_back: &BTreeSet<String>) -> Vec<String> {
    let names = match serde_json::from_str::<Value>(text) {
        Ok(value) => json_audit_names(&value),
        Err(_) => text_audit_names(text),
    };
    let mut seen = BTreeSet::new();
    names
        .into_iter()
        .filter(|name| !held_back.contains(name) && seen.insert(name.clone()))
        .take(MAX_NAMES)
        .collect()
}

fn json_audit_names(value: &Value) -> Vec<String> {
    let mut names = Vec::new();
    for kind in ["unmaintained", "yanked"] {
        let Some(list) = value
            .pointer(&format!("/warnings/{kind}"))
            .and_then(Value::as_array)
        else {
            continue;
        };
        for entry in list {
            let name = entry
                .pointer("/advisory/package")
                .or_else(|| entry.pointer("/package/name"))
                .and_then(Value::as_str);
            if let Some(name) = name {
                names.push(name.to_string());
            }
        }
    }
    names
}

fn text_audit_names(output: &str) -> Vec<String> {
    // Audit tools print one block per advisory; the crate name comes on its
    // own labelled line before the status lines, so the last name seen is the
    // one the warning belongs to.
    let mut current: Option<String> = None;
    let mut names = Vec::new();
    for line in output.lines() {
        if let Some(name) = name_after_label(line) {
            current = Some(name);
        }
        let lower = line.to_lowercase();
        if lower.contains("unmaintained") || lower.contains("yanked") {
            names.push(
                current
                    .clone()
                    .unwrap_or_else(|| line.trim().chars().take(80).collect()),
            );
        }
    }
    names
}

fn name_after_label(line: &str) -> Option<String> {
    for label in ["Crate:", "Package:", "name:"] {
        if let Some(index) = line.find(label) {
            let rest = line[index + label.len()..].trim();
            if let Some(name) = rest.split_whitespace().next() {
                return Some(name.to_string());
            }
        }
    }
    None
}

/// Count and first names of outdated dependencies, held-back ones excluded.
/// Accepts cargo-outdated JSON, npm outdated JSON, or a text table with one
/// `name current latest` row per dependency.
pub fn outdated_count(text: &str, held_back: &BTreeSet<String>) -> (usize, Vec<String>) {
    let candidates = match serde_json::from_str::<Value>(text) {
        Ok(value) => json_outdated_names(&value),
        Err(_) => text
            .lines()
            .filter(|line| is_outdated_row(line))
            .filter_map(|line| line.split_whitespace().next().map(str::to_string))
            .collect(),
    };
    let names: Vec<String> = candidates
        .into_iter()
        .filter(|name| !held_back.contains(name))
        .collect();
    (names.len(), names.into_iter().take(MAX_NAMES).collect())
}

fn json_outdated_names(value: &Value) -> Vec<String> {
    if let Some(deps) = value.get("dependencies").and_then(Value::as_array) {
        return deps
            .iter()
            .filter(|dep| dep.get("latest") != dep.get("project"))
            .filter_map(|dep| dep.get("name").and_then(Value::as_str))
            .map(str::to_string)
            .collect();
    }
    value
        .as_object()
        .map(|map| map.keys().cloned().collect())
        .unwrap_or_default()
}

fn is_outdated_row(line: &str) -> bool {
    let mut parts = line.split_whitespace();
    let first = parts.next();
    matches!(first, Some(name) if !name.is_empty())
        && has_version(parts.next())
        && has_version(parts.next())
}

fn has_version(token: Option<&str>) -> bool {
    token.is_some_and(|token| {
        token
            .as_bytes()
            .windows(3)
            .any(|w| w[0].is_ascii_digit() && w[1] == b'.' && w[2].is_ascii_digit())
    })
}

/// Mutation score in percent: cargo-mutants `outcomes.json` (`caught` over
/// `caught + missed + timeout`), a JSON `mutationScore`/`mutation_score`
/// number, or the last percentage in a text summary.
pub fn mutation_score(text: &str) -> Option<f64> {
    if let Ok(value) = serde_json::from_str::<Value>(text) {
        return explicit_mutation_score(&value).or_else(|| cargo_mutants_score(&value));
    }
    last_percent(text)
}

fn explicit_mutation_score(value: &Value) -> Option<f64> {
    value
        .get("mutationScore")
        .or_else(|| value.get("mutation_score"))
        .and_then(Value::as_f64)
}

fn cargo_mutants_score(value: &Value) -> Option<f64> {
    let count = |key: &str| value.get(key).and_then(Value::as_u64);
    let caught = count("caught")?;
    let total = caught + count("missed")? + count("timeout").unwrap_or(0);
    (total > 0).then(|| caught as f64 * 100.0 / total as f64)
}

/// Warning text when a release tag is newer than the last commit touching the
/// evidence file: the file then describes a state older than the release.
pub fn staleness(relative: &str, git: &Git) -> Option<String> {
    let changed = git.last_commit_time(relative)?;
    let (tag, tag_date) = git.latest_tag()?;
    (tag_date > changed).then(|| format!("evidence older than release {tag}"))
}

#[cfg(test)]
mod tests;
