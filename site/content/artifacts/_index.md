---
title: Artifacts
description: Reference for every artifact — purpose, paths, lifetime, and a skeleton to copy.
---

Every artifact has its own page: what it answers, where it lives, how long it
lives there, and — except for Runbooks and Incidents, which this project
doesn't use yet — a skeleton to copy and a worked example.

The table orders the six that can sit at the repository root first, then the
three that live only under `docs/`; within each group, in the order their
handoff happens in the lifecycle — see the Charter's
[`## The pattern`](https://github.com/phatblat/conventional-docs/blob/main/CHARTER.md#the-pattern)
table for the full phase-by-phase mapping.

| Artifact                        | Small repo   | Graduated                           | Lifetime              | Answers                                            |
| ------------------------------- | ------------ | ----------------------------------- | --------------------- | -------------------------------------------------- |
| [Charter](charter/index.md)     | `CHARTER.md` | `docs/charter.md`                   | project               | why it exists, goals, route                        |
| [Roadmap](roadmap/index.md)     | `ROADMAP.md` | `docs/roadmap.md`                   | living                | what's next, in order                              |
| [Plan](plan/index.md)           | `PLAN.md`    | —                                   | one branch / worktree | exact steps for the current decision               |
| [Todo](todo/index.md)           | `TODO.md`    | —                                   | one branch / worktree | where the work left off                            |
| [Design](design/index.md)       | `DESIGN.md`  | `docs/design.md`                    | living                | what the system is and does _now_                  |
| [Events](../events/index.md)    | `EVENTS.md`  | `docs/events.md`                    | living                | which lifecycle events the repo's commits announce |
| [Decisions](decisions/index.md) | —            | `docs/decisions/YYYY-MM-DD-slug.md` | append-only           | what changed, why, what it cost                    |
| [Runbooks](runbooks/index.md)   | —            | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                          |
| [Incidents](incidents/index.md) | —            | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                        |

_Events is proposed, not settled: the vocabulary is in use, but `EVENTS.md`
as its home is
[still under review](https://github.com/phatblat/conventional-docs/blob/main/docs/decisions/2026-09-05-give-events-their-own-artifact.md);
its detail lives on the [Events](../events/index.md) page rather than here._

`README.md`, `LICENSE`, `CHANGELOG.md`, and `AGENTS.md` never graduate;
the [Changelog](changelog/index.md) has a page of its own because the
convention fixes its shape.

See [wheredoc](../routing/index.md) to decide which artifact a piece of new
content belongs in, and [Graduating](../graduating/index.md) for when and how
one moves from its small-repo path to its graduated path.
