---
title: Graduating
description: When and how an artifact moves from a root file to a graduated location.
---

Small repos keep everything as `UPPERCASE.md` at the root. Move a document to
`docs/` when either trigger fires:

1. the root is getting cluttered with top-level files and folders (dotfiles
   don't count — that's where config conventions live), or
2. the document has outgrown a single file: it needs siblings, per-item
   status, or internal structure (a `ROADMAP.md` that needs per-item status
   becomes `docs/roadmap.md`; a `DECISIONS.md` splits into
   `docs/decisions/`).

## The one-commit move

Graduate in a single commit: move the file, rewrite every inbound link, and
update the Charter's `## Artifacts` section — all in that commit. No stub
files at the old path, and no mirrors in either direction. A link check in CI
catches anything the commit missed.

## Decisions specifically

`docs/decisions/` is the canonical graduated location. For `adr-tools`
compatibility, add a root `.adr-dir` file containing `docs/decisions`; MADR
already defaults to this path.

## Graduating out of the repository entirely

An artifact may leave the repository altogether — a Roadmap that moves into an
external planning tool, for example. When that happens, the Charter's
`## Artifacts` section records the external location, and the stale in-repo
file is deleted in the same commit. The convention doesn't require everything
to live in git forever; it requires that the Charter always says where the
artifact currently lives.
