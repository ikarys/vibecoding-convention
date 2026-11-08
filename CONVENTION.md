# Vibecoding Convention

Version 0.1, draft. Last changed 2026-11-08.

Software written wholly or partly by AI assistants can be as trustworthy as any
other software, provided it is framed, reviewed and audited. This page defines
those three words in checkable terms. It is a self-declaration backed by
evidence kept in the project's own repository; nobody grants it, anyone can
check it. Each requirement answers a criticism actually made of vibecoded
software; the criticisms and their sources are in
[docs/criticisms.md](docs/criticisms.md). The full statement, evidence and
check of every requirement are in [docs/requirements.md](docs/requirements.md).
Tools read the same list from [convention.yaml](convention.yaml).

## Principles

- **Transparency over purity.** Say what the AI wrote; nobody is asked to avoid AI.
- **Evidence over claims.** Every requirement names an artifact or a command anyone can run.
- **Proportionality.** Levels. A project picks the level that fits and proves it.
- **Humans decide.** AI proposes; a named human validates, decides and is accountable. Nothing has to be written by a human; everything has to be validated by one.
- **Nothing silent.** AI-written changes are attributed where they land.
- **Measure the product, not the process.** The convention says what the result must show, not how to work.
- **Outcomes, not forms.** The convention says what must be visible, never which file name, trailer wording or tool to use. Where tools need a form, the project declares the one it uses; the defaults are the common ones.

## Parity with human-led projects

Most requirements below are what a careful human-led project of the same size
is already expected to have, and a human-led project can adopt the convention
unchanged. Five exist only because AI is involved: D1, D2, D3, D7 and A3. They
are the price of a trust deficit, and four of them are text. The parity test
governs the convention's evolution: a requirement that a human-led project
would not be expected to meet is not added unless it is one of those five.

## Levels

| Level | Name | Means |
|---|---|---|
| 1 | Declared | Says it is AI-assisted, with which tools and models, who decides, how it handles contributions, its provenance stance, and when it was last checked. All text. |
| 2 | Tested | Tests that assert behaviour, declared and met coverage, green CI, lint, secrets scanning, static analysis, dependency hygiene and freshness, measured duplication and size. |
| 3 | Audited | Versioned audit reports: architecture, security, blind usability test, validated architecture documents, each finding with a decision. |

Each level includes the ones below. Suggested fit: level 1 for scripts and
prototypes, level 2 for anything others depend on, level 3 for anything that
handles other people's data, money or safety.

## Requirements

Check: **M** mechanical (a tool verifies it with no judgement), **J** judgement
(a reviewer, human or agent, decides), **M+J** both.

| ID | Level | Requirement | Evidence | Check |
|---|---|---|---|---|
| D1 | 1 | README discloses AI assistance and links to `VIBECODING.md` | README | M |
| D2 | 1 | `VIBECODING.md` declares AI involvement (AI-DECLARATION.md keys), convention level, tools and models, maintainers, review policy, status, provenance, adoption commit, thresholds | the file | M |
| D3 | 1 | AI-written commits carry a machine-readable trailer naming the tool and model (`Assisted-by:`, `Generated-by:`, `Co-Authored-By:` or another the project declares); a model never signs a certificate of origin | git history | M |
| D4 | 1 | No AI-written change reaches the default branch without a human review step; the generator is never the only reviewer | policy, PR history, review gates | M+J |
| D5 | 1 | `audit/status.json` holds the date, commit and verdict of the last check, feeds the README badge, and marks itself stale after the declared interval or a release | status file, badge | M |
| D6 | 1 | A change that deletes, skips or weakens a test states why, and a human approves | git history | M+J |
| D7 | 1 | Declaration states licence, copyright stance on AI-written parts, no known licence-incompatible reproduction, safeguards used | the declaration | M |
| D8 | 1 | `CONTRIBUTING.md` states the policy for incoming AI contributions and asks for C1 to C3 in pull requests; outgoing contributions follow Part B | the file, PR template | M |
| D9 | 1 | Declaration states project status (active, maintenance, archived) and security response time, matching `SECURITY.md` | the files | M |
| D10 | 1 | Where the project has agent instruction files (whatever their name), they name the level, the commands to pass, the edit-time rules and what the agent never does alone | the files | M |
| T1 | 2 | Tests run in CI on every change from a clean checkout; default branch green | CI config, last run | M |
| T2 | 2 | Coverage measured by a declared tool is at or above the declared threshold (80 % recommended) | coverage report | M |
| T3 | 2 | Tests assert behaviour; no unexplained skips; every behaviour change lands with its test | test suite, history | J |
| T4 | 2 | User-facing software has end-to-end tests of its main journeys | e2e suite | J |
| T5 | 2 | Lint and format run in CI and fail on warnings, exceptions listed with a reason | CI config | M |
| T6 | 2 | Lockfile committed; vulnerability and licence checks run; every new dependency is checked against the registry before merge and the check is recorded | lockfile, CI, history | M |
| T7 | 2 | Secrets scanner in pre-commit and in CI; static analyser in CI; findings fixed or accepted with a reason | hook config, CI | M |
| T8 | 2 | Duplication, complexity, function length and file length measured with declared tool and thresholds; numbers recorded in the status file | tool output | M |
| T9 | 2 | Dependencies are current: new ones taken at the registry's current version, an update mechanism in place, lag measured against declared limits, no abandoned dependency; exceptions declared with a reason | update config, outdated report, declaration | M |
| A1 | 3 | Architecture review against named design principles with verdict `prototype`, `usable with listed gaps` or `production-grade`; renewed at each minor or structural change | `audit/*-architecture.md` | J |
| A2 | 3 | `SECURITY.md` with contact, threat model, response time; every security claim traced to a passing test; every user-data path has an authorization test; re-verified at each release | `audit/*-security.md` | M+J |
| A3 | 3 | Blind usability test: an AI with no access to the source installs and uses the software from the docs alone; blindness enforced by the environment | `audit/*-blind-*.md` | J |
| A4 | 3 | `audit/README.md` indexes every report with date, commit, kind, tool, reader, verdict; reports never rewritten, decisions appended | the index | M |
| A5 | 3 | No release with an open `BLOCKER` or `DOC-WRONG`; each release names the human who approved it | index at tag, release notes | M |
| A6 | 3 | Architecture overview and decision records exist, each validated by a named human with a date, revalidated on structural change, and match the code | the documents | M+J |
| A7 | 3 | Mutation score measured on core modules and at or above the declared threshold (recommended) | tool output | M |
| A8 | 3 | Everything that executes with repository access (agent configs, skills, hooks, CI actions, scripts) is versioned, reviewed, pinned, secret-free, with a stated permission model | the files | M+J |

## Part B: contributing to other projects

Most of the harm attributed to vibecoding is done not by projects but by
individual contributions: a generated pull request sent unread, a bug report
or a vulnerability report that nobody reproduced. Part A applies to a
repository; Part B applies to a person or an agent sending one contribution
to a project they do not maintain, whatever that project's own stance. The
evidence is in the contribution itself, and the receiving maintainer is the
one who checks it. A project at any level of Part A makes C1 to C3 the
checkboxes of its pull request template (D8).

| ID | Requirement | Evidence |
|---|---|---|
| C1 | The pull request or report discloses AI assistance, with tool and model, in its text and as the trailer the target project expects (the D3 defaults otherwise) | description, trailers |
| C2 | The contributor has read every line of the diff and can explain each change when asked | a statement in the description, answers in review |
| C3 | The contribution is complete: a behaviour change comes with its tests and with the documentation it affects; the description says what was run and shows the result; a bug report was reproduced, with exact steps, versions and output; a vulnerability report has a working reproduction and no speculative claim | the diff, the description |
| C4 | The target project's contribution guide and AI policy are read and respected; a project that refuses AI-assisted contributions receives none; one change per pull request; no unsolicited generated changes sent across many repositories | the description, the contributor's history |
| C5 | The contributor answers review themselves, within the time the project expects; an AI-assisted contribution whose author does not answer may be closed without discussion | the review thread |

Parity: a human contributor is already expected to do all of this. C1 is the
only AI-specific line.

## Findings and decisions

Severities: `BLOCKER` (a documented task cannot be completed, or a security
claim is false), `DOC-WRONG` (the docs say something the software does not
do), `PAINFUL`, `MINOR`. Every finding gets one decision by a human
maintainer: `fixed` (commit named), `documented` (commit named), `won't fix`
(reason given), `open`. Findings are never deleted or downgraded. No release
ships with an open `BLOCKER` or `DOC-WRONG`.

## Tooling

The convention can be met by hand. The tooling in this repository runs the
same checks the same way everywhere:

- `vibecheck`, a command-line validator with no AI in it, runs every **M**
  check locally, in pre-commit and in CI, and writes `audit/status.json`.
- Agents (`vibecoding-audit`, `vibecoding-blind-test`,
  `vibecoding-architecture`, `vibecoding-security`) do the **J** checks and
  write the reports. They read `vibecheck`'s output rather than redoing it.
- Skills (`vibecoding-init`, `vibecoding-release`) bootstrap a project and
  prepare a release; `vibecoding-contribute` checks a contribution against
  Part B before it is sent.
- A ten-line block in the agent instruction file (D10) is all that sits in a
  model's context during ordinary work.

## Related work

Disclosure formats ([AI-DECLARATION.md](https://github.com/DimwitLabs/AI-DECLARATION.md),
whose keys D2 reuses, and several badge schemes) are self-declared and say so;
the convention adds the evidence behind the declaration. The `Assisted-by:` and
`Generated-by:` trailers come from the Linux kernel, Fedora, LLVM, OpenInfra,
OpenTelemetry and the Apache Software Foundation; D3 accepts them, and any
other trailer a project declares, without taking a side. The
[Vibe Coding Repository Standard](https://github.com/cz1993/vibe-coding-repository-standard)
organises an AI-built repository's governance files; it does not cover
disclosure, attribution, coverage, blind tests or versioned audits. Studies of
open source AI policies ([arXiv 2609.07542](https://arxiv.org/html/2609.07542),
[arXiv 2609.07919](https://arxiv.org/pdf/2609.07919)) describe how projects
regulate incoming AI contributions; D8 asks a vibecoded project to have such a
policy and respect those of others.

## Non-goals

The convention does not say whether to use AI, nor whether it is faster or
cheaper. It is not a certification. It does not watch every commit; audits
happen at releases and structural changes. It never requires that a human
wrote something, only that a human validated it. It does not address the
effect of AI on developers' skills. Where a requirement makes no sense for a
project, the declaration says so and why, and the auditor reports the
exception rather than the requirement.
