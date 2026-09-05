---
title: Design
description: What the system is and does right now.
weight: 20
---

What the system is and does right now.

- Paths: `DESIGN.md` (small repo) or `docs/design.md` (graduated).
- Lifetime: living — kept current as the system changes.
- Answers: what the system is and does _now_.

## Skeleton

```markdown
# Design

## Overview

## Components

## Data flow

## Constraints
```

## Example

```markdown
# Design

## Overview

Bramble is a single Go binary. It walks a directory of markdown files,
extracts headings and body text, and writes a SQLite full-text index next to
the directory it scanned.

## Components

- `scanner` — walks the directory, skipping anything matched by
  `.brambleignore`.
- `indexer` — tokenizes each file and writes rows into `bramble.db`.
- `cli` — `bramble index <dir>` and `bramble search <query>`.

## Data flow

`scanner` emits one `Document` per file → `indexer` batches documents into a
single SQLite transaction → `cli search` queries the resulting FTS5 table.

## Constraints

- No network access at any point; the index is local-only.
- `bramble.db` MUST be readable by an unmodified `sqlite3` CLI, so a user can
  inspect it without Bramble installed.
```

**Anti-pattern:** describing planned or hypothetical behavior. "Bramble will
eventually support PDF files" belongs in a
[Decision](../decisions/index.md) once it's proposed, or the
[Roadmap](../roadmap/index.md) before that — Design describes only what
exists.
