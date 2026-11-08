# Vibecoding Convention

A checkable standard for software written with AI assistance.

AI-assisted code is not a problem. Unframed, unreviewed, unaudited code is,
whoever or whatever wrote it. This convention says what "framed, reviewed and
audited" means in terms anyone can verify, and what evidence a project must
keep in its repository to back the claim.

**Status: draft 0.1.** Nothing here is final. Open an issue to argue.

## The three levels

| Level | Name | A project at this level has... |
|---|---|---|
| 1 | **Declared** | told its users that AI wrote part of it, which tools and models, and who reviews and decides |
| 2 | **Tested** | automated tests that assert behaviour, a declared and met coverage threshold, green CI, lint, dependency checks |
| 3 | **Audited** | versioned audit reports in the repository: architecture review, security audit, blind usability test when applicable, each finding with a decision |

A project picks a level and proves it. Nobody is asked to reach level 3 for a
weekend script. The full text is in [CONVENTION.md](CONVENTION.md).

## Adopting it

1. Add a `VIBECODING.md` declaration at the root of your repository (template in
   `templates/`), and a short "AI assistance" section in your README.
2. Co-sign AI-written commits with the model that wrote them, as a
   `Co-Authored-By:` trailer.
3. Run the audit tooling for your level and commit the reports under `audit/`.

## Tooling

The convention is a text. The tooling in this repository helps apply it from
AI coding tools:

- `skills/` — procedures loaded into the main assistant: bootstrap a project,
  write the disclosure, produce an audit.
- `agents/claude-code/` — Claude Code subagents: blind tester, architecture
  reviewer, security auditor, convention auditor.
- `agents/fennec/` — the same agents for [Fennec](https://github.com/ikarys/fennec).
- `templates/` — `VIBECODING.md`, `audit/README.md`, report skeletons.

Tooling is optional. A project can satisfy the convention by hand.

## This repository

This repository follows its own convention. See [VIBECODING.md](VIBECODING.md).

## License

MIT. See [LICENSE](LICENSE).
