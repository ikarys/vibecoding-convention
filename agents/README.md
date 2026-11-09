# Agents and skills

The mechanical checks of the convention are done by `vibecheck`. These agents
do the judgement checks and write the audit reports; the skills do the steps
that change a repository. Four agents, the same ids on both platforms:

| Agent | Writes | Judges |
|---|---|---|
| `vibecoding-audit` | `audit/<date>-convention.md` | D4, D6, T3, T4, T9, and at level 3 A4, A5, A6 and the presence of the other reports |
| `vibecoding-blind-test` | `audit/<date>-blind-<campaign>.md` | A3, from the documentation only, in an environment that holds no source |
| `vibecoding-architecture` | `audit/<date>-architecture.md` | A1 and A6 |
| `vibecoding-security` | `audit/<date>-security.md` | A2, the T6 and T7 scans, A8 |

Skills: `vibecoding-init` (bring a project to a level), `vibecoding-release`
(check a release), `vibecoding-contribute` (check a contribution against C1 to
C4 before sending it). The skill format is the same for both tools.

## Claude Code

Copy the agents to `~/.claude/agents/` (all projects) or `.claude/agents/` in
a project, and the skills to `~/.claude/skills/<name>/SKILL.md` or
`.claude/skills/<name>/SKILL.md`:

```
cp agents/claude-code/*.md ~/.claude/agents/
cp -r skills/vibecoding-init skills/vibecoding-release skills/vibecoding-contribute ~/.claude/skills/
```

Invoke an agent by asking Claude to use it by name, or with the Agent tool;
invoke a skill with `/vibecoding-init` or `/vibecoding-release`. The blind
tester makes its own scratch directory with `mktemp -d`; launch it from the
project, it never reads the source.

## Fennec

Copy the agents to `~/.agents/agents/` and the skills to the skills
directory Fennec reads:

```
cp agents/fennec/*.agent.md ~/.agents/agents/
```

The three auditors are read-only (`commands: read-only`). The blind tester
cannot make a directory outside the session, so launch it on a dedicated
working directory that holds only the documentation copy; it stops if it sees
a source tree.

## Before running an auditor

Have `vibecheck` installed and the project's `VIBECODING.md` in place; the
auditors read `vibecheck --format json` and do not redo it. Tell the agent
whether it may run the project's commands (tests, scanners); without that it
reports those checks as not run. vibecheck itself never runs them.
