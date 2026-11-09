---
name: vibecoding-contribute
description: Pre-flight for a pull request, issue or vulnerability report to a project you do not maintain, against Part B of the Vibecoding Convention (C1 to C4); drafts the description and stops before sending. Use before opening any contribution on someone else's repository.
---

# Vibecoding contribute

You check one contribution before the human sends it, against Part B of the
convention. You never open the pull request or the issue yourself: you leave
the human one command away from it, with a description that is true.

## 1. The target's rules (C4)

Read the target project's `CONTRIBUTING.md`, its AI policy if separate, its
pull request or issue templates, and its `SECURITY.md` for a vulnerability
report. If the project refuses AI-assisted contributions, say so and stop:
the human may still contribute by hand, not through you. Note the trailer and
the disclosure form the project expects, and any rule on scope (one change
per pull request, issue first, tests required).

## 2. The diff (C2, C3)

Show the human the whole diff, file by file, and ask them to confirm they
know each change and can say what it does, why it is there and what it
affects (the change, not every line or construct of the language). For each hunk that changes behaviour, name the test that covers
it; if none does, write it or say the pull request is not complete. For each
user-visible change, name the documentation it affects; update it or say it
is missing. Check the scope: one change, no unrelated reformatting, no
dependency bump the project did not ask for. Run the project's tests and lint
as its guide says, and keep the output.

## 3. For a bug or vulnerability report (C3)

Reproduce it. Record the exact steps, the versions (project, runtime, OS) and
the output. If you cannot reproduce it, say so to the human and do not send a
report that claims otherwise. For a vulnerability: a working reproduction and
only what you saw happen, no reasoning about what might be possible; send it
through the channel `SECURITY.md` names, never as a public issue.

## 4. Disclosure and description (C1)

Write the description from what was actually done:

```
## What and why
<one or two sentences>

## AI assistance
<none | Assisted by <Tool>:<model>; every commit carries the trailer <the project's trailer>>

## Checked
I know every change in the diff and can say what each does and why. Tests: <added or updated, names>. Docs: <updated pages, or "no user-visible change">.
Ran: <commands> → <result>
```

Make sure every commit with AI-written content carries the trailer the target
expects (the D3 defaults when it has no preference). Then show the command
that opens the pull request or the issue, and stop. The human sends it, and
answers the review themselves (C5).
