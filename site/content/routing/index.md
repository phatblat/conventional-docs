---
title: wheredoc
description: Answer the routing question for any piece of content in one pass.
---

**Where does this go?** Every piece of durable text you're about to write
already has a place. Work down this list; the first question that fits is
the answer.

## Answer in order

1. **Is it true for the life of the project?** → Charter
   (`CHARTER.md` / `docs/charter.md`).
2. **Does it describe how the system works right now?** → Design
   (`DESIGN.md` / `docs/design.md`).
3. **Is it a choice between alternatives, made once, with consequences?** →
   Decision (`DECISIONS.md` / `docs/decisions/NNNN-slug.md`).
4. **Are these the ordered steps to carry out an accepted decision?** → Plan
   (`PLAN.md` / `docs/plan.md`).
5. **Is it something intended for later, not now?** → Roadmap
   (`ROADMAP.md` / `docs/roadmap.md`).
6. **Will a user of the software notice this change?** → a fragment at
   `.changes/<slug>.md`.
7. **Is it what to do when a named alarm fires?** → Runbook
   (`docs/runbooks/<trigger>.md`).
8. **Did something break in production?** → Incident
   (`docs/incidents/YYYY-MM-DD-slug.md`).
9. **Are these instructions for how agents work in this repo?** →
   `AGENTS.md`.
10. **Is it how an outsider installs or uses the project?** → `README.md`, or
    user documentation — [Diátaxis](https://diataxis.fr/) territory, outside
    this convention.
11. **Is it true only for the next hour?** → a session Todo. Uncommitted by
    default; `docs/todo.md` only if you deliberately opt in.

## By lifetime

| Lifetime              | Artifact                  |
| --------------------- | ------------------------- |
| Project               | Charter                   |
| Living                | Design, Roadmap, Runbooks |
| Append-only           | Decisions, Incidents      |
| One branch / worktree | Plan                      |
| Per-release           | Changes                   |
| One session           | Todo                      |

## Common mis-routes

| Wrong place                                          | Right place                             |
| ---------------------------------------------------- | --------------------------------------- |
| Planned work written into Design                     | Decision, plus a Plan for the steps     |
| An accepted decision edited in place                 | A new decision that supersedes it       |
| A changelog reconstructed from commits at release    | Fragments written alongside each change |
| This branch's steps appended to the Roadmap          | Plan                                    |
| Architecture rationale left only in a PR description | Decision, committed to the repo         |

## If two answers fit

Split the content. Put the durable half in the longer-lived artifact — the
Charter or Design, usually — and a pointer or one-sentence summary in the
shorter-lived one. Cross-link rather than duplicate; a fact maintained in two
places will eventually be true in only one of them.
