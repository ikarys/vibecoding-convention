---
name: code-audit
description: Use when the user asks to audit a project's code quality or architecture, asks whether it is clean, modular, simple, secure or "expert-level", mentions god files, duplication, SOLID, KISS or clean code, or wants the architecture and security reports the Vibecoding Convention requires (A1, A2).
---

# Code audit

One pass answering: is this project clean, modular, secure and simple, at expert level? You
measure first, then send reviewers in parallel, then consolidate into the two reports the
convention expects: `<date>-architecture.md` (A1) and `<date>-security.md` (A2). You never fix,
commit or push: the human reads, fills `read_by` and the decisions, and decides.

## 1. Frame (no question unless blocked)

- Root: current repo. Detect backend dirs (`src/`, `crates/`, `server/`, `api/`...) and frontend dir
  (`web/`, `frontend/`, `app/`, `client/`...). No frontend → skip the frontend reviewer.
- Read the project's rules: `VIBECODING.md`, `AGENTS.md`, `CLAUDE.md`, `ARCHITECTURE.md`, `docs/`,
  `SECURITY.md`. Declared rules are the yardstick; the defaults below fill gaps.
- Audit directory: the one `VIBECODING.md` declares, else `audit/`.
- Find the previous `*-architecture.md` there, if any, to report what changed.

## 2. Measure (deterministic, before any model judgement)

Write raw output to `/tmp/code-audit-<repo>/`. Skip a step whose tool is missing and say so.

- Biggest files: `git ls-files | grep -E '\.(rs|ts|tsx|js|jsx|vue|svelte|py|go|java|kt|cs|css|scss)$' | xargs wc -l | sort -rn | head -30`.
  Over 400 lines (code) or 300 (component/style) = god-file candidate. Use the project's T8
  thresholds instead when `VIBECODING.md` declares them.
- Duplication: when `VIBECODING.md` declares a maintainability evidence file (T8), read it and skip jscpd.
  Else: `npx --yes jscpd@5.4.3 --silent --min-lines 8 --reporters json --output /tmp/code-audit-<repo>/jscpd --ignore "**/node_modules/**,**/target/**,**/dist/**,**/build/**,**/*.lock,**/*.test.*" <dirs>`.
  Keep the duplication % and the 10 largest clones.
- The project's own gate (`just check`, `just lint`, `npm run lint`, `cargo clippy -- -D warnings`...),
  only if it runs in under ~5 minutes. Record pass/fail, not the full log.
- Catch-all names: `git ls-files | grep -iE '/(utils?|helpers?|misc|common)(/|\.)'`.

Dependency audit, secrets and static analysis belong to the security reviewer; do not run them here.

## 3. Review (parallel subagents, one message)

Pass each the root, the scope, the project rules found, and the metrics directory.

- `vibecoding-security`: whole repo, commands allowed for the project's declared read-only checks
  (audit, scanners, targeted tests). It writes `<audit-dir>/<date>-security.md` itself in its own
  format; do not restate its steps, do not duplicate its findings elsewhere. Note `tool` and `model`
  are yours to fill from the session; `read_by` stays empty.
- `vibecoding-architecture`: backend scope, findings returned in the answer, no file. Add: SOLID,
  KISS, clean architecture (dependencies inward, domain without IO, persistence apart from
  transport), one responsibility per file, functions over duplication (use the jscpd clones), naming.
- `frontend-architecture`: frontend scope, findings returned, no file, with the size and jscpd
  results for that dir.
- Simplicity pass (`ponytail:ponytail-audit` style: speculative layers, single-use abstractions,
  wrappers, dependencies doing what the platform does): do it yourself from the metrics and the
  reviewers' output; no extra agent.

## 4. Consolidate

Open the cited lines of every BLOCKER and DOC-WRONG; drop or downgrade what does not hold. Merge
duplicates across reviewers. Write `<audit-dir>/<YYYY-MM-DD>-architecture.md`:

```
---
date:
commit:
kind: architecture
tool:
model:
read_by:
scope:
verdict: prototype | usable with listed gaps | production-grade
---

# Architecture audit

<Summary paragraph: the state of the project, the two or three things that matter most,
two or three reasons for the verdict. Link the security report and quote its verdict.>

## Scores (0-5)
| Axis | Score | Evidence |
| Architecture (layers, SOLID, clean arch) | | |
| Modularity (one responsibility, no god file) | | |
| Duplication / reuse | | jscpd: x % |
| Simplicity (KISS, no speculative layer) | | |
| Frontend components | | or N/A |
| Styling system (shared classes, tokens) | | or N/A |
| Security | | verdict of <date>-security.md |
| Tests and gate | | |

## Metrics
<largest files, duplication %, gate result>

## Findings
### Finding N: <imperative title>
- Severity: BLOCKER | DOC-WRONG | PAINFUL | MINOR
- Axis: one axis of the scores table
- Requirement: A1 | A6 | T8 | ...
- Where: path:line
- Problem:
- Fix:

## Since last audit
<fixed / still open / new, or "first audit">

## Decisions
| # | Decision | Commit or reason |
```

Each finding names exactly one axis; an axis is scored from its own findings only. Score = worst
finding on the axis: none → 5, MINOR → 4, PAINFUL → 3, one BLOCKER or DOC-WRONG → 2, several → 1
or 0. The Security score is a number computed by the same rule from the security report's findings;
its verdict goes in Evidence. PAINFUL = a maintainer pays on every change (god file, suite copied
per backend, missing boundary); MINOR = local, fixed in one commit without redesign.
At most 25 findings, worst first; the rest in one line. Severities are the convention's
(`BLOCKER`, `DOC-WRONG`, `PAINFUL`, `MINOR`); never `MAJOR`. Leave `read_by` and the Decisions
table empty for the human. Add one row per report to the index table of `<audit-dir>/README.md` (A4), `read_by` empty; columns
may be in any order, vibecheck finds them by header. No index table yet: add the one from the
convention's `templates/audit/README.md`.

## 5. Answer

In the user's language: both report paths, both verdicts, the scores table, the top 5 findings in
one line each, and a reminder that `read_by` and the decisions are theirs to fill. Offer to turn
findings into fixes or backlog items; do nothing more without a yes.
