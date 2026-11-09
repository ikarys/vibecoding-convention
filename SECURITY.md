# Security

This file covers `vibecheck`, the only software in this repository. The
convention text, templates and the static site carry no code that runs.

## Reporting a vulnerability

Report privately on GitHub, acknowledged within 7 days: use "Report a
vulnerability" on the Security tab of this repository. Do not open a public
issue for a vulnerability.

## What vibecheck protects, and what it does not

vibecheck reads a repository you may not trust and writes a report. The
asset is the host running it: a hostile repository must not make vibecheck
run code, read files outside the repository or exhaust the machine. The
host itself, the `git` binary on it and the user reading the report are
trusted. vibecheck's verdict is not a security assessment of the repository
it checks: it verifies that the declared evidence exists and matches the
declaration, nothing more.

## Threat model

| Class | Where it could happen here | Covered by |
|---|---|---|
| Running untrusted code | the declared `commands` block | By construction: vibecheck never runs a declared command; the only subprocess is `git` (`grep -rn Command::new vibecheck/src` lists `git.rs` only). |
| Command injection | `git.rs` | By construction: `git` is called with fixed arguments and no shell; the only values interpolated are commit hashes `git` itself returned. **Untested.** |
| Path traversal | evidence paths in the declaration, `@path` imports in agent files | `evidence::read` refuses absolute paths, `..` and symlinks leaving the root: `evidence::tests::read_rejects_escaping_paths`, `read_rejects_directories_and_symlinks_out_of_root`, `checks::d10::tests::ignores_imports_leaving_root`. |
| Resource exhaustion | evidence files, scanned source files, the declaration | Evidence capped at 16 MiB (`evidence::MAX_EVIDENCE_BYTES`), files scanned for secrets capped at 1 MiB (`support::text_files`). T8 reads source files whole, `VIBECODING.md` is read whole, and YAML alias expansion relies on the parser's limits: **known gap, untested.** |
| Malformed input | YAML front matter, evidence in unknown formats | Invalid YAML is an error, not a panic: `tests/declaration.rs::read_declaration_errors_on_invalid_yaml`. Unrecognised evidence yields a warning: the `evidence::tests::*_from_*` tests. |
| Regular expression denial of service | secret and marker scanning on hostile text | By construction: the `regex` crate guarantees linear time. |
| Network access | none | By construction: vibecheck has no network dependency (`cargo tree --manifest-path vibecheck/Cargo.toml`). |
| Secrets in the repository | every file | `gitleaks` in pre-commit (`.pre-commit-config.yaml`); vibecheck holds no credential. |
| Vulnerable dependencies | `vibecheck/Cargo.lock` | `cargo audit` run by `just evidence`, output in `audit/evidence/audit.json`, read by vibecheck's T6; the held-back `unsafe-libyaml` advisory is documented in `VIBECODING.md`. |

## Claims and their tests

| Claim | Test |
|---|---|
| Safe to run on a repository you do not trust | the three path traversal tests above; the "running untrusted code" and "network" rows are by construction and **untested** |
| A hostile evidence file cannot crash vibecheck | `evidence::tests` (format parsers on malformed input); the 16 MiB cap is **untested** |
| Invalid declarations are reported, not panicked on | `tests/declaration.rs::read_declaration_errors_on_invalid_yaml` |
