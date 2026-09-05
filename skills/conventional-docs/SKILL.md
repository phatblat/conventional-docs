---
name: conventional-docs
description: Follow the Conventional Docs convention for a repository documentation set - Charter, Design, Decisions, Roadmap, Plan, and .changes release-note fragments. Use when creating or updating any of those documents, when deciding whether a change needs a decision record or a plan, when adding a user-facing change that needs a release-note fragment, when writing decision, plan, release, or deploy commit events, or when graduating root documents into docs/.
license: MIT
---

# Conventional Docs

Conventional Docs is a predictable shape for a repository's documentation set —
the same set of files, at the same paths, with the same lifetimes, in every
project — so a human or an agent can walk into any repo cold and know where to
look. Full rationale and rendered tables:
[phatblat/conventional-docs](https://github.com/phatblat/conventional-docs).

## When to use this skill

Use it when creating or editing a `CHARTER.md`, `DESIGN.md`, `DECISIONS.md`,
`ROADMAP.md`, or `PLAN.md` (or their graduated `docs/` forms); when deciding
whether a change needs a Decision or a Plan; when a change is user-facing and
needs a `.changes/<slug>.md` fragment; when writing a `decision:`, `plan:`,
`release:`, or `deploy:` commit; or when a root document has outgrown a single
file and needs to graduate into `docs/`.

Do not use it for user-facing documentation — tutorials, how-tos, and
reference docs belong to [Diátaxis](https://diataxis.fr/), not this
convention. Do not scaffold these artifacts into a repository that has not
adopted Conventional Docs; ask first.

## Orient before writing

1. Look for root `CHARTER.md`, `DESIGN.md`, `DECISIONS.md`, `ROADMAP.md`,
   `PLAN.md`, and a `.changes/` directory.
2. Look for the graduated forms under `docs/` (`docs/charter.md`,
   `docs/design.md`, `docs/decisions/`, `docs/roadmap.md`, `docs/plan.md`,
   `docs/runbooks/`, `docs/incidents/`, `docs/todo.md`).
3. When a Charter exists, its `## Artifacts` table is authoritative for where
   each document lives; trust it over guessing.
4. When `PLAN.md`/`docs/plan.md` exists, read it first — it is the cold-start
   handoff for the current branch, and names the decision it implements.
5. When none of these exist, the repo has not adopted the convention: say so
   and ask before creating artifacts.

## Artifacts

| Artifact  | Small repo           | Graduated                           | Lifetime              | Answers                                               |
| --------- | -------------------- | ----------------------------------- | --------------------- | ----------------------------------------------------- |
| Charter   | `CHARTER.md`         | `docs/charter.md`                   | project               | why it exists, goals, route                           |
| Design    | `DESIGN.md`          | `docs/design.md`                    | living                | what the system is and does _now_                     |
| Decisions | `DECISIONS.md`       | `docs/decisions/NNNN-slug.md`       | append-only           | what changed, why, what it cost                       |
| Roadmap   | `ROADMAP.md`         | `docs/roadmap.md`                   | living                | what's next, in order                                 |
| Plan      | `PLAN.md`            | `docs/plan.md`                      | one branch / worktree | exact steps for the current decision                  |
| Changes   | `.changes/<slug>.md` | `.changes/<slug>.md`                | per-release           | what will ship in the next release, in plain language |
| Runbooks  | —                    | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                             |
| Incidents | —                    | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                           |
| Todo      | agent memory         | `docs/todo.md` (opt-in)             | one session           | where this session is                                 |

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
   Decision → implemented          (Todo)
   Plan deleted
```

- **Decision (proposed)** — written when a change crosses a threshold above;
  committed with `decision: propose NNNN <title>`.
- **Decision (accepted)** — the spec is frozen after review; committed with
  `decision: accept NNNN`.
- **Plan written** — the accepted decision's exact steps, for handoff or a
  cold restart; not a separate commit event of its own.
- **execute** — the Plan's steps happen; per-session state goes in agent
  memory or an opt-in `docs/todo.md`, not a commit.
- **PR merged / Design updated** — the living Design doc is updated to match
  reality; the Plan file is deleted in the same branch.
- **Decision → implemented** — committed with
  `decision: implement NNNN (#PR)` once merged.

## Decisions

Numbering is 4-digit zero-padded and sequential (`0001`, `0002`, …); the next
number is the highest existing number plus one. Statuses: `proposed`,
`accepted`, `implemented`, `superseded by NNNN`. Never rewrite an accepted
decision; supersede it with a new one instead.

Entry template (MADR-shaped):

```markdown
# NNNN. Title in the imperative

- Status: proposed
- Date: YYYY-MM-DD

## Context

The forces and constraints that make this a decision.

## Decision

What we will do.

## Consequences

What this costs, what it rules out, what follows from it.
```

In a single-file `DECISIONS.md`, the same entry appears one heading level
deeper (`## NNNN. Title`, `### Context`, …) and is **appended** to the end of
the file, never inserted or reordered. In `docs/decisions/`, the file is
`NNNN-slug.md` and keeps the heading levels shown above.

`docs/decisions/` is the canonical graduated location. For `adr-tools`
compatibility, a root `.adr-dir` file containing `docs/decisions` points
existing ADR tooling at it; MADR already defaults to this path.

## Plans

A Plan implements exactly one accepted decision, is committed on the branch
for backup and handoff, and is deleted in the branch before merge.

```markdown
# Plan: NNNN Title

Decision: <link or path to the accepted decision>

## Steps

- [ ] First step
- [ ] Second step

## Verification

The commands or checks that prove the work is done.

## Status

Where this branch left off, for whoever picks it up next.
```

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
files:

```text
decision: propose 0007 <title>
decision: accept 0007
decision: implement 0007 (#88)
plan: start 0007
plan: done 0007
release: v1.2.0
deploy: prod v1.2.0
```

These are ordinary Conventional Commits types, so an adopting repo's
commitlint config must extend `type-enum` with `decision`, `deploy`, `plan`,
and `release`:

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

Notifications are doorbells: they say where to look, never what to do. The
commit is the event.

## Graduating to `docs/`

Small repos keep everything as `UPPERCASE.md` at the root. Move a document to
`docs/` when either trigger fires:

1. the root is getting cluttered with top-level files and folders (dotfiles
   don't count — that's where config conventions live), or
2. the document has outgrown a single file: it needs siblings, status, or
   structure (a `ROADMAP.md` that needs per-item status becomes
   `docs/roadmap.md`; a `DECISIONS.md` splits into `docs/decisions/`).

Graduate in one commit, rewrite inbound links in the same commit. No stub
file at the old path, and no mirror in either direction. Update the
Charter's `## Artifacts` table to record the new location, and let a link
check in CI catch stale links.

## Files that never graduate

`README.md`, `LICENSE`, `CHANGELOG.md`, and `AGENTS.md` stay at the root
permanently. `AGENTS.md` is the canonical, tool-agnostic agent instruction
file; a tool that reads a different filename gets a real root file that
imports it with the `[@AGENTS.md](AGENTS.md)` link idiom — never a copy,
never a symlink.
