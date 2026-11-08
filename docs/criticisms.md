# Criticisms of vibecoding, and what the convention answers

Working document for version 0.1. Last changed 2026-11-08.

A convention worth adopting has to answer the criticisms people actually make,
with evidence anyone can check. This page lists the main criticisms found in
the literature, the press and maintainers' own statements, says what the
convention already answers, and names the gaps. Each gap becomes a proposed
requirement, marked **NEW**, to be folded into `CONVENTION.md` once agreed.

Figures are quoted with their source. Many come from security vendors or
single blog posts; the most defensible ones are from peer-reviewed or
large-sample studies, and the text says which is which.

## Summary table

| # | Criticism | Convention today | Gap | Proposed |
|---|---|---|---|---|
| 1 | AI code ships vulnerabilities, and real apps leak data | A2 security audit, T6 dependency checks | Security only at level 3; nothing on data exposure by default | **NEW T7** secrets and static analysis at level 2; A2 adds an authorization check on every data path |
| 2 | Nobody understands the code; comprehension debt | A1 architecture review | Nothing proves a human understands the system | **NEW A6** human-written architecture overview and decision records, checked against the code |
| 3 | Duplication, churn and technical debt grow; god files | A1 | No measurement | **NEW T8** maintainability signals (duplication, complexity, function and file length) with declared tool and thresholds; A1 names the design principles it judges against |
| 4 | AI tests assert nothing; agents weaken tests to pass CI | T3 sampled review | Judgement only; no objective measure; no rule on test removal | T3 strengthened; **NEW A7** mutation score recommended at level 3; **NEW D6** test changes are explained |
| 5 | Hallucinated packages, slopsquatting, supply chain | T6 | New dependencies not covered | T6 extended: lockfile, every new dependency checked against the registry before merge, by tool, agent or human |
| 6 | Copyright, licence and provenance are unknowable | D3 trailers | No provenance statement | D3 accepts any declared trailer and notes the `Assisted-by` practice; **NEW D7** provenance and licence statement |
| 7 | AI slop floods maintainers; drive-by PRs and imagined bug reports | none | Convention only covers the project's own code; nothing addresses the individual who sends one contribution | **NEW D8** contribution policy on the receiving side; **NEW Part B (C1 to C5)** on the sending side: disclosure, diff read, tests and docs with the change, reproduction for reports, target policy respected, follow-up |
| 8 | Nobody is accountable | D2 maintainers, D4 human review, "Humans decide" | Release not signed by a named human | A5 adds "released by" a named human |
| 9 | Projects are abandoned fast; badges go stale | D5 dated check (proposed) | No project status | **NEW D9** project status and maintenance statement |
| 10 | Agent tooling is itself an attack surface | none | Agent config, skills and plugins not treated as code | **NEW A8** agent configuration is code: reviewed, versioned, pinned, no secrets |
| 11 | Docs describe intent, not behaviour | A3 blind test, `DOC-WRONG` severity | none | keep |
| 12 | Hidden AI use | D1, D2, D3, D5 | none | keep |
| 13 | Over-confidence; perceived versus measured productivity | principle "Evidence over claims" | Not a property of the artifact | Non-goal: the convention measures the product, not the process |
| 14 | Deskilling of developers | none | Out of scope for an artifact standard | Partly answered by A6; say so in non-goals |
| 15 | Frozen in time: models pick the library versions and APIs they memorised; projects start behind and stay behind | T6 (known vulnerabilities only) | No requirement on currency or on keeping up | **NEW T9** dependency freshness: current version at addition, update mechanism, measured lag, exceptions with a reason, no abandoned dependency |

## 1. Security: AI code ships vulnerabilities, and real apps leak data

**The criticism.** Veracode's 2025 GenAI Code Security Report tested over 100
models on 80 tasks and found that AI-generated code introduced a vulnerability
in about 45 % of cases, with no improvement across later testing cycles
([CSA note](https://labs.cloudsecurityalliance.org/research/csa-research-note-ai-generated-code-security-vibe-coding-202/)).
A controlled study found developers using an assistant submitted more insecure
code and were more confident about it. Security degrades with iteration: a
study that had a model modify code up to 40 times saw vulnerabilities
accumulate ([OX Security](https://www.ox.security/blog/vibe-coding-security/)).
In the field: a scan of 1,645 live Lovable apps found 170 or more with
Supabase Row Level Security off, exposing user data (CVE-2025-48757, CVSS
9.3); a later scan reported 7,403 of 159,041 apps exposing e-mail data
([thecybersecguru](https://thecybersecguru.com/news/lovable-supabase-rls-data-exposure/),
single source for the second figure). Escape.tech scanned 5,600 vibe-coded
apps and found over 2,000 vulnerabilities and 400 or more exposed secrets
([Cycode](https://cycode.com/blog/vibe-coding-security/)). The first
large-scale academic study of real vibe-coded applications is
[arXiv 2606.23130](https://arxiv.org/pdf/2606.23130).

**What the convention answers.** A2 (security audit, claims traced to tests),
T6 (dependency vulnerability check), A5 (no release with an open blocker).

**Gaps.** Security starts at level 3. The cheapest, highest-yield checks
(secrets in the repository, static analysis) belong at level 2, where most
libraries and tools sit. The recurring field failure is not a subtle bug but a
missing authorization rule on a data store or endpoint: the convention should
name that check.

**Proposed.**
- **NEW T7. Secrets and static analysis.** Level 2. A secrets scanner and a
  static analyser for the language run in CI; findings are fixed or listed as
  accepted with a reason. Evidence: CI configuration and last run.
- **A2 extended.** The security audit includes a data exposure check: every
  data store, table, bucket and endpoint that holds user data has an
  authorization rule, and a test that an unauthorized caller is refused.
- **A2 extended.** Security claims are re-verified at each release, because
  security degrades with iteration (already implied by "per minor version",
  now stated).

## 2. Comprehension debt: nobody understands the code

**The criticism.** Developers who vibecode rely on regenerating output rather
than understanding it, which erodes ownership and knowledge retention
([arXiv 2512.11922](https://arxiv.org/pdf/2512.11922)). A 2026 paper argues
that AI generation invalidates authorship-based knowledge metrics altogether
([arXiv 2606.20882](https://arxiv.org/pdf/2606.20882)). The practical form of
the criticism: when the system breaks, no human can explain how it works.

**What the convention answers.** A1 asks an agent whether the architecture is
production-grade. It does not ask whether a human understands it.

**Gap.** The convention has no evidence that a human holds the model of the
system in their head.

**Proposed.**
- **NEW A6. Human-held understanding.** Level 3. The repository has an
  architecture overview (`ARCHITECTURE.md` or a docs page) written or
  rewritten by a human maintainer, naming the components, the data flow, the
  main invariants and the known weak points, and decision records
  (`docs/adr/`) for the structural choices, each stating who decided and why.
  Evidence: the files, with human authorship visible (no `Assisted-by`
  trailer on the commit that wrote them, or the trailer plus a stated human
  rewrite). Checked: the architecture reviewer compares the overview to the
  code and reports every mismatch; an overview that describes a system other
  than the one in the repository is a `DOC-WRONG` finding.

## 3. Maintainability: duplication, churn and technical debt

**The criticism.** GitClear's analysis of 211 million changed lines reports
code duplication rising from 8.3 % to 12.3 % between 2021 and 2024, refactoring
falling from 25 % of changed lines to under 10 %, and churn rising from 3.1 % to
5.7 % ([Keyhole Software](https://keyholesoftware.com/vibe-coding-trends-2026/),
citing GitClear). A widely repeated "30 to 41 % more technical debt" figure
could not be traced to a primary study and is not used here.

**What the convention answers.** A1, by judgement.

**Gap.** No measurement, so no trend. A project cannot show that its debt is
under control, only that an agent thought so once.

**Proposed.**
- **NEW T8. Maintainability signals.** Level 2. The project declares a tool
  and thresholds for at least duplication and cyclomatic (or cognitive)
  complexity, runs it in CI or at release, and keeps the numbers in the audit
  index so the trend is visible. Thresholds are the project's own; the
  convention only asks that they are declared, measured and not exceeded.
  Evidence: the tool output. Checked: the auditor runs the tool.

## 4. Tests that test nothing, and tests that get weakened

**The criticism.** A 2026 study of agent-generated tests in open-source
projects found 11.58 % ambiguous or unclassified assertions against 1.46 % in
human-written tests, and higher flakiness
([arXiv 2607.12068](https://arxiv.org/html/2607.12068v1)). Another study found
many generated tests behave as observational probes rather than regression
tests, and prompting for more tests did not change outcomes
([arXiv 2609.04681](https://arxiv.org/pdf/2609.04681)). Practitioners add a
second failure that no study measures yet: an agent told to make CI green
weakens or deletes the failing test instead of fixing the code.

**What the convention answers.** T3, by sampling.

**Gaps.** Sampling is a judgement and does not scale. Nothing stops a test
from being deleted in the same change that broke the behaviour it covered.

**Proposed.**
- **T3 strengthened.** The auditor reports the share of sampled tests with no
  meaningful assertion, and any test that passes with the implementation
  stubbed out.
- **NEW A7. Mutation score.** Level 3, recommended. The project runs a mutation
  testing tool on its core modules and declares the score and threshold.
  Mutation testing is the objective answer to "do these tests detect
  anything": it measures whether the tests fail when the code is changed.
- **NEW D6. Test changes are explained.** Level 1. A commit or pull request
  that deletes a test, weakens an assertion, or marks a test as skipped states
  why in its message, and the reviewer approves that statement. Checked: the
  auditor lists test removals since the last audit without a stated reason.

## 5. Supply chain: hallucinated packages and slopsquatting

**The criticism.** Models invent package names; across 16 models roughly one in
five suggested packages did not exist, and the same names recur, so attackers
register them ([Trend Micro](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/slopsquatting-when-ai-agents-hallucinate-malicious-packages),
[Snyk](https://snyk.io/articles/package-hallucinations/)). A reported January
2026 case had a hallucinated npm package spread to 237 repositories through
AI-generated agent skill files ([anomity](https://anomity.ai/blog/slopsquatting-hallucinated-package-installs/),
single source).

**What the convention answers.** T6, for known vulnerabilities and licences.

**Gap.** T6 checks the dependencies you have. The attack is in the dependency
you are about to add.

**Proposed.**
- **T6 extended. Dependency provenance.** Lockfiles are committed. Every new
  dependency is checked against the registry before merge, by a tool, an agent
  or a human: it
  exists on the registry under exactly that name, it is established (age,
  maintainers, download trend) or the reason to take a young one is stated,
  and it is the package the code actually needs. The check is recorded in the
  commit or pull request. Checked: the auditor lists dependencies added since
  the last audit and whether each has a recorded check.

## 6. Copyright, licence and provenance

**The criticism.** The GCC steering committee rejects AI-generated
contributions above 15 lines to protect GPL copyright integrity; Codeberg's
members voted to ban mostly AI-generated repositories
([Northeast Times](https://northeasttimes.com/2026/07/31/gcc-bans-ai-generated-code-over-gpl-copyright-fears/)).
In a study of open-source AI policies, 57 % of projects cited copyright and
licence uncertainty, noting that provenance of generated code is unknowable
and so a Developer Certificate of Origin cannot be honestly certified
([arXiv 2609.07919](https://arxiv.org/pdf/2609.07919)). Courts have indicated
that AI output without meaningful human input may not be copyrightable; vendor
terms disclaim infringement in outputs
([crashoverride](https://crashoverride.com/resources/knowledge-base/ai-code-governance/ai-coding-license-risk)).
`Co-Authored-By` for a model implies legal personhood a model does not have;
the kernel, Fedora, LLVM, OpenInfra, OpenTelemetry and Rocky Linux use
`Assisted-by` ([All Things Open](https://allthingsopen.org/articles/open-source-ai-contributions-assisted-by-git-trailer-standard)).

**What the convention answers.** D3 attributes commits to a model.

**Gaps.** The trailer name is the wrong one. Nothing states what the project
believes about the copyright status of its AI-written parts or what it did to
avoid reproducing licence-incompatible code.

**Proposed.**
- **D3 changed.** A machine-readable trailer naming tool and model on every
  commit with AI-written content; the project declares which trailer it uses
  (`Assisted-by`, `Generated-by`, `Co-Authored-By`, or another). The convention
  records that large projects converged on `Assisted-by` and why, without
  making it a rule. A human signs `Signed-off-by` where a DCO applies; the
  model never does.
- **NEW D7. Provenance and licence statement.** Level 1. `VIBECODING.md`
  states: the project licence; that AI-written parts may not be protected by
  copyright and the maintainers accept this; that no verbatim reproduction of
  licence-incompatible code was knowingly included; and which safeguards were
  used, if any (vendor code-reference filters, snippet scanning). Checked: the
  statement is present; where a snippet scanner is declared, the auditor runs
  it.

## 7. AI slop and the maintainer burden

**The criticism.** In the Linux Foundation's October 2026 survey, maintainers
estimate a quarter of incoming pull requests are low-quality AI output, 31 % on
larger projects ([The Register](https://www.theregister.com/ai-and-ml/2026/10/21/ai-cuts-coding-time-but-its-also-filling-open-sources-inbox-with-crap/5304389)).
curl closed its bug bounty after a flood of AI-written reports, about 20 % of
them hallucinating vulnerabilities; tldraw and Ghostty close drive-by pull
requests ([The New Stack](https://thenewstack.io/ai-generated-code-crisis/)).
CodeRabbit measured about 1.7 times more issues in AI co-authored pull requests
([CodeRabbit](https://www.coderabbit.ai/blog/ai-is-burning-out-the-people-who-keep-open-source-alive)).

**What the convention answers.** Nothing. The convention covers the code a
project ships, not the contributions it receives or sends.

**Gap.** A vibecoded project that wants to be taken seriously has to say how
it handles AI contributions in both directions, and must not make another
maintainer pay for its tooling.

**Proposed.**
- **NEW D8. Contribution policy.** Level 1. `CONTRIBUTING.md` states the
  project's policy for incoming AI-assisted contributions (accepted with
  disclosure, accepted with conditions, or refused) and asks for C1 to C3 in
  pull requests. The agent or tool that produced a change is never its only
  reviewer; a human approves. Checked: the file and the pull request template
  exist; the auditor samples merged pull requests for a human approval.
- **NEW Part B, C1 to C5.** The convention as first drafted spoke only to
  projects. The slop problem is made by individuals sending one contribution
  each to projects they do not maintain, and the worst of it, the imagined
  vulnerability report, has no repository at all. Part B gives those five
  rules a name and an identifier that a maintainer can point to: disclosure,
  the diff was read, tests and documentation come with the change and reports
  come with a reproduction, the target's policy is respected, the author
  answers review. It costs nothing to a project and needs no declaration.

## 8. Accountability

**The criticism.** When AI-written code fails, vendor terms put liability on
whoever shipped it; "the AI did it" is not an answer a user, a court or a
downstream maintainer accepts ([reptile.haus](https://reptile.haus/journal/ai-code-liability-gap-ownership-eu-product-liability-2026/)).

**What the convention answers.** The principle "Humans decide", D2 (named
maintainers), D4 (human review), the `author` field on every report.

**Gap.** A release is the moment accountability is exercised and the
convention does not name who exercised it.

**Proposed.**
- **A5 extended.** Each release names the human who approved it, in the
  release notes or tag message, and that person is one of the maintainers
  listed in `VIBECODING.md`.

## 9. Abandonment and stale claims

**The criticism.** Vibecoded projects are said to be abandoned quickly; the
widely quoted "3.2 times more likely" figure has no traceable source
([digitalbiztalk](https://digitalbiztalk.com/article/vibe-coding-is-killing-open-source-the-2026-developer-crisis))
and is not relied on here. The sound part of the criticism: a badge or a
charter that nobody has checked for a year is worse than none.

**What the convention answers.** D5, the dated check with a stale state
(proposed in discussion, not yet in the text).

**Gap.** No statement of whether the project is maintained at all.

**Proposed.**
- **NEW D9. Project status.** Level 1. `VIBECODING.md` states the project
  status (active, maintenance only, archived) and the expected response time
  for security reports, matching `SECURITY.md`. The dated check (D5) marks the
  badge stale when the last check is older than the declared interval or than
  the last release.

## 10. The agent tooling is an attack surface

**The criticism.** Agent instruction files, skills and plugins are executed by
tools with broad permissions. The hallucinated-package case above spread
through AI-generated skill files; prompt injection through repository content
is documented by every agent vendor. Secrets end up in prompts and in
agent configuration ([CSA note](https://labs.cloudsecurityalliance.org/research/csa-research-note-ai-generated-code-security-vibe-coding-202/)).

**What the convention answers.** Nothing.

**Proposed.**
- **NEW A8. Agent configuration is code.** Level 3. Agent instruction files
  (`AGENTS.md`, `CLAUDE.md`, `.cursorrules` and the like), skills, hooks and
  plugin lists are versioned, reviewed like code, and contain no secrets.
  Third-party skills and plugins are pinned to a version and listed with
  their source. Agents run with the least permissions the task needs; the
  project says which sandbox or permission model it uses. Checked: the
  security auditor reads these files and lists unpinned or unreviewed
  third-party items and any secret.

## 11. Documentation describes intent, not behaviour

**The criticism.** AI-written docs describe what the code was asked to do. The
code may do something else.

**What the convention answers.** A3 (blind test) is built for this; the
`DOC-WRONG` severity names it and blocks a release. Keep.

## 12. Hidden AI use

**The criticism.** Disclosure is rare in repositories; where it exists it is in
pull request descriptions, not in the artifact
([arXiv 2609.07542](https://arxiv.org/html/2609.07542)).

**What the convention answers.** D1, D2, D3 and D5. Keep.

## 13. Over-confidence and the productivity perception gap

**The criticism.** In METR's 2025 trial experienced developers were about 19 %
slower with AI tools while believing they were 20 % faster; the 2026
follow-up points the other way but METR itself calls the data unreliable
([ingenire](https://ingenire.com/blog/metr-2026-developer-productivity-study)).

**What the convention answers.** This is a criticism of the process, not of
the artifact. The convention's answer is its first principle: measure the
product, do not believe the feeling.

**Proposed.** Add to non-goals: the convention says nothing about whether
vibecoding is faster or cheaper. It says whether the result is sound.

## 14. Deskilling

**The criticism.** Novices do not learn when the assistant does the work;
experts cannot self-detect decline.

**What the convention answers.** Partly, through A6: a human must be able to
write and defend the architecture overview. Beyond that, out of scope for an
artifact standard. Say so in non-goals.

## 15. Frozen in time: the model's library versions, not today's

**The criticism.** A model has a training cutoff. Asked for a dependency it
names the version and the API it memorised, which may be a year or more old,
deprecated, or abandoned. Regenerating code later repeats the choice. The
project is behind from its first day, and the gap grows with every prompt.
This happened while writing this convention's own tooling: the model chose
`serde_yaml` 0.9, unmaintained since 2024, and only the dependency audit
caught it.

**What the convention answers.** T6 checks for known vulnerabilities and
licences, not for currency.

**Gap.** Nothing asks that versions be current when added, nor that they be
kept current, nor that a dependency held back be justified.

**Proposed.**
- **NEW T9. Dependency freshness.** Level 2. The version taken at addition is
  the registry's current one; an update mechanism exists (bot, or declared
  cadence and command); the lag is measured and compared to declared limits;
  dependencies held back are listed with a reason; no abandoned dependency
  without such a reason. "Always the latest" is not the rule, since blind
  updates break things too; "no unjustified lag" is. Parity: a human-led
  project is already expected to keep its dependencies current.

## Changes to the level structure

If the proposals above are accepted, level 2 grows by T7, T8, T9 and the
extended T6; level 1 grows by D6, D7, D8 and D9 (all text, no tooling); level 3
grows by A6, A7 and A8. Level 1 stays achievable in an afternoon. Level 2
adds three CI steps. Level 3 adds one human-written document and one optional
measurement.

## Sources

- [CSA, Vibe coding security crisis](https://labs.cloudsecurityalliance.org/research/csa-research-note-ai-generated-code-security-vibe-coding-202/)
- [CSA, AI-generated CVE surge](https://labs.cloudsecurityalliance.org/research/csa-research-note-ai-generated-code-vulnerability-surge-2026/)
- [OX Security, Vibe coding security](https://www.ox.security/blog/vibe-coding-security/)
- [Cycode, Vibe coding security](https://cycode.com/blog/vibe-coding-security/)
- [arXiv 2606.23130, (In)Security of vibe-coded applications](https://arxiv.org/pdf/2606.23130)
- [Lovable / Supabase RLS exposure](https://thecybersecguru.com/news/lovable-supabase-rls-data-exposure/)
- [arXiv 2512.11922, Vibe coding in practice](https://arxiv.org/pdf/2512.11922)
- [arXiv 2606.20882, The substrate collapse](https://arxiv.org/pdf/2606.20882)
- [Keyhole Software, Vibe coding trends 2026 (GitClear figures)](https://keyholesoftware.com/vibe-coding-trends-2026/)
- [arXiv 2607.12068, Beyond test presence](https://arxiv.org/html/2607.12068v1)
- [arXiv 2609.04681, Beyond code generation](https://arxiv.org/pdf/2609.04681)
- [Trend Micro, Slopsquatting](https://www.trendmicro.com/vinfo/us/security/news/cybercrime-and-digital-threats/slopsquatting-when-ai-agents-hallucinate-malicious-packages)
- [Snyk, Package hallucinations](https://snyk.io/articles/package-hallucinations/)
- [Northeast Times, GCC bans AI-generated code](https://northeasttimes.com/2026/07/31/gcc-bans-ai-generated-code-over-gpl-copyright-fears/)
- [arXiv 2609.07919, AI contribution policies in OSS](https://arxiv.org/pdf/2609.07919)
- [arXiv 2609.07542, Landscape of AI policies](https://arxiv.org/html/2609.07542)
- [crashoverride, AI coding licence risk](https://crashoverride.com/resources/knowledge-base/ai-code-governance/ai-coding-license-risk)
- [All Things Open, Assisted-by trailer](https://allthingsopen.org/articles/open-source-ai-contributions-assisted-by-git-trailer-standard)
- [The Register, AI slop in open source inboxes](https://www.theregister.com/ai-and-ml/2026/10/21/ai-cuts-coding-time-but-its-also-filling-open-sources-inbox-with-crap/5304389)
- [The New Stack, AI-generated code crisis](https://thenewstack.io/ai-generated-code-crisis/)
- [CodeRabbit, AI is burning out maintainers](https://www.coderabbit.ai/blog/ai-is-burning-out-the-people-who-keep-open-source-alive)
- [reptile.haus, AI code liability gap](https://reptile.haus/journal/ai-code-liability-gap-ownership-eu-product-liability-2026/)
- [ingenire, METR 2026 study](https://ingenire.com/blog/metr-2026-developer-productivity-study)
