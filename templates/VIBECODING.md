---
# AI involvement, AI-DECLARATION.md keys (https://github.com/DimwitLabs/AI-DECLARATION.md)
# levels: none | hint | assist | pair | copilot | auto
version: "0.1.1"
level: <highest level below>
processes:
  design: <level>
  implementation: <level>
  testing: <level>
  documentation: <level>
  review: <level>
  deployment: <level>
# Vibecoding Convention (https://github.com/ikarys/vibecoding-convention)
convention:
  version: "0.1"
  level: <1 | 2 | 3>
  adopted_at: <commit hash>
  status: <active | maintenance | archived>
  tools:
    - "<Tool>:<model>"            # same values as in the attribution trailers
  attribution_trailers: [Assisted-by]   # the trailers this project uses; default accepts Assisted-by, Generated-by, Co-Authored-By, AI-Tool
  agent_instruction_files: [AGENTS.md]  # optional; the known names are always looked at
  maintainers: [<handle>]
  review: "<reviewed pull requests | single maintainer reads every diff; enforced by <tool or workflow>>"
  security_response: "<72h | none>"
  licence: <SPDX identifier>
  # level 2 and above
  coverage: { threshold: <N>, kind: <line | branch>, tool: <tool> }
  maintainability: { tool: <tool>, duplication_max: <percent>, complexity_max: <N>, function_max: <lines>, file_max: <lines>, markers_max: 0 }
  freshness: { max_outdated: <N>, max_lag_months: <N> }   # T9; update bot config or commands.outdated must exist
  # level 3
  mutation: { tool: <tool>, threshold: <N> }   # recommended, remove if not measured
  audit_dir: audit
  audit_cadence: <minor | major | structural>
  blind_test: <required | not-applicable>
  check_interval: <6m>
  agent_permissions: "<sandbox or permission model the agents run under>"
  # commands tools may run (all optional)
  commands:
    test: "<command>"
    coverage: "<command>"
    lint: "<command>"
    format: "<command>"
    audit: "<command>"
    static_analysis: "<command>"
    secrets: "<command>"
    maintainability: "<command>"
    outdated: "<command>"           # cargo outdated, npm outdated, pip list --outdated ...
    doc_lint: "<command>"           # public API documentation lint, if the ecosystem has one
    mutation: "<command>"
---

# Vibecoding declaration

This project follows the [Vibecoding Convention](https://github.com/ikarys/vibecoding-convention) version 0.1 at level <N>.

## AI assistance

<Two or three sentences: what the AI tools do here, what the humans do, how
much of the code is AI-written as an honest estimate.>

## Review

<How a change reaches the default branch, and what makes the human reading a
step rather than an intention.>

## Provenance and licence

This project is licensed under <licence>. Parts of it are written by AI
assistants and may not be protected by copyright in some jurisdictions; the
maintainers accept this. To the maintainers' knowledge, no verbatim
reproduction of licence-incompatible code was included. Safeguards used:
<vendor code-reference filter | snippet scanning with <tool> | human check of
long generated blocks | none>.

## Status

<active | maintenance | archived>. Security reports are acknowledged within
<72 hours | best effort, no commitment>; see `SECURITY.md`.

## Exceptions

<Requirements that do not apply to this project, with the reason, one line each. Delete this section if none.>

## Dependencies held back

<Dependency, version held, reason (incompatibility, stability), date. Delete this section if none.>
