---
title: Roadmap
description: What's next, in order.
weight: 20
---

What's next, in order.

- Paths: `ROADMAP.md` (small repo) or `docs/roadmap.md` (graduated).
- Lifetime: living — items are deleted, not archived, once they no longer
  apply.
- Answers: what's next, in order.

## Skeleton

```markdown
# Roadmap

- [ ] Next thing to build
- [ ] Then this
```

A graduated Roadmap that needs per-item status or grouping can grow headings
and metadata; the ordering is the one thing that must survive that growth.

## As the index over a Backlog

Once a repository also has a [Backlog](../backlog/index.md), the Roadmap
stops describing work and starts indexing it: an ordered list of links, one
per item, never a restated body. Bullets stay plain `-`, not checkboxes — an
item's presence in `docs/backlog/` is its open state, so there is nothing to
check off:

```markdown
# Roadmap

## Now

- [Support .brambleignore glob negation](docs/backlog/support-brambleignore-glob-negation.md)
```

An item's line here and its file are added and deleted together, by the same
change.

## Example

```markdown
# Roadmap

What's next, in order. Check items off in place rather than filing a
separate issue for each one, and delete anything that no longer applies.

## Now

- [ ] Support `.brambleignore` glob negation (`!keep-this.md`).

## Next

- [ ] Add `bramble search --json` for scripting.
- [ ] Index PDF text once a decision picks an extraction library.

## Later

- [ ] Investigate an incremental index instead of a full rescan.
```

**Anti-pattern:** letting finished or abandoned items pile up. A Roadmap is a
queue, not a history — once an item ships it belongs in the
[Changelog](../changelog/index.md), and once it's abandoned it's deleted
outright.

A repo whose existing `TODO.md` is a single durable file is actually a
Roadmap under the wrong name; rename it to `ROADMAP.md`. Where it is a
directory of per-item files, it is a [Backlog](../backlog/index.md) and
moves to `docs/backlog/`. Either way, keep the real
[Todo](../todo/index.md) for branch-scoped work in flight.
