---
date: <YYYY-MM-DD>
commit: <hash>
kind: blind-<campaign>
tool: <Tool>
model: <model>
scope: <the app or task the tester had to build or do>
verdict: <score>/10
---

# Blind test: <campaign>

## Brief

- Persona: <who the tester plays, and their assumed experience>
- Task: <what to build or do, in a few lines>
- Allowed inputs: <the user docs copied, the README, the published package or built API docs>
- How blindness was enforced: <the environment: a scratch directory holding only the docs copy, the project reachable only as an installed package or a path dependency whose files were not opened; or the tool's sandbox rule>
- Versions: <database, runtime, tool versions>

## (a) Friction log

| # | Step | Severity | What happened | Expected | Exact error | Suggested fix |
|---|---|---|---|---|---|---|
| 1 | <install> | <BLOCKER / DOC-WRONG / PAINFUL / MINOR> | | | | |

## (b) Effort per step

| Step | Lines of code written | Calls or queries made |
|---|---|---|
| | | |

## (c) What was simple

- ...

## (d) Documentation pages wrong, missing or misleading

- `<page>`: <what is wrong>

## (e) Verdict

Score: <N>/10. Would the tester use this software for this task: <yes | no | with reservations>, because <one sentence>.

## (f) Decisions

Filled by the maintainer. One row per finding of (a).

| # | Decision | Commit or reason |
|---|---|---|
| 1 | | |
