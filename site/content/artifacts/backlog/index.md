---
title: Backlog
description: What one queued work item is, in full.
weight: 25
---

What one queued work item is, in full.

- Paths: none at the root — a Backlog item is always its own file at
  `docs/backlog/<slug>.md`. There is no small-repo form: a repository either
  has no Backlog or has the directory.
- Lifetime: until done — an item is deleted by the change that implements it.
- Answers: what one queued item is, in full.

A repository with a Backlog MUST have a [Roadmap](../roadmap/index.md); the
Roadmap's own location is independent of whether a Backlog exists. Once a
Backlog exists, the Roadmap stops describing work and starts indexing it: an
ordered list of links, never a restated body.

## The id

A Backlog item's id is a kebab-case slug of its title, and it is the filename
without `.md`. It carries no number and no date prefix — both are allocated
at write time and race the same way a sequential decision number once did,
and parallel work on a Backlog is exactly the scenario that race breaks. The
slug may change while the item exists; a rename is a file rename plus a
Roadmap link update, in one commit. Two items whose slugs collide are the
same item and must be merged into one file.

## Skeleton

Every item uses exactly this skeleton, with H2 sections in this order:

```markdown
# <Title>

## Problem

<What is wrong or missing, and who it costs. For a bug: what happens, what
should happen, and how to reproduce it.>

## Outcome

<What is observably true when this is done.>

## Notes

<Optional, last: evidence, links, a related decision, a suggested direction
the decision record is free to overrule.>
```

An item carries no status, priority, assignment, claim, or date field:
presence in `docs/backlog/` is the open state, order is the Roadmap's, and
dates are `git log`'s. A decision record MUST NOT link to a Backlog item,
because the item is deleted at implementation and the link would go stale; a
decision that grew out of an item restates the problem in its own `## Issue`.
A Backlog item MAY link to a decision.

An item is deleted by the change that implements it, together with its
Roadmap line, in the same pull request — never by a separate bookkeeping
commit. An item that will not be implemented is deleted rather than kept or
marked closed.

## Who is working on it

A Backlog item never records who is working on it. A branch that implements
one SHOULD end with the item's slug, so any prefix a team already uses
(`feat/`, `fix/`, an author or agent name) survives and concurrent work is
found by a suffix match over the refs a clone already fetches:

```bash
git ls-remote --heads origin '*<slug>'   # anyone, anywhere
git branch --all --list '*<slug>'        # what this clone already knows
```

This is detection, not arbitration: two workers who both start get two
branches, and the second to open a pull request loses the work, not the
repo. Push the branch before starting — an unpushed branch is an invisible
claim — and rename the branch along with the item if its slug changes while
the branch is open.

An arbitrated claim (first push wins, a reaped claim for a worker that dies
mid-task) is a dispatch queue, not a Backlog: it needs a form for the claim,
a compare-and-swap on write, and a way to reclaim a stale one, all of which
presume a trunk every worker can push. A Backlog item is what such a queue's
records point at, not the queue itself.

## Example

`docs/backlog/enable-github-pages.md`:

```markdown
# Enable GitHub Pages

## Problem

The site builds locally but is not published anywhere: `Settings → Pages` has
no source configured, and `.github/workflows/pages.yml` only builds on pull
requests.

## Outcome

GitHub Pages is set to deploy from GitHub Actions, `pages.yml` also runs on
`push: branches: [main]`, and the built site is reachable at its `github.io`
URL.
```

Indexed from `ROADMAP.md`:

```markdown
## Site

- [Enable GitHub Pages](docs/backlog/enable-github-pages.md)
```

**Anti-pattern:** keeping a finished item around with a `[done]` marker
instead of deleting it — a Backlog is a queue, not a history, and what
shipped is the [Changelog](../changelog/index.md)'s question. Letting an item
grow into a spec, with alternatives weighed and a rationale recorded, is
also a mis-route: that reasoning belongs in a [Decision](../decisions/index.md).
