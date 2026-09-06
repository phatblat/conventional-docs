---
title: Events
description: Publish lifecycle activity as Conventional Commits, so the repo stays the record.
---

_Status: proposed. This is where the convention is considering putting the
event vocabulary; the vocabulary itself is in use. See
[the decision](https://github.com/phatblat/conventional-docs/blob/main/docs/decisions/2026-09-05-give-events-their-own-artifact.md)._

- Paths: `EVENTS.md` (small repo) or `docs/events.md` (graduated).
- Lifetime: living — the vocabulary grows as a repo announces new event
  types.
- Answers: which lifecycle events the repo's commits announce.

Lifecycle transitions are commits with
[Conventional Commits](https://www.conventionalcommits.org/) types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing
files:

```text
decision: draft 2026-02-11-split-the-scheduler
decision: propose 2026-02-11-split-the-scheduler
decision: accept 2026-02-11-split-the-scheduler
decision: reject 2026-02-11-split-the-scheduler
plan: start 2026-02-11-split-the-scheduler
plan: done 2026-02-11-split-the-scheduler
todo: sync
todo: clear
release: v1.2.0
deploy: prod v1.2.0
```

- `decision: draft <id>` — a record is being written; it is not the spec yet.
- `decision: propose <id>` — a decision record is submitted for review; it is
  the spec.
- `decision: accept <id>` — review ended and the record's body is frozen.
- `decision: reject <id>` — review refused the proposal; the record stays in
  the log.
- `plan: start <id>` — a Plan for the accepted decision is on a branch.
- `plan: done <id>` — the Plan's steps are finished and its file is deleted.
- `todo: sync` — `TODO.md` is refreshed from the agent's live list.
- `todo: clear` — `TODO.md` is deleted; the session's list is done with.
- `release: v<version>` — a version shipped.
- `deploy: <environment> v<version>` — a version reached an environment.

`<id>` is a decision's id: its date plus a kebab-case slug of its title,
`YYYY-MM-DD-slug`. It is the whole reference — the slug is the title, so
`propose` carries no second copy of it and the subject stays inside
Conventional Commits' 100-character header limit.

`todo: sync` and `todo: clear` carry no id, because the cache belongs to a
session rather than a decision. A `plan:` or `todo:` commit touches only its
own artifact, which is what makes each pair net-zero and the bookkeeping
mechanically removable.

## What consumes them

- **Dashboards** query `git log --grep` for these subjects instead of
  scraping issue trackers.
- **Chat notifications** fire off a commit webhook and link back to it.
- **Release automation** treats a `release:` commit as the point where
  `[Unreleased]` in [`CHANGELOG.md`](../artifacts/changelog/index.md)
  becomes the new version.
- **[`condoc`](../condoc/index.md)** writes these commits — each event has a
  subcommand that spells it.

## Wiring it into commitlint

```js
'type-enum': [
  2,
  'always',
  [
    'build', 'chore', 'ci', 'decision', 'deploy', 'docs', 'feat', 'fix',
    'perf', 'plan', 'refactor', 'release', 'revert', 'style', 'test',
    'todo',
  ],
],
```

A repo that announces its own events adds them here and to `type-enum` in
the same commit: this file is the human-facing source, the config is what
rejects a bad subject.

## The doorbell rule

Notifications are doorbells: they say where to look, never what to do. The
commit is the event of record; a notification MUST NOT carry state that isn't
also in the repository.
