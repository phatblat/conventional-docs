---
title: Changelog
description: What shipped, and what will ship next.
weight: 70
---

What shipped, and what will ship next.

- Paths: `CHANGELOG.md` at the repository root — the only path; it never
  graduates.
- Lifetime: living at the top, append-only below — `## [Unreleased]` is
  curated by hand, and a released section is never edited after release.
- Answers: what shipped, and what will ship next.

`CHANGELOG.md` follows [Keep a Changelog 2.0.0](https://keepachangelog.com/en/2.0.0/):
a `# Changelog` heading with the fixed preamble, `## [Unreleased]` at the top,
released versions as `## [x.y.z] - YYYY-MM-DD` newest first, the six
categories — `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`,
`Security` — as `###` subsections, `**Breaking:**` markers inside the
category they belong to, and reference-style links at the bottom resolving
each version to a compare diff, with `[Unreleased]` comparing the newest tag
to `HEAD`.

Add a notable user-facing change to `## [Unreleased]` in the same commit or
pull request as the change. Notable is a judgment: no check requires a
changelog edit, and a change no user would notice gets no line.

Never hand-edit a released section, and never hand-bump a version heading or
a version field. The `release:` event renames `[Unreleased]` to the new
version in both the heading and its reference link, and opens a fresh empty
`[Unreleased]`. Resolve a conflict in `[Unreleased]` by keeping both lines;
order within a category carries no meaning.

A line may cite the decision id it came from — a path in the repo rather than
one host's number, the portable form Keep a Changelog 2.0.0 recommends over a
bare `(#1234)`.

## Skeleton

```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
```

## Example

```markdown
## [Unreleased]

### Added

- A standalone `scheduler` process, so a slow job no longer blocks API
  requests (`2026-02-11-split-the-scheduler`).

### Changed

- Job status is now visible via `bramble jobs status` while the scheduler is
  running.

### Deprecated

- The `--inline-scheduler` flag; it will be removed once the in-process
  poller is deleted.

## [1.4.0] - 2026-01-30

### Fixed

- A race between two schedulers claiming the same job.

[Unreleased]: https://github.com/example/bramble/compare/v1.4.0...HEAD
[1.4.0]: https://github.com/example/bramble/compare/v1.3.0...v1.4.0
```

**Anti-pattern:** reconstructing release notes from commit messages at
release time. Write the line in the same commit as the change, while the
reason for it is still in your head.
