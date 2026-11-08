# vibecheck

Command-line validator for the [Vibecoding Convention](../CONVENTION.md). It
contains no AI. It reads a project's `VIBECODING.md`, runs the mechanical
checks of the declared level, prints findings, and writes `audit/status.json`
for the README badge. It works on any repository, whatever the language.

## Install

```
cargo install --path vibecheck
```

## Usage

```
vibecheck [PATH] [--level N] [--format text|json] [--write-status] [--run] [--strict]
```

| Flag | Effect |
|---|---|
| `PATH` | repository root, default `.` |
| `--level N` | check at level N instead of the level declared in `VIBECODING.md` |
| `--format json` | machine-readable output: findings, summary, metrics |
| `--write-status` | write `audit/status.json` after the run (the badge reads it) |
| `--run` | also execute the commands declared under `convention.commands` (each under `sh -c`, in its own process group, killed after 15 minutes; `VIBECHECK_RUNNING=1` is set so a nested vibecheck refuses to run commands again) |
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

Level 2, Tested: T1 test command present in CI, T2 coverage declared and (with
`--run`) measured against the threshold, T3 test files, unexplained disabled
tests, T4 an end-to-end suite, T5 lint and format in CI, T6 lockfile,
vulnerability check, new dependencies without a recorded registry check, T7
secrets scanner in pre-commit and CI, static analyser in CI, a built-in scan of
the tree for secret patterns, T8 file and function length against the declared
thresholds, leftover markers, and `doc_lint` in CI (duplication and complexity
need the declared tool), T9 dependency freshness: an update bot or
`commands.outdated`, outdated dependency count against `max_outdated`, and
unmaintained or yanked audit items without a declared held-back exception.

CI configuration is recognised for GitHub Actions, GitLab CI, Woodpecker,
Forgejo, Jenkins, CircleCI, Azure Pipelines, Bitbucket and Drone. Ecosystems
for lockfiles and default test commands: Rust, Node, Python, Go, Java, .NET,
Ruby, PHP, in the root and its direct subdirectories.

Level 3 checks are not implemented yet.

## `audit/status.json`

Written by `--write-status`. The top-level fields follow the shields.io
endpoint schema so the badge is a dynamic one:

```
[![Vibecoding Convention](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/<owner>/<repo>/main/audit/status.json)](VIBECODING.md)
```

The `vibecoding` object holds the level, verdict, date, commit, open findings,
metrics (coverage, longest file and function, when measured) and who ran the
check. `report` and `read_by` are preserved from the previous file for the
agents and the maintainer to fill.

## Development

```
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run -- ..          # the convention repository checks itself
