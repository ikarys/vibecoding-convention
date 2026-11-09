# Contributing

Issues and pull requests are welcome. The convention is a draft; arguing with
it is the point.

## AI-assisted contributions

Accepted with disclosure. If an AI assistant wrote part of your change, add a
trailer to the commit naming the tool and model:

```
Assisted-by: Claude Code:claude-opus-5-5
```

Use `Generated-by:` instead when the change is substantially generated without
human edits. This project uses these two trailers; the convention itself
accepts any declared trailer.

The pull request template asks for the three things Part B of the convention
asks of any contributor: disclosure (C1), that you read your diff and can say
what each change does and why (C2), and that tests and documentation come with the change, with what
you ran (C3). A pull request whose author cannot say what its changes do is closed.

The tool that wrote a change never approves it. A human maintainer reviews
every pull request before merge.

## Commit messages

Subjects follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/):
`type(scope): summary`, lower case, imperative, no trailing period. Types
in use: `feat`, `fix`, `docs`, `refactor`, `test`, `ci`, `build`, `chore`,
`revert`. Scopes are the areas of this repository: `vibecheck`, `spec` (the
three convention texts), `site`, `templates`, `agents`, `skills`, `ci`. A
breaking change to the convention text or to the `vibecheck` interface is
marked with `!` after the scope. The scope may be left out for a change that
spans several areas.

```
feat(vibecheck): --pr-review reviews what a pull request adds
docs(spec): T7 asks for a scanner in CI, the hook is recommended
```

The history is the changelog: release notes are generated from these
subjects, which is why the format is checked by a `commit-msg` hook
(`pre-commit install` sets it up). Merge commits keep their
standard git message. Commits made before this rule are not rewritten.

## Language

Everything in this repository is in English.
