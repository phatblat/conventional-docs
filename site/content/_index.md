---
title: Conventional Docs
description: A predictable shape for a repository's docs, so humans and agents know where to look.
tagline: A predictable shape for a repository's docs, so humans and agents know where to look.
actions:
  - text: Where does this go?
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

## The loop

```text
intent → Decision (proposed) → review → Decision (accepted)
                                              ↓
   Design updated ← PR merged ← execute ← Plan written
   Decision → implemented          (Todo)
   Plan deleted
```

A PR over ~100 lines, or one that changes behavior, a published interface, or a
dependency, needs a Decision before it merges. Work spanning more than one
session, or handed to another agent, needs a Plan.

## What this is not

- **Not user documentation.** Tutorials, how-tos, and reference docs are
  [Diátaxis](https://diataxis.fr/)'s territory. Where they overlap, Design is
  explanation/reference and runbooks are how-to.
- **Not a tool.** Like Keep a Changelog, this convention should cost only
  attention — no required CI check, no required generator.
