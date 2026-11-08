---
name: vibecoding-init
description: Put a project under the Vibecoding Convention at a chosen level; writes the declaration, disclosure, contribution policy, hooks and audit directory, then stops for the human to commit. Use when a project adopts the convention or changes level.
---

# Vibecoding init

You bring a repository to a chosen level of the Vibecoding Convention. You
write files, you never commit, tag or push: the human reads the diff and
commits. Reuse what the project already has; the convention asks for outcomes,
not for particular tools or file names.

## 1. Frame with the human

Ask, in one message, for what only they can decide:

- The level, with the suggested fit: 1 for scripts and prototypes, 2 for
  anything others depend on, 3 for anything handling other people's data,
  money or safety. Say that each level includes the ones below.
- The maintainers who review and decide (names or handles), the review
  policy (reviewed pull requests, or a single maintainer reading every diff
  and what makes that a step rather than an intention), the project status
  (active, maintenance, archived) and the security response time or "none".
- The AI tools and models in use, and which attribution trailer the project
  wants on AI-written commits (the default accepts `Assisted-by`,
  `Generated-by`, `Co-Authored-By`, `AI-Tool`; `Assisted-by: <Tool>:<model>` is
  what the large projects converged on).
- The licence and the copyright stance on AI-written parts (the template has
  a sentence to accept or edit).
- For level 2 and above: coverage threshold (80 % recommended, a lower value
  needs one sentence of reason), maintainability thresholds, freshness
  limits, audit cadence; for level 3: audit directory, check interval,
  whether a blind test applies.

Do not ask what you can read: ecosystem, existing CI, hooks, instruction
files, licence file.

## 2. Read the repository

Detect the ecosystem from the manifests. Read the CI configuration,
pre-commit or hook configuration, `README.md`, `CONTRIBUTING.md`,
`SECURITY.md`, the agent instruction files (`AGENTS.md`, `CLAUDE.md`,
`.cursorrules`, `GEMINI.md` and the like) and the licence file. From the CI and
the ecosystem, propose the commands for `test`, `lint`, `format`, `coverage`,
`audit`, `static_analysis`, `secrets` and `outdated`, preferring tools the
project already runs. At level 2 and above also propose the evidence paths
(`audit/evidence/coverage.txt`, `audit/evidence/audit.json`, and
`outdated.txt` when no update bot is configured). Show the list and let the
human confirm or change it.

## 3. Write the declaration

Write `VIBECODING.md` from `templates/VIBECODING.md`: the AI-DECLARATION keys
and the `convention` block in the front matter (level, `adopted_at` set to the
current HEAD, status, tools, `attribution_trailers`, maintainers, review,
security response, licence, the thresholds of the level, the confirmed
`commands`, the `evidence` block (level 2 and above), `agent_instruction_files` when the project uses non-standard
names); in the body the sections AI assistance, Review, Provenance and
licence, Status, and Exceptions for any requirement that does not apply, each
with its reason and date.

## 4. Disclosure and contribution policy

- `README.md`: a short "AI assistance" line linking `VIBECODING.md`, and the
  status badge (`templates/` has the badge line; replace owner and repository).
- `CONTRIBUTING.md`: write or complete the AI contribution policy (accepted
  with disclosure, with conditions, or refused), the trailer the project
  expects, and the verification evidence a pull request must carry. Add the
  pull request template from `templates/` if the platform supports one.
- Agent instruction files: add the section from `templates/AGENTS-section.md`
  to each file the project has, with the real commands. Do not create one
  when none exists.
- Commit attribution: configure the tool the human uses so that its commits
  carry the chosen trailer (Claude Code: the `attribution` setting in
  `settings.json`; other tools: their own setting). Say what you changed.

## 5. Hooks, CI and audit directory

- Level 2 and above: a pre-commit secrets scanner, preferring one already in
  use, `gitleaks` otherwise; the same scanner in CI; the confirmed commands in
  CI where missing. Tell the human how to install the hook.
- Level 2 and above: add the CI steps from `templates/ci/github-actions.yml`
  (or the platform's equivalent) that write the evidence files and run
  `vibecheck . --write-status`.
- `audit/README.md` from the template. Level 3: `SECURITY.md` from the
  template if missing; `ARCHITECTURE.md` and `docs/adr/` skeletons, each with a
  "Validated by <maintainer> on <date>" line left for the human to fill.
- Dependency updates (T9): a Dependabot or Renovate configuration if the
  platform supports one, otherwise the `outdated` command and a cadence in the
  declaration.

## 6. Check and hand over

Run `vibecheck .` (no `--write-status`: the evidence does not exist yet) and
show the findings. Explain that the first status will be written by CI. Fix what is yours to
fix (a missing field, a wrong path); leave to the human what needs a decision
and say so. Show the whole diff. Give the first commit message as text to
copy, ending with the chosen trailer. Stop: the human commits.
