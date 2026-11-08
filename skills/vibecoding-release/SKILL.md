---
name: vibecoding-release
description: Run the Vibecoding Convention's release check before a version; stops on any open blocking finding, drafts the release note with the human approver, never tags or publishes. Use before tagging a release.
---

# Vibecoding release

You check that a release may ship under the convention's rules and prepare
what the human needs to decide. You do not tag, publish, or commit.

## 1. Mechanical checks

Read `VIBECODING.md` for the level, commands, audit directory and maintainers.
Run `vibecheck . --strict` (it is read-only and runs none of the project's
commands). Group the findings by requirement, failures first, then warnings,
then checks that could not run and why. A failure stops the release until fixed
or declared as an exception with a reason; say which.

Check that `audit/status.json` was written at the current HEAD or at the
release candidate commit by CI. Otherwise ask the human to let CI run, or to
regenerate the evidence and run `vibecheck . --write-status` themselves. A
stale or missing evidence file is a stop.

## 2. Audit reports (level 3)

Find the current minor version from the version file or the last tag. For
each kind the declaration requires (architecture, security, blind test when
`blind_test: required`), check that a report exists for this minor version in
the audit directory, that every finding in it has a decision, and that no
`BLOCKER` or `DOC-WRONG` is `open`. Check that `audit/README.md` lists every
report and matches their headers. A missing or stale report: propose to run
the matching agent (`vibecoding-architecture`, `vibecoding-security`,
`vibecoding-blind-test`) and stop there. An open blocking finding: name the
report, the finding and the decision that is missing; the maintainer decides,
not you.

## 3. Declaration still true

Compare the declaration to the repository: tools and models declared against
the trailers actually used since adoption, status, response time against
`SECURITY.md`, thresholds against the CI configuration. Propose the edits.

## 4. Release note and hand-over

Ask the human who approves this release; the name must be in
`convention.maintainers`. Draft the release note lines:

```
Approved by <maintainer>.
Vibecoding Convention level <N>: checked <date> at <commit>, <verdict>; audit reports: <kinds and dates>.
```

Show the commands to tag and publish for this ecosystem, and stop. The human
runs them. After the release, remind them to let CI run on the tagged commit so the
badge is refreshed (D5).
