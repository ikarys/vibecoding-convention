# Tooling

The convention is a text and can be met by hand. The tooling in this
repository runs the same checks the same way everywhere, from AI coding
tools.

- [`vibecheck/`](../vibecheck/README.md), a command-line validator with no AI
  in it. It runs none of the project's code: it reads the declaration, the
  repository and the evidence files the project's CI wrote, does every
  mechanical check and writes `audit/status.json`. It is safe to run on any
  repository, which is the point: auditors and blind testers run it on code
  they do not trust. Its `--pr-review` mode checks only what a pull request
  adds (trailers, test changes, dependencies, secrets, markers), for the
  owner's CI.
- [`convention.yaml`](../convention.yaml), the requirements as data, for tools.
- [`skills/`](../skills), procedures loaded into the main assistant:
  `vibecoding-init` bootstraps a project, `vibecoding-release` prepares a
  release, `vibecoding-contribute` checks a contribution against Part B
  before it is sent, `code-audit` measures size and duplication, then runs
  the architecture and security agents and writes the A1 and A2 reports.
- [`agents/`](../agents/README.md), the judgement checks: `vibecoding-audit`,
  `vibecoding-blind-test`, `vibecoding-architecture` and
  `vibecoding-security`, for Claude Code and for
  [Fennec](https://github.com/ikarys/fennec). They read `vibecheck`'s output
  rather than redoing it.
- [`templates/`](../templates), `VIBECODING.md`, `CONTRIBUTING.md`,
  `SECURITY.md`, `audit/README.md`, `audit/status.json`, report skeletons, and `ci/` with two CI jobs for
  GitHub Actions, GitLab CI and Forgejo Actions: one writes the evidence and
  the status, one reviews pull requests.

A ten-line block in the agent instruction file (D10) is all that sits in a
model's context during ordinary work.
