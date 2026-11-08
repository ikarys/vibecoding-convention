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
    - "<Tool>:<model>"            # same values as the Assisted-by trailers
  maintainers: [<handle>]
  review: "<reviewed pull requests | single maintainer reads every diff; enforced by <tool or workflow>>"
  security_response: "<72h | none>"
  licence: <SPDX identifier>
  # level 2 and above
  coverage: { threshold: <N>, kind: <line | branch>, tool: <tool> }
  maintainability: { tool: <tool>, duplication_max: <percent>, complexity_max: <N> }
  # level 3
  mutation: { tool: <tool>, threshold: <N> }   # recommended, remove if not measured
  audit_dir: audit
  audit_cadence: <minor | major | structural>
  blind_test: <required | not-applicable>
  check_interval: <6m>
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
