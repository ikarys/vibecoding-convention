---
date: <YYYY-MM-DD>
commit: <hash>
kind: security
tool: <Tool>
model: <model>
scope: <whole repository, or the change range reviewed>
verdict: <pass | pass with findings | fail>
---

# Security audit

<Summary paragraph: claims checked, tests run, scanners run or not run and
why, data exposure check done or not applicable.>

## Claim traceability

| Claim (from SECURITY.md or docs) | Test | Asserts the claim | Result |
|---|---|---|---|
| <claim> | `path/to/test.rs::name` | yes / no | pass / fail / not run |

## Threat model coverage

| Class | Where it could happen | Covered by (rule or test) | Verified |
|---|---|---|---|
| <class> | <modules> | <rule or `path::test`> or **no coverage** | yes / no |

## Data exposure check

<For a library or a tool that holds no user data: one sentence saying so, no table.>

| Store, table, bucket or endpoint | Authorization rule | Test that an unauthorized caller is refused |
|---|---|---|
| <name> | <where the rule lives> | `path::test` or **missing** |

## Findings

### Finding 1: <imperative title of the fix>
- Severity: <BLOCKER | DOC-WRONG | PAINFUL | MINOR>
- Requirement: <A2 | T6 | T7 | A8>
- Where: `path/to/file:line` (entry → sink for a reachable vulnerability)
- Confirmed by: <path read | `command run` | unconfirmed>
- Problem: <what is wrong and how it is reached>
- Fix: <the concrete change>

## Claims without tests (declared)

- <claims that SECURITY.md openly marks as untested, one per line>

## Decisions

| # | Decision | Commit or reason |
|---|---|---|
| 1 | | |
