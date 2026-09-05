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
way a sequential number can, because it isn't allocated at merge time.

## Skeleton

Every decision uses exactly this skeleton, with H2 sections in this order:

```markdown
# <Decision title>

## Issue

<The problem requiring a decision, with links to the motivating issue and PRs.>

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- <Facts bounding the choice: environment, compatibility guarantees, prior
  decisions.>

## Argument

<Why the chosen direction beats the alternatives. Name any alternative that
shapes the choice, with its verdict (**Chosen.** / **Rejected.**), and reserve
full reasoning for Positions.>

## Architectural Decision

<The decision itself, as numbered clauses a reviewer can point at.>

## Positions

<Alternatives considered and rejected, each with its reason, or `N/A`.>

## Dates

- Published: TBD (set at merge).
```

`## Consequences` MAY appear between Architectural Decision and Positions
(rollout order, breaking changes, migration burden), and `## References` MAY
appear before Dates (links to the tracking issue, implementation PRs, related
decisions). H3 subsections are permitted inside Argument, Architectural
Decision, and Positions when they improve skimmability.

## Status lifecycle

Status is prose, and it moves with the commit that announces the transition
(see [Events](../../events/index.md)):

- **Proposed** (`decision: propose <id>`) — `This is a proposal that is
**awaiting review**.` with `- Published: TBD (set at merge).`
- **Accepted** (`decision: accept <id>`) — `This is a proposal that is
**accepted**.` and `- Published: YYYY-MM-DD` set to the merge date.
- **Implemented** (`decision: implement <id> (#PR)`) — `This is a proposal
that is **implemented**.`
- **Superseded** — `This proposal is **superseded** by
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).`

A merged decision is never rewritten silently; a later material edit appends
`- Updated: YYYY-MM-DD (<what changed>)` under Dates instead.

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

## Dates

- Published: 2026-02-11
```

**Anti-pattern:** editing an accepted decision in place. Reverse it with a new
decision that references and supersedes the old one — see
[wheredoc](../../routing/index.md#answer-in-order) for where the reversal's
reasoning belongs.

A [Plan](../plan/index.md) implements one accepted decision's steps, and a
[Todo](../todo/index.md) caches where that work left off.
