---
title: Conventional Docs
description: A predictable shape for a repository's docs, so humans and agents know where to look.
tagline: A predictable shape for a repository's docs, so humans and agents know where to look.
actions:
  - text: wheredoc
    href: /routing
  - text: Read the spec
    href: /spec/next
  - text: Quickstart
    href: /quickstart
---

## The idea

Every project keeps a small set of markdown files in git that describe its
intent, its current state, and how it changes. They live at the same paths in
every repo, so a human or an agent can walk in cold and know where to look.
Two axes decide everything else: **lifetime** — how long a document stays
true — and **audience** — outsiders and machines read the repo root,
maintainers read `docs/`.

{{< cards >}}

## The artifacts

| Artifact                                  | Small repo           | Graduated                           | Lifetime              | Answers                                               |
| ----------------------------------------- | -------------------- | ----------------------------------- | --------------------- | ----------------------------------------------------- |
| [Charter](artifacts/charter/index.md)     | `CHARTER.md`         | `docs/charter.md`                   | project               | why it exists, goals, route                           |
| [Roadmap](artifacts/roadmap/index.md)     | `ROADMAP.md`         | `docs/roadmap.md`                   | living                | what's next, in order                                 |
| [Plan](artifacts/plan/index.md)           | `PLAN.md`            | —                                   | one branch / worktree | exact steps for the current decision                  |
| [Todo](artifacts/todo/index.md)           | `TODO.md`            | —                                   | one branch / worktree | where the work left off                               |
| [Design](artifacts/design/index.md)       | `DESIGN.md`          | `docs/design.md`                    | living                | what the system is and does _now_                     |
| [Events](events/index.md)                 | `EVENTS.md`          | `docs/events.md`                    | living                | which lifecycle events the repo's commits announce    |
| [Decisions](artifacts/decisions/index.md) | —                    | `docs/decisions/YYYY-MM-DD-slug.md` | append-only           | what changed, why, what it cost                       |
| [Changes](artifacts/changes/index.md)     | `.changes/<slug>.md` | `.changes/<slug>.md`                | per-release           | what will ship in the next release, in plain language |
| [Runbooks](artifacts/runbooks/index.md)   | —                    | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                             |
| [Incidents](artifacts/incidents/index.md) | —                    | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                           |

_Events is proposed, not settled: the vocabulary is in use, but `EVENTS.md`
as its home is
[still under review](https://github.com/phatblat/conventional-docs/blob/main/docs/decisions/2026-09-05-give-events-their-own-artifact.md)._

## The loop

![The Conventional Docs loop: intent leads to a proposed Decision, review, and an accepted Decision; a Plan is written and executed while the Todo is cached; the pull request merges, Design updates, and the Decision is marked implemented, closing back to the next intent.](loop.svg)

```text
intent → Decision (proposed) → review → Decision (accepted)
                                              ↓
   Design updated ← PR merged ← execute ← Plan written
   Decision → implemented       Todo cached
   Plan and Todo deleted
```

A PR over ~100 lines, or one that changes behavior, a published interface, or a
dependency, needs a Decision before it merges. Work spanning more than one
session, or handed to another agent, needs a Plan.

## What this is not

- **Not user documentation.** Tutorials, how-tos, and reference docs are
  [Diátaxis](https://diataxis.fr/)'s territory. Where they overlap, Design is
  explanation/reference and runbooks are how-to.
- **Not a tool, but tooling helps.** Like [Keep a Changelog](https://keepachangelog.com/en/2.0.0/), this convention
  costs only attention to follow by hand — no CI check or generator is
  required. Both are highly recommended once you adopt it: a required CI
  check is a checkpoint that confirms the right artifact was captured before
  an agent launches, and a generator is an integration point where future
  tools or spawned agents get event visibility and rule enforcement.
