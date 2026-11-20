# Vibecoding Convention

[![Vibecoding Convention](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/ikarys/vibecoding-convention/main/audit/status.json)](VIBECODING.md)

> ## ⚠️ Work in progress
>
> This repository is public so that the work can be read and argued, not
> because it is ready. The convention text, its identifiers, the templates
> and the `vibecheck` validator are all under active revision and change
> without notice. Do not adopt it in a project yet. Open an issue if you
> disagree with something; that is what this stage is for.

**Let's vibecode properly.** A checkable standard for software written with
AI assistance. The evidence lives in your repository, and anyone can check it.

AI-assisted code is not a problem. Unframed, unreviewed, unaudited code is,
whoever or whatever wrote it. This convention says what "framed, reviewed and
audited" means in terms anyone can verify, and what evidence a project keeps
to back the claim.

**Read the convention: [CONVENTION.md](CONVENTION.md)** (one page), or on the
[website](https://ikarys.github.io/vibecoding-convention/).

## What you get

- **A maintainer** shows the quality of an AI-assisted project with proof
  instead of promises: a declaration, a badge computed from the repository,
  and audits anyone can re-run.
- **A user** checks a project before depending on it: who decides, what is
  tested, when it was last checked, what is not covered.
- **A contributor** gets an AI-assisted pull request taken seriously, because
  it says what it did and shows what was run.

## Three levels

| Level | Name | A project at this level has... |
|---|---|---|
| 1 | **Declared** | told its users that AI wrote part of it, which tools and models, who reviews and decides, how it handles contributions, and its licence and provenance stance. Text only, an afternoon's work, no tool to install. |
| 2 | **Tested** | tests that assert behaviour, a met coverage threshold, green CI, lint, secrets scanning, static analysis, dependency checks and freshness, measured duplication and size, and a dated check behind the badge. |
| 3 | **Audited** | versioned audit reports in the repository: architecture review, security audit, blind usability test, architecture documents validated by a named human, each finding with a decision. |

A project picks a level and proves it. A second part, five rules named C1 to
C5, applies to anyone sending a pull request or a bug report to a project
they do not maintain.

## Adopting it

1. Add four texts, all from [`templates/`](templates): a `VIBECODING.md`
   declaration at the root of your repository, a `CONTRIBUTING.md` that
   states your AI policy and names the trailer, a `SECURITY.md` with your
   response time (or declare `none`), and a README line that says the project
   follows the convention and links to the declaration. The badge comes with
   level 2.
2. Attribute AI-written commits with an `Assisted-by:` trailer naming the
   tool and model.
3. From level 2, run the checks for your level in CI and commit the evidence
   under `audit/`.

## Where to go next

- [CONVENTION.md](CONVENTION.md): the one-page text.
- [docs/requirements.md](docs/requirements.md): each requirement, its
  evidence and how it is checked.
- [docs/rationale.md](docs/rationale.md): why it is shaped this way, what it
  reuses, what it does not try to do.
- [docs/criticisms.md](docs/criticisms.md): the criticisms of vibecoded
  software it answers, with sources.
- [docs/tooling.md](docs/tooling.md): the validator, agents, skills and
  templates that apply it from AI coding tools. All optional.

## This repository

This repository follows its own convention. See [VIBECODING.md](VIBECODING.md).

## License

MIT. See [LICENSE](LICENSE).
