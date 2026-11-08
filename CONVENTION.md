# Vibecoding Convention

Version 0.1, draft. Last changed 2026-11-08.

## 1. Purpose

Software written wholly or partly by AI assistants ("vibecoded" software) can be
as trustworthy as any other software, provided it is framed, reviewed and
audited. This convention defines those three words in checkable terms. A
project that follows it can show, with evidence kept in its own repository,
that it was not thrown together.

The convention is a self-declaration backed by evidence. It is not a
certification, and nobody grants it. Anyone can read the declaration, re-run the
commands and check that the evidence matches.

It was written against the criticisms actually made of vibecoded software:
insecure code and leaking apps, code nobody understands, growing duplication,
tests that assert nothing, hallucinated dependencies, unknowable provenance,
maintainers flooded with low-effort contributions, nobody accountable,
projects abandoned behind a stale badge. [docs/criticisms.md](docs/criticisms.md)
lists each one with its sources and the requirement that answers it.

## 2. Principles

- **Transparency over purity.** Say what the AI wrote. Nobody is asked to hide,
  limit or avoid AI assistance.
- **Evidence over claims.** Every requirement names an artifact in the
  repository or a command anyone can run. A claim without evidence does not
  count.
- **Proportionality.** The convention has levels. A project picks the level
  that fits its purpose and proves it. No level asks for more control than a
  human-led project of the same size and purpose would have.
- **Humans decide.** AI proposes. A named human reviews, validates, decides and
  is accountable. This holds for code, for documents, for audit findings and
  for releases. The convention never requires that a human wrote something; it
  requires that a human validated it.
- **Nothing silent.** AI-written changes are attributed where they land: in the
  README, in the commit history, in the audit reports.
- **Measure the product, not the process.** The convention does not say how to
  work. It says what the result must show.

## 3. Parity with human-led projects

A fair objection: the convention asks of AI-assisted projects more than anyone
asks of human-written ones. The answer has two parts.

Most requirements are not new. CI, coverage, lint, secrets scanning,
dependency checks, `SECURITY.md`, architecture documents and decision records,
human review before merge, a release approved by a named person: a careful
human-led project of the same size is already expected to have them, and many
do not. Those requirements apply to any project, and a human-led project can
adopt this convention at any level unchanged.

Five requirements exist only because AI is involved: the disclosure (D1), the
declaration (D2), the commit attribution (D3), the provenance statement (D7)
and the blind test (A3, because AI-written documentation describes intent more
often than behaviour). They are the price of entering with a trust deficit,
and they are cheap: four of the five are text.

The parity test is a rule for this convention's own evolution: a requirement
that a careful human-led project of the same size would not be expected to
meet is not added, unless it is one of the AI-specific five. Each requirement
below that fails the test is a bug in the convention. Open an issue.

## 4. Levels

| Level | Name | Means |
|---|---|---|
| 1 | Declared | The project says it is AI-assisted, with which tools and models, who reviews and decides, how it handles contributions, and when it was last checked. |
| 2 | Tested | Level 1, plus automated tests that assert behaviour, a declared and met coverage threshold, green CI, lint, secrets and static analysis, dependency hygiene, maintainability signals. |
| 3 | Audited | Level 2, plus versioned audit reports: architecture review, security audit, blind usability test when applicable, validated architecture documents, each finding with a decision. |

A project declares one level in `VIBECODING.md`. Each level includes the
requirements of the levels below it. A project may satisfy some requirements of
a higher level without claiming that level.

Suggested fit:

- Level 1: scripts, prototypes, experiments, anything that says "do not use in
  production". Achievable in an afternoon; it is all text.
- Level 2: libraries, tools and services that other people depend on.
- Level 3: anything that handles other people's data, money or safety, and any
  project that wants its trustworthiness to be checked rather than believed.

## 5. Requirements

Each requirement has an identifier, a statement, the evidence expected, and how
it is checked. "Checked" says what an auditor, human or agent, does to verify
it. Identifiers are stable: a requirement is never renumbered.

### Level 1: Declared

**D1. README disclosure.** The README has a section, or the badge of D5, that
says the project is wholly or partly written with AI assistance and links to
`VIBECODING.md`.
- Evidence: the README section or badge.
- Checked: the section or badge exists and links to `VIBECODING.md`.

**D2. Declaration file.** The repository root has a `VIBECODING.md` whose
front matter uses the [AI-DECLARATION.md](https://github.com/DimwitLabs/AI-DECLARATION.md)
keys for the degree of AI involvement (`version`, `level`, `processes`,
`components`), so that the same information is written once and existing
badges keep working, plus a `convention` block for this convention's fields.
A project that already has an `AI-DECLARATION.md` may point to it instead of
repeating those keys. Section 6 gives the format. The file states, at least:
- the AI involvement level per process (design, implementation, testing,
  documentation, review, deployment), in AI-DECLARATION terms;
- the convention level claimed;
- the AI tools and models used, kept current;
- the human maintainers who review, validate and decide, by name or handle;
- the review policy (D4);
- the project status and security response time (D9);
- the provenance and licence statement (D7);
- the commit at which the convention was adopted;
- for level 2 and above: the thresholds and tools of T2, T7 and T8;
- for level 3: the audit directory, the audit cadence, and whether a blind
  test applies.
- Evidence: the file. A template is in `templates/VIBECODING.md`.
- Checked: every field is present and the stated values match the repository
  (tools named in commit trailers, audit dates in `audit/status.json`,
  thresholds in CI configuration).

**D3. Commit attribution.** Every commit that contains AI-written content
carries an `Assisted-by:` trailer naming the tool and model, in the format the
Linux kernel adopted: `Assisted-by: <Tool>:<model> [tool2] [tool3]`, for
example `Assisted-by: Claude Code:claude-opus-5-5`. A commit whose change is
substantially generated without human edits uses `Generated-by:` with the same
value. `Co-Authored-By:` is not used for a model: a model is not an author in
the legal sense and cannot sign a Certificate of Origin. Where the project uses
`Signed-off-by:`, a human signs; the model never does. A commit without a
trailer is understood as human-written.
- Evidence: the git history from the adoption commit named in `VIBECODING.md`.
- Checked: sample commits since adoption. Substantial commits without a
  trailer are a finding if the declaration says the project is mostly
  AI-written. The share of attributed commits is reported, not judged against
  a threshold. Any `Co-Authored-By:` naming a model, or `Signed-off-by:`
  naming a model, is a finding.

**D4. Human review before merge.** No AI-written change reaches the default
branch without a human having read it and approved it. For a team this is a
reviewed pull request. For a single maintainer this is the maintainer's own
reading of every diff, stated in `VIBECODING.md`, and a tool or workflow that
makes that reading a step rather than an intention (review gates, a
pull-request flow against oneself, a signed-off trailer). The agent or tool
that produced a change is never its only reviewer.
- Evidence: the review policy in `VIBECODING.md`, branch protection or pull
  request history where available, review-gate configuration of the tool.
- Checked: the policy exists; the history shows no AI-attributed commit merged
  without a human review event where the platform records one.

**D5. Dated check.** The repository carries the date and commit of the last
convention check, produced by the check itself and not typed by hand:
- `audit/status.json` holds the claimed level, the date, the commit checked,
  the verdict (`pass`, `pass with findings`, `fail`, `stale`), the number of
  open findings by severity, and the tool and model that ran the check. Its
  top-level fields follow the shields.io endpoint schema (`schemaVersion`,
  `label`, `message`, `color`), so the README badge is a dynamic badge that
  reads this file and cannot be edited without a visible commit under
  `audit/`.
- The check is stale when it is older than the last release, or older than
  the interval declared in `VIBECODING.md` (default: six months). A stale
  check is shown as such in the badge.
- Evidence: `audit/status.json`, the badge, the report the status points to.
- Checked: the file exists, matches `VIBECODING.md` and the latest report,
  and the commit it names exists; staleness is computed, not read.

**D6. Test changes are explained.** A change that deletes a test, weakens an
assertion, marks a test as skipped, or raises a tolerance states why in its
commit message or pull request, and the human reviewer approves that
statement. Making CI green by removing the check that failed is the failure
mode this requirement names.
- Evidence: the git history.
- Checked: the auditor lists test removals and weakenings since the last
  check and reports those without a stated reason.

**D7. Provenance and licence statement.** `VIBECODING.md` states the project
licence; acknowledges that AI-written parts may not be protected by copyright
in some jurisdictions and that the maintainers accept this; states that no
verbatim reproduction of licence-incompatible code was knowingly included; and
names the safeguards used, if any (vendor code-reference filters, snippet
scanning, a human check of long generated blocks).
- Evidence: the statement.
- Checked: the statement is present and complete; where a snippet scanner is
  declared, the auditor runs it.

**D8. Contribution policy.** `CONTRIBUTING.md` states the project's policy for
incoming AI-assisted contributions: accepted with disclosure, accepted with
conditions, or refused. Where accepted, it requires the D3 trailer and, in the
pull request, what the contributor ran to verify the change. Contributions the
project makes to other projects follow the target project's AI policy and
disclose AI assistance by default.
- Evidence: `CONTRIBUTING.md` and the pull request template.
- Checked: both exist and say this; the auditor samples merged pull requests
  from outside contributors for the trailer and a human approval.

**D9. Project status.** `VIBECODING.md` states the project status: `active`,
`maintenance` (fixes only), or `archived`. For `active` and `maintenance`
projects that accept security reports, `SECURITY.md` states the time within
which a report is acknowledged (for example 72 hours), and `VIBECODING.md`
matches it. A project that cannot commit to a response time says so; that is
honest, and it is the information a user needs.
- Evidence: the fields.
- Checked: present and consistent between the two files; an `archived`
  project is not required to keep its check fresh (D5).

### Level 2: Tested

**T1. Tests run in CI on every change.** Automated tests exist and run on every
push or pull request. The default branch is green. The build runs from a clean
checkout with the steps the documentation gives, so that a stranger can build
the project.
- Evidence: the CI configuration and the latest run on the default branch.
- Checked: the CI job runs the test command from a clean checkout; the last
  run passed.

**T2. Coverage at the declared threshold.** `VIBECODING.md` declares a line (or
statement) coverage threshold and the tool that measures it. The measured
coverage is at or above the threshold. The recommended threshold for software
used in production is 80 %. A lower threshold is allowed and must be
explained in one sentence.
- Evidence: the coverage report produced by CI, or the command that produces it
  locally.
- Checked: the auditor runs the command and compares.

**T3. Tests assert behaviour.** Tests check results, not only that the code ran.
No test is skipped or disabled without a comment giving the reason. A failing
test means the software is wrong, not that the test is flaky. Every change of
behaviour lands with the test that covers it, in the same change. Test-driven
development is a good way to get there and is recommended, not required: the
convention checks the result, not the order in which it was written.
- Evidence: the test suite and the history.
- Checked: the auditor samples tests in proportion to the suite size and
  reports: the share with no meaningful assertion (a call with no check, a
  check that the result is not null, a check against the value the code
  returns rather than the value it should), disabled tests without a reason,
  tests that still pass with the implementation stubbed out, and behaviour
  changes since the last check that landed without a test. The report says
  which tests were read.

**T4. End-to-end tests for user-facing software.** Software with a user
interface, an API or a command line has tests that exercise its main user
journeys end to end, against real dependencies or declared stand-ins.
- Evidence: the e2e suite and the journeys it covers, listed in the test
  directory or the docs.
- Checked: the main journeys named in the documentation have a test.

**T5. Lint and format enforced.** A linter and a formatter run in CI and fail
the build on warnings, or the exceptions are listed in the configuration with
a reason.
- Evidence: CI configuration, lint configuration.
- Checked: the CI step exists and the default branch passes it.

**T6. Dependency hygiene and provenance.** Lockfiles are committed.
Dependencies are checked for known vulnerabilities and for licence
compatibility in CI or before every release. Every new dependency is checked
against the registry before merge, by a tool, an agent or a human: it exists
under exactly that name, it is established (age, maintainers, download trend)
or the reason for taking a young one is stated, and it is the package the code
needs. The check is recorded in the commit or pull request. Who or what runs
the check does not matter; that it runs against the registry, and not against
the model's memory, does. This is the answer to hallucinated package names and
to the packages attackers register under them.
- Evidence: the lockfile, the CI step or release checklist and the tool used,
  the recorded checks.
- Checked: the auditor runs the tools, lists dependencies added since the last
  check and reports those without a recorded check.

**T7. Secrets and static analysis.** A secrets scanner runs as a pre-commit
hook, so that a secret never enters the history, and again in CI, so that the
hook cannot be bypassed silently. A static analyser for the language runs in
CI. Findings are fixed, or listed as accepted with a reason in the tool's
configuration.
- Evidence: the pre-commit configuration (for example `.pre-commit-config.yaml`
  with gitleaks or an equivalent), the CI steps, the last run.
- Checked: both steps exist; the auditor runs the secrets scanner over the
  full history and the static analyser over the tree and reports findings not
  listed as accepted.

**T8. Maintainability signals.** The project declares a tool and thresholds for
at least: code duplication, function complexity (cyclomatic or cognitive),
function length, and file length. It runs the tool in CI or at each release
and records the numbers in `audit/status.json` so that the trend is visible
from one check to the next. Thresholds are the project's own; the convention
asks that they are declared, measured and not exceeded. This is the answer to
the growth of duplication and churn in AI-written code, including its everyday
forms: the same logic regenerated in three places, a style rule repeated on
every element instead of a class, a helper rewritten because the model did not
find the existing one, and the single file that grows with every prompt until
it holds the whole program.
- Evidence: the tool output, the numbers in `audit/status.json`.
- Checked: the auditor runs the tool and compares with the thresholds and with
  the previous check.

### Level 3: Audited

**A1. Architecture review.** A versioned report assesses whether the software
is built to be used, not only to demonstrate. It covers module boundaries,
error handling, configuration, persistence and migrations, observability,
failure modes, test strategy, and agreement with the architecture documents of
A6. It judges the design against the usual principles, named so that two
reviews are comparable: one responsibility per module and type; dependencies
pointing inward (domain logic without IO, persistence apart from transport,
authorization as a policy rather than scattered checks); no god object or god
file; no layer, trait or abstraction with a single use that exists for a
future that has not come; names that say what things do. Simplicity wins over
speculative structure: a review flags the missing boundary and the useless one
alike. The verdict is one of `prototype`, `usable with listed gaps`,
`production-grade`. The review is renewed at each minor release or major
structural change, as the declaration states.
- Evidence: `audit/<date>-architecture.md` with findings and decisions.
- Checked: the report exists for the current minor version; every finding has
  a decision (section 7).

**A2. Security audit.** The repository has a `SECURITY.md` with a disclosure
contact, a threat model and the response time of D9. Every security claim made
in `SECURITY.md` or in the documentation points to a test that asserts it. The
audit includes a data exposure check: every data store, table, bucket and
endpoint that holds user data has an authorization rule, and a test that an
unauthorized caller is refused. Because security degrades as code is
iterated, the claims are re-verified at each release. The result is a
versioned report.
- Evidence: `SECURITY.md`, `audit/<date>-security.md`.
- Checked: each claim has a test that exists, asserts the claim and passes;
  every user-data path has an authorization test; the scans of T6 and T7 ran;
  findings have decisions.

**A3. Blind usability test.** When the software has user documentation and is
meant to be used by people other than its authors (libraries, command-line
tools, APIs, applications), an AI tester with no access to the source code uses
it from the documentation alone to build or do something real, and writes a
friction report. The tester's blindness is enforced by the environment it runs
in, not by an instruction. The tester installs the software the way the
documentation says, so an installation that does not work, or a dependency
that does not exist, is found here as well. The report follows the format in
section 8.
- Evidence: `audit/<date>-blind-<campaign>.md` with the brief, the friction
  log, the verdict and the decisions.
- Checked: a campaign exists for the current minor version if the declaration
  says one is required at each minor; every finding has a decision; the report
  states how blindness was enforced.

**A4. Audit index.** `audit/README.md` lists every report: date, commit, kind,
tool and model that produced it, human who commissioned and read it, verdict,
number of open findings. Reports are not rewritten after the fact. Decisions
are appended.
- Evidence: `audit/README.md`, `audit/status.json`.
- Checked: every file in `audit/` is listed; every listed commit exists;
  commits and dates in the index match the report headers and the status file.

**A5. Release rule.** No release is published with an open finding of
severity `BLOCKER` or `DOC-WRONG` (section 7) in any audit report of the
current minor version. Each release names, in its notes or tag message, the
human maintainer who approved it; that person is listed in `VIBECODING.md`.
- Evidence: the audit index at the release commit, the release notes.
- Checked: the auditor lists open findings by severity at the release tag and
  reads the approver's name.

**A6. Validated architecture documents.** The repository has an architecture
overview (`ARCHITECTURE.md` or a docs page) naming the components, the data
flow, the main invariants and the known weak points, and decision records
(`docs/adr/`) for the structural choices. Each of these documents may be
written by anyone or anything; each carries the name of the human maintainer
who validated it and the date, and is revalidated when the structure changes.
This is the convention's answer to "nobody understands the code": a human has
put their name on a description of the system, and the architecture reviewer
checks that the description is true.
- Evidence: the documents with their validation line.
- Checked: the architecture reviewer compares the overview and the decision
  records to the code; a mismatch is a `DOC-WRONG` finding; a document without
  a validation line, or validated before the last structural change it
  describes, is a finding.

**A7. Mutation score (recommended).** The project runs a mutation testing tool
on its core modules and declares the score and its threshold in
`VIBECODING.md`. Mutation testing measures whether the tests fail when the
code is changed, which is the objective form of T3. It is recommended rather
than required because tooling is uneven across languages.
- Evidence: the tool output, the numbers in `audit/status.json`.
- Checked: where declared, the auditor runs the tool and compares.

**A8. Tooling configuration is code.** Everything that executes with access to
the repository or the developers' machines is versioned, reviewed like code
and pinned: agent instruction files (`AGENTS.md`, `CLAUDE.md`, `.cursorrules`
and the like), agent skills, hooks and plugins, CI actions and workflows,
pre-commit hooks, build scripts, editor tasks. Third-party items are pinned to
a version and listed with their source. None contains a secret. The project
states which sandbox or permission model its agents run under. The same rule
applies to the tooling of a human-led project; the convention names it
because agent tooling is new, runs with broad permissions, and has already
been a vector.
- Evidence: the files and their history.
- Checked: the security auditor reads these files and lists unpinned or
  unreviewed third-party items, secrets, and agents running without a stated
  permission model.

## 6. Declaration file

`VIBECODING.md` is the single place where the project states what it claims.
Its front matter reuses the AI-DECLARATION.md keys, then adds a `convention`
block. The template in `templates/VIBECODING.md` has one field per requirement
that needs a declared value. Keep the body short; the evidence lives elsewhere.
Update the file in the same commit as the change it describes (a new model, a
new threshold, a new status).

```yaml
---
# AI-DECLARATION.md keys (https://github.com/DimwitLabs/AI-DECLARATION.md)
version: "0.1.1"
level: copilot
processes:
  design: pair
  implementation: copilot
  testing: copilot
  documentation: copilot
  review: assist
  deployment: hint
# Vibecoding Convention
convention:
  version: "0.1"
  level: 3
  adopted_at: 03c8df2
  status: active
  tools:
    - "Claude Code:claude-opus-5-5"
    - "Fennec:qwen3.8-27b"
  maintainers: [ikarys]
  review: "single maintainer reads every diff; enforced by Fennec review gates"
  security_response: "72h"
  licence: MIT
  coverage: { threshold: 80, kind: line, tool: cargo-llvm-cov }
  maintainability: { tool: rust-code-analysis, duplication_max: 5, complexity_max: 15, function_max: 60, file_max: 600 }
  mutation: { tool: cargo-mutants, threshold: 70 }
  audit_dir: audit
  audit_cadence: minor
  blind_test: required
  check_interval: 6m
---
```

The README disclosure (D1) is the dynamic badge of D5:

```
[![Vibecoding Convention](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/<owner>/<repo>/main/audit/status.json)](VIBECODING.md)
```

`audit/status.json` is written by the convention check:

```json
{
  "schemaVersion": 1,
  "label": "vibecoding",
  "message": "level 3 · checked 2026-11-08 · 03c8df2",
  "color": "blue",
  "vibecoding": {
    "level": 3,
    "verdict": "pass",
    "date": "2026-11-08",
    "commit": "03c8df2",
    "report": "audit/2026-11-08-convention.md",
    "open_findings": { "BLOCKER": 0, "DOC-WRONG": 0, "PAINFUL": 1, "MINOR": 4 },
    "metrics": { "coverage": 86.2, "duplication": 2.1, "complexity_max": 12, "function_max": 48, "file_max": 512, "mutation": 74 },
    "checked_by": "Claude Code:claude-opus-5-5",
    "read_by": "ikarys"
  }
}
```

Colour: `blue` for `pass`, `yellow` for `pass with findings`, `red` for
`fail`, `lightgrey` for `stale`.

## 7. Findings and decisions

Audit reports use four severities:

| Severity | Meaning |
|---|---|
| `BLOCKER` | A user cannot complete a documented task, or a security claim is false. |
| `DOC-WRONG` | The documentation says something the software does not do. |
| `PAINFUL` | The task can be completed, at a cost a reasonable user would resent. |
| `MINOR` | Everything else worth writing down. |

Every finding gets exactly one decision, written by a human maintainer, in a
"Decisions" table appended to the report:

| Decision | Meaning |
|---|---|
| `fixed` | Changed; the commit is named. |
| `documented` | The behaviour stays; the docs now say so; the commit is named. |
| `won't fix` | The behaviour stays and the docs stay; the reason is given. |
| `open` | Not decided yet. Blocks a release when the severity is `BLOCKER` or `DOC-WRONG`. |

Findings are not deleted or downgraded after the fact. A finding that turns out
to be wrong gets the decision `won't fix` with the reason "not reproduced" or
"tester error".

## 8. Report format

Every report in `audit/` starts with a header and ends with the decisions
table.

Header fields: `date`, `commit`, `kind` (convention, architecture, security,
blind), `tool` and `model` that produced it, `read_by` (the human who
commissioned and read it), `scope`.

Convention, architecture and security reports: a summary paragraph, then one
section per finding with `severity`, `requirement` (the identifier it relates
to), `where` (paths and lines), `problem`, `fix`, then the decisions table.

Blind test reports: the brief (persona, task, allowed inputs, how blindness was
enforced), then
(a) the friction log, one row per event: step, severity, what happened, what
was expected, exact error, suggested fix;
(b) per step, the amount of code written and calls made;
(c) what was simple;
(d) documentation pages found wrong, missing or misleading;
(e) the verdict: a score out of 10 and whether the tester would use the
software for this task;
(f) the decisions table, filled by the maintainer.

Report skeletons are in `templates/`.

## 9. Tooling

The convention can be satisfied by hand. The tooling in this repository
exists so that the same checks are run the same way on every project.

Two kinds of tooling, by what they need:

- **Skills** are procedures loaded into the main AI assistant, which keeps the
  conversation and edits the project. They fit the steps that change the
  repository: bootstrapping the declaration, the disclosure, the audit
  directory, the hooks; writing the decisions table after a human has decided;
  preparing a release check.
- **Agents** are separate assistants with their own context and a restricted
  tool set. They fit the steps that need isolation or must not change
  anything: the blind tester, whose isolation from the source is the whole
  point; the architecture reviewer, the security auditor and the convention
  auditor, which must read and run but never write to the project, and whose
  long reports should not fill the main conversation.

Planned set, for Claude Code and for Fennec:

| Name | Kind | Does |
|---|---|---|
| `vibecoding-init` | skill | Bootstraps a project at the chosen level: declaration, disclosure, `CONTRIBUTING.md` policy, `audit/`, pre-commit secrets hook, commit trailer setup. |
| `vibecoding-release` | skill | Runs the level's checks before a release, fills what is missing, stops on an open blocking finding, records the approver. |
| `convention-auditor` | agent | Checks every requirement of the declared level by running the real commands, writes the convention report and `audit/status.json`. |
| `blind-tester` | agent | Runs a blind campaign in an environment that contains only the user documentation and writes the report, parts (a) to (e). |
| `arch-reviewer` | agent | Writes the architecture review with the three-way verdict and checks the A6 documents against the code. |
| `security-auditor` | agent | Traces claims to tests, runs the data exposure check and the scans, reads the tooling configuration (A8), writes the security report. |

## 10. Related work

The convention reuses what exists and adds the part that did not.

- Disclosure formats: [AI-DECLARATION.md](https://github.com/DimwitLabs/AI-DECLARATION.md),
  whose keys D2 reuses; [ai-contribution-level](https://github.com/Essk/ai-contribution-level),
  [ai-badge](https://github.com/jacksonjost/ai-badge),
  [AI Influence Level](https://danielmiessler.com/blog/ai-influence-level-ail).
  All are self-declared and say so. The convention adds the evidence and the
  check behind the declaration.
- Commit trailers: `Assisted-by:` as adopted by the Linux kernel, Fedora, LLVM,
  OpenInfra, OpenTelemetry and Rocky Linux; `Generated-by:` as used by the
  Apache Software Foundation and OpenInfra. D3 adopts them unchanged.
- Repository standards: the [Vibe Coding Repository Standard](https://github.com/cz1993/vibe-coding-repository-standard)
  organises the governance files of an AI-built repository and separates the
  executor from the reviewer, which D4 also requires. It does not cover
  disclosure, attribution, coverage, blind tests or versioned audits.
- Project AI policies: the studies [arXiv 2609.07542](https://arxiv.org/html/2609.07542)
  and [arXiv 2609.07919](https://arxiv.org/pdf/2609.07919) describe how open
  source projects regulate incoming AI contributions. D8 asks a vibecoded
  project to have such a policy and to respect those of others.

## 11. Non-goals

- The convention does not say whether a project should use AI, nor whether
  vibecoding is faster or cheaper. It says whether the result is sound.
- It is not a certification and has no authority. It is a declaration and the
  evidence behind it.
- It does not ask for continuous surveillance of every commit by an auditor.
  Audits happen at releases and at structural changes, as the project declares.
- It does not require that any artifact be written by a human. It requires
  that a human validated it and put their name on it.
- It does not address the effect of AI assistance on developers' skills. That
  is a question for education and for teams, not for an artifact standard.
- It does not replace the project's own engineering judgement. Where a
  requirement makes no sense for a project, the declaration says so and why,
  and the auditor reports the exception rather than the requirement.

## 12. Glossary

- **AI-written**: produced by an AI assistant and accepted by a human, with or
  without edits.
- **Human-written**: typed by a human without AI assistance for that change.
- **Validated**: read by a named human maintainer who confirms it is true and
  puts their name and the date on it.
- **Blind**: the tester has no access to the source code, tests, internal
  documentation or history of the project, and that access is prevented by
  the environment, not by instruction.
- **Finding**: one observed problem, with a severity and a decision.
- **Audit**: a report produced by a human or an agent at a given commit and
  kept in the repository unchanged.
- **Check**: the run of the convention auditor that produces
  `audit/status.json`.

## 13. Open questions

These are being decided for version 0.1. Opinions welcome.

1. Is 80 % the right recommended coverage threshold, and should it be line
   coverage or branch coverage?
2. Should level 3 require a blind test for all software with user docs, or
   only for libraries and tools, where the docs are the product?
3. Should T8 name default thresholds for duplication and complexity, or leave
   them entirely to the project?
4. Should A7 (mutation score) become required at level 3 once tooling is
   common in the main languages?
5. Should the convention define a level 0, "Undeclared", or is the absence of
   `VIBECODING.md` statement enough?
