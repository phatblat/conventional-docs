---
title: Artifacts
description: Reference for every artifact — purpose, paths, lifetime, and a skeleton to copy.
---

Every artifact has its own page: what it answers, where it lives, how long it
lives there, and — except for Runbooks and Incidents, which this project
doesn't use yet — a skeleton to copy and a worked example.

| Artifact                        | Small repo           | Graduated                           | Lifetime              | Answers                                               |
| ------------------------------- | -------------------- | ----------------------------------- | --------------------- | ----------------------------------------------------- |
| [Charter](charter/index.md)     | `CHARTER.md`         | `docs/charter.md`                   | project               | why it exists, goals, route                           |
| [Design](design/index.md)       | `DESIGN.md`          | `docs/design.md`                    | living                | what the system is and does _now_                     |
| [Decisions](decisions/index.md) | —                    | `docs/decisions/YYYY-MM-DD-slug.md` | append-only           | what changed, why, what it cost                       |
| [Roadmap](roadmap/index.md)     | `ROADMAP.md`         | `docs/roadmap.md`                   | living                | what's next, in order                                 |
| [Plan](plan/index.md)           | `PLAN.md`            | —                                   | one branch / worktree | exact steps for the current decision                  |
| [Changes](changes/index.md)     | `.changes/<slug>.md` | `.changes/<slug>.md`                | per-release           | what will ship in the next release, in plain language |
| [Events](../events/index.md)    | `EVENTS.md`          | `docs/events.md`                    | living                | which lifecycle events the repo's commits announce    |
| [Runbooks](runbooks/index.md)   | —                    | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                             |
| [Incidents](incidents/index.md) | —                    | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                           |
| [Todo](todo/index.md)           | `TODO.md`            | —                                   | one branch / worktree | where the work left off                               |

_Events is proposed, not settled: the vocabulary is in use, but `EVENTS.md`
as its home is
[still under review](https://github.com/phatblat/conventional-docs/blob/main/docs/decisions/2026-09-05-give-events-their-own-artifact.md);
its detail lives on the [Events](../events/index.md) page rather than here._

See [wheredoc](../routing/index.md) to decide which artifact a piece of new
content belongs in, and [Graduating](../graduating/index.md) for when and how
one moves from its small-repo path to its graduated path.
