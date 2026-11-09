# Requirements in detail

The normative one-page text is [CONVENTION.md](../CONVENTION.md). This page
gives, for each requirement, the full statement, the evidence expected and how
it is checked. Identifiers are stable: a requirement is never renumbered.
[convention.yaml](../convention.yaml) carries the same list for tools.

## Part A: a project

Part A applies to a repository and its maintainers: three levels, the
requirements below. Identifiers are stable.

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
keys for the degree of AI involvement (`version`, `level`, `processes`, and
`components` where the project uses it), so that the same information is written once and existing
badges keep working, plus a `convention` block for this convention's fields.
A project that already has an `AI-DECLARATION.md` may point to it instead of
repeating those keys. The Declaration file section gives the format. The file states, at least:
- the AI involvement level per process (design, implementation, testing,
  documentation, review, deployment), in AI-DECLARATION terms;
- the convention level claimed;
- the AI tools and models used, kept current;
- the human maintainers who review, validate and decide, by name or handle;
- the review policy (D4);
- the project status and security response time (D9);
- the provenance and licence statement (D7);
- the commit at which the convention was adopted;
- for level 2 and above: the thresholds and tools of T2, T7 and T8, and the
  evidence files for coverage and audit (`convention.evidence.coverage`,
  `convention.evidence.audit`), plus `outdated` when no update bot is configured;
- for level 3: the audit directory, the audit cadence, and whether a blind
  test applies.
- Evidence: the file. A template is in `templates/VIBECODING.md`.
- Checked: every field is present and the stated values match the repository
  (tools named in commit trailers, audit dates in `audit/status.json`,
  thresholds in CI configuration).

**D3. Commit attribution.** Every commit that contains AI-written content
carries a git trailer that names the tool and the model, so that attribution
can be read by a machine from the history alone. The convention does not pick
the trailer. The project declares the ones it uses in `VIBECODING.md`
(`convention.attribution_trailers`); when it declares none, the common ones are
accepted: `Assisted-by:` and `Generated-by:` (Linux kernel, Fedora, LLVM,
OpenInfra, OpenTelemetry, Apache), `Co-Authored-By:` (the default of several
coding tools), `AI-Tool:`. For the record, `Assisted-by: <Tool>:<model>` is the
form the large projects converged on, and `Co-Authored-By:` has been criticised
for presenting a model as a legal author; that is information for the project,
not a rule of this convention. Two rules only, because they are about honesty
and not form: a trailer that certifies origin or legal responsibility
(`Signed-off-by:` under a Developer Certificate of Origin) names a human, never
a model; and an attribution trailer names a model the project declares (D2),
so the declaration cannot fall behind the history. `Co-Authored-By:` is exempt
from the second rule because it also names humans. A commit without any attribution trailer is understood as
human-written.
- Evidence: the git history from the adoption commit named in `VIBECODING.md`.
- Checked: sample commits since adoption. Substantial commits without a
  trailer are a finding if the declaration says the project is mostly
  AI-written. The share of attributed commits is reported, not judged against
  a threshold. A `Signed-off-by:` naming a model is a failure, and so is an
  attribution trailer other than `Co-Authored-By:` naming no declared model.

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
  open findings by severity, the evidence files it was computed from, and the
  tool and model that ran the check. It is written by `vibecheck
  --write-status`, normally from the project's CI after its own checks passed,
  otherwise by a maintainer. Its
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
  declared, CI runs it and the auditor reads its output.

**D8. Contribution policy.** `CONTRIBUTING.md` states the project's policy for
incoming AI-assisted contributions: accepted with disclosure, accepted with
conditions, or refused. Where accepted, it asks contributors for C1 to C3 of
Part B: disclosure with the project's trailer, a statement that the diff was
read, tests and documentation with the change, and what was run. The pull
request template carries them as checkboxes. Contributions the project makes
to other projects follow Part B.
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

**D10. Agent instructions carry the convention.** Where the project has agent
instruction files, under whatever name its tools use (`AGENTS.md`, `CLAUDE.md`,
`.cursorrules`, `GEMINI.md` and the like, or the paths the project declares in
`convention.agent_instruction_files`), each such file names the convention level and links to
`VIBECODING.md`; gives the build, test, lint and scan commands an agent must
pass before calling a change done; states the edit-time rules (the D3 trailer,
a reason for every test change under D6, the registry check of T6 for new
dependencies and the registry's current version under T9, no secrets, every
input treated as hostile and the project's security checklist followed, and
signal over noise: a comment states a reason, a constraint or a trap, never
what the line does; a docstring says what the caller must know, not the
signature again; the public API is documented; no TODO, placeholder,
commented-out or dead code in a delivered change; documentation proportionate
to the project, a README that says what it is, how to install it, how to use
it and where to go next, and nothing generated to fill space); and lists what
the agent does not do without a human (merge, release, change the CI, delete
data). The noise rules are here because the instruction file is the only
place where noise is prevented rather than reviewed away. This is the only place
where the convention acts while the code is being written rather than at
audit time. Keep the file short enough to be read whole by the model; no size,
no structure and no file name is mandated: tools differ, and the convention has
no opinion on them. A project with no such file is not required to add one.
- Evidence: the file.
- Checked: where present, the file contains the four points and no secret;
  `vibecheck` verifies the declared level, the `VIBECODING.md` link, a
  declared trailer and the declared test command.

### Level 2: Tested

**T1. Tests run in CI on every change.** Automated tests exist and run on every
push or pull request. The default branch is green. The build runs from a clean
checkout with the steps the documentation gives, so that a stranger can build
the project.
- Evidence: the CI configuration and the latest run on the default branch.
- Checked: the CI configuration contains the test command and runs it on
  every push or pull request. Whether the default branch is green is read by
  the auditor on the platform; vibecheck does not check it.

**T2. Coverage at the declared threshold.** `VIBECODING.md` declares a line (or
statement) coverage threshold and the tool that measures it. The measured
coverage is at or above the threshold. The recommended threshold for software
used in production is 80 %. A lower threshold is allowed and must be
explained in one sentence.
- Evidence: the coverage report CI writes to the file named by
  `convention.evidence.coverage`.
- Checked: the coverage tool appears in the CI configuration; CI runs the tool
  and writes `evidence.coverage`; vibecheck reads it and compares the figure to
  the threshold (lcov: LH/LF; cobertura: line-rate; cargo-llvm-cov summary or
  JSON; otherwise the last percentage in the text).

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
  the audit output in `convention.evidence.audit`, the recorded checks.
- Checked: the lockfile is committed; CI or the release checklist contains the
  audit command; CI runs the tool and writes `evidence.audit`, which vibecheck
  reads (any unresolved vulnerability is a failure); vibecheck lists
  dependencies added since the last check and reports those without a recorded
  check.

**T7. Secrets and static analysis.** A secrets scanner runs as a pre-commit
hook, so that a secret never enters the history, and again in CI, so that the
hook cannot be bypassed silently. A static analyser for the language runs in
CI, with a rule set that covers the known vulnerability classes the project
names in its threat model (A2) or, below level 3, the classes relevant to its
kind of software: injection, path traversal, unsafe deserialization, broken
authorization, secrets in code, for most software; the OWASP Top 10 for web
software; the CWE Top 25 as the general list. Findings are fixed, or listed as
accepted with a reason in the tool's configuration. The convention does not
promise the absence of vulnerabilities, which nobody can prove; it asks that
the known classes be looked for by a tool, every time.
- Evidence: the pre-commit configuration (for example `.pre-commit-config.yaml`
  with gitleaks or an equivalent), the CI steps, the last run.
- Checked: both steps exist; vibecheck scans the working tree for secrets
  with its built-in scanner; the secrets scanner and the static analyser run in
  CI, where their findings not listed as accepted fail the job.

**T8. Maintainability signals.** The project declares a tool and thresholds for
at least: code duplication, function complexity (cyclomatic or cognitive),
function length, file length, and leftover markers (`TODO`, `FIXME`, `XXX`,
placeholder text, commented-out code blocks) in delivered code. Where the
ecosystem has a lint for public API documentation (`missing_docs`,
pydocstyle, eslint-plugin-jsdoc and the like), it runs in CI. The project
runs the tools in CI or at each release and records the numbers in
`audit/status.json` so that the trend is visible from one check to the next. Thresholds are the project's own; the convention
asks that they are declared, measured and not exceeded. This is the answer to
the growth of duplication and churn in AI-written code, including its everyday
forms: the same logic regenerated in three places, a style rule repeated on
every element instead of a class, a helper rewritten because the model did not
find the existing one, the single file that grows with every prompt until it
holds the whole program, and the `TODO: implement` a model leaves behind. A
comment density is deliberately not measured: it is a bad metric in both
directions; the signal-over-noise judgement belongs to A1.
- Evidence: `convention.evidence.maintainability`, the numbers in
  `audit/status.json`.
- Checked: the thresholds are declared; vibecheck measures file length,
  function length and leftover markers itself; CI mentions the maintainability
  and documentation lint commands when declared; CI runs the declared tool and
  writes `evidence.maintainability`, which vibecheck records in the status file
  for the agents.

**T9. Dependency freshness.** A model proposes the library versions and APIs
it memorised, not today's, so an AI-written project is behind from its first
day and falls further behind at every regeneration. The rule is not "always
the latest" but "no unjustified lag". Three moments:
- At addition: the version taken is the registry's current one, read from the
  registry and not from memory (this completes the T6 check).
- Over time: an update mechanism exists, either a bot (Dependabot, Renovate or
  an equivalent) or a declared cadence with a declared command (`cargo
  outdated`, `npm outdated`, `pip list --outdated`, and the like), and the
  check measures the lag: the number of dependencies with a newer major or
  minor version available, and the age of the oldest lag. The project declares
  its limits (`convention.freshness`: `max_outdated`, `max_lag_months`).
- Exceptions: a dependency held back for incompatibility or stability is
  listed in `VIBECODING.md` with the reason. That is the human
  counter-indication; it is the only thing that justifies a lag.
No dependency flagged as unmaintained or abandoned by the ecosystem's advisory
database stays without such an exception.
- Evidence: the update configuration or `convention.evidence.outdated`, the
  exceptions, `metrics.outdated` in `audit/status.json`.
- Checked: an update bot configuration exists or `evidence.outdated` is
  declared (neither is a failure); CI runs the outdated tool and writes
  `evidence.outdated`; vibecheck reads it, counts the outdated dependencies not
  listed under "Dependencies held back" against `max_outdated` and records
  `metrics.outdated`; unmaintained or yanked items in `evidence.audit` without
  a held-back entry are failures.

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
future that has not come; names that say what things do; and signal over
noise: comments that state a reason rather than paraphrase the line,
docstrings that tell the caller something the signature does not, no dead or
commented-out code, no defensive handling of errors that cannot happen. A
comment that paraphrases, a docstring that restates the signature, dead code
or needless defensiveness are `MINOR` findings; a comment that contradicts the
code is `DOC-WRONG`. Simplicity wins over speculative structure: a review
flags the missing boundary and the useless one alike. The verdict is one of `prototype`, `usable with listed gaps`,
`production-grade`. The review is renewed at each minor release or major
structural change, as the declaration states.
- Evidence: `audit/<date>-architecture.md` with findings and decisions.
- Checked: the report exists for the current minor version; every finding has
  a decision (Findings and decisions).

**A2. Security audit.** The repository has a `SECURITY.md` with a disclosure
contact, the response time of D9, and a threat model that names the known
vulnerability classes relevant to this kind of software (from the OWASP Top
10, the CWE Top 25, or the ecosystem's own list) and, for each, the tool rule
or the test that covers it; a class with no coverage is a finding. Every
security claim made in `SECURITY.md` or in the documentation points to a test
that asserts it. The audit includes a data exposure check: every data store, table, bucket and
endpoint that holds user data has an authorization rule, and a test that an
unauthorized caller is refused. Because security degrades as code is
iterated, the claims are re-verified at each release. The result is a
versioned report.
- Evidence: `SECURITY.md`, `audit/<date>-security.md`.
- Checked: the threat model names its classes and each has a tool rule or a
  test; each claim has a test that exists, asserts the claim and passes; every
  user-data path has an authorization test; the scans of T6 and T7 ran;
  findings have decisions.

**A3. Blind usability test.** When the software has user documentation and is
meant to be used by people other than its authors (libraries, command-line
tools, APIs, applications), an AI tester with no access to the source code uses
it from the documentation alone to build or do something real, and writes a
friction report. The tester's blindness is enforced by the environment it runs
in, not by an instruction. The tester also judges the documentation itself:
pages that drown the needed information in generated prose, a README that
lists every feature instead of saying how to start, are `PAINFUL` findings;
the right amount of documentation is the amount the task needed. The tester installs the software the way the
documentation says, so an installation that does not work, or a dependency
that does not exist, is found here as well. The report follows the format in
the Report format section.
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
severity `BLOCKER` or `DOC-WRONG` (Findings and decisions) in any audit report of the
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
- Evidence: `convention.evidence.mutation`, the numbers in
  `audit/status.json`.
- Checked: where `convention.mutation` and `evidence.mutation` are declared, CI
  runs the tool and writes the evidence file; vibecheck reads it and compares
  the figure to the threshold.

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

## Part B: contributing to other projects

Part A applies to a repository and its maintainers. Part B applies to anyone,
person or agent, who sends a pull request, an issue or a vulnerability report
to a project they do not maintain. It needs no declaration and has no level:
the evidence is the contribution itself, and the receiving maintainer checks
it. A project under Part A asks for C1 to C3 in its pull request template
(D8); a project that has never heard of the convention can still read them.

**C1. Disclosure.** The pull request or report says that AI assisted in
writing it, with the tool and model, in its text and, for commits, as the
trailer the target project expects; when the project expresses no preference,
the D3 defaults apply. A contribution with AI-written content and no
disclosure is dishonest, whatever its quality.
- Evidence: the description and the commit trailers.

**C2. Read and understood.** The contributor has read the whole diff and
can say, for each change, what it does and why it is there. Understanding the
change is required; understanding every construct of the language is not: a
maintainer who validates a model's Rust without being able to write it by
hand is in the same position as any contributor working in a language they
know less well than the project does. The description says so in one line.
How the description was written does not matter; that it is true does.
- Evidence: the statement, and the answers given in review. A reviewer may
  ask what a change does and why; an answer that does not match the diff
  shows the statement was false, and the project may close the contribution.

**C3. Complete.** A change of behaviour comes with the tests that cover it and
with the documentation it affects, in the same pull request. The description
says what was run (tests, lint, the software itself) and shows the result.
For a bug report: the contributor reproduced the bug and gives the exact steps,
the versions and the output. For a vulnerability report: a working
reproduction, and no claim the contributor has not seen happen; a report that
describes a vulnerability a model imagined is the single most damaging form of
slop, and it is what closed curl's bug bounty.
- Evidence: the diff, the description, the report.

**C4. Scope and policy.** The contributor has read the target project's
contribution guide and its AI policy and follows them; a project that refuses
AI-assisted contributions receives none. One change per pull request. No
unsolicited generated changes sent across many repositories (typo sweeps,
"modernisation", dependency bumps the project did not ask for).
- Evidence: the description names the guide; the contributor's recent
  activity shows no mass sending.

**C5. Follow-up.** The contributor answers review themselves, within the time
the project expects, and does not hand the thread to a model that has not seen
the review. An AI-assisted contribution whose author does not answer may be
closed without discussion; the convention says this is legitimate, so that
maintainers who do it are not accused of hostility to AI.
- Evidence: the review thread.

The skill `vibecoding-contribute` runs C1 to C4 as a pre-flight before a
contribution is sent.

## Declaration file

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
  evidence:                                    # tool outputs written by CI, read by vibecheck; paths relative to the repository root
    coverage: audit/evidence/coverage.txt
    audit: audit/evidence/audit.json
    outdated: audit/evidence/outdated.txt      # optional when an update bot is configured
    maintainability: audit/evidence/maintainability.json   # optional
    mutation: audit/evidence/mutation.txt      # optional, level 3
  mutation: { tool: cargo-mutants, threshold: 70 }
  audit_dir: audit
  audit_cadence: minor
  blind_test: required
  check_interval: 6m
---
```

The `evidence` block names the files in which the project's tools leave their
output. The project's CI runs the tools, writes the outputs there, and then runs
`vibecheck . --write-status`, which reads the files, compares the figures to
the declared thresholds and writes `audit/status.json`; CI commits both back to
the default branch (or a maintainer does). vibecheck never executes the
project's commands, so it is safe to run on any repository. At level 2 and
above `evidence.coverage` and `evidence.audit` are required, `outdated` only
when no update bot is configured. A declared file that is missing or unreadable
fails the requirement it serves; one whose last commit is older than the latest
release tag is stale (a warning). The `commands` block stays: it tells CI
matching and agents and humans what to run.

The README disclosure (D1) is the dynamic badge of D5:

```
[![Vibecoding Convention](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/<owner>/<repo>/main/audit/status.json)](VIBECODING.md)
```

`audit/status.json` is written by `vibecheck --write-status`:

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
    "evidence": { "coverage": "audit/evidence/coverage.txt", "audit": "audit/evidence/audit.json" },
    "checked_by": "vibecheck 0.4.0",
    "read_by": "ikarys"
  }
}
```

Colour: `blue` for `pass`, `yellow` for `pass with findings`, `red` for
`fail`, `lightgrey` for `stale`.

## Findings and decisions

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

## Report format

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

## Glossary

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

## Open questions

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
