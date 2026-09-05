---
title: Artifacts
description: Reference for every artifact — purpose, paths, lifetime, and a skeleton to copy.
---

One section per artifact, in the order they appear in the
[spec](../spec/next/index.md).

## Charter

Why the project exists, who it is for, and what is deliberately out of scope.

- Paths: `CHARTER.md` (small repo) or `docs/charter.md` (graduated).
- Lifetime: project — written once, revised rarely.
- Answers: why it exists, goals, route.

```markdown
# Charter

## Purpose

## Audience

## Out of scope

## Artifacts
```

Anti-pattern: letting the Charter drift into a status report. It should read
the same on day 1,000 as it did on day 1.

## Design

What the system is and does right now.

- Paths: `DESIGN.md` (small repo) or `docs/design.md` (graduated).
- Lifetime: living — kept current as the system changes.
- Answers: what the system is and does _now_.

```markdown
# Design

## Overview

## Components

## Data flow

## Constraints
```

Anti-pattern: describing planned or hypothetical behavior. That belongs in a
Decision or a Roadmap, not in Design.

## Decisions

What changed, why, and what it cost.

- Paths: `DECISIONS.md` (small repo) or one file per decision at
  `docs/decisions/NNNN-slug.md` (graduated).
- Lifetime: append-only — a decision is never rewritten, only superseded.
- Answers: what changed, why, what it cost.

```markdown
# 0007: <title>

- Status: proposed
- Date: 2026-01-01

## Context

## Decision

## Consequences
```

Anti-pattern: editing an accepted decision in place. Reverse it with a new
decision that references and supersedes the old one.

## Roadmap

What's next, in order.

- Paths: `ROADMAP.md` (small repo) or `docs/roadmap.md` (graduated).
- Lifetime: living — items are deleted, not archived, once they no longer
  apply.
- Answers: what's next, in order.

```markdown
# Roadmap

- [ ] Next thing to build
- [ ] Then this
```

Anti-pattern: letting finished or abandoned items pile up. A Roadmap is a
queue, not a history.

## Plan

The exact steps for the current decision.

- Paths: `PLAN.md` (small repo) or `docs/plan.md` (graduated).
- Lifetime: one branch or worktree — deleted no later than merge.
- Answers: exact steps for the current decision.

```markdown
# Plan: <decision title>

## Steps

1. ...
2. ...

## Verification
```

Anti-pattern: leaving a Plan committed after the work merges. It is scaffolding
for one branch, not a durable record — the Decision is the record.

## Todo

Where this session is.

- Paths: agent memory by default; `docs/todo.md` only if a repo opts in to
  committing it.
- Lifetime: one session.
- Answers: where this session is.

Anti-pattern: treating a committed Todo as durable state. It is scratch space
for picking a session back up, not a Plan and not a Decision.

## Changes

What will ship in the next release, in plain language.

- Paths: `.changes/<slug>.md`, the same path whether the repo is small or
  graduated.
- Lifetime: per-release — consumed and deleted when the release is cut.
- Answers: what will ship in the next release, in plain language.

Each line is a markdown unordered list item starting with one of the six
[Keep a Changelog](https://keepachangelog.com/) categories:

```markdown
- Added: support for custom output formats.
- Fixed: a race condition when releasing concurrently.
```

Anti-pattern: reconstructing release notes from commit messages after the
fact. Write the fragment in the same commit as the change.

## Runbooks

What to do when a named alarm fires.

- Paths: none at the root — runbooks only exist graduated, at
  `docs/runbooks/<trigger>.md`, one file per trigger.
- Lifetime: living.
- Answers: what to do when _x_ fires.

```markdown
# Runbook: <trigger>

## Symptoms

## Diagnosis

## Mitigation
```

Anti-pattern: a runbook that only a specific person can execute. Write for
whoever is paged.

## Incidents

What broke, what we learned.

- Paths: none at the root — incidents only exist graduated, at
  `docs/incidents/YYYY-MM-DD-slug.md`.
- Lifetime: append-only — not rewritten after the incident closes, except to
  append follow-up references.
- Answers: what broke, what we learned.

```markdown
# Incident: <slug>

- Date: 2026-01-01
- Severity:

## Timeline

## Root cause

## Follow-ups
```

Anti-pattern: closing the incident record before the follow-up items are
tracked somewhere durable — usually the Roadmap.
