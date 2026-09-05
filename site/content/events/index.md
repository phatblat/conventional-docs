---
title: Events
description: Publish lifecycle activity as Conventional Commits, so the repo stays the record.
---

_Status: proposed. This is where the convention is considering putting the
event vocabulary; the vocabulary itself is in use. See
[the decision](https://github.com/phatblat/conventional-docs/blob/main/docs/decisions/2026-09-05-give-events-their-own-artifact.md)._

Lifecycle transitions are commits with
[Conventional Commits](https://www.conventionalcommits.org/) types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing
files:

```text
decision: propose 2026-02-11-split-the-scheduler
decision: accept 2026-02-11-split-the-scheduler
decision: implement 2026-02-11-split-the-scheduler (#88)
plan: start 2026-02-11-split-the-scheduler
plan: done 2026-02-11-split-the-scheduler
todo: sync
todo: clear
release: v1.2.0
deploy: prod v1.2.0
```

- `decision: propose <id>` — a decision record is written; it is the spec.
- `decision: accept <id>` — the spec is frozen after review.
- `decision: implement <id> (#PR)` — the change merged.
- `plan: start <id>` — a Plan for the accepted decision is on a branch.
- `plan: done <id>` — the Plan's steps are finished and its file is deleted.
- `todo: sync` — `TODO.md` is refreshed from the agent's live list.
- `todo: clear` — `TODO.md` is deleted; the session's list is done with.
- `release: v<version>` — a version shipped.
- `deploy: <environment> v<version>` — a version reached an environment.

`<id>` is a decision's id: its date plus a kebab-case slug of its title,
`YYYY-MM-DD-slug`. It is the whole reference — the slug is the title, so
`propose` carries no second copy of it and the subject stays inside
Conventional Commits' 100-character header limit. `todo: sync` and
`todo: clear` carry no id, because the cache belongs to a session rather than
a decision.

## What consumes them

- **Dashboards** query `git log --grep` for these subjects instead of
  scraping issue trackers.
- **Chat notifications** fire off a commit webhook and link back to it.
- **Release automation** treats `release:` commits as the trigger to cut a
  changelog from `.changes/` fragments.

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

## The doorbell rule

Notifications are doorbells: they say where to look, never what to do. The
commit is the event of record; a notification MUST NOT carry state that isn't
also in the repository.
