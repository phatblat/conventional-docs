---
title: Changes
description: What will ship in the next release, in plain language.
weight: 70
---

What will ship in the next release, in plain language.

- Paths: `.changes/<slug>.md` — the same path whether the repository is small
  or graduated.
- Lifetime: per-release — consumed and deleted when the release is cut.
- Answers: what will ship in the next release, in plain language.

One file per user-facing change, added in the same commit or pull request as
the change itself, with a kebab-case slug matching the change. Every
non-blank line is a markdown unordered list item opening with one of the six
[Keep a Changelog](https://keepachangelog.com/) categories — `Added`,
`Changed`, `Deprecated`, `Removed`, `Fixed`, `Security` — in the exact form
`- <Category>: <description>`.

## Skeleton

```markdown
- Added: <what a user of the software gets>.
- Fixed: <what was broken and now isn't>.
```

## Example

`.changes/scheduler-process.md`:

```markdown
- Added: a standalone `scheduler` process, so a slow job no longer blocks API
  requests.
- Changed: job status is now visible via `bramble jobs status` while the
  scheduler is running.
- Deprecated: the `--inline-scheduler` flag; it will be removed once the
  in-process poller is deleted.
```

At release time, every fragment is concatenated, grouped by category, and
folded into that release's notes and `CHANGELOG.md`. The consumed fragment
files are deleted in the same commit as the version bump —
`.changes/` holds only what hasn't shipped yet. `CHANGELOG.md` itself follows
[Keep a Changelog](https://keepachangelog.com/) and is never hand-edited;
version fields are never hand-bumped.

**Anti-pattern:** reconstructing release notes from commit messages after the
fact. Write the fragment in the same commit as the change, while the reason
for it is still in your head.
