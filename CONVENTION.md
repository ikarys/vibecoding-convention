# Vibecoding Convention

Version 0.1. Last changed 2026-11-21.

Software written wholly or partly by AI assistants can be as trustworthy as any
other software, provided it is framed, reviewed and audited. This page defines
those three words in checkable terms. Nobody grants the convention; a project
declares a level and keeps the evidence in its own repository, where anyone
can check it.

A maintainer uses it to show quality with proof instead of promises. A user
uses it to check a project before depending on it. A contributor uses it to
get an AI-assisted pull request taken seriously.

Each row below is one sentence. The full statement, evidence and check of
every requirement are in [docs/requirements.md](docs/requirements.md); the
reasons behind the shape of the convention are in
[docs/rationale.md](docs/rationale.md); the criticisms it answers, with
sources, are in [docs/criticisms.md](docs/criticisms.md). Tools read the same
list from [convention.yaml](convention.yaml).

## Principles

- **Transparency over purity.** Say what the AI wrote; nobody is asked to avoid AI.
- **Evidence over claims.** Every requirement names an artifact or a command anyone can run.
- **Proportionality.** A project picks the level that fits and proves it.
- **Humans decide.** AI proposes; a named human validates, decides and is accountable.
- **Outcomes, not forms.** The convention says what must be visible, never which tool to use or how to work.

## Part A: a project

Part A applies to a repository and the people who maintain it. Each level
includes the ones below.

### Levels

| Level | You do | It proves | Costs | Fits |
|---|---|---|---|---|
| 1 **Declared** | Say that AI wrote part of the project, with which tools and models, who decides, how you take contributions and your licence stance. Tag AI-written commits. A human reviews every change. | Nothing is hidden: who wrote what, who is accountable. | An afternoon. Text only, no tool to install. | scripts, prototypes, anything public |
| 2 **Tested** | Level 1, plus tests, coverage, lint, secrets scanning, static analysis and dependency checks in CI, their results committed as evidence behind a dated badge. | The code behaves as claimed and is checked on every change, not once. | A day or two of CI set-up, then it runs itself. | anything others depend on |
| 3 **Audited** | Level 2, plus versioned audits in the repository: architecture review, security audit, blind usability test, validated architecture documents, each finding with a decision. | Someone other than the author looked, and what they found is on record. | A few hours per release, repeated. | other people's data, money or safety |

### Requirements

Bold identifiers in the tables below are the core of a level: the requirements
that make it. The others consolidate it. All are required: a project that
cannot meet one says so, and why, under `Exceptions` in its `VIBECODING.md`.
The convention is not à la carte, but it never asks for silence.

Check: **M** mechanical (a tool verifies it with no judgement), **J** judgement
(a reviewer, human or agent, decides), **M+J** both.

#### Level 1: Declared

| ID | Requirement | Evidence | Check |
|---|---|---|---|
| D1 | The README discloses AI assistance and links to `VIBECODING.md` | README | M |
| **D2** | `VIBECODING.md` declares the AI involvement, convention level, tools and models, maintainers, review policy, status, provenance, adoption commit and thresholds | the file | M |
| **D3** | AI-written commits carry a trailer naming the tool and model, and a model never signs a certificate of origin | git history | M |
| **D4** | No AI-written change reaches the default branch without a human review, and the generator is never the only reviewer | policy, PR history | M+J |
| D7 | The declaration states the licence, the copyright stance on AI-written parts, that no known licence-incompatible reproduction was included, and the safeguards used | the declaration | M |
| D8 | `CONTRIBUTING.md` states the policy for incoming AI contributions, names the project's trailer and asks for C1 to C3 in pull requests | the file, PR template | M |
| D9 | The declaration states the project status and the security response time, matching `SECURITY.md` | the files | M |

#### Level 2: Tested

Level 1, plus:

| ID | Requirement | Evidence | Check |
|---|---|---|---|
| **T1** | Tests run in CI on every change from a clean checkout | CI config | M |
| **T2** | Coverage, measured by a declared tool, is at or above the declared threshold, and CI writes the report to the evidence file | coverage evidence file | M |
| T3 | Tests assert behaviour, no test is skipped without a reason, and every behaviour change lands with its test | test suite, history | J |
| T4 | User-facing software has end-to-end tests of its main journeys | e2e suite | J |
| T5 | Lint and format run in CI and fail on warnings, with exceptions listed with a reason | CI config | M |
| **T6** | The lockfile is committed, vulnerability and licence checks run in CI, and every new dependency is checked against the registry before merge | lockfile, CI, evidence file, history | M |
| **T7** | A secrets scanner and a static analyser run in CI on every change, and findings are fixed or accepted with a reason | CI config | M |
| T8 | Duplication, complexity, function and file length and leftover markers are measured with a declared tool and thresholds, and recorded in the status file | evidence file, status file | M |
| T9 | Dependencies are current, kept current by a declared mechanism, none is unmaintained, and exceptions are declared with a reason | update config, outdated evidence file | M |
| D5 | `audit/status.json` holds the date, commit and verdict of the last check, is written by the check, feeds the badge and marks itself stale | status file, badge | M |
| D6 | A change that deletes, skips or weakens a test states why, and a human approves | git history | M+J |
| D10 | Agent instruction files name the level, link `VIBECODING.md`, give the commands to pass, the rules humans are held to, and what the agent never does alone | the files | M |

#### Level 3: Audited

Level 2, plus:

| ID | Requirement | Evidence | Check |
|---|---|---|---|
| A1 | An architecture review judges the code against named design principles, including signal over noise, and gives a verdict, renewed at each minor or structural change | `audit/*-architecture.md` | J |
| **A2** | `SECURITY.md` names a contact, a response time and a threat model, every security claim is traced to a passing test, and user data paths have authorization tests | `audit/*-security.md` | M+J |
| **A3** | Where the software has user documentation, a tester with no access to the source installs and uses it from the docs alone | `audit/*-blind-*.md` | J |
| **A4** | `audit/README.md` indexes every report with date, commit, kind, tool, reader and verdict, and reports are never rewritten, only appended to | the index | M |
| A5 | No release ships with an open `BLOCKER` or `DOC-WRONG`, and each release names the human who approved it | index at tag, release notes | M |
| A6 | An architecture overview and decision records exist, each validated by a named human with a date, revalidated on structural change, and match the code | the documents | M+J |
| A7 | Mutation score is measured on core modules and at or above the declared threshold (recommended) | mutation evidence file | M |
| A8 | Everything that executes with repository access is versioned, reviewed, pinned, free of secrets, and has a stated permission model | the files | M+J |

## Part B: contributing to other projects

Part B applies to a person or an agent sending one contribution to a project
they do not maintain, whatever that project's own stance. The evidence is in
the contribution itself, and the receiving maintainer is the one who checks
it. A project at any level of Part A makes C1 to C3 the checkboxes of its
pull request template (D8).

| ID | Requirement | Evidence |
|---|---|---|
| C1 | The contribution discloses AI assistance, with tool and model, in its text and as the trailer the target project expects | description, trailers |
| C2 | The contributor knows every change in the diff and can say what it does, why it is there and what it affects | description, review answers |
| C3 | The contribution is complete: a change brings its tests and docs and says what was run, a bug or vulnerability report brings a reproduction | the diff, the description |
| C4 | The target project's contribution guide and AI policy are respected, one change per pull request, no unsolicited generated changes across many repositories | description, contributor's history |
| C5 | The contributor answers review themselves, within the time the project expects, or the contribution may be closed without discussion | the review thread |

## Findings and decisions

Severities are judged from the person affected, a user, a maintainer or an
operator: `BLOCKER` (a documented task cannot be completed, or a security
claim is false), `DOC-WRONG` (the docs say something the software does not
do), `PAINFUL` (possible, at a cost that person would resent every time),
`MINOR` (everything else worth writing down). Every finding gets one decision
by a human maintainer: `fixed`, `documented`, `won't fix` with a reason, or
`open`. Findings are never deleted or downgraded. No release ships with an
open `BLOCKER` or `DOC-WRONG`.

## Non-goals

The convention does not say whether to use AI, nor whether it is faster or
cheaper. It is not a certification. It never requires that a human wrote
something, only that a human validated it. Where a requirement makes no
sense for a project, the declaration says so and why.

## Tooling

The convention can be met by hand. A validator, agents, skills and templates
that apply it from AI coding tools are described in
[docs/tooling.md](docs/tooling.md).
