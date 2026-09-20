---
name: conventional-docs
description: Follow the Conventional Docs convention for a repository documentation set - Charter, Design, Decisions, Roadmap, Backlog, Plan, Todo, Events, and the CHANGELOG. Use when creating or updating any of those documents, when deciding whether a change needs a decision record or a plan, when recording a user-facing change in the changelog's Unreleased section, when caching an agent's todo list in TODO.md, when filing a work item under docs/backlog/, when writing decision, plan, todo, release, or deploy commit events, or when graduating root documents into docs/. Also use when placing an adjacent file - CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, SUPPORT.md, CODEOWNERS, CITATION.cff, or a LICENSE - or when deciding where such a file belongs; CONTRIBUTING.md's content is this convention's own, the rest defer to their owning standard.
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
`ROADMAP.md`, `docs/backlog/`, or `EVENTS.md` (or their graduated `docs/`
forms), or a `PLAN.md` or `TODO.md`; when deciding whether a change needs a
Decision or a Plan; when a change is user-facing and needs a changelog line;
when writing a `decision:`, `plan:`, `todo:`, `release:`, or `deploy:`
commit; or when a root document has outgrown a single file and needs to
graduate into `docs/`.

Do not use it for user-facing documentation — tutorials, how-tos, and
reference docs belong to [Diátaxis](https://diataxis.fr/), not this
convention. Do not scaffold these artifacts into a repository that has not
adopted Conventional Docs; ask first.

## Orient before writing

1. Look for root `CHARTER.md`, `DESIGN.md`, `ROADMAP.md`, `PLAN.md`,
   `TODO.md`, and `EVENTS.md`.
2. Look for the graduated forms under `docs/` (`docs/charter.md`,
   `docs/design.md`, `docs/decisions/`, `docs/roadmap.md`, `docs/backlog/`,
   `docs/events.md`, `docs/runbooks/`, `docs/incidents/`); `PLAN.md` and
   `TODO.md` have no graduated form.
3. When a Charter exists, its `## Artifacts` table is authoritative for where
   each document lives; trust it over guessing.
4. When `PLAN.md` exists, read it first — it is the cold-start handoff for
   the current branch, and names the decision it implements. When `TODO.md`
   exists, read it next: it is where the last session left off.
5. When none of these exist, the repo has not adopted the convention: say so
   and ask before creating artifacts.

## Artifacts

| Artifact  | Small repo   | Graduated                           | Lifetime              | Answers                                            |
| --------- | ------------ | ----------------------------------- | --------------------- | -------------------------------------------------- |
| Charter   | `CHARTER.md` | `docs/charter.md`                   | project               | why it exists, goals, route                        |
| Design    | `DESIGN.md`  | `docs/design.md`                    | living                | what the system is and does _now_                  |
| Decisions | —            | `docs/decisions/YYYY-MM-DD-slug.md` | append-only           | what changed, why, what it cost                    |
| Roadmap   | `ROADMAP.md` | `docs/roadmap.md`                   | living                | what's next, in order                              |
| Backlog   | —            | `docs/backlog/<slug>.md`            | until done            | what one queued item is, in full                   |
| Plan      | `PLAN.md`    | —                                   | one branch / worktree | exact steps for the current decision               |
| Events    | `EVENTS.md`  | `docs/events.md`                    | living                | which lifecycle events the repo's commits announce |
| Runbooks  | —            | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                          |
| Incidents | —            | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                        |
| Todo      | `TODO.md`    | —                                   | one branch / worktree | where the work left off                            |

_Events is proposed, not settled: the vocabulary is in use, but `EVENTS.md`
as its home is still under review._

A _proposed_ decision is the spec. Once accepted it is frozen; changing your
mind is a new decision that supersedes it. The Plan is written from an
accepted decision, committed for backup and handoff, and deleted before
merge.

## Thresholds

A PR over ~100 lines, or one that changes behavior, an interface, or a
dependency, needs a Decision. Work spanning more than one session, or handed
to another agent, needs a Plan. File a Backlog item when the work is real but
not being done now; work being done now needs no item. Anything smaller just
happens.

## The loop

```text
intent → Decision (proposed) → review → Decision (accepted)
                                              ↓
   Design updated ← PR merged ← execute ← Plan written
                                Todo cached
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

## Backlog

A Backlog item is one file per work item, `docs/backlog/<slug>.md`, with no
small-repo form: a repository either has no Backlog or has the directory. Its
id is a kebab-case slug of the title — never a number or a date, both of
which race the same way an allocated decision number once did. The slug may
change while the item exists; two items whose slugs collide are the same
item and must be merged into one file.

Use exactly this skeleton:

```markdown
# <Title>

## Problem

<What is wrong or missing, and who it costs. For a bug: what happens, what
should happen, and how to reproduce it.>

## Outcome

<What is observably true when this is done.>

## Notes

<Optional, last: evidence, links, a related decision, a suggested direction
the decision record is free to overrule.>
```

No status, priority, assignment, claim, or date field: presence in
`docs/backlog/` is the item's open state, order is the Roadmap's, and dates
are `git log`'s.

Once a repository has a Backlog, the Roadmap stops describing work and
starts indexing it: an ordered list of links, never a restated body. A
decision record must never link to a Backlog item — the item is deleted at
implementation and the link would go stale — but a Backlog item may link to
a decision. An item is deleted by the change that implements it, together
with its Roadmap line, in the same pull request; an item that will not be
implemented is deleted rather than kept.

### Before starting an item

A Backlog item never records who is working on it. A branch that implements
one should end with the item's slug — any prefix (`feat/`, `fix/`, an author
or agent name) is fine — so concurrent work is found by suffix match before
you start, and your own claim is visible the moment you push:

```bash
git ls-remote --heads origin '*<slug>'   # a hit means someone is on it
git switch -c <prefix>/<slug> && git push -u origin HEAD
```

This is detection, not arbitration: a hit means look before you start, not
that the item is locked. Push before doing the work, not after, so the next
agent who checks sees it too.

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
```

Optional sections, in position:

- `## Consequences` — after Architectural Decision, before Positions: rollout
  order, breaking changes, migration burden, follow-up documentation owed.
- `## References` — after Positions: bulleted links with `—` descriptions
  (tracking issue, implementation PRs, related decisions, external specs).
- `## Errata` — last, after References: append-only corrections and
  supersession pointers, added only once the record is frozen.

H3 subsections are permitted inside Argument, Architectural Decision, and
Positions when they improve skimmability.

### Status lifecycle

A record has four states and moves through them in one direction:
**draft → proposed → accepted | rejected**. Each transition is a commit, and
`## Status` carries exactly one line:

- **draft** (`decision: draft <id>`) — `This is a **draft**; it is not ready
for review.` The record exists and is being written; it is not the spec, and
  nothing may be planned against it. The state is optional — a record written
  in one sitting is proposed directly — but it is the only honest status for a
  record committed before it is ready to be read.
- **proposed** (`decision: propose <id>`) — `This is a proposal that is
**awaiting review**.` It is the spec.
- **accepted** (`decision: accept <id>`) — `This is a proposal that is
**accepted**.`
- **rejected** (`decision: reject <id>`) — `This proposal was **rejected**.`
  A rejected record stays in the log: it says what was considered and why it
  was refused.

`accept` and `reject` end review, and that commit is the last write to the
record's body. A record carries no dates: its date is its id, and every other
date it could carry is a commit date `git log` already holds. There is no
`implemented`, `superseded`, or `deprecated` status — what shipped is
`CHANGELOG.md`'s question, and `plan: done <id>` already announces that an
accepted decision's work is finished.

### Errata

A frozen record is corrected by appending to `## Errata`, its last section,
one dated line per entry, newest last:

```markdown
## Errata

- 2026-03-04: The second clause named `--strict`; the flag shipped as
  `--pedantic`. The decision is unchanged.
```

Exactly two kinds of entry are admissible: a correction of fact or expression
that leaves the decision itself unchanged, and a pointer to a record that
supersedes or extends this one. Anything that changes the decision is a new
decision. Never edit an existing erratum, and never edit the body above the
heading.

Supersession is stated twice — in the superseding record's Issue, and as an
erratum on the superseded record. Neither record's status changes.

### Cross-linking decisions

An extension decision opens Issue with `This decision extends
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` In the same PR, give the extended
record its reciprocal link: a draft or proposed record is edited in place,
adding `This decision is extended by
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` under its Issue heading, while a
frozen record receives an erratum instead. Reference decisions from prose as
`[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md)`, with relative links.

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
- A repo whose existing `TODO.md` is a single durable file has a Roadmap
  under the wrong name and renames it to `ROADMAP.md` when adopting; where
  it is a directory of per-item files, it is a Backlog and moves to
  `docs/backlog/`.

## Changelog

`CHANGELOG.md` at the repository root answers both what shipped and what will
ship next. It follows
[Keep a Changelog 2.0.0](https://keepachangelog.com/en/2.0.0/): a `# Changelog`
heading with the fixed preamble, `## [Unreleased]` at the top, released
versions as `## [x.y.z] - YYYY-MM-DD` newest first, the six categories —
`Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security` — as `###`
subsections, `**Breaking:**` markers inside the type they belong to, and
reference-style links at the bottom resolving each version to a compare diff.

Add a notable user-facing change to `## [Unreleased]` in the same commit or PR
as the change:

```markdown
## [Unreleased]

### Added

- Support for custom output formats.
```

- Notable is a judgment: no check requires a changelog edit, and a change no
  user would notice gets no line.
- Never hand-edit a released section, and never hand-bump a version heading or
  a version field. The `release:` event renames `[Unreleased]` to the new
  version in both the heading and its reference link, and opens a fresh empty
  `[Unreleased]`.
- Resolve a conflict in `[Unreleased]` by keeping both lines; order within a
  category carries no meaning.
- A line may cite the decision id it came from — a portable reference (a path
  in the repo, not one host's number), which is the form Keep a Changelog
  recommends over a bare `(#1234)`.

## Commit events

Lifecycle transitions are commits with Conventional Commits types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing
files.

A repo's own vocabulary lives in `EVENTS.md` (graduated: `docs/events.md`)
when it has one — read it before writing an event commit, because a repo may
announce types beyond the ones below. `EVENTS.md` is a proposed artifact, not
a settled one: read an existing file, and do not create one unless asked.

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
   `docs/roadmap.md`; a `ROADMAP.md` whose items need real descriptions or
   conflict between concurrent branches decomposes into `docs/backlog/` plus
   a Roadmap that indexes it, per `## Backlog` above).

`PLAN.md` and `TODO.md` never move: neither trigger can fire for a document
that lives on one branch and is deleted rather than grown; there is no
`docs/plan.md` and no `docs/todo.md`.

Graduate in one commit, rewrite inbound links in the same commit. No stub file
at the old path, and no mirror in either direction; renaming a numbered
decision log, above, is the one exception. Update the Charter's `## Artifacts`
table to record the new location, and let a link check in CI catch stale links.

Decomposing a Roadmap into a Backlog is the same one-commit move with one
addition: write one `docs/backlog/<slug>.md` per item first, then rewrite
the Roadmap as the ordered index over them, then update the Charter. No stub
is left at any old anchor inside the Roadmap.

## Adjacent files

An _adjacent file_ is a document a repository is expected to have that this
convention places but does not define — its content belongs to whatever
standard already owns it. `README.md`, the license (written as `LICENSE.md`
by this convention's tooling), and `CITATION.cff` are the root-only case.
`CHANGELOG.md` and `AGENTS.md` are not adjacent files — they are artifacts
this convention defines and binds to the root. `AGENTS.md` is the canonical,
tool-agnostic agent instruction file; a tool that reads a different filename
gets a real root file that imports it with the `[@AGENTS.md](AGENTS.md)` link
idiom — never a copy, never a symlink.

The rest of the ring may live in `.github/`, the repository root, or `docs/` —
GitHub resolves them in that order, first found wins — but only in one of
those places at a time.

| File                 | Content standard              | Locations                 |
| -------------------- | ----------------------------- | ------------------------- |
| `CONTRIBUTING.md`    | this convention               | `.github/`, root, `docs/` |
| `CODE_OF_CONDUCT.md` | Contributor Covenant, or none | `.github/`, root, `docs/` |
| `SECURITY.md`        | none                          | `.github/`, root, `docs/` |
| `SUPPORT.md`         | none                          | `.github/`, root, `docs/` |
| `CODEOWNERS`         | GitHub syntax                 | `.github/`, root, `docs/` |
| `CITATION.cff`       | Citation File Format 1.2.0    | root                      |

An adjacent file never graduates: it keeps its host-recognized name, and a
copy under `docs/` is a placement choice, not a graduation. Record its
location in the Charter's `## Artifacts` section when it isn't the root.

When working in this ring:

- **Never scaffold a license or a code of conduct into a repository without
  asking.** They are legal and community commitments, not boilerplate — a
  human picks the SPDX identifier and the reporting contact.
- **A root copy and a `.github/` copy both existing is a finding, not a
  choice.** Report the duplicate rather than picking one to keep.
- **Write `CONTRIBUTING.md` once a repository has adopted this convention and
  its contributors have nowhere to learn the loop** — the Decision and Plan
  thresholds, the changelog rule, the event vocabulary above.
- Everything else in the table (`SECURITY.md`, `SUPPORT.md`, `CODEOWNERS`,
  `CITATION.cff`) is deferred entirely to its owning standard; this skill adds
  no content rules for them.
