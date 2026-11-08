use std::path::Path;

use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ecosystem {
    Rust,
    Node,
    Python,
    Go,
    Java,
    Dotnet,
    Ruby,
    Php,
    Unknown,
}

impl std::fmt::Display for Ecosystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ecosystem::Rust => write!(f, "Rust"),
            Ecosystem::Node => write!(f, "Node"),
            Ecosystem::Python => write!(f, "Python"),
            Ecosystem::Go => write!(f, "Go"),
            Ecosystem::Java => write!(f, "Java"),
            Ecosystem::Dotnet => write!(f, ".NET"),
            Ecosystem::Ruby => write!(f, "Ruby"),
            Ecosystem::Php => write!(f, "PHP"),
            Ecosystem::Unknown => write!(f, "unknown"),
        }
    }
}

/// Directories where manifests may live: the root and its direct, non-hidden
/// subdirectories, so that a repository holding its crate or package in a
/// folder is still recognised.
pub fn manifest_dirs(root: &Path) -> Vec<std::path::PathBuf> {
    let mut dirs = vec![root.to_path_buf()];
    if let Ok(entries) = std::fs::read_dir(root) {
        let mut subdirs: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .filter(|e| {
                let name = e.file_name();
                let name = name.to_string_lossy();
                !name.starts_with('.')
                    && !["target", "node_modules", "vendor", "dist", "build"]
                        .contains(&name.as_ref())
            })
            .map(|e| e.path())
            .collect();
        subdirs.sort();
        dirs.extend(subdirs);
    }
    dirs
}

/// Ecosystems detected in the root or its direct subdirectories, without duplicates.
pub fn detect(root: &Path) -> Vec<Ecosystem> {
    let mut found = Vec::new();
    for dir in manifest_dirs(root) {
        for ecosystem in detect_in(&dir) {
            if !found.contains(&ecosystem) {
                found.push(ecosystem);
            }
        }
    }
    found
}

fn detect_in(root: &Path) -> Vec<Ecosystem> {
    let mut found = Vec::new();

    if root.join("Cargo.toml").is_file() {
        found.push(Ecosystem::Rust);
    }
    if root.join("package.json").is_file() {
        found.push(Ecosystem::Node);
    }
    if ["pyproject.toml", "setup.py", "requirements.txt"]
        .iter()
        .any(|name| root.join(name).is_file())
    {
        found.push(Ecosystem::Python);
    }
    if root.join("go.mod").is_file() {
        found.push(Ecosystem::Go);
    }
    if ["pom.xml", "build.gradle", "build.gradle.kts"]
        .iter()
        .any(|name| root.join(name).is_file())
    {
        found.push(Ecosystem::Java);
    }
    if !util::find_files_by_glob(root, "*.csproj").is_empty()
        || !util::find_files_by_glob(root, "*.sln").is_empty()
    {
        found.push(Ecosystem::Dotnet);
    }
    if root.join("Gemfile").is_file() {
        found.push(Ecosystem::Ruby);
    }
    if root.join("composer.json").is_file() {
        found.push(Ecosystem::Php);
    }

    found
}

pub fn lockfiles(ecosystem: Ecosystem) -> &'static [&'static str] {
    match ecosystem {
        Ecosystem::Rust => &["Cargo.lock"],
        Ecosystem::Node => &[
            "package-lock.json",
            "yarn.lock",
            "pnpm-lock.yaml",
            "bun.lockb",
            "bun.lock",
        ],
        Ecosystem::Python => &[
            "poetry.lock",
            "uv.lock",
            "Pipfile.lock",
            "pdm.lock",
            "requirements.lock",
            "requirements.txt",
        ],
        Ecosystem::Go => &["go.sum"],
        Ecosystem::Java => &["gradle.lockfile", "pom.xml"],
        Ecosystem::Dotnet => &["packages.lock.json"],
        Ecosystem::Ruby => &["Gemfile.lock"],
        Ecosystem::Php => &["composer.lock"],
        Ecosystem::Unknown => &[],
    }
}

pub fn default_test_command(ecosystem: Ecosystem) -> Option<&'static str> {
    match ecosystem {
        Ecosystem::Rust => Some("cargo test"),
        Ecosystem::Node => Some("npm test"),
        Ecosystem::Python => Some("pytest"),
        Ecosystem::Go => Some("go test ./..."),
        Ecosystem::Java => Some("mvn test"),
        Ecosystem::Dotnet => Some("dotnet test"),
        Ecosystem::Ruby => Some("bundle exec rspec"),
        Ecosystem::Php => Some("composer test"),
        Ecosystem::Unknown => None,
    }
}

pub fn known_vuln_tools() -> &'static [&'static str] {
    &[
        "cargo audit",
        "cargo deny",
        "npm audit",
        "pnpm audit",
        "yarn audit",
        "pip-audit",
        "safety",
        "govulncheck",
        "osv-scanner",
        "trivy",
        "snyk",
        "grype",
        "bundler-audit",
        "composer audit",
    ]
}

pub fn known_secret_scanners() -> &'static [&'static str] {
    &[
        "gitleaks",
        "trufflehog",
        "detect-secrets",
        "secretlint",
        "ggshield",
    ]
}

pub fn known_static_analysers() -> &'static [&'static str] {
    &[
        "clippy",
        "eslint",
        "biome",
        "ruff",
        "mypy",
        "pyright",
        "bandit",
        "semgrep",
        "golangci-lint",
        "staticcheck",
        "spotbugs",
        "pmd",
        "sonar",
        "phpstan",
        "psalm",
        "rubocop",
    ]
}

pub fn known_lint_tools() -> &'static [&'static str] {
    &[
        "clippy",
        "eslint",
        "biome",
        "ruff",
        "mypy",
        "pyright",
        "bandit",
        "semgrep",
        "golangci-lint",
        "staticcheck",
        "spotbugs",
        "pmd",
        "sonar",
        "phpstan",
        "psalm",
        "rubocop",
        "rustfmt",
        "cargo fmt",
        "prettier",
        "black",
        "gofmt",
        "dotnet format",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn detect_rust_and_node() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[package]\n").unwrap();
        fs::write(dir.path().join("package.json"), "{}\n").unwrap();
        let found = detect(dir.path());
        assert!(found.contains(&Ecosystem::Rust));
        assert!(found.contains(&Ecosystem::Node));
    }
}
