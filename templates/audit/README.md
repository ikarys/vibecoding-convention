# Audit index

Every report under `audit/` is listed here. Reports are never rewritten after
the fact; decisions are appended to them. `status.json` is written by
`vibecheck` and feeds the README badge.

| Date | Commit | Kind | Tool / model | Read by | Verdict | Open findings |
|---|---|---|---|---|---|---|
| <YYYY-MM-DD> | <hash> | convention | vibecheck <version> | <maintainer> | pass | 0 |
| <YYYY-MM-DD> | <hash> | architecture | <Tool>:<model> | <maintainer> | usable with listed gaps | 2 |
| <YYYY-MM-DD> | <hash> | security | <Tool>:<model> | <maintainer> | pass with findings | 1 |
| <YYYY-MM-DD> | <hash> | blind-<campaign> | <Tool>:<model> | <maintainer> | 7/10 | 3 |

Kinds: `convention`, `architecture`, `security`, `blind-<campaign>`.
File names: `<date>-<kind>.md`, for example `2026-11-08-architecture.md`.
