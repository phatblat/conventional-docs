# Events

> Status: proposed. This file is where the convention is considering putting
> the event vocabulary; the vocabulary itself is in use. See
> [the decision](docs/decisions/2026-09-05-give-events-their-own-artifact.md).

Lifecycle transitions are commits with
[Conventional Commits](https://www.conventionalcommits.org/) types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing files.

```text
decision: propose 2026-02-11-split-the-scheduler
decision: accept 2026-02-11-split-the-scheduler
decision: implement 2026-02-11-split-the-scheduler (#88)
plan: start 2026-02-11-split-the-scheduler
plan: done 2026-02-11-split-the-scheduler
release: v1.2.0
deploy: prod v1.2.0
```

- `decision: propose <id>` — a decision record is written; it is the spec.
- `decision: accept <id>` — the spec is frozen after review.
- `decision: implement <id> (#PR)` — the change merged.
- `plan: start <id>` — a Plan for the accepted decision is on a branch.
- `plan: done <id>` — the Plan's steps are finished and its file is deleted.
- `release: v<version>` — a version shipped.
- `deploy: <environment> v<version>` — a version reached an environment.

`<id>` is a decision's id: its date plus a kebab-case slug of its title,
`YYYY-MM-DD-slug`. It is the whole reference — the slug is the title, so
`propose` carries no second copy of it and the subject stays inside
Conventional Commits' 100-character header limit.

These are ordinary Conventional Commits types, so this repo's commitlint config
extends `type-enum` with `decision`, `deploy`, `plan`, and `release`:

```js
'type-enum': [
  2,
  'always',
  [
    'build', 'chore', 'ci', 'decision', 'deploy', 'docs', 'feat', 'fix',
    'perf', 'plan', 'refactor', 'release', 'revert', 'style', 'test',
  ],
],
```

A repo that announces its own events adds them here and to `type-enum` in the
same commit: this file is the human-facing source, the config is what rejects a
bad subject.

Notifications are doorbells: they say where to look, never what to do. The
commit is the event.
