use std::fs;

use vibecheck::ecosystem::{default_test_command, detect, lockfiles, Ecosystem};

#[test]
fn detect_recognizes_multiple_manifests() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("go.mod"), "module x\n").unwrap();
    fs::write(dir.path().join("pom.xml"), "<project/>\n").unwrap();
    fs::write(dir.path().join("Gemfile"), "source 'rubygems.org'\n").unwrap();
    fs::write(dir.path().join("composer.json"), "{}\n").unwrap();
    fs::write(dir.path().join("pyproject.toml"), "[project]\n").unwrap();
    fs::write(dir.path().join("app.csproj"), "<Project/>\n").unwrap();

    let found = detect(dir.path());
    assert!(found.contains(&Ecosystem::Go));
    assert!(found.contains(&Ecosystem::Java));
    assert!(found.contains(&Ecosystem::Ruby));
    assert!(found.contains(&Ecosystem::Php));
    assert!(found.contains(&Ecosystem::Python));
    assert!(found.contains(&Ecosystem::Dotnet));
}

#[test]
fn lockfiles_and_default_test_commands_are_known() {
    assert!(lockfiles(Ecosystem::Rust).contains(&"Cargo.lock"));
    assert!(lockfiles(Ecosystem::Node).contains(&"package-lock.json"));
    assert!(lockfiles(Ecosystem::Python).contains(&"requirements.txt"));
    assert!(lockfiles(Ecosystem::Go).contains(&"go.sum"));
    assert!(lockfiles(Ecosystem::Java).contains(&"pom.xml"));
    assert!(lockfiles(Ecosystem::Dotnet).contains(&"packages.lock.json"));
    assert!(lockfiles(Ecosystem::Ruby).contains(&"Gemfile.lock"));
    assert!(lockfiles(Ecosystem::Php).contains(&"composer.lock"));

    assert_eq!(default_test_command(Ecosystem::Rust), Some("cargo test"));
    assert_eq!(default_test_command(Ecosystem::Node), Some("npm test"));
    assert_eq!(default_test_command(Ecosystem::Python), Some("pytest"));
    assert_eq!(default_test_command(Ecosystem::Go), Some("go test ./..."));
    assert_eq!(default_test_command(Ecosystem::Java), Some("mvn test"));
    assert_eq!(default_test_command(Ecosystem::Dotnet), Some("dotnet test"));
    assert_eq!(
        default_test_command(Ecosystem::Ruby),
        Some("bundle exec rspec")
    );
    assert_eq!(default_test_command(Ecosystem::Php), Some("composer test"));
    assert_eq!(default_test_command(Ecosystem::Unknown), None);
}
