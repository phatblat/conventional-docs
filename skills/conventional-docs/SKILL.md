---
name: conventional-docs
description: Follow the Conventional Docs convention for a repository documentation set - Charter, Design, Decisions, Roadmap, Plan, Todo, Events, and .changes release-note fragments. Use when creating or updating any of those documents, when deciding whether a change needs a decision record or a plan, when adding a user-facing change that needs a release-note fragment, when caching an agent's todo list in TODO.md, when writing decision, plan, todo, release, or deploy commit events, or when graduating root documents into docs/.
license: MIT
---

# Conventional Docs

Conventional Docs is a predictable shape for a repository's documentation set —
the same set of files, at the same paths, with the same lifetimes, in every
project — so a human or an agent can walk into any repo cold and know where to
look. Full rationale and rendered tables:
[phatblat/conventional-docs](https://github.com/phatblat/conventional-docs).

## When to use this skill

Use it when creating or editing a `CHARTER.md`, `DESIGN.md`, `docs/decisions/`,
`ROADMAP.md`, or `EVENTS.md` (or their graduated `docs/` forms), or a
`PLAN.md` or `TODO.md`; when deciding whether a change needs a Decision or a
Plan; when a change is user-facing and needs a `.changes/<slug>.md` fragment;
when writing a `decision:`, `plan:`, `todo:`, `release:`, or `deploy:` commit;
or when a root document has outgrown a single file and needs to graduate into
`docs/`.

Do not use it for user-facing documentation — tutorials, how-tos, and
reference docs belong to [Diátaxis](https://diataxis.fr/), not this
convention. Do not scaffold these artifacts into a repository that has not
adopted Conventional Docs; ask first.

## Orient before writing

1. Look for root `CHARTER.md`, `DESIGN.md`, `ROADMAP.md`, `PLAN.md`,
   `TODO.md`, `EVENTS.md`, and a `.changes/` directory.
2. Look for the graduated forms under `docs/` (`docs/charter.md`,
   `docs/design.md`, `docs/decisions/`, `docs/roadmap.md`, `docs/events.md`,
   `docs/runbooks/`, `docs/incidents/`); `PLAN.md` and `TODO.md` have no
   graduated form.
3. When a Charter exists, its `## Artifacts` table is authoritative for where
   each document lives; trust it over guessing.
4. When `PLAN.md` exists, read it first — it is the cold-start handoff for
   the current branch, and names the decision it implements. When `TODO.md`
   exists, read it next: it is where the last session left off.
5. When none of these exist, the repo has not adopted the convention: say so
   and ask before creating artifacts.

## Artifacts

| Artifact  | Small repo           | Graduated                           | Lifetime              | Answers                                               |
| --------- | -------------------- | ----------------------------------- | --------------------- | ----------------------------------------------------- |
| Charter   | `CHARTER.md`         | `docs/charter.md`                   | project               | why it exists, goals, route                           |
| Design    | `DESIGN.md`          | `docs/design.md`                    | living                | what the system is and does _now_                     |
| Decisions | —                    | `docs/decisions/YYYY-MM-DD-slug.md` | append-only           | what changed, why, what it cost                       |
| Roadmap   | `ROADMAP.md`         | `docs/roadmap.md`                   | living                | what's next, in order                                 |
| Plan      | `PLAN.md`            | —                                   | one branch / worktree | exact steps for the current decision                  |
| Changes   | `.changes/<slug>.md` | `.changes/<slug>.md`                | per-release           | what will ship in the next release, in plain language |
| Events    | `EVENTS.md`          | `docs/events.md`                    | living                | which lifecycle events the repo's commits announce    |
| Runbooks  | —                    | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                             |
| Incidents | —                    | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                           |
| Todo      | `TODO.md`            | —                                   | one branch / worktree | where the work left off                               |

_Events is proposed, not settled: the vocabulary is in use, but `EVENTS.md`
as its home is still under review._

A _proposed_ decision is the spec. Once accepted it is frozen; changing your
mind is a new decision that supersedes it. The Plan is written from an
accepted decision, committed for backup and handoff, and deleted before
merge.

## Thresholds

A PR over ~100 lines, or one that changes behavior, an interface, or a
dependency, needs a Decision. Work spanning more than one session, or handed
to another agent, needs a Plan. Anything smaller just happens.

## The loop

```text
intent → Decision (proposed) → review → Decision (accepted)
                                              ↓
   Design updated ← PR merged ← execute ← Plan written
   Decision → implemented       Todo cached
   Plan and Todo deleted
```

- **Decision (proposed)** — written when a change crosses a threshold above;
  committed with `decision: propose <id>`.
- **Decision (accepted)** — the spec is frozen after review; committed with
  `decision: accept <id>`.
- **Plan written** — the accepted decision's exact steps, for handoff or a
  cold restart; not a separate commit event of its own.
- **execute** — the Plan's steps happen, and the agent's own list is cached
  in `TODO.md` and committed with `todo: sync` at each checkpoint.
- **PR merged / Design updated** — the living Design doc is updated to match
  reality; `PLAN.md` is deleted by `plan: done` and `TODO.md` by
  `todo: clear`, both before merge.
- **Decision → implemented** — committed with
  `decision: implement <id> (#PR)` once merged.

## Decisions

A decision record is always its own file, `docs/decisions/YYYY-MM-DD-slug.md`.
There is no single-file form and no graduation step.

The id is `YYYY-MM-DD-slug` — the date the record was written plus a kebab-case
slug of its title — and it is the filename without `.md`. It is fixed at
creation: never re-date, renumber, or rename a record, not when its status
changes and not when a later decision supersedes it. Several decisions may
share a date; their slugs tell them apart. Never add a counter or suffix, and
never renumber to settle a merge.

`docs/decisions/` is canonical, and MADR already defaults to this path. A root
`.adr-dir` file containing `docs/decisions` points location-only ADR tooling
(`adr list`, `adr generate`) at it. Do not use `adr new`: it allocates the next
sequential number.

### Structure

Use exactly this skeleton, H2 sections in this order:

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
full reasoning for Positions. `N/A` is acceptable when the constraints make the
decision self-evident.>

## Architectural Decision

<The decision itself, as numbered clauses a reviewer can point at. Include code
or YAML only where it pins down a contract — a field name, a struct variant,
one representative manifest — never to reproduce the implementation.>

## Positions

<Alternatives considered and rejected, each with its reason, or `N/A`.>

## Dates

- Published: TBD (set at merge).
```

Optional sections, in position:

- `## Consequences` — after Architectural Decision, before Positions: rollout
  order, breaking changes, migration burden, follow-up documentation owed.
- `## References` — before Dates: bulleted links with `—` descriptions
  (tracking issue, implementation PRs, related decisions, external specs).

H3 subsections are permitted inside Argument, Architectural Decision, and
Positions when they improve skimmability.

### Status and date lifecycle

Status is prose, and it moves with the commit events:

- **Proposed** (`decision: propose <id>`) — `This is a proposal that is
**awaiting review**.` with `- Published: TBD (set at merge).`
- **Accepted** (`decision: accept <id>`) — `This is a proposal that is
**accepted**.` and `- Published: YYYY-MM-DD` set to the merge date.
- **Implemented** (`decision: implement <id> (#PR)`) — `This is a proposal that
is **implemented**.`, or lead with `Implemented in [owner/repo#NN](<url>).`,
  and append `- Updated: YYYY-MM-DD (implemented)` under Dates.
- **Superseded** — `This proposal is **superseded** by
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` and append an `- Updated:` line.

Material later edits append `- Updated: YYYY-MM-DD (<what changed>)` under
Dates. Never rewrite a merged decision silently; extend it with a new decision
instead.

### Cross-linking decisions

An extension decision opens Issue with `This decision extends
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` In the same PR, edit the extended
decision to add `This decision is extended by
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` under its Issue heading. Reference
decisions from prose as `[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md)`, with
relative links.

### Keep it brief

- Record the decision and its rationale only. Implementation diaries, debugging
  history, delivery staging, and local setup notes belong in PR descriptions,
  not the durable record.
- Link another decision's content; never copy it.
- One decision per file when the parts can land independently.

### Adopting in a repo with a numbered log

1. Rename each `NNNN-slug.md` to `YYYY-MM-DD-slug.md`, taking the date from the
   record's published date, or from the date the file was added when it has
   none: `git log --diff-filter=A --format=%ad --date=short -1 -- <file>`.
2. Rewrite inbound links in the same commit.
3. Leave a redirect at each old filename — the one place this convention keeps
   a file at an old path, because citations of the old id outside the repo
   cannot be rewritten by the rename:

   ```markdown
   # Moved

   Moved to [@2026-02-11-split-the-scheduler.md](2026-02-11-split-the-scheduler.md).
   ```

   Stubs are redirects, never content, and they stay. Never a symlink.

A repo on the single-file `DECISIONS.md` form splits it in one commit: one
`docs/decisions/YYYY-MM-DD-slug.md` per entry, dated from the entry's own date,
inbound links rewritten, and `DECISIONS.md` deleted with no stub at the old
path — the file was never an id anyone cited.

## Plans

A Plan implements exactly one accepted decision, is committed on the branch
for backup and handoff, and is deleted in the branch before merge.

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

## Todo

`TODO.md` at the repository root is a cache of the list the agent is already
keeping, never a source and never a backlog; write it whenever tracking a
list at all, refresh it as the list changes, and commit it at each checkpoint
with `todo: sync`; there is no threshold, so a change too small for a Plan
still gets a Todo when the agent is tracking steps for it.

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

- Refresh `Synced` on every write, so a reader can tell the cache from the
  session that is gone.
- The file belongs to the branch, not the session: a later session on the
  same branch reads it and takes it over.
- Delete it before merge, in a `todo: clear` commit of its own.
- The single-artifact rule for `todo:` and `plan:` commits and its net-zero
  consequence applies here too: never fold the cache into a commit that
  carries work.
- `TODO.md` passes the same markdown gates as every other file, so keep the
  skeleton's shape: `-` bullets, a blank line around every list and heading.
- A repo whose existing `TODO.md` is a durable backlog has a Roadmap under
  the wrong name and renames it to `ROADMAP.md` when adopting.

## Release-note fragments

One `.changes/<slug>.md` file per user-facing change, with a kebab-case slug
matching the change, added in the same commit or PR as the change. Every
non-blank line is a markdown list item opening with one of the six
[Keep a Changelog](https://keepachangelog.com/) categories — `Added`,
`Changed`, `Deprecated`, `Removed`, `Fixed`, `Security` — in the exact form
`- <Category>: <description>`:

```markdown
- Added: support for custom output formats.
- Fixed: a race condition when releasing concurrently.
```

Fragments are concatenated, grouped by category, and deleted at release time.
`CHANGELOG.md` is assembled from them and is never hand-edited; version
fields are never hand-bumped.

## Commit events

Lifecycle transitions are commits with Conventional Commits types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing
files.

A repo's own vocabulary lives in `EVENTS.md` (graduated: `docs/events.md`)
when it has one — read it before writing an event commit, because a repo may
announce types beyond the ones below. `EVENTS.md` is a proposed artifact, not
a settled one: read an existing file, and do not create one unless asked.

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

The id is the entire reference; there is no separate title argument, which
also keeps the subject inside the 100-character header limit.

These are ordinary Conventional Commits types, so an adopting repo's
commitlint config must extend `type-enum` with `decision`, `deploy`, `plan`,
`release`, and `todo`:

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

`type-enum` is the half that always exists, and it is what rejects a bad
subject, so extend it first when a repo adds an event type; where a repo
writes the prose half down is what `EVENTS.md` is still proposing.

Notifications are doorbells: they say where to look, never what to do. The
commit is the event.

## Graduating to `docs/`

Small repos keep everything as `UPPERCASE.md` at the root, except the per-file
logs — decisions, runbooks, incidents — which live under `docs/` from their
first entry. Move a document to `docs/` when either trigger fires:

1. the root is getting cluttered with top-level files and folders (dotfiles
   don't count — that's where config conventions live), or
2. the document has outgrown a single file: it needs siblings, status, or
   structure (a `ROADMAP.md` that needs per-item status becomes
   `docs/roadmap.md`).

`PLAN.md` and `TODO.md` never move: neither trigger can fire for a document
that lives on one branch and is deleted rather than grown; there is no
`docs/plan.md` and no `docs/todo.md`.

Graduate in one commit, rewrite inbound links in the same commit. No stub file
at the old path, and no mirror in either direction; renaming a numbered
decision log, above, is the one exception. Update the Charter's `## Artifacts`
table to record the new location, and let a link check in CI catch stale links.

## Files that never graduate

`README.md`, `LICENSE`, `CHANGELOG.md`, and `AGENTS.md` stay at the root
permanently. `AGENTS.md` is the canonical, tool-agnostic agent instruction
file; a tool that reads a different filename gets a real root file that
imports it with the `[@AGENTS.md](AGENTS.md)` link idiom — never a copy,
never a symlink.
