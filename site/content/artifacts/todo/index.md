---
title: Todo
description: Where this branch's work left off.
weight: 40
---

Where this branch's work left off.

- Paths: `TODO.md` at the repository root. There is no graduated form, for the
  same reason as the [Plan](../plan/index.md): it lives on one branch and is
  deleted rather than grown.
- Lifetime: one branch or worktree — the same lifetime as the Plan, because
  the cache outlives the session that wrote it, and that's exactly what makes
  it a handoff.
- Answers: where the work left off.

`TODO.md` is a cache of the list an agent or contributor is already keeping,
never a source and never a backlog. It carries the identity and revision of
its source — the session id and the time the list was taken — so a reader can
tell a live cache from one whose session is gone.

## Skeleton

```markdown
# Todo

- Session: <agent session id, or the agent's name when it has none>
- Synced: <YYYY-MM-DDTHH:MM:SSZ>
- Plan: `PLAN.md` — or `none` when the change needs no Plan

## Steps

- [x] A finished step
- [ ] The step in progress
- [ ] A step not started

## Notes

What the list cannot carry: a blocked step, a command that failed, a choice
made mid-flight. Omit this section when there is nothing to say.
```

## Example

`TODO.md`, alongside the [Plan](../plan/index.md) for
[2026-02-11-split-the-scheduler](../decisions/index.md):

```markdown
# Todo

- Session: agent-7f3a
- Synced: 2026-02-13T16:40:00Z
- Plan: `PLAN.md`

## Steps

- [x] Add the `scheduler` binary, polling `jobs` on the existing schema.
- [x] Wire its systemd unit into the deploy pipeline, dark.
- [ ] Flip the feature flag that stops the API process from polling.
- [ ] Remove the in-process poller and its now-dead config flag.

## Notes

Staging has been running the dark deploy since 2026-02-12; holding the flag
flip until a full day of job history confirms parity.
```

Write and refresh `TODO.md` whenever tracking a list at all — there's no
threshold below which the cache is skipped — and commit it at each checkpoint
with `todo: sync` (see [Events](../../events/index.md)). Delete it before
merge with `todo: clear`; `todo: sync` and `todo: clear` are a matched pair
for `TODO.md`, so dropping both from a branch's history leaves the tree
exactly as it was.
The file belongs to the branch, not the session: a later session on the same
branch reads it and takes it over.

**Anti-pattern:** treating a committed `TODO.md` as a durable backlog. A repo
whose existing `TODO.md` already means "durable backlog" has a
[Roadmap](../roadmap/index.md) under the wrong name, and renames it on
adoption — the two are told apart by lifetime: the Roadmap outlives every
branch, the Todo does not outlive the one it's on.
