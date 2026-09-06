---
title: FAQ
description: Answers to common adoption questions.
---

## Why markdown instead of issues or a wiki?

Markdown in git reviews alongside the code that motivated it, survives a
change of issue tracker or wiki host, and is readable by any coding agent
without an API call. Issues and wikis live outside the commit history that
explains them.

## Why not just use ADRs?

Decisions here _are_ an ADR log with a status lifecycle — a proposed decision
is the spec. Conventional Docs adds the other artifacts an ADR log doesn't
cover: a Charter, a living Design doc, a Roadmap, a Plan, a Todo, and a
curated changelog, all at predictable paths.

## How is this different from spec-kit or Kiro?

Spec-kit and Kiro drive a spec → plan → tasks chain per feature, usually
discarded once the feature ships. Conventional Docs is project-level state
plus an append-only decision log; only the Plan is discarded, at merge, and
the Decision it came from stays forever.

## Do I need all nine artifacts?

No. Charter, Design, and Decisions are the load-bearing three. Roadmap, Plan,
Todo, Runbooks, and Incidents are OPTIONAL — add them when the project
actually needs them.

## What if my repo already has a `docs/` site?

Nothing conflicts. This convention governs a handful of specific files;
`docs/` can otherwise hold whatever your existing documentation tool expects.

## Does this replace user documentation?

No. Tutorials, how-tos, and reference material are
[Diátaxis](https://diataxis.fr/) territory. Design overlaps with
explanation/reference, and runbooks overlap with how-to, but neither replaces
a documentation site aimed at end users.

## Can agents be trusted to maintain these files?

The predictable paths and small, fixed shapes are precisely what make that
tractable: an agent doesn't have to guess where a decision belongs, it looks
it up. The reference implementation includes an agent skill built on exactly
this assumption.

## Do I need condoc?

No. [`condoc`](../condoc/index.md) is experimental, macOS arm64 only, and
writes nothing a person could not write by hand; it exists so the exact
status prose, ids, reciprocal links, and commit subjects are produced
mechanically.

## What happens when a document gets too big?

Graduate it. See [Graduating](../graduating/index.md) for the two triggers
and the one-commit move.

## How do I version my own extension of this convention?

Use [SemVer](https://semver.org/), the same way Conventional Commits
recommends for extensions to its own type list.
