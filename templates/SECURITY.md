# Security

## Reporting a vulnerability

Write to <address or private channel>. We acknowledge within <72 hours> and
say what we will do within <7 days>. Do not open a public issue for a
vulnerability.

## What this software protects, and what it does not

<Two or three sentences: the assets, the trust boundaries, what is out of
scope (for example: "the host running this tool is trusted").>

## Threat model: known vulnerability classes and their coverage

The classes relevant to this kind of software, from the OWASP Top 10, the CWE
Top 25 or the ecosystem's list, each with what covers it. A class with no
coverage is a known gap and says so.

| Class | Where it could happen here | Covered by (tool rule or test) |
|---|---|---|
| Injection (SQL, command, template) | <modules> | <lint rule or `path::test`> |
| Path traversal | <modules> | |
| Unsafe deserialization | <modules> | |
| Broken authorization | <endpoints, tables> | <unauthorized-caller tests> |
| Secrets in code or logs | repository | <secrets scanner in pre-commit and CI> |
| Vulnerable dependencies | manifests | <audit tool in CI> |
| <ecosystem-specific class> | | |

## Claims and their tests

Each claim this project makes about its security points to the test that
proves it. A claim without a test says so.

| Claim | Test |
|---|---|
| <claim> | `path::test` or **untested** |
