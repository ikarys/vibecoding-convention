use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use walkdir::WalkDir;

pub fn read_file_if_exists(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

pub fn case_insensitive_contains(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

pub fn find_file_case_insensitive(root: &Path, name: &str) -> Option<PathBuf> {
    fs::read_dir(root).ok()?.into_iter().find_map(|entry| {
        let entry = entry.ok()?;
        let file_name = entry.file_name();
        if file_name.to_string_lossy().eq_ignore_ascii_case(name) {
            Some(entry.path())
        } else {
            None
        }
    })
}

pub fn find_files_by_glob(root: &Path, pattern: &str) -> Vec<PathBuf> {
    let pattern_parts: Vec<&str> = pattern.split('/').collect();
    let mut found = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| {
            if entry.file_type().is_dir() {
                let name = entry.file_name();
                name != ".git" && name != "target" && name != "node_modules"
            } else {
                true
            }
        })
        .filter_map(|entry| entry.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let relative = entry.path().strip_prefix(root).unwrap_or(entry.path());
        let relative_parts: Vec<String> = relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy().into_owned())
            .collect();

        if relative_parts.len() != pattern_parts.len() {
            continue;
        }

        let matches = pattern_parts
            .iter()
            .zip(relative_parts.iter())
            .all(|(pattern, actual)| glob_match(pattern, actual));

        if matches {
            found.push(entry.path().to_path_buf());
        }
    }

    found.sort();
    found
}

pub fn glob_match(pattern: &str, value: &str) -> bool {
    let mut regex = String::from("^");
    for character in pattern.chars() {
        match character {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            _ => regex.push_str(&regex::escape(&character.to_string())),
        }
    }
    regex.push('$');

    Regex::new(&regex)
        .map(|re| re.is_match(value))
        .unwrap_or(false)
}

pub fn secret_patterns() -> Vec<&'static str> {
    vec![
        r"AKIA[0-9A-Z]{16}",
        r"ghp_[A-Za-z0-9]{36}",
        r"sk-[A-Za-z0-9]{20,}",
        r"-----BEGIN [A-Z ]*PRIVATE KEY-----",
        r"xox[baprs]-[A-Za-z0-9-]{10,}",
    ]
}

pub fn scan_secrets(text: &str) -> Vec<(String, String)> {
    let mut hits = Vec::new();
    for pattern in secret_patterns() {
        if let Ok(re) = Regex::new(pattern) {
            for match_ in re.find_iter(text) {
                hits.push((pattern.to_string(), match_.as_str().to_string()));
            }
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive_contains_works() {
        assert!(case_insensitive_contains(
            "Hello VIBECODING.md",
            "vibecoding.md"
        ));
    }

    #[test]
    fn glob_match_works() {
        assert!(glob_match("LICENSE*", "LICENSE.mit"));
        assert!(glob_match("*.md", "README.md"));
        assert!(!glob_match("*.md", "README.txt"));
    }

    #[test]
    fn scans_secrets() {
        let token = format!("ghp_{}", "abcdefghijklmnopqrstuvwxyz0123456789");
        let hits = scan_secrets(&format!("key={token}"));
        assert!(!hits.is_empty());
    }
}
