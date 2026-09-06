---
title: Example
description: A conforming repository, small and graduated.
---

A conforming small repo:

```text
CHARTER.md
DESIGN.md
ROADMAP.md
AGENTS.md
README.md
CHANGELOG.md
docs/
  decisions/
    2026-02-11-split-the-scheduler.md
```

The same repository after graduating:

```text
docs/
  charter.md
  design.md
  decisions/
    2026-02-11-split-the-scheduler.md
    2026-03-02-drop-plugin-api.md
  roadmap.md
  runbooks/
    high-error-rate.md
  incidents/
    2026-03-02-billing-outage.md
AGENTS.md
README.md
CHANGELOG.md
```

`README.md`, `AGENTS.md`, and `CHANGELOG.md` never move. Decisions, Runbooks, and
Incidents have only a graduated path and live under `docs/` from their first
entry, whether or not the rest of the repository has graduated. Nothing exists
at both the small-repo path and the graduated path at once.

See [Artifacts](../artifacts/index.md) for what each file answers, its
lifetime, and a skeleton and worked example for each; [Graduating](../graduating/index.md)
for when and how to make this move; and the
[spec](../spec/next/index.md) for the normative rules this example follows.
