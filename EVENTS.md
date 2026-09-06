# Events

> Status: proposed. This file is where the convention is considering putting
> the event vocabulary; the vocabulary itself is in use. See
> [the decision](docs/decisions/2026-09-05-give-events-their-own-artifact.md).

Lifecycle transitions are commits with
[Conventional Commits](https://www.conventionalcommits.org/) types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing files.

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

These are ordinary Conventional Commits types, so this repo's commitlint config
extends `type-enum` with `decision`, `deploy`, `plan`, `release`, and `todo`:

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

A repo that announces its own events adds them here and to `type-enum` in the
same commit: this file is the human-facing source, the config is what rejects a
bad subject.

Notifications are doorbells: they say where to look, never what to do. The
commit is the event.
