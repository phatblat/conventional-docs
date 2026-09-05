---
title: Events
description: Publish lifecycle activity as Conventional Commits, so the repo stays the record.
---

Lifecycle transitions are commits with Conventional Commits types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing
files:

```text
decision: propose 0007 <title>
decision: accept 0007
decision: reject 0007
decision: implement 0007 (#88)
plan: start 0007
plan: done 0007
release: v1.2.0
deploy: prod v1.2.0
```

## What consumes them

- **Dashboards** query `git log --grep` for these subjects instead of
  scraping issue trackers.
- **Chat notifications** fire off a commit webhook and link back to it.
- **Release automation** treats `release:` commits as the trigger to cut a
  changelog from `.changes/` fragments.

## Wiring it into commitlint

```js
'type-enum': [2, 'always', [
  'build', 'chore', 'ci',
  'decision', 'deploy', 'docs', 'feat', 'fix', 'perf',
  'plan', 'refactor', 'release', 'revert', 'style', 'test',
]],
```

## The doorbell rule

Notifications are doorbells: they say where to look, never what to do. The
commit is the event of record; a notification MUST NOT carry state that isn't
also in the repository.
