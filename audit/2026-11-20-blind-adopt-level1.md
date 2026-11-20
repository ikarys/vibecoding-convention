---
date: 2026-11-20
commit: ec98f79
kind: blind-adopt-level1
tool: Claude Code
model: claude-sonnet-5-5
scope: a small Rust library maintainer reaches level 1 from the docs alone
verdict: 6/10
---

# Blind test: adopt level 1

## Brief

- Persona: maintainer of a one-crate MIT Rust library with CI running `cargo test`, writes most code with an AI assistant, never heard of the convention, about 20 minutes, no tooling to install for level 1.
- Task: build a fake repo (git init, Cargo.toml, src/lib.rs, README, MIT LICENSE, CI, one commit), reach level 1 from the docs, then run `vibecheck .` as verification.
- Allowed inputs: README.md, CONVENTION.md, docs/requirements.md, rationale.md, tooling.md, criticisms.md, templates/, `vibecheck --help`.
- How blindness was enforced: `mktemp -d` scratch directory holding only a copy of those files; the validator was run as a binary and its sources never opened; no other repository file (skills, agents, AGENTS.md, VIBECODING.md, audit, convention.yaml, git history) was read. Not enforced by sandbox, by discipline of the tester.
- Versions: vibecheck debug build (version not queried), git, Rust project never compiled.

## (a) Friction log

| # | Step | Severity | What happened | Expected | Exact error | Suggested fix |
|---|---|---|---|---|---|---|
| 1 | README start | PAINFUL | The first screen is a "Work in progress ... Do not adopt it in a project yet" box. A persona with 20 minutes whose goal is to adopt would stop here or carry on knowing it may change without notice. | A clear statement of whether level 1 is stable enough to try. | n/a | Say what is stable (level 1 text) vs not, or give the date the warning goes. |
| 2 | Adopting step 1 | DOC-WRONG | README says add "the badge in your README". The badge reads `audit/status.json`, which only exists from level 2 (D5, `--write-status`). At level 1 the badge is broken (404). requirements.md D1 says "section or badge", and the Declaration file section says "The README disclosure (D1) is the dynamic badge of D5". | One consistent level-1 instruction: a text section linking VIBECODING.md. | n/a | README: "level 1: a README section or link; the badge comes with level 2". |
| 3 | Adopting step 1 | PAINFUL | "a contribution policy" has no template. `templates/` has no CONTRIBUTING.md. I had to assemble it from D8 in requirements.md and the PR template. D8 text is spread between CONVENTION.md (one sentence) and requirements.md. | A CONTRIBUTING template. | `D8 FAIL CONTRIBUTING.md not found` | Add `templates/CONTRIBUTING.md`. |
| 4 | Adopting | PAINFUL | Nothing in README "Adopting it" says SECURITY.md is required at level 1, nor that the PR template is. It is only discoverable in D9 (requirements.md: "for projects that accept security reports") and the validator then fails with `SECURITY.md not found`, a hard failure although the doc phrases it conditionally. A project that accepts no reports has no documented way out (Exceptions section exists in the template but the validator behaviour is unknown). | A checklist of the files level 1 creates: VIBECODING.md, README section, CONTRIBUTING.md, SECURITY.md, PR template. | `D9 FAIL SECURITY.md not found` | Add the checklist to README "Adopting it" and to CONVENTION.md Levels. |
| 5 | adopted_at | PAINFUL | Template says `adopted_at: <commit hash>` with no hint which commit. The adoption commit cannot contain its own hash, so I used the previous commit. Not stated anywhere. | "The last commit before the declaration was added", or "the commit that adds VIBECODING.md (amend the hash in a second commit)". | n/a | One sentence in the declaration section. |
| 6 | Deliberate mistake 1 (no adopted_at) | MINOR | Caught by the validator, clear and actionable. The docs list it in D2 but the Declaration-file prose example in the README flow does not lead you to it; fine. | | `D2 FAIL missing required key: convention.adopted_at` / `fix: Add convention.adopted_at to VIBECODING.md` | none |
| 7 | Mistake 2 (AI commit, no trailer) | MINOR | Not caught at all, by design ("understood as human-written"). As a newcomer this is surprising: the one rule I broke is silent; the validator reports only `0 of 1 commits ... carry an AI attribution trailer (0 %)` as INFO. | A warning when the declaration says implementation is `copilot` and the share is 0 %. | `D3 INFO 0 of 1 commits since adoption carry an AI attribution trailer (0 %)` | Warn when declared involvement is high and the share is low (docs D3 says it is a finding for an auditor). |
| 8 | D9 vs SECURITY.md | DOC-WRONG | Docs say `VIBECODING.md` "matches" `SECURITY.md`; the SECURITY template uses `<72 hours>`, the declaration template `<72h | none>`. I wrote "72 hours" in SECURITY.md and the validator failed: it needs the literal string `72h`. | Equivalent durations accepted, or the two templates use the same literal. | `D9 FAIL SECURITY.md does not contain security_response value: 72h` / `fix: Document the same security response in SECURITY.md` | Align the two templates on one form and document that the match is a literal one. |
| 9 | Mistake 3 (Co-Authored-By model) | MINOR | Docs: allowed by default, exempt from the model-must-be-declared rule, but only counted if the project's `attribution_trailers` lists it. With `[Assisted-by]` declared (the template default) a `Co-Authored-By: Claude ...` commit passes silently and is not counted as attributed (percentage unchanged). Nothing tells the user their commit was ignored. `Signed-off-by:` naming a model was caught clearly. | A note that the trailer is not in the declared list. | none (silent); for Signed-off-by: `D3 FAIL Signed-off-by names a model: Claude Sonnet 5.5 <noreply@anthropic.com>` | Info line "N commits carry an undeclared attribution trailer". |
| 10 | D2 warning | MINOR | Right after writing a correct declaration: `tool Claude Code:claude-sonnet-5-5 declared but never attributed`, because no commit had a trailer yet. Fine logically, but it reads as an error to a first-time adopter and the adoption commit itself cannot satisfy it. | | `D2 WARN tool Claude Code:claude-sonnet-5-5 declared but never attributed` | Mention in docs that the first attributed commit clears it. |
| 11 | Validator summary | MINOR | Summary line prints `level 1` even with 3 failures (`3 fail, ... · level 1 · commit ...`). It reads like "you are at level 1", which is wrong. A pass is shown only as `0 fail`; no explicit "level 1 confirmed". | A line "level 1: PASS" or "level 1: NOT MET". | `3 fail, 1 warn, 0 info, 4 not run · level 1 · commit eaebc0b` | Print PASS or FAIL for the claimed level. |
| 12 | D4 / D7 | MINOR | Validator prints nothing for D4 (review policy) and D7 when they pass or when they are only judgement, so I cannot tell whether they were checked. D4 is M+J and nothing in output says "needs a human". | Pass lines for checked requirements. | none | `--format text` verbose mode listing all requirements with PASS / JUDGEMENT. |
| 13 | criticisms.md | DOC-WRONG | Says D6 is "Level 1" (line "NEW D6. Test changes are explained. Level 1."); CONVENTION.md puts D6 at level 2. The file also says "level 1 grows by D6, D7, D8 and D9". | Consistent with CONVENTION.md. | n/a | Update or mark that page as historical. |
| 14 | Report template | MINOR | The campaign brief asked for a section "(e) Readability" but the template's (e) is Verdict. Added both. | | | |
| 15 | CONVENTION.md | MINOR | Level 1 requirements are identifiable (D1, D2, D3, D4, D7, D8, D9, Level column), good, but the order in the table puts level 2 IDs D5, D6, D10 after T-rows, so "D1..D10 are level 1" is a wrong guess. Terms needing lookup: "licence stance" fine; "AI-DECLARATION" levels (`none|hint|assist|pair|copilot|auto`) are only defined in the template comment with no meaning of each value, so I guessed the per-process values. | Definitions of the six values. | n/a | Link the AI-DECLARATION.md levels or one line each. |

Not logged separately: "provenance" vs "licence" terms are clear from D7; CONVENTION.md rows are readable in one pass.

## (b) Effort per step

| Step | Lines of code written | Calls or queries made |
|---|---|---|
| Fixture (Cargo.toml, lib.rs, README, LICENSE, CI) | about 20 | 1 |
| VIBECODING.md from template | 40 | 2 validator runs |
| README section, SECURITY.md, CONTRIBUTING.md, PR template copy | about 20 | 1 validator run |
| Fixing D9, deliberate-mistake commits | 2 | 5 validator runs |

## (c) What was simple

- VIBECODING.md template: filling it was mechanical, and the validator caught the missing `adopted_at` clearly.
- The Level column in CONVENTION.md says which requirements are level 1.
- The Signed-off-by-by-a-model failure is clear and tells the fix.
- No tool needed to write the files; the validator only to check.
- PR template copied as is.

## (d) Documentation pages wrong, missing or misleading

- `README.md` Adopting it: badge advice wrong at level 1 (finding 2); no file checklist (4); no CONTRIBUTING template (3).
- `docs/requirements.md` D9 / Declaration file: "matches" is a literal string match (8); `adopted_at` meaning (5).
- `templates/`: no CONTRIBUTING.md; SECURITY.md and VIBECODING.md use different forms for the response time.
- `docs/criticisms.md`: stale level for D6 (13).

## (e) Verdict

Score: 6/10. Would the tester use this software for this task: with reservations, because the level-1 path is achievable in well under 20 minutes but the doc never lists the five files to create, and the validator disagrees with the docs on the badge and on the response time form.

Final state: the fixture passes `vibecheck .` with `0 fail, 0 warn, 1 info`, exit code 0, level 1.

## (e) Readability

Timings: not measured with a clock. README plus CONVENTION.md read in one pass in roughly 5 minutes of reading; requirements.md level 1 part about 5 more; building the fixture and passing the validator about 5 more, so about 15 to 20 minutes end to end.

- After paragraph 1 (the WIP box): I could not say what it is for; I knew only that I should not adopt it. After paragraph 2 ("Let's vibecode properly. A checkable standard for software written with AI assistance"): yes, one sentence. After paragraph 3 (unframed code is the problem): yes, who it is for is clearer only in "What you get". No re-read needed, but the WIP box nearly stops reading.
- Would I adopt it: with reservations. Level 1 is cheap and honest, text only. The validator is a nice optional check. I would wait until the WIP warning is lifted.
- What would make me stop reading: the "Do not adopt it yet" box, and the open questions list at the end of requirements.md.
- One sentence for a colleague: "A short checklist that makes you say, in a file and in commit trailers, which AI wrote what and who reviewed it, with a validator that checks the declaration."

## (f) Decisions

| # | Decision | Commit or reason |
|---|---|---|
| 1 | Kept | The warning stays until the first release; the maintainer decides when it goes. |
| 2 | Fixed | README and `index.html` step 1 list the level-1 files and say the badge comes with level 2; `docs/requirements.md` says the same under the declaration file. |
| 3 | Fixed | `templates/CONTRIBUTING.md` added, listed in `docs/tooling.md`. |
| 4 | Fixed | README step 1 is now the checklist (VIBECODING.md, CONTRIBUTING.md, SECURITY.md or `security_response: none`, README line). The validator already accepts `none` without a SECURITY.md; the README says so. |
| 5 | Fixed | Template comment and D3 evidence line: `adopted_at` is the last commit before the declaration was added. |
| 6 | No change | Works as documented. |
| 7 | Deferred | vibecheck change: warn when declared involvement is high and the attributed share is low. Tracked for the next vibecheck release, not in this docs branch. |
| 8 | Fixed | Both templates use `72h`; D9 in `docs/requirements.md` says the match is literal. |
| 9 | Deferred | vibecheck change: count or report trailers outside the declared list. Same release as 7. |
| 10 | Deferred | A sentence in D2 when the vibecheck message itself is adjusted (7, 9). |
| 11 | Deferred | vibecheck change: print PASS or NOT MET for the claimed level. Same release as 7. |
| 12 | Deferred | vibecheck change: a verbose listing of every requirement with its outcome. Same release as 7. |
| 13 | Fixed | `docs/criticisms.md` puts D6 at level 2 and the level-structure paragraph matches `CONVENTION.md`. |
| 14 | Kept | Both sections stay; the brief named a section the template does not have. |
| 15 | Fixed | The template comment gives the six AI-DECLARATION levels in one line each. The requirement order in `CONVENTION.md` is kept: identifiers are stable and the Level column is the source of truth. |
