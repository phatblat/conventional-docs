# Replace change fragments with Unreleased

## Issue

The convention defines a tenth artifact, `.changes/<slug>.md`: one file per
user-facing change, each line a Keep a Changelog category, concatenated and
deleted at release time. Keep a Changelog has carried an `[Unreleased]` section
for the same purpose since its own 0.0.5 release — 2014-08-09, "Unreleased
section to gather unreleased changes and encourage note keeping prior to
releases", recorded in
[its changelog](https://github.com/olivierlacan/keep-a-changelog/blob/main/CHANGELOG.md).
Its [2.0.0](https://keepachangelog.com/en/2.0.0/) guidance makes the practice
explicit: keep `Unreleased` at the top, and let automation "move the
`Unreleased` section into a dated version at release time".

So the convention specifies an artifact, a validator, and a release plugin to
reproduce a section of the format it already cites. It also drives a rule that
2.0.0 names as a bad pattern. `AGENTS.md` says "Every user-facing change adds a
`.changes/<slug>.md` fragment in the same commit", and `just lint-changes`
enforces the format; 2.0.0 answers: "Do not make a changelog edit a required
check on every change. That teaches people to add a line to pass the check,
which fills the changelog with noise."

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- Keep a Changelog 2.0.0 (2026-06-07) changed the guidance, not the format. The
  six types, `YYYY-MM-DD` dates, and the `Unreleased` and `[YANKED]` markers are
  unchanged, so nothing here is a reaction to a new capability.
- `CHANGELOG.md` is one of the four files this convention pins to the root
  permanently, and GitHub renders it without anything having to assemble it.
- The Charter names "costs attention, not tooling" as a goal. A fragment
  directory cannot be read as a changelog until a tool runs.
- The known cost of one hot section is merge conflicts between concurrent
  branches. This is real, and it is the reason
  [towncrier](https://towncrier.readthedocs.io/),
  [Changesets](https://github.com/changesets/changesets), and
  [changie](https://changie.dev/) exist.
- This repo implements fragments in `scripts/changes-lib.mjs`,
  `scripts/validate-changes.mjs`, and `scripts/semantic-release-changes.mjs`,
  wired through `.releaserc.json` and `just lint-changes`.

## Argument

Adopt `[Unreleased]` and delete the artifact. **Chosen.** The test for an
artifact in this convention is whether it answers a question no other artifact
answers, at a path a reader can guess. `.changes/` answers "what will ship
next" — which `[Unreleased]` answers at a path every reader already knows, in
the file that is already permanent and already rendered. Two files answering one
question is the duplication this convention exists to remove.

The merge-conflict argument is the honest case for fragments, and it loses on
the shape of the conflict rather than its frequency. A conflict inside a
category list resolves by keeping both lines, in either order, because order
within a type carries no meaning — the cheapest resolution git has. It only
arises between branches that both ship a notable user-facing change, which is a
small fraction of branches once clause 4 stops requiring a line per commit.
Weighed against that: a permanent tenth artifact, a `just` recipe, and three
scripts, in a convention whose stated price is attention.

Generating the changelog from commits instead (**Rejected**) is the option 2.0.0
argues against at length, and Positions records why.

## Architectural Decision

1. The Changes row is deleted from the artifacts table; the set drops to nine.
   The Charter's SDLC table names `CHANGELOG.md` directly for the Release phase
   instead of `Changes → CHANGELOG`.
2. `CHANGELOG.md` answers both "what shipped" and "what will ship next". It
   stays at the root permanently and never graduates.
3. Its shape follows Keep a Changelog 2.0.0: a `# Changelog` heading with the
   fixed preamble, its format link pinned to `/en/2.0.0/`; `## [Unreleased]` at
   the top; released versions as `## [x.y.z] - YYYY-MM-DD`, newest first; the
   six types as `###` subsections; `**Breaking:**` markers inside the type they
   belong to; and reference-style links at the bottom resolving each version to
   a compare diff, with `[Unreleased]` comparing the latest tag to `HEAD`.
4. A notable user-facing change adds its line to `[Unreleased]` in the same
   commit or pull request as the change, keeping the latitude `README.md`
   already allows fragments. Notable is a judgment, not a gate: no check
   requires a changelog edit on a change, and nothing validates that one is
   present.
5. The `release:` event renames `[Unreleased]` to the new version in both the
   heading and its reference link, adds a fresh empty `[Unreleased]` pointing at
   `HEAD`, and leaves the released section untouched thereafter.
6. Everything that exists to serve fragments is removed in one commit, and the
   fragments outstanding at that moment are folded into `[Unreleased]` in the
   same commit:
   - `.changes/`, including its `README.md`;
   - `scripts/changes-lib.mjs`, `scripts/validate-changes.mjs`, and
     `scripts/semantic-release-changes.mjs`;
   - the `lint-changes` recipe in `justfile`, and its entry in `check`;
   - in `.releaserc.json`, the `./scripts/semantic-release-changes.mjs` plugin
     entry and `".changes"` from the `@semantic-release/git` assets list;
   - the Release notes section of `README.md`, the Release-note fragments
     section of `skills/conventional-docs/SKILL.md`, the fragment rule in
     `AGENTS.md`, and the Changes row in the artifacts tables of `README.md`,
     `skills/conventional-docs/SKILL.md`, and `CHARTER.md`.
7. `@semantic-release/changelog` is removed from `.releaserc.json` in the same
   commit. It generates entries from commits, which clause 4 makes wrong for a
   hand-curated file. semantic-release keeps deciding the version, publishing,
   and committing `release: v<x>`; promoting `[Unreleased]` per clause 5 becomes
   a step of its own. That step is a prerequisite of this decision, not a
   follow-up: it must exist in the same commit, whether as a small release
   plugin or as `condoc release`.
8. A conflict in `[Unreleased]` is resolved by keeping both lines. Order within
   a type is not meaningful.

## Consequences

`CHANGELOG.md` becomes hand-curated above the newest released heading. Clause 7
makes replacing `@semantic-release/changelog` part of this change rather than a
thing to settle afterwards, because a release cut between the two states would
either lose the curated section or regenerate over it.

`just check` loses `lint-changes`. The convention gains no replacement check:
clause 4 is deliberate, and there is nothing mechanical left to validate beyond
the markdown gates every file already passes.

The nine-artifact table is a user-visible change to the convention and needs a
`CHANGELOG.md` entry of its own.

## Positions

- **Keep `.changes/`, reframed as staging for `[Unreleased]`.** _Rejected._
  Removes the format duplication in prose but not in files: still a tenth
  artifact, a validator, and a release plugin reproducing a section the cited
  format defines. Its one real advantage is conflict avoidance, answered above.
- **Keep both, fragments feeding `[Unreleased]` continuously.** _Rejected._ Two
  sources for one answer, and a reader has no way to tell which is current.
- **Generate the changelog from Conventional Commits.** _Rejected._ Keep a
  Changelog 2.0.0: "a commit and a changelog entry are written for different
  people, and one does not convert cleanly into the other", and "machines can
  draft, but humans curate". It would also make the changelog a function of
  commit subjects, which this convention already loads with lifecycle events
  that are not user-facing changes at all.
- **Rely on the host's generated release notes.** _Rejected._ The Charter's
  founding rule is that every artifact lives in git; 2.0.0 makes the same
  argument about vendor lock-in.

## References

- [Keep a Changelog 2.0.0](https://keepachangelog.com/en/2.0.0/) — the format,
  the `[Unreleased]` guidance, and the automation and curation advice quoted
  above.
- [Changesets](https://github.com/changesets/changesets) and
  [towncrier](https://towncrier.readthedocs.io/) — the fragment-directory
  precedent being dropped.
- [CHARTER.md](../../CHARTER.md) — the "costs attention, not tooling" goal.

## Dates

- Published: TBD (set at merge).
