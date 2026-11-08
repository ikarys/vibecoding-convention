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

## 2. Principles

- **Transparency over purity.** Say what the AI wrote. Nobody is asked to hide,
  limit or avoid AI assistance.
- **Evidence over claims.** Every requirement names an artifact in the
  repository or a command anyone can run. A claim without evidence does not
  count.
- **Proportionality.** The convention has levels. A project picks the level
  that fits its purpose and proves it. No level asks for more control than a
  human-led project of the same size and purpose would have.
- **Humans decide.** AI proposes. A named human reviews, decides and is
  accountable. This holds for code, for audit findings and for releases.
- **Nothing silent.** AI-written changes are attributed where they land: in the
  README, in the commit history, in the audit reports.

## 3. Levels

| Level | Name | Means |
|---|---|---|
| 1 | Declared | The project says it is AI-assisted, with which tools and models, and who reviews. |
| 2 | Tested | Level 1, plus automated tests that assert behaviour, a declared and met coverage threshold, green CI, lint and dependency checks. |
| 3 | Audited | Level 2, plus versioned audit reports: architecture review, security audit, blind usability test when applicable, each finding with a decision. |

A project declares one level in `VIBECODING.md`. Each level includes the
requirements of the levels below it. A project may satisfy some requirements of
a higher level without claiming that level.

Suggested fit:

- Level 1: scripts, prototypes, experiments, anything that says "do not use in
  production".
- Level 2: libraries, tools and services that other people depend on.
- Level 3: anything that handles other people's data, money or safety, and any
  project that wants its trustworthiness to be checked rather than believed.

## 4. Requirements

Each requirement has an identifier, a statement, the evidence expected, and how
it is checked. "Checked" says what an auditor, human or agent, does to verify
it.

### Level 1: Declared

**D1. README disclosure.** The README has a section, or a badge linking to
`VIBECODING.md`, that says the project is wholly or partly written with AI
assistance.
- Evidence: the README section or badge.
- Checked: the section exists and links to `VIBECODING.md`.

**D2. Declaration file.** The repository root has a `VIBECODING.md` that
states, at least:
- the claimed level;
- the AI tools and models used (for example "Claude Code with Claude Opus 5.5,
  Fennec with a local Qwen model"), kept current;
- the share of the code that is AI-written, as an honest estimate ("most of
  it", "about half", "the tests only");
- the human maintainers who review and decide, by name or handle;
- the review policy (requirement D4);
- for level 2 and above: the coverage threshold and tool (T2);
- for level 3: the path of the audit directory and the date and commit of the
  last audit of each kind.
- Evidence: the file. A template is in `templates/VIBECODING.md`.
- Checked: every field is present and the stated values match the repository
  (tools named in commit trailers, audit dates in `audit/README.md`).

**D3. Commit co-signature.** Every commit that contains AI-written content
carries a `Co-Authored-By:` trailer naming the model, in the form
`Co-Authored-By: <Model name> <noreply@vendor.example>`. The tool used may be
named in an `AI-Tool:` trailer. A commit without a trailer is understood as
human-written. The trailer format follows the one Git and code forges already
display, so no new tooling is needed to read it.
- Evidence: the git history from the commit that adopted the convention, named
  in `VIBECODING.md`.
- Checked: sample commits since adoption. Commits with substantial changes and
  no trailer are a finding if the declaration says the project is mostly
  AI-written. The exact share is reported, not judged against a threshold.

**D4. Human review before merge.** No AI-written change reaches the default
branch without a human having read it and approved it. For a team this is a
reviewed pull request. For a single maintainer this is the maintainer's own
reading of every diff, stated in `VIBECODING.md`, and a tool or workflow that
makes that reading a step rather than an intention (review gates, a
pull-request flow against oneself, a signed-off trailer).
- Evidence: the review policy in `VIBECODING.md`, branch protection or pull
  request history where available, review-gate configuration of the tool.
- Checked: the policy exists; the history shows no AI-signed commit merged
  without a review event where the platform records one.

### Level 2: Tested

**T1. Tests run in CI on every change.** Automated tests exist and run on every
push or pull request. The default branch is green.
- Evidence: the CI configuration and the latest run on the default branch.
- Checked: the CI job runs the test command; the last run passed.

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
test means the software is wrong, not that the test is flaky.
- Evidence: the test suite itself.
- Checked: the auditor samples tests, in proportion to the suite size, and
  reports tests without assertions, tautological assertions, disabled tests
  without reason, and tests that pass with the implementation stubbed out.
  This is a judgement; the report says which tests were read.

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

**T6. Dependency hygiene.** Dependencies are checked for known
vulnerabilities and for licence compatibility, in CI or before every release.
- Evidence: the CI step or the release checklist, and the tool used.
- Checked: the auditor runs the tool and reports the result.

### Level 3: Audited

**A1. Architecture review.** A versioned report assesses whether the software
is built to be used, not only to demonstrate: module boundaries, error
handling, configuration, persistence and migrations, observability, failure
modes, test strategy. The verdict is one of `prototype`, `usable with listed
gaps`, `production-grade`. The review is renewed at each minor release or
major structural change, as the declaration states.
- Evidence: `audit/<date>-architecture.md` with findings and decisions.
- Checked: the report exists for the current minor version; every finding has
  a decision (section 6).

**A2. Security audit.** The repository has a `SECURITY.md` with a disclosure
contact. Every security claim made in `SECURITY.md` or in the documentation
points to a test that asserts it. Secrets scanning and dependency
vulnerability checks have run. The result is a versioned report.
- Evidence: `SECURITY.md`, `audit/<date>-security.md`.
- Checked: each claim has a test that exists, asserts the claim and passes; the
  scans ran; findings have decisions.

**A3. Blind usability test.** When the software has user documentation and is
meant to be used by people other than its authors (libraries, command-line
tools, APIs, applications), an AI tester with no access to the source code uses
it from the documentation alone to build or do something real, and writes a
friction report. The tester's blindness is enforced by the environment it runs
in, not by an instruction. The report follows the format in section 7.
- Evidence: `audit/<date>-blind-<campaign>.md` with the brief, the friction
  log, the verdict and the decisions.
- Checked: a campaign exists for the current minor version if the declaration
  says one is required at each minor; every finding has a decision; the report
  states how blindness was enforced.

**A4. Audit index.** `audit/README.md` lists every report: date, commit, kind,
tool and model that produced it, verdict, number of open findings. Reports are
not rewritten after the fact. Decisions are appended.
- Evidence: `audit/README.md`.
- Checked: every file in `audit/` is listed; every listed commit exists;
  commits and dates in the index match the report headers.

**A5. Release rule.** No release is published with an open finding of
severity `BLOCKER` or `DOC-WRONG` (section 6) in any audit report of the
current minor version.
- Evidence: the audit index at the release commit.
- Checked: the auditor lists open findings by severity at the release tag.

## 5. Declaration file

`VIBECODING.md` is the single place where the project states what it claims.
The template in `templates/VIBECODING.md` has one field per requirement that
needs a declared value. Keep it short; the evidence lives elsewhere. Update it
in the same commit as the change it describes (a new model, a new threshold, a
new audit).

The README disclosure (D1) can be a badge:

```
[![Vibecoding Convention: level 3](https://img.shields.io/badge/vibecoding-level%203-blue)](VIBECODING.md)
```

## 6. Findings and decisions

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

## 7. Report format

Every report in `audit/` starts with a header and ends with the decisions
table.

Header fields: `date`, `commit`, `kind` (architecture, security, blind),
`tool` and `model` that produced it, `author` (the human who commissioned and
read it), `scope`.

Architecture and security reports: a summary paragraph, then one section per
finding with `severity`, `where` (paths and lines), `problem`, `fix`, then the
decisions table.

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

## 8. Tooling

The convention can be satisfied by hand. The tooling in this repository
exists so that the same checks are run the same way on every project.

Two kinds of tooling, by what they need:

- **Skills** are procedures loaded into the main AI assistant, which keeps the
  conversation and edits the project. They fit the steps that change the
  repository: bootstrapping `VIBECODING.md`, the README disclosure and the
  `audit/` directory; writing the decisions table after a human has decided;
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
| `vibecoding-init` | skill | Bootstraps a project: declaration, disclosure, audit directory, commit trailer setup, chosen level. |
| `vibecoding-release` | skill | Runs the level's checks before a release, fills what is missing, stops on an open blocking finding. |
| `convention-auditor` | agent | Checks every requirement of the declared level, running the real commands, and writes the convention report. |
| `blind-tester` | agent | Runs a blind campaign in an environment that contains only the user documentation and writes the report, parts (a) to (e). |
| `arch-reviewer` | agent | Writes the architecture review with the three-way verdict. |
| `security-auditor` | agent | Traces claims to tests, runs scans, writes the security report. |

## 9. Non-goals

- The convention does not say whether a project should use AI. It says how to
  show the result is sound when it does.
- It is not a certification and has no authority. It is a declaration and the
  evidence behind it.
- It does not ask for continuous surveillance of every commit by an auditor.
  Audits happen at releases and at structural changes, as the project declares.
- It does not replace the project's own engineering judgement. Where a
  requirement makes no sense for a project, the declaration says so and why,
  and the auditor reports the exception rather than the requirement.

## 10. Glossary

- **AI-written**: produced by an AI assistant and accepted by a human, with or
  without edits.
- **Human-written**: typed by a human without AI assistance for that change.
- **Blind**: the tester has no access to the source code, tests, internal
  documentation or history of the project, and that access is prevented by
  the environment, not by instruction.
- **Finding**: one observed problem, with a severity and a decision.
- **Audit**: a report produced by a human or an agent at a given commit and
  kept in the repository unchanged.

## 11. Open questions

These are being decided for version 0.1. Opinions welcome.

1. Should D3 require a trailer on every AI-written commit, or accept a single
   statement in `VIBECODING.md` for projects that are entirely AI-written?
2. Is 80 % the right recommended coverage threshold, and should it be line
   coverage or branch coverage?
3. Should level 3 require a blind test for all software with user docs, or
   only for libraries and tools, where the docs are the product?
4. Should `AI-Tool:` be a required trailer, so that tools are traceable as well
   as models?
5. Should there be a level 0, "Undeclared", to name projects that make no
   statement, or is the absence of `VIBECODING.md` statement enough?
