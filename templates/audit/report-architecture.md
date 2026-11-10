---
date: <YYYY-MM-DD>
commit: <hash>
kind: architecture
tool: <Tool>
model: <model>
scope: <what was reviewed: whole repository, or the named crates/modules>
verdict: <prototype | usable with listed gaps | production-grade>
---

# Architecture review

<Summary paragraph: what the system is, what was read, the verdict and the
two or three reasons for it. Then agreement with the architecture documents of
A6: does `ARCHITECTURE.md` describe the code that exists?>

Principles judged against: one responsibility per module and type; dependencies
pointing inward (domain logic without IO, persistence apart from transport,
authorization as a policy); no god object or god file; no single-use layer or
abstraction kept for a future that has not come; names that say what things do.
Simplicity wins over speculative structure.

## Finding 1: <imperative title of the change>
- Severity: <BLOCKER | DOC-WRONG | PAINFUL | MINOR>
- Requirement: A1
- Principle: <which one>
- Where: `path/to/file.rs:120`
- Problem: <what is wrong, in two or three sentences>
- Fix: <the concrete change>

## Finding 2: ...

## Decisions

Filled by the maintainer. One row per finding.

| # | Decision | Commit or reason |
|---|---|---|
| 1 | <fixed | documented | won't fix | open> | <hash or reason> |
| 2 | | |
