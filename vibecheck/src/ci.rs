use std::fs;
use std::path::{Path, PathBuf};

use crate::util;

pub fn ci_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    push_dir_files(
        &mut files,
        &root.join(".github/workflows"),
        &["yml", "yaml"],
    );
    push_dir_files(&mut files, &root.join(".woodpecker"), &["yml"]);
    push_dir_files(&mut files, &root.join(".forgejo/workflows"), &["yml"]);

    for name in [
        ".gitlab-ci.yml",
        ".woodpecker.yml",
        "Jenkinsfile",
        "azure-pipelines.yml",
        "bitbucket-pipelines.yml",
        ".drone.yml",
    ] {
        let path = root.join(name);
        if path.is_file() {
            files.push(path);
        }
    }

    let circle = root.join(".circleci/config.yml");
    if circle.is_file() {
        files.push(circle);
    }

    files.sort();
    files.dedup();
    files
}

fn push_dir_files(files: &mut Vec<PathBuf>, dir: &Path, extensions: &[&str]) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .map(|ext| extensions.contains(&ext.to_str().unwrap_or_default()))
                .unwrap_or(false)
        {
            files.push(path);
        }
    }
}

pub fn ci_mentions(root: &Path, needle: &str) -> Option<PathBuf> {
    ci_files(root)
        .into_iter()
        .find(|path| file_mentions(path, needle))
}

pub fn ci_mentions_any(root: &Path, needles: &[&str]) -> Option<(PathBuf, String)> {
    for path in ci_files(root) {
        for needle in needles {
            if file_mentions(&path, needle) {
                return Some((path, needle.to_string()));
            }
        }
    }
    None
}

fn file_mentions(path: &Path, needle: &str) -> bool {
    let Some(text) = util::read_file_if_exists(path) else {
        return false;
    };
    whitespace_contains(&text, needle)
}

pub fn whitespace_contains(haystack: &str, needle: &str) -> bool {
    normalize_ws(haystack).contains(&normalize_ws(needle))
}

fn normalize_ws(text: &str) -> String {
    let re = regex::Regex::new(r"\s+").expect("static regex");
    re.replace_all(text, " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_contains_collapses_whitespace() {
        assert!(whitespace_contains(
            "cargo   test\n--all",
            "cargo test --all"
        ));
        assert!(!whitespace_contains("cargo test", "cargo build"));
    }
}
