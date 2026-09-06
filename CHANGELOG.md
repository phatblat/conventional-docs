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
- A website presenting Conventional Docs as a versioned specification, with a routing guide, artifact reference, and quickstart.
- A `details` shortcode that renders code samples as native, no-JS collapsible `<details>`/`<summary>` blocks, initially collapsed.
- An `Example` page showing a conforming repository, small and graduated, moved off the spec page.
- A detail page for every artifact under `/artifacts/` (Charter, Design, Decisions, Roadmap, Plan, Changelog, Runbooks, Incidents, Todo), each with a skeleton and a worked example — Runbooks and Incidents stay a one-paragraph hint, since this project doesn't use them yet.
- A diagram of the loop on the homepage, alongside the existing text version.
- The Events artifact to the site's artifacts tables, with the same "proposed, not settled" note as `README.md`.
- A `/condoc/` page documenting the `condoc` binary's commands, design constraints, and roadmap.

### Changed

- Decision records are identified by date (`YYYY-MM-DD-slug`) instead of a sequential number, so ids no longer race between branches.
- The Todo artifact is now `TODO.md` at the repository root, committed as a cache of the agent's working list, replacing `agent memory` and the opt-in `docs/todo.md`.
- Release notes are curated in this file's `[Unreleased]` section instead of `.changes/*.md` fragment files, and no check requires a changelog edit on a change.
- A decision record is frozen when review ends: four states (draft, proposed, accepted, rejected), no `## Dates` section, and later corrections in an append-only `## Errata` tail (`2026-09-05-freeze-a-decision-when-review-ends`).
- The five code samples on the Quickstart page now use the `details` shortcode.
- Every mention of Keep a Changelog on the site now links to the pinned `2.0.0` version (`https://keepachangelog.com/en/2.0.0/`).
- The artifacts table (homepage and `/artifacts/`) now lists the six root-capable artifacts first, then the three that live only under `docs/`, each group ordered by where it falls in the Charter's lifecycle phases.
- The spec page now links to `Artifacts` and `Example` instead of duplicating their tables and file trees.
- The site now ships a single Doks theme; the preview theme switcher, FixIt, and Hugoplate styles are gone.
- The "Where does this go?" nav item, hero button, and card are now labeled `wheredoc`.
- The page container grows wider on large displays instead of staying letterboxed.
- Todo is now listed and documented immediately after Plan wherever artifacts are enumerated.
- The "no required CI check, no required generator" wording now recommends both as checkpoints and integration points, without making them mandatory.
- The site now documents the four-state decision lifecycle (draft, proposed, accepted, rejected) with its append-only `## Errata` tail, and the `[Unreleased]` changelog rules, replacing the removed Changes artifact and `.changes/` fragments.

### Removed

- The single-file `DECISIONS.md` form; every decision is its own file under `docs/decisions/`, following a fixed skeleton.
- The `docs/plan.md` and `docs/todo.md` graduated forms — a Plan and a Todo never graduate.
- The Changes artifact (`.changes/<slug>.md`), its validator, and its release plugin; the convention's artifact set is nine documents.

### Fixed

- The spec's `decision: propose <id>` subject no longer carries a leftover `<title>` argument from the old numbered-id era — the id is the whole reference, matching `EVENTS.md`.
- The site's Decisions, Plan, and Todo paths, and its `by lifetime` table, to match the current convention — Decisions is `docs/decisions/YYYY-MM-DD-slug.md` only, and Plan and Todo live at the repository root with no graduated form.
