# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- A `conventional-docs` agent skill, installable with `npx skills add phatblat/conventional-docs` or as a Claude Code plugin.
- `EVENTS.md`, a proposed home for the lifecycle event vocabulary, graduating to `docs/events.md`.
- `todo: sync` and `todo: clear` commit events, plus the rule that a `plan:` or `todo:` commit touches only its own artifact so the bookkeeping can be dropped or squashed away.
- `doc`, a binary that writes the convention's artifacts and their lifecycle commits: `doc init`, `doc <artifact> new`, and the `doc decision` verbs, with `dec` as its one alias (`2026-09-05-condoc-a-binary-for-the-document-lifecycle`, `2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact`).
- `doc readme|contributing|security|support new`, `doc license new --spdx <id>`, `doc code-of-conduct new --contact <method>`, and `doc pack list [--json]`, writing the adjacent-files ring's convention-owned templates and standards-pack bodies (`2026-09-07-adjacent-files-and-the-standards-pack`).
- A standards pack embedded in the binary: Apache-2.0, BSD-2-Clause, BSD-3-Clause, GPL-3.0-or-later, ISC, MIT, MPL-2.0, and Unlicense license texts, plus the Contributor Covenant 3.0 code of conduct, resolved in order from `--pack <dir>`, `$XDG_DATA_HOME/conventional-docs/pack`, then the embedded copy.
- The adjacent-files ring — `README.md`, the license, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `SUPPORT.md`, `CODEOWNERS`, `CITATION.cff`, `.github/FUNDING.yml`, and issue/PR/discussion templates — as files this convention places but does not define, with a fixed set of recognized locations per file and a one-location rule.
- `doc init --semver <x.y.z>`, so a repository on a different SemVer version isn't forced to hand-edit the generated changelog preamble.
- A website presenting Conventional Docs as a versioned specification, with a routing guide, artifact reference, and quickstart.
- A `details` shortcode that renders code samples as native, no-JS collapsible `<details>`/`<summary>` blocks, initially collapsed.
- An `Example` page showing a conforming repository, small and graduated, moved off the spec page.
- A detail page for every artifact under `/artifacts/` (Charter, Design, Decisions, Roadmap, Plan, Changelog, Runbooks, Incidents, Todo), each with a skeleton and a worked example — Runbooks and Incidents stay a one-paragraph hint, since this project doesn't use them yet.
- A diagram of the loop on the homepage, alongside the existing text version.
- The Events artifact to the site's artifacts tables, with the same "proposed, not settled" note as `README.md`.
- A `/doc/` page documenting the `doc` binary's commands, design constraints, and roadmap.
- A Backlog artifact — one work item per file at `docs/backlog/<slug>.md`, deleted by the change that implements it — as the second remedy for a Roadmap that has outgrown one file (`2026-09-18-graduate-the-roadmap-into-a-backlog`).
- A `What's different` section in `README.md` and a `What this convention adds` section in `CHARTER.md`, naming the three rules that distinguish the convention from the artifact set it assembles: a decision that freezes when review ends, a declared lifetime as a commitment to delete, and net-zero `plan:`/`todo:` bookkeeping commits.

### Changed

- Decision records are identified by date (`YYYY-MM-DD-slug`) instead of a sequential number, so ids no longer race between branches.
- The Todo artifact is now `TODO.md` at the repository root, committed as a cache of the agent's working list, replacing `agent memory` and the opt-in `docs/todo.md`.
- Release notes are curated in this file's `[Unreleased]` section instead of `.changes/*.md` fragment files, and no check requires a changelog edit on a change.
- A decision record is frozen when review ends: four states (draft, proposed, accepted, rejected), no `## Dates` section, and later corrections in an append-only `## Errata` tail (`2026-09-05-freeze-a-decision-when-review-ends`).
- The five code samples on the Quickstart page now use the `details` shortcode.
- Every mention of Keep a Changelog on the site now links to the pinned `2.0.0` version (`https://keepachangelog.com/en/2.0.0/`).
- The artifacts table (homepage and `/artifacts/`) now lists the six root-capable artifacts first, then the four that live only under `docs/`, each group ordered by where it falls in the Charter's lifecycle phases.
- The spec page now links to `Artifacts` and `Example` instead of duplicating their tables and file trees.
- The site now ships a single Doks theme; the preview theme switcher, FixIt, and Hugoplate styles are gone.
- The "Where does this go?" nav item, hero button, and card are now labeled `wheredoc`.
- The page container grows wider on large displays instead of staying letterboxed.
- Todo is now listed and documented immediately after Plan wherever artifacts are enumerated.
- The "no required CI check, no required generator" wording now recommends both as checkpoints and integration points, without making them mandatory.
- The site now documents the four-state decision lifecycle (draft, proposed, accepted, rejected) with its append-only `## Errata` tail, and the `[Unreleased]` changelog rules, replacing the removed Changes artifact and `.changes/` fragments.
- A graduated Roadmap is the ordered index over `docs/backlog/`, linking each item instead of describing it; this project's own `ROADMAP.md` is now that index.

### Removed

- The single-file `DECISIONS.md` form; every decision is its own file under `docs/decisions/`, following a fixed skeleton.
- The `docs/plan.md` and `docs/todo.md` graduated forms — a Plan and a Todo never graduate.
- The Changes artifact (`.changes/<slug>.md`), its validator, and its release plugin; the convention's artifact set is ten documents.

### Fixed

- The spec's `decision: propose <id>` subject no longer carries a leftover `<title>` argument from the old numbered-id era — the id is the whole reference, matching `EVENTS.md`.
- The site's Decisions, Plan, and Todo paths, and its `by lifetime` table, to match the current convention — Decisions is `docs/decisions/YYYY-MM-DD-slug.md` only, and Plan and Todo live at the repository root with no graduated form.
