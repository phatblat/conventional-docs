---
title: Quickstart
description: Four files to add today, plus the badge to tell people you conform.
---

Adoption fits in one sitting. No tool is required.

## Start here

Add these to the repository:

{{< details summary="CHARTER.md" >}}

```markdown
# Charter

## Purpose

## Audience

## Out of scope

## Artifacts

- Charter: CHARTER.md
- Design: DESIGN.md
- Decisions: docs/decisions/
```

{{< /details >}}

{{< details summary="DESIGN.md" >}}

```markdown
# Design

## Overview

## Components
```

{{< /details >}}

Decisions have no root form; the first one starts the directory:

{{< details summary="docs/decisions/YYYY-MM-DD-slug.md" >}}

```markdown
# <Decision title>

## Issue

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

## Argument

## Architectural Decision

## Positions

## Dates

- Published: TBD (set at merge).
```

{{< /details >}}

**`.changes/`** — an empty directory that gets a `<slug>.md` fragment file
alongside every user-facing change from now on.

## Wire the events (optional)

Publishing lifecycle events as Conventional Commits lets hooks and dashboards
key off `git log` without parsing files. Add the custom types to your
commitlint config:

{{< details summary="commitlint.config.js" >}}

```js
'type-enum': [2, 'always', [
  'build', 'chore', 'ci',
  'decision', 'deploy', 'docs', 'feat', 'fix', 'perf',
  'plan', 'refactor', 'release', 'revert', 'style', 'test',
  'todo',
]],
```

{{< /details >}}

## Tell people

{{< details summary="README.md" >}}

```markdown
[![Conventional Docs](https://img.shields.io/badge/Conventional%20Docs-draft-5B3DF5)](https://phatblat.github.io/conventional-docs/)
```

{{< /details >}}

## Required vs. recommended

No tool is required to start: everything above is plain markdown in git. A
CI check (a link checker, a fragment-format lint) and a generator or
coding-agent skill that reads and writes these files are not required
either — but both are highly recommended once adopted. A required check is
the checkpoint that confirms the right artifact was captured before an
agent launches, and a generator is the integration point where future
tools or spawned agents get event visibility and rule enforcement.
