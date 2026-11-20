# Contributing

## AI assistance

AI tools are welcome here, and you say so. A pull request or commit with
AI-written content ends with one trailer per model:

    Assisted-by: <Tool>:<model>

Never name a model in `Signed-off-by` or any other certificate of origin. A
contribution without a trailer is understood as human-written.

## What a pull request must contain

- Disclosure (C1): the trailer above, and the tool and model in the
  description.
- Understanding (C2): you can explain every change in the diff, what it does,
  why it is there and what it affects.
- Completeness (C3): a behaviour change comes with its test and its
  documentation, and the description shows how you ran the tests and the
  result.

One change per pull request. Unsolicited generated changes (mass refactors,
style passes, rewrites nobody asked for) are closed.

## Review

A human reviews every change before merge. Answer review comments within
<N days>; an unanswered AI-assisted pull request may be closed.

## Setup

<commands to build, test and lint, the same as in `VIBECODING.md`>
