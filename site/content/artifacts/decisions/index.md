---
title: Decisions
description: What changed, why, and what it cost.
weight: 60
---

What changed, why, and what it cost.

- Paths: none at the root — a decision is always its own file at
  `docs/decisions/YYYY-MM-DD-slug.md`, from the first decision a repository
  records, whether or not the rest of the repository has graduated.
- Lifetime: append-only — a decision is never rewritten, only superseded.
- Answers: what changed, why, what it cost.

## The id

A decision's id is `YYYY-MM-DD-slug` — the date the record was written plus a
kebab-case slug of its title — and it is the filename without `.md`. The id is
fixed at creation: it is never re-dated, renumbered, or renamed, not when the
decision's status changes and not when a later decision supersedes it. Several
decisions may share a date; their slugs tell them apart. A date can't race the
way a sequential number can, because it isn't allocated at merge time. Never
add a counter or suffix, and never renumber to settle a merge; a same-date
slug collision is resolved by choosing a different slug.

## Skeleton

Every decision uses exactly this skeleton, with H2 sections in this order:

```markdown
# <Decision title>

## Issue

<The problem requiring a decision, with links to the motivating issue and PRs.
If this decision extends another, the first sentence is `This decision extends
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` and nothing from that decision is
restated.>

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- <Facts bounding the choice: environment, compatibility guarantees, prior
  decisions. For a change to a public surface, state the compatibility
  guarantee it has to keep here.>

## Argument

<Why the chosen direction beats the alternatives. Name any alternative that
shapes the choice, with its verdict (**Chosen.** / **Rejected.**), and reserve
full reasoning for Positions. `N/A` is acceptable when the constraints make
the decision self-evident.>

## Architectural Decision

<The decision itself, as numbered clauses a reviewer can point at. Include
code or YAML only where it pins down a contract — a field name, a struct
variant, one representative manifest — never to reproduce the
implementation.>

## Positions

<Alternatives considered and rejected, each with its reason, or `N/A`.>
```

`## Consequences` MAY appear between Architectural Decision and Positions
(rollout order, breaking changes, migration burden, follow-up documentation
owed); `## References` MAY appear after Positions (bulleted links with `—`
descriptions — tracking issue, implementation PRs, related decisions,
external specs); and `## Errata` MAY appear last, after References, added
only once the record is frozen. H3 subsections are permitted inside
Argument, Architectural Decision, and Positions when they improve
skimmability.

## Status lifecycle

A record has four states and moves through them in one direction:
**draft → proposed → accepted | rejected**. Each transition is a commit
(see [Events](../../events/index.md)), and `## Status` carries exactly one
line:

- **draft** (`decision: draft <id>`) — `This is a **draft**; it is not ready
for review.` The record exists and is being written; it is not the spec,
  and nothing may be planned against it. The state is optional — a record
  written in one sitting is proposed directly — but it is the only honest
  status for a record committed before it is ready to be read.
- **proposed** (`decision: propose <id>`) — `This is a proposal that is
**awaiting review**.` It is the spec.
- **accepted** (`decision: accept <id>`) — `This is a proposal that is
**accepted**.`
- **rejected** (`decision: reject <id>`) — `This proposal was **rejected**.`
  A rejected record stays in the log: it says what was considered and why it
  was refused.

`accept` and `reject` end review, and that commit is the last write to the
record's body. A record carries no dates: its date is its id, and every
other date it could carry is a commit date `git log` already holds. There is
no `implemented`, `superseded`, or `deprecated` status — what shipped is the
[Changelog](../changelog/index.md)'s question, and `plan: done <id>` already
announces that an accepted decision's work is finished.

### Errata

A frozen record is corrected by appending to `## Errata`, its last section,
one dated line per entry, newest last:

```markdown
## Errata

- 2026-03-04: The second clause named `--strict`; the flag shipped as
  `--pedantic`. The decision is unchanged.
```

Exactly two kinds of entry are admissible: a correction of fact or
expression that leaves the decision itself unchanged, and a pointer to a
record that supersedes or extends this one. Anything that changes the
decision is a new decision. Never edit an existing erratum, and never edit
the body above the heading.

Supersession is stated twice — in the superseding record's Issue, and as an
erratum on the superseded record. Neither record's status changes.

### Cross-linking

An extension decision opens Issue with `This decision extends
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` In the same PR, give the extended
record its reciprocal link: a draft or proposed record is edited in place,
adding `This decision is extended by
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` under its Issue heading, while a
frozen record receives an erratum instead. Reference decisions from prose as
`[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md)`, with relative links.

## Example

`docs/decisions/2026-02-11-split-the-scheduler.md`:

```markdown
# Split the scheduler out of the API process

## Issue

Job scheduling runs inside the API process, so a slow query in one job's
handler blocks unrelated HTTP requests. See #142.

## Status

This is a proposal that is **accepted**.

## Assumptions and Constraints

- Jobs are already queued through `jobs.Enqueue`; nothing calls the scheduler
  directly.
- The API process's deploy pipeline is reused as-is — this decision does not
  introduce a new deployment target.

## Argument

Run the scheduler as its own process, sharing the job queue table.
**Chosen.** It isolates a slow job from API latency at the cost of one more
process to deploy. Keeping it in-process (**Rejected**, see Positions) leaves
the coupling that caused #142 in place.

## Architectural Decision

1. A new `scheduler` binary polls the `jobs` table and dispatches due jobs;
   the API process only enqueues.
2. Both processes share `jobs.Enqueue`'s schema; no new table is introduced.
3. The `scheduler` binary is deployed alongside the API process in the same
   pipeline, as its own systemd unit.

## Consequences

The API process no longer needs the `scheduler` package as a runtime
dependency, only as a schema definition. Rollout is additive: the new binary
ships first, dark, before the in-process poller is removed in a follow-up PR.

## Positions

- **Keep the scheduler in-process, on its own goroutine.** _Rejected._ Still
  shares the process's memory and CPU budget, so a slow job still affects
  request latency under load.

## Errata

- 2026-03-02: Superseded by
  [2026-03-02-drop-plugin-api](./2026-03-02-drop-plugin-api.md).
```

**Anti-pattern:** editing an accepted decision in place. A correction is an
erratum; a change of mind is a new decision that supersedes it — see
[wheredoc](../../routing/index.md#answer-in-order) for where the reversal's
reasoning belongs.

A [Plan](../plan/index.md) implements one accepted decision's steps, and a
[Todo](../todo/index.md) caches where that work left off.
