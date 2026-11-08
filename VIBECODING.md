---
version: "0.1.1"
level: copilot
processes:
  design: pair
  implementation: copilot
  testing: none
  documentation: copilot
  review: hint
  deployment: none
convention:
  version: "0.1"
  level: 1
  adopted_at: 03c8df2
  status: active
  tools:
    - "Claude Code:claude-fable-5-1"
    - "Claude Code:claude-opus-5-5"
    - "Fennec:qwen3.8-27b"
  maintainers: [ikarys]
  review: "single maintainer reads every diff before it reaches main"
  security_response: "none"
  licence: MIT
---

# Vibecoding declaration

This repository follows its own [Vibecoding Convention](CONVENTION.md) version
0.1 at level 1. It holds text, templates and agent definitions, with no code to
test yet; the level will be revised when executable tooling is added.

## AI assistance

The convention text, the criticism survey and the templates are drafted by AI
assistants in conversation with the maintainer, who frames, argues, edits and
decides. Agent and skill definitions are drafted by a local model through
Fennec and reviewed in Claude Code. Most of the text is AI-written.

## Review

Single maintainer. Every change is read by the maintainer before it reaches
`main`. AI-written commits carry an `Assisted-by:` trailer naming the tool and
model.

## Provenance and licence

This repository is licensed under MIT. Its text is written with AI assistants
and may not be protected by copyright in some jurisdictions; the maintainer
accepts this. To the maintainer's knowledge, no verbatim reproduction of
licence-incompatible text was included. Safeguards used: human reading of
every change.

## Status

Active. This repository holds no running software; there is no security
response commitment.
