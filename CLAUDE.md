# Agent instructions

This repository holds the Vibecoding Convention text, its machine-readable
list, templates, and the `vibecheck` validator (Rust, in `vibecheck/`).

## Vibecoding Convention (level 2)

This project follows its own convention at level 2; see `VIBECODING.md`.

Before calling a change done, these must pass:
- `cargo test --manifest-path vibecheck/Cargo.toml`
- `cargo clippy --manifest-path vibecheck/Cargo.toml --all-targets -- -D warnings`
- `cargo fmt --manifest-path vibecheck/Cargo.toml --check`
- `vibecheck .` (build it with `cargo build --manifest-path vibecheck/Cargo.toml`)

Rules while editing:
- Every commit with AI-written content ends with `Assisted-by: <Tool>:<model>`,
  the trailer this project declares.
- Deleting, skipping or weakening a test requires the reason in the commit message.
- A new dependency is checked against crates.io (exists under that exact name,
  established, the one the code needs) and taken at its current version, not the
  one you remember; the check is noted in the commit. Existing dependencies are
  not held back without a reason written in `VIBECODING.md`.
- No secret in any file, prompt or config.
- Every input is hostile: arguments, files, environment, network, model output.
  Follow the project's security checklist in `SECURITY.md`; a change that touches
  parsing, SQL or command building, authentication, crypto, credentials or file
  import names the vulnerability class it guards against and its test.
- Signal over noise. A comment states a reason, a constraint or a trap, never
  what the line does. A docstring tells the caller what the signature does
  not. The public API is documented. No TODO, placeholder, commented-out or
  dead code in a delivered change. Documentation proportionate to the
  project: a README says what it is, how to install, how to use, where to go
  next; nothing written to fill space.
- `CONVENTION.md`, `docs/requirements.md` and `convention.yaml` must say the
  same thing; a change to one is a change to all three.
- Everything in this repository is in English.

Never without a human: merge, release, change the CI, delete data.
