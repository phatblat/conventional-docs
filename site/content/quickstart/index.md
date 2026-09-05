---
title: Quickstart
description: Four files to add today, plus the badge to tell people you conform.
---

Adoption fits in one sitting. No tool is required.

## Start here

Add these four files at the repository root:

**`CHARTER.md`**

```markdown
# Charter

## Purpose

## Audience

## Out of scope

## Artifacts

- Charter: CHARTER.md
- Design: DESIGN.md
- Decisions: DECISIONS.md
```

**`DESIGN.md`**

```markdown
# Design

## Overview

## Components
```

**`DECISIONS.md`**

```markdown
# Decisions

## 0001: <first decision>

- Status: proposed
- Date: 2026-01-01

### Context

### Decision

### Consequences
```

**`.changes/`** — an empty directory that gets a `<slug>.md` fragment file
alongside every user-facing change from now on.

## Wire the events (optional)

Publishing lifecycle events as Conventional Commits lets hooks and dashboards
key off `git log` without parsing files. Add the custom types to your
commitlint config:

```js
'type-enum': [2, 'always', [
  'build', 'chore', 'ci',
  'decision', 'deploy', 'docs', 'feat', 'fix', 'perf',
  'plan', 'refactor', 'release', 'revert', 'style', 'test',
]],
```

## Tell people

```markdown
[![Conventional Docs](https://img.shields.io/badge/Conventional%20Docs-draft-5B3DF5)](https://phatblat.github.io/conventional-docs/)
```

## Required vs. recommended

No tool is required to start: everything above is plain markdown in git. A
CI check (a link checker, a fragment-format lint) and a generator or
coding-agent skill that reads and writes these files are not required
either — but both are highly recommended once adopted. A required check is
the checkpoint that confirms the right artifact was captured before an
agent launches, and a generator is the integration point where future
tools or spawned agents get event visibility and rule enforcement.
