---
version: "0.1.1"
level: copilot
processes:
  design: pair
  implementation: copilot
  testing: none
  documentation: copilot
  review: hint
  deployment: none
convention:
  version: "0.1"
  level: 2
  adopted_at: 03c8df2
  status: active
  tools:
    - "Claude Code:claude-fable-5-1"
  attribution_trailers: [Assisted-by, Generated-by]
  coverage: { threshold: 80, kind: line, tool: cargo-llvm-cov }
  maintainability: { tool: none, duplication_max: 5, complexity_max: 15, function_max: 80, file_max: 400 }
  freshness: { max_outdated: 3, max_lag_months: 6 }
  commands:
    test: "cargo test --manifest-path vibecheck/Cargo.toml"
    lint: "cargo clippy --manifest-path vibecheck/Cargo.toml --all-targets -- -D warnings"
    format: "cargo fmt --manifest-path vibecheck/Cargo.toml --check"
    coverage: "cargo llvm-cov --manifest-path vibecheck/Cargo.toml --summary-only"
    audit: "cargo audit --file vibecheck/Cargo.lock"
    static_analysis: "cargo clippy --manifest-path vibecheck/Cargo.toml --all-targets -- -D warnings"
    secrets: "gitleaks detect --no-banner"
    outdated: "cargo outdated --manifest-path vibecheck/Cargo.toml --exit-code 1"
  maintainers: [ikarys]
  review: "single maintainer reads every diff before it reaches main"
  security_response: "none"
  licence: MIT
---

# Vibecoding declaration

This repository follows its own [Vibecoding Convention](CONVENTION.md) version
0.1 at level 2. It holds text, templates and the `vibecheck` validator, which
has tests, CI, coverage, lint, secrets scanning and dependency checks. Level 3
will be claimed once the audit reports exist.

## AI assistance

The convention text, the criticism survey and the templates are drafted by AI
assistants in conversation with the maintainer, who frames, argues, edits and
decides. Code and agent definitions are drafted by a local model (qwen3.8-27b)
called from Claude Code, then reviewed and corrected there; commits are
attributed to the tool that committed them. Most of the text and code is
AI-written.

## Review

Single maintainer. Every change is read by the maintainer before it reaches
`main`. AI-written commits carry an `Assisted-by:` trailer naming the tool and
model.

## Provenance and licence

This repository is licensed under MIT. Its text is written with AI assistants
and may not be protected by copyright in some jurisdictions; the maintainer
accepts this. To the maintainer's knowledge, no verbatim reproduction of
licence-incompatible text was included. Safeguards used: human reading of
every change.

## Status

Active. This repository holds no running software; there is no security
response commitment.

## Exceptions

- T8: duplication and complexity are not measured yet; no tool is chosen for
  Rust. File and function length are measured by `vibecheck` itself. The
  thresholds above for duplication and complexity are targets, not evidence.
