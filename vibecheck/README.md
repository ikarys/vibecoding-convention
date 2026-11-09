# vibecheck

Command-line validator for the [Vibecoding Convention](../CONVENTION.md). It
contains no AI. It reads a project's `VIBECODING.md`, runs the mechanical
checks of the declared level (reading files, never running the project's
code), prints findings, and writes `audit/status.json`
for the README badge. It works on any repository, whatever the language.

## Install

```
cargo install --path vibecheck
```

## Usage

```
vibecheck [PATH] [--level N] [--format text|json] [--write-status] [--strict]
```

| Flag | Effect |
|---|---|
| `PATH` | repository root, default `.` |
| `--level N` | check at level N instead of the level declared in `VIBECODING.md` |
| `--format json` | machine-readable output: findings, summary, metrics |
| `--write-status` | write `audit/status.json` after the run (the badge reads it) |
| `--strict` | warnings make the exit code non-zero too |

Exit code 0 when no requirement fails (and no warning with `--strict`), 1
otherwise, 2 for a usage error.

Findings carry the requirement identifier, a severity, the place, the problem
and the fix:

```
T7  FAIL  no secrets scanner in a pre-commit hook  (.pre-commit-config.yaml)
    fix: add gitleaks (or an equivalent) to a pre-commit hook
```

Severities: `FAIL` (requirement not met), `WARN` (cannot be determined, or
recommended only), `INFO` (a figure worth knowing), `NOT RUN` (an input is
missing, for example a command not declared).

## What is checked

Only the **mechanical** part of each requirement; the judgement parts
(architecture, security claims, blind tests) belong to the agents.

Level 1, Declared: D1 README disclosure, D2 declaration fields and consistency
with the repository, D3 attribution trailers since adoption (any trailer the
project declares; a `Signed-off-by` naming a model fails), D4 review policy
stated, D5 dated check and staleness, D6 test removals without a stated reason,
D7 provenance statement and licence file, D8 contribution policy and pull
request template, D9 status and security response time, D10 agent instruction
files carry the rules.

Level 2, Tested: T1 test command present in CI, T2 coverage declared, the coverage tool in CI,
and the figure in the coverage evidence file measured against the threshold, T3 test files, unexplained disabled
tests, T4 an end-to-end suite, T5 lint and format in CI, T6 lockfile,
vulnerability check in CI, unresolved vulnerabilities in the audit evidence,
new dependencies without a recorded registry check, T7
secrets scanner in pre-commit and CI, static analyser in CI, a built-in scan of
the tree for secret patterns, T8 file and function length against the declared
thresholds, leftover markers, and `doc_lint` in CI (duplication and complexity
need the declared tool; its evidence file is noted, agents read it), T9
dependency freshness: an update bot, or `evidence.outdated` counted against
`max_outdated` (required only then), and unmaintained or yanked audit
items without a declared held-back exception. At level 2, D2 also requires
`evidence.coverage` and `evidence.audit`.

CI configuration is recognised for GitHub Actions, GitLab CI, Woodpecker,
Forgejo, Jenkins, CircleCI, Azure Pipelines, Bitbucket and Drone. Ecosystems
for lockfiles and default test commands: Rust, Node, Python, Go, Java, .NET,
Ruby, PHP, in the root and its direct subdirectories.

Level 3: A4 reads the audit index, the report headers and decisions, and
the git history of each report (a report edited after its first commit is a
WARN); A5 reads the reports, the tag message and the release notes section
at the latest tag; A7 compares the mutation score to the declared threshold.
The M half of A2, A6 and A8 is done by the agents.

## Evidence

vibecheck runs none of the project's code: no test suite, no tool, no
declared command. It is safe on any repository, including one you do not
trust. The project's CI runs the tools and writes their outputs to files; the
`evidence` block of the declaration names them, as paths relative to the
repository root:

```yaml
convention:
  evidence:
    coverage: audit/evidence/coverage.txt
    audit: audit/evidence/audit.json
    outdated: audit/evidence/outdated.json
```

| Key | Accepted formats | Used by |
|---|---|---|
| `coverage` | lcov (`LF:`/`LH:` sums), Cobertura XML (`line-rate`), cargo-llvm-cov JSON, or a text summary (the last percentage wins) | T2 |
| `audit` | cargo-audit JSON or text, npm/pnpm audit JSON, pip-audit JSON, or a text with `N vulnerabilities` | T6 (count), T9 (unmaintained, yanked) |
| `outdated` | cargo-outdated JSON, npm outdated JSON, or a text table with `name current latest` rows | T9 |
| `maintainability` | any file; its presence is noted for agents | T8 |
| `mutation` | cargo-mutants `outcomes.json` (`caught` over `caught + missed + timeout`), a JSON `mutationScore`/`mutation_score` number, or a text summary (the last percentage wins) | A7 |

A declared file that is missing, unreadable, outside the repository (absolute
path, `..`, a symlink out of the tree) or larger than 16 MiB is a FAIL; a file
without a figure is a FAIL for coverage and a WARN for the vulnerability
count. When a release tag is newer than the last commit that changed an
evidence file, a warning says the evidence is older than the release. The
`commands` block is still matched against the CI configuration but is never
executed.

## `audit/status.json`

Written by `--write-status`. The top-level fields follow the shields.io
endpoint schema so the badge is a dynamic one:

```
[![Vibecoding Convention](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/<owner>/<repo>/main/audit/status.json)](VIBECODING.md)
```

The `vibecoding` object holds the level, verdict, date, commit, open findings,
metrics (coverage, outdated count, longest file and function, when
measured), an `evidence` map (declared key to path) and who ran the check. `report` and `read_by` are preserved from the previous file for the
agents and the maintainer to fill.

## Development

```
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run -- ..          # the convention repository checks itself
