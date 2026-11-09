# Vibecoding Convention

[![Vibecoding Convention](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/ikarys/vibecoding-convention/main/audit/status.json)](VIBECODING.md)

> ## ⚠️ Work in progress
>
> This repository is public so that the work can be read and argued, not
> because it is ready. The convention text, its identifiers, the templates
> and the `vibecheck` validator are all under active revision and change
> without notice. Do not adopt it in a project yet. Open an issue if you
> disagree with something; that is what this stage is for.

A checkable standard for software written with AI assistance.

**Read the convention: [CONVENTION.md](CONVENTION.md)** (one page), or browse it on
the [website](https://ikarys.github.io/vibecoding-convention/).

AI-assisted code is not a problem. Unframed, unreviewed, unaudited code is,
whoever or whatever wrote it. This convention says what "framed, reviewed and
audited" means in terms anyone can verify, and what evidence a project must
keep in its repository to back the claim.

It was written against the criticisms actually made of vibecoded software,
with their sources: see [docs/criticisms.md](docs/criticisms.md).

**Status: draft 0.1.** Nothing here is final. Open an issue to argue.

## The three levels

| Level | Name | A project at this level has... |
|---|---|---|
| 1 | **Declared** | told its users that AI wrote part of it, which tools and models, who reviews and decides, how it handles contributions, its licence and provenance stance, and when it was last checked. All text, an afternoon's work. |
| 2 | **Tested** | tests that assert behaviour, a declared and met coverage threshold, green CI, lint, a secrets scanner in CI, static analysis, dependency checks and freshness, and measured duplication and size. |
| 3 | **Audited** | versioned audit reports in the repository: architecture review, security audit, blind usability test where the software has user documentation, architecture documents validated by a named human, each finding with a decision. |

A project picks a level and proves it. Nobody is asked to reach level 3 for a
weekend script. A second part, five rules named C1 to C5, applies to anyone
sending a pull request or a bug report to a project they do not maintain:
disclose, know what your changes do, bring tests and docs (or a reproduction),
respect the target's policy, answer review. The one-page text is [CONVENTION.md](CONVENTION.md); the
detail of each requirement is in [docs/requirements.md](docs/requirements.md).

## What it reuses

- The [AI-DECLARATION.md](https://github.com/DimwitLabs/AI-DECLARATION.md)
  keys for the degree of AI involvement.
- The `Assisted-by:` and `Generated-by:` commit trailers adopted by the Linux
  kernel, Fedora, LLVM and others.

What it adds is the evidence behind the declaration: tests that are checked
for asserting something, audits kept in the repository, blind tests by an AI
that cannot see the source, and a dated check that marks itself stale.

## Adopting it

1. Add a `VIBECODING.md` declaration at the root of your repository (template in
   `templates/`), a contribution policy, and the badge in your README.
2. Attribute AI-written commits with an `Assisted-by:` trailer naming the tool and model.
3. Run the checks for your level and commit the reports under `audit/`.

## Tooling

The convention is a text. The tooling in this repository helps apply it from
AI coding tools:

- `vibecheck/` — a command-line validator with no AI in it. It is read-only: it
  never runs the project's commands; it reads the declaration, the repository
  and the evidence files the project's CI wrote, runs every mechanical check
  and writes `audit/status.json`.
- `convention.yaml` — the requirements as data, for tools.
- `skills/` — procedures loaded into the main assistant: bootstrap a project,
  prepare a release, check a contribution before sending it.
- `agents/claude-code/` — Claude Code subagents `vibecoding-audit`,
  `vibecoding-blind-test`, `vibecoding-architecture`, `vibecoding-security`.
- `agents/fennec/` — the same four agents for [Fennec](https://github.com/ikarys/fennec).
- `templates/` — `VIBECODING.md`, `audit/README.md`, `audit/status.json`,
  report skeletons, and `ci/` with a GitHub Actions job that writes the
  evidence and the status.

Tooling is optional. A project can satisfy the convention by hand.

## This repository

This repository follows its own convention. See [VIBECODING.md](VIBECODING.md).

## License

MIT. See [LICENSE](LICENSE).
