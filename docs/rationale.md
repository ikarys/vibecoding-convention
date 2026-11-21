# Why the convention is shaped this way

The one-page text is [CONVENTION.md](../CONVENTION.md). This page keeps the
arguments behind it, so that the text itself can stay short. The criticisms
each requirement answers, with their sources, are in
[criticisms.md](criticisms.md).

## Outcomes, not forms

The convention says what must be visible, never which tool to use or how to
work. Where a check needs a form (a trailer wording, a report layout, a file
to read), the project declares the one it uses and the defaults are the
common ones. A fixed name is kept only where a reader or a tool needs an
anchor: the declaration, the status file, the audit index, and the files the
hosting platforms already recognise.

The same rule, seen from the other side: the convention measures the product,
not the process. It never asks how the code was produced, only what the
result shows. The one exception is the recorded registry check of a new
dependency inside T6: a hallucinated package name cannot be measured after
the fact, only caught before merge.

## Parity with human-led projects

Most requirements are what a careful human-led project of the same size is
already expected to have, and a human-led project can adopt the convention
unchanged. Six exist only because AI is involved: D1, D2, D3, D7, D10 and A3.
They are the price of a trust deficit, and all but A3 are text. The parity
test governs the convention's evolution: a requirement that a human-led
project would not be expected to meet is not added unless it is one of those.

In Part B, a human contributor is already expected to do all of C1 to C5; C1
is the only AI-specific line.

## Why three levels

Nobody is asked to reach level 3 for a weekend script. Level 1 is text only,
written in an afternoon, with no tool to install: it exists so that the
cheapest possible declaration is still a true one. Level 2 is what anyone
depending on the code would want to see in CI. Level 3 is for software that
handles other people's data, money or safety, where a reader needs an audit,
not a badge.

## Related work

Disclosure formats ([AI-DECLARATION.md](https://github.com/DimwitLabs/AI-DECLARATION.md),
whose keys D2 reuses, and several badge schemes) are self-declared and say so;
the convention adds the evidence behind the declaration. The `Assisted-by:` and
`Generated-by:` trailers come from the Linux kernel, Fedora, LLVM, OpenInfra,
OpenTelemetry and the Apache Software Foundation; D3 accepts them, and any
other trailer a project declares, without taking a side. The
[Vibe Coding Repository Standard](https://github.com/cz1993/vibe-coding-repository-standard)
organises an AI-built repository's governance files; it does not cover
disclosure, attribution, coverage, blind tests or versioned audits. Studies of
open source AI policies ([arXiv 2609.07542](https://arxiv.org/html/2609.07542),
[arXiv 2609.07919](https://arxiv.org/pdf/2609.07919)) describe how projects
regulate incoming AI contributions; D8 asks a vibecoded project to have such a
policy and respect those of others.

## Non-goals

The convention does not say whether to use AI, nor whether it is faster or
cheaper. It is not a certification. It does not watch every commit; audits
happen at releases and structural changes. It never requires that a human
wrote something, only that a human validated it. It does not address the
effect of AI on developers' skills. Where a requirement makes no sense for a
project, the declaration says so and why, and the auditor reports the
exception rather than the requirement.
