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
  established, the one the code needs) and the check is noted in the commit.
- No secret in any file, prompt or config.

Never without a human: merge, release, change the CI, delete data.
