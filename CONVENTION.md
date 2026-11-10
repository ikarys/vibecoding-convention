# Vibecoding Convention

Version 0.1, draft. Last changed 2026-11-09.

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
- **Outcomes, not forms.** The convention says what must be visible, never which tool to use or how to work. Where a check needs a form (a trailer wording, a report layout, a file to read), the project declares the one it uses and the defaults are the common ones. A fixed name is kept only where a reader or a tool needs an anchor: the declaration, the status file, the audit index, and the files the hosting platforms already recognise.

## Parity with human-led projects

Most requirements below are what a careful human-led project of the same size
is already expected to have, and a human-led project can adopt the convention
unchanged. Six exist only because AI is involved: D1, D2, D3, D7, D10 and A3.
They are the price of a trust deficit, and all but A3 are text. One step
inside T6, the recorded registry check of a new dependency, is there for the
same reason, and is the one place where the convention asks for a recorded
step rather than a measured result: a hallucinated package name cannot be
measured after the fact, only caught before merge. The parity test governs
the convention's evolution: a requirement that a human-led project would not
be expected to meet is not added unless it is one of those.

## Part A: a project

Part A applies to a repository and the people who maintain it. It has three
levels and twenty-seven requirements. Part B, below, applies to anyone sending
one contribution to a project they do not maintain.

### Levels

| Level | Name | Means |
|---|---|---|
| 1 | Declared | Says it is AI-assisted, with which tools and models, who decides, how it handles contributions, its provenance stance, and when it was last checked. All text. |
| 2 | Tested | Tests that assert behaviour, declared and met coverage, green CI, lint, secrets scanning, static analysis, dependency hygiene and freshness, measured duplication and size. |
| 3 | Audited | Versioned audit reports: architecture, security, blind usability test, validated architecture documents, each finding with a decision. |

Each level includes the ones below. Suggested fit: level 1 for scripts and
prototypes, level 2 for anything others depend on, level 3 for anything that
handles other people's data, money or safety.

### Requirements

Check: **M** mechanical (a tool verifies it with no judgement), **J** judgement
(a reviewer, human or agent, decides), **M+J** both.

| ID | Level | Requirement | Evidence | Check |
|---|---|---|---|---|
| D1 | 1 | README discloses AI assistance and links to `VIBECODING.md` | README | M |
| D2 | 1 | `VIBECODING.md` declares AI involvement (AI-DECLARATION.md keys), convention level, tools and models, maintainers, review policy, status, provenance, adoption commit, thresholds; from level 2, the evidence files for coverage and audit | the file | M |
| D3 | 1 | AI-written commits carry a machine-readable trailer naming the tool and model (`Assisted-by:`, `Generated-by:`, `Co-Authored-By:`, `AI-Tool:` or another the project declares); a model never signs a certificate of origin | git history | M |
| D4 | 1 | No AI-written change reaches the default branch without a human review step; the generator is never the only reviewer | policy, PR history, review gates | M+J |
| D5 | 1 | `audit/status.json` holds the date, commit and verdict of the last check and names the evidence it was computed from; it is written by the check, not by hand, and feeds the README badge; it marks itself stale after the declared interval or a release | status file, badge | M |
| D6 | 1 | A change that deletes, skips or weakens a test states why, and a human approves | git history | M+J |
| D7 | 1 | Declaration states licence, copyright stance on AI-written parts, no known licence-incompatible reproduction, safeguards used | the declaration | M |
| D8 | 1 | `CONTRIBUTING.md` states the policy for incoming AI contributions and, where they are accepted, names the project's trailer and asks for C1 to C3 in pull requests; outgoing contributions follow Part B | the file, PR template | M |
| D9 | 1 | Declaration states project status (active, maintenance, archived) and security response time, matching `SECURITY.md` | the files | M |
| D10 | 1 | Where the project has agent instruction files (whatever their name), they name the level and link `VIBECODING.md`, give the commands to pass before a change is done, the rules the project's humans are held to (attribution, test changes, dependencies, secrets) and what the agent never does alone | the files | M |
| T1 | 2 | Tests run in CI on every change from a clean checkout | CI config | M |
| T2 | 2 | Coverage measured by a declared tool is at or above the declared threshold (80 % recommended); CI writes the report to the declared evidence file | coverage evidence file | M |
| T3 | 2 | Tests assert behaviour; no unexplained skips; every behaviour change lands with its test | test suite, history | J |
| T4 | 2 | User-facing software has end-to-end tests of its main journeys | e2e suite | J |
| T5 | 2 | Lint and format run in CI and fail on warnings, exceptions listed with a reason | CI config | M |
| T6 | 2 | Lockfile committed; vulnerability and licence checks run in CI or before each release and their output is the declared evidence file; every new dependency is checked against the registry before merge and the check is recorded | lockfile, CI, audit evidence file, history | M |
| T7 | 2 | Secrets scanner in CI on every change (a pre-commit hook is recommended); static analyser in CI whose rules cover the vulnerability classes the project names; findings fixed or accepted with a reason | CI, hook config | M |
| T8 | 2 | Duplication, complexity, function length, file length, and leftover markers (TODO, placeholders, commented-out code) measured with declared tool and thresholds; public API documentation linted where the ecosystem has a lint; numbers recorded in the status file | tool output, evidence file | M |
| T9 | 2 | Dependencies are current: new ones taken at the registry's current version, an update mechanism in place (a bot, or a measured lag against a declared limit), no unmaintained dependency; exceptions declared with a reason | update config, outdated evidence file, declaration | M |
| A1 | 3 | Architecture review against named design principles, including signal over noise (comments that say why, no paraphrase, no dead code), with verdict `prototype`, `usable with listed gaps` or `production-grade`; renewed at each minor or structural change | `audit/*-architecture.md` | J |
| A2 | 3 | `SECURITY.md` with contact, response time and a threat model that names the known vulnerability classes relevant to this kind of software, each with the tool or test covering it; every security claim traced to a passing test; when the software holds or serves user data, every such path has an authorization test; re-verified at each release | `audit/*-security.md` | M+J |
| A3 | 3 | Blind usability test, where the software has user documentation: a tester, human or AI, with no access to the source installs and uses the software from the docs alone; blindness enforced by the environment | `audit/*-blind-*.md` | J |
| A4 | 3 | `audit/README.md` indexes every report with at least date, commit, kind, tool, reader, verdict (columns in any order); reports never rewritten, decisions appended | the index | M |
| A5 | 3 | No release with an open `BLOCKER` or `DOC-WRONG`; each release names the human who approved it | index at tag, release notes | M |
| A6 | 3 | Architecture overview and decision records exist where the overview says, each validated by a named human with a date, revalidated on structural change, and match the code | the documents | M+J |
| A7 | 3 | Mutation score measured on core modules and at or above the declared threshold (recommended) | mutation evidence file | M |
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
| C2 | The contributor knows every change in the diff and can say, for each one, what it does, why it is there and what it affects; not how every line of the language works | a statement in the description, answers in review |
| C3 | The contribution is complete: a behaviour change comes with its tests and with the documentation it affects; the description says what was run and shows the result; a bug report was reproduced, with exact steps, versions and output; a vulnerability report has a working reproduction and no speculative claim | the diff, the description |
| C4 | The target project's contribution guide and AI policy are read and respected; a project that refuses AI-assisted contributions receives none; one change per pull request; no unsolicited generated changes sent across many repositories | the description, the contributor's history |
| C5 | The contributor answers review themselves, within the time the project expects; an AI-assisted contribution whose author does not answer may be closed without discussion | the review thread |

Parity: a human contributor is already expected to do all of this. C1 is the
only AI-specific line.

## Findings and decisions

Severities, judged from the person affected (a user, a maintainer, an
operator): `BLOCKER` (a documented task cannot be completed, or a security
claim is false), `DOC-WRONG` (the docs say something the software does not
do), `PAINFUL` (possible, at a cost that person would resent on every
occurrence), `MINOR` (everything else worth writing down). Every finding gets
one decision by a human maintainer: `fixed` (commit named), `documented` (commit named), `won't fix`
(reason given), `open`. Findings are never deleted or downgraded. No release
ships with an open `BLOCKER` or `DOC-WRONG`.

## Tooling

The convention can be met by hand. The tooling in this repository runs the
same checks the same way everywhere:

- `vibecheck`, a command-line validator with no AI in it. It runs none of
  the project's code: it reads the declaration, the repository and the
  evidence files the project's CI wrote, does every **M** check (at level 3,
  the M half of A2, A6 and A8 stays with the agents) and writes
  `audit/status.json`. It is safe to run on any repository, which is the
  point: auditors and blind testers run it on code they do not trust. The
  CI already runs the tools and is the single source of the figures; the
  convention requires that this evidence exists and is current, it does not
  redo the work.
- Agents (`vibecoding-audit`, `vibecoding-blind-test`,
  `vibecoding-architecture`, `vibecoding-security`) do the **J** checks and
  write the reports. They read `vibecheck`'s output rather than redoing it.
- Skills (`vibecoding-init`, `vibecoding-release`) bootstrap a project and
  prepare a release; `vibecoding-contribute` checks a contribution against
  C1 to C4 of Part B before it is sent; `code-audit` measures size and
  duplication, then runs the architecture and security agents and writes
  the A1 and A2 reports.
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
