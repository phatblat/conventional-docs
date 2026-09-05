---
title: Charter
description: Why the project exists, who it is for, and what is deliberately out of scope.
weight: 10
---

Why the project exists, who it is for, and what is deliberately out of scope.

- Paths: `CHARTER.md` (small repo) or `docs/charter.md` (graduated).
- Lifetime: project — written once, revised rarely.
- Answers: why it exists, goals, route.

## Skeleton

```markdown
# Charter

## Purpose

## Audience

## Out of scope

## Artifacts
```

The `## Artifacts` section is the one part every conforming Charter MUST
have: it records the current location of every artifact the repository uses,
so it is the first place to check when in doubt about where something lives.

## Example

```markdown
# Charter

## Purpose

Bramble is a CLI that turns a folder of markdown notes into a static search
index. It exists so a note-taker can grep their own thinking without running
a server.

## Audience

People who already keep notes in markdown and are comfortable with a
terminal. Not aimed at teams or at anyone who wants a hosted product.

## Out of scope

No sync, no accounts, no web UI. Bramble reads a directory and writes an
index file next to it; anything past that is a different tool.

## Artifacts

- Charter: CHARTER.md
- Design: DESIGN.md
- Decisions: docs/decisions/
- Roadmap: ROADMAP.md
```

**Anti-pattern:** letting the Charter drift into a status report. It should
read the same on day 1,000 as it did on day 1 — what changed lately belongs in
[Design](../design/index.md) or the [Roadmap](../roadmap/index.md), not here.
