# vibecheck

`vibecheck` is a command-line validator for the Vibecoding Convention. It reads
`VIBECODING.md`, runs the mechanical checks for the declared level, prints findings, and can
write `audit/status.json`.

## Usage

```text
vibecheck [PATH] [--level N] [--format text|json] [--write-status] [--strict]
```

- `PATH` is the repository root, default `.`.
- `--level` overrides the declared level.
- `--format json` prints a JSON report.
- `--write-status` writes `audit/status.json`.
- `--strict` makes warnings affect the exit code.

## Exit codes

- `0`: no `fail` findings, and no `warn` findings when `--strict` is used.
- `1`: at least one `fail` finding, or at least one `warn` finding with `--strict`.
- `2`: CLI misuse or an unreadable path.

## Checked in this version

Level 1 checks D1 through D10 are implemented for the mechanical parts of the convention:
README disclosure, declaration shape, commit attribution, human review, dated status, test
change explanations, provenance, contribution policy, security status, and agent instruction
files.
