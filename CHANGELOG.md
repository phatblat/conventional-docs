# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- A `conventional-docs` agent skill, installable with `npx skills add phatblat/conventional-docs` or as a Claude Code plugin.
- `EVENTS.md`, a proposed home for the lifecycle event vocabulary, graduating to `docs/events.md`.
- `todo: sync` and `todo: clear` commit events, plus the rule that a `plan:` or `todo:` commit touches only its own artifact so the bookkeeping can be dropped or squashed away.
- `condoc`, a binary that writes the convention's artifacts and their lifecycle commits: `init`, `new charter|design|roadmap|runbook|incident`, and the `dec` verbs (`2026-09-05-condoc-a-binary-for-the-document-lifecycle`).

### Changed

- Decision records are identified by date (`YYYY-MM-DD-slug`) instead of a sequential number, so ids no longer race between branches.
- The Todo artifact is now `TODO.md` at the repository root, committed as a cache of the agent's working list, replacing `agent memory` and the opt-in `docs/todo.md`.
- Release notes are curated in this file's `[Unreleased]` section instead of `.changes/*.md` fragment files, and no check requires a changelog edit on a change.
- A decision record is frozen when review ends: four states (draft, proposed, accepted, rejected), no `## Dates` section, and later corrections in an append-only `## Errata` tail (`2026-09-05-freeze-a-decision-when-review-ends`).

### Removed

- The single-file `DECISIONS.md` form; every decision is its own file under `docs/decisions/`, following a fixed skeleton.
- The `docs/plan.md` and `docs/todo.md` graduated forms — a Plan and a Todo never graduate.
- The Changes artifact (`.changes/<slug>.md`), its validator, and its release plugin; the convention's artifact set is nine documents.
