---
title: Plan
description: The exact steps for the current decision.
weight: 30
---

The exact steps for the current decision.

- Paths: `PLAN.md` at the repository root. There is no graduated form —
  neither trigger in [Graduating](../../graduating/index.md) can fire for a
  document that lives on one branch and is deleted rather than grown.
- Lifetime: one branch or worktree — deleted no later than merge.
- Answers: exact steps for the current decision.

A Plan implements exactly one accepted decision. It's committed for backup and
handoff, and deleted in the same branch before merge, in a `plan: done <id>`
commit (see [Events](../../events/index.md)).

## Skeleton

```markdown
# Plan: YYYY-MM-DD-slug

Decision: <link or path to the accepted decision>

## Steps

- [ ] First step
- [ ] Second step

## Verification

The commands or checks that prove the work is done.

## Status

Where this branch left off, for whoever picks it up next.
```

## Example

`PLAN.md`, implementing
[2026-02-11-split-the-scheduler](../decisions/index.md):

```markdown
# Plan: 2026-02-11-split-the-scheduler

Decision: docs/decisions/2026-02-11-split-the-scheduler.md

## Steps

- [x] Add the `scheduler` binary, polling `jobs` on the existing schema.
- [x] Wire its systemd unit into the deploy pipeline, dark.
- [ ] Flip the feature flag that stops the API process from polling.
- [ ] Remove the in-process poller and its now-dead config flag.

## Verification

`go test ./scheduler/...` and a staging deploy with the flag flipped, watched
for a full job cycle before flipping it in production.

## Status

Dark deploy is live in staging; waiting on one full day of job history before
flipping the flag in production.
```

**Anti-pattern:** leaving a Plan committed after the work merges. It is
scaffolding for one branch, not a durable record — the
[Decision](../decisions/index.md) is the record, and the
[Todo](../todo/index.md) is the other file that leaves with it.
