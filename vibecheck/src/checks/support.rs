use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::checks::Context;
use crate::ci;
use crate::evidence;
use crate::finding::Finding;
use crate::util;

pub fn missing_declaration(id: &str) -> Vec<Finding> {
    vec![Finding::not_run(
        id,
        "declaration missing",
        "VIBECODING.md is missing",
        None,
    )]
}

pub fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string_lossy().into_owned())
}

pub fn last_lines(text: &str, count: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() <= count {
        text.trim().to_string()
    } else {
        lines[lines.len() - count..].join("\n")
    }
}

/// Reads the evidence file `path`; a missing or unreadable file is a FAIL
/// for the requirement `id`.
pub fn read_evidence(ctx: &Context, id: &str, path: &str) -> Result<String, Box<Finding>> {
    evidence::read(&ctx.root, path).map_err(|err| {
        Box::new(Finding::fail(
            id,
            "evidence",
            err.to_string(),
            Some(path.to_string()),
            Some("have CI write the declared evidence file".to_string()),
        ))
    })
}

/// A warning when a release tag is newer than the evidence file.
pub fn stale_evidence(ctx: &Context, id: &str, path: &str) -> Option<Finding> {
    evidence::staleness(path, &ctx.git).map(|text| {
        Finding::warn(
            id,
            "stale evidence",
            text,
            Some(path.to_string()),
            Some("regenerate the evidence in CI for the release".to_string()),
        )
    })
}

pub fn no_ci(id: &str) -> Finding {
    Finding::fail(
        id,
        "no CI",
        "no CI configuration found",
        None,
        Some("add CI configuration".to_string()),
    )
}

pub fn command_not_in_ci(id: &str, command: &str) -> Finding {
    Finding::fail(
        id,
        "not in CI",
        format!("CI does not run `{command}`"),
        None,
        Some("add it to a CI job".to_string()),
    )
}

pub fn known_tool_in_ci(
    ctx: &Context,
    id: &str,
    tools: &[&str],
    fail_message: &str,
) -> Vec<Finding> {
    if let Some((path, tool)) = ci::ci_mentions_any(&ctx.root, tools) {
        vec![Finding::info(
            id,
            "known tool in CI",
            format!("CI uses {tool} in {}", relative_path(&ctx.root, &path)),
            None,
        )]
    } else {
        vec![Finding::fail(
            id,
            "no tool in CI",
            fail_message.to_string(),
            None,
            Some("add a known tool to CI".to_string()),
        )]
    }
}

pub fn declared_or_known_ci(
    ctx: &Context,
    id: &str,
    name: &str,
    command: Option<String>,
    known_tools: &[&str],
    fail_message: &str,
) -> Vec<Finding> {
    match command {
        Some(command) => {
            if ci::ci_mentions(&ctx.root, &command).is_some() {
                Vec::new()
            } else {
                vec![command_not_in_ci(id, &command)]
            }
        }
        None => {
            let mut findings = vec![Finding::warn(
                id,
                "command not declared",
                format!("commands.{name} not declared"),
                None,
                Some("declare the command".to_string()),
            )];
            findings.extend(known_tool_in_ci(ctx, id, known_tools, fail_message));
            findings
        }
    }
}

pub fn files_with_extension(root: &Path, extensions: &[&str], skip_dirs: &[&str]) -> Vec<PathBuf> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| !in_skipped_dir(entry.path(), root, skip_dirs))
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| extensions.contains(&ext.to_str().unwrap_or_default()))
                .unwrap_or(false)
        })
        .map(|entry| entry.into_path())
        .collect()
}

pub fn source_files(ctx: &Context, extensions: &[&str]) -> Vec<PathBuf> {
    let skip_dirs = [".git", "target", "node_modules", "dist", "build", "vendor"];
    files_with_extension(&ctx.root, extensions, &skip_dirs)
        .into_iter()
        .filter(|path| !is_generated(path))
        .collect()
}

pub fn text_files(root: &Path, max_bytes: u64) -> Vec<PathBuf> {
    let skip_dirs = [".git", "target", "node_modules"];
    walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| !in_skipped_dir(entry.path(), root, &skip_dirs))
        .filter(|entry| {
            entry
                .metadata()
                .map(|meta| meta.len() <= max_bytes)
                .unwrap_or(false)
        })
        .filter(|entry| !is_binary(entry.path()))
        .map(|entry| entry.into_path())
        .collect()
}

pub fn find_path_containing_any(root: &Path, needles: &[&str]) -> Option<PathBuf> {
    let skip_dirs = [".git", "target", "node_modules"];
    let mut paths = walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| !in_skipped_dir(entry.path(), root, &skip_dirs))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    paths.sort();
    paths.into_iter().find(|path| {
        let text = path.to_string_lossy();
        needles
            .iter()
            .any(|needle| util::case_insensitive_contains(&text, needle))
    })
}

pub fn precommit_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for name in [".pre-commit-config.yaml", "lefthook.yml"] {
        let path = root.join(name);
        if path.is_file() {
            files.push(path);
        }
    }
    for dir in [".husky", ".githooks", ".git-hooks"] {
        let dir_path = root.join(dir);
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    files.push(entry.path());
                }
            }
        }
    }
    files.sort();
    files.dedup();
    files
}

pub fn files_mentioning_any(
    root: &Path,
    paths: &[PathBuf],
    needles: &[&str],
) -> Option<(PathBuf, String)> {
    for path in paths {
        if let Some(text) = util::read_file_if_exists(path) {
            for needle in needles {
                if ci::whitespace_contains(&text, needle) {
                    return Some((path.clone(), needle.to_string()));
                }
            }
        }
    }
    let _ = root;
    None
}

fn in_skipped_dir(path: &Path, root: &Path, skip_dirs: &[&str]) -> bool {
    let Ok(rel) = path.strip_prefix(root) else {
        return false;
    };
    rel.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|name| skip_dirs.contains(&name))
    })
}

fn is_generated(path: &Path) -> bool {
    util::read_file_if_exists(path)
        .map(|text| {
            text.lines()
                .take(3)
                .any(|line| util::case_insensitive_contains(line, "generated"))
        })
        .unwrap_or(false)
}

fn is_binary(path: &Path) -> bool {
    let Ok(mut file) = fs::File::open(path) else {
        return false;
    };
    let mut buf = [0u8; 8192];
    let Ok(len) = file.read(&mut buf) else {
        return false;
    };
    buf[..len].contains(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_lines_keeps_tail() {
        assert_eq!(last_lines("a\nb\nc\nd\ne", 2), "d\ne");
        assert_eq!(last_lines("a\nb", 5), "a\nb");
    }
}
