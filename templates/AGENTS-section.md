## Vibecoding Convention (level <N>)

This project follows the Vibecoding Convention at level <N>; see `VIBECODING.md`.

Before calling a change done, these must pass:
- `<test command>`
- `<lint command>` and `<format command>`
- `<secrets scan command>`

Rules while editing:
- Every commit with AI-written content ends with the attribution trailer this
  project uses, naming the tool and model, for example `Assisted-by: <Tool>:<model>`.
- Deleting, skipping or weakening a test requires the reason in the commit message.
- A new dependency is checked against the registry (exists under that exact name,
  established, the one the code needs) and taken at the registry's current
  version, not the one you remember; the check is noted in the commit. Existing
  dependencies are not held back without a reason written in `VIBECODING.md`.
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

Never without a human: merge, release, change the CI, delete data.
