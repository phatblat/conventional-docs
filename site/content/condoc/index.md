---
title: condoc
description: The optional binary that writes the artifacts and their lifecycle commits.
---

_`condoc` is **experimental**. v1 is macOS arm64 only, no binaries are
published yet, and it is optional — a repo maintained entirely by hand stays
conformant._

Build and run from a clone (Rust 1.98+):

```bash
cargo install --path crates/condoc   # or: cargo run -p condoc -- <args>
condoc --version
```

## Commands

| Command                                       | Writes                                                  | Commit subject             |
| --------------------------------------------- | ------------------------------------------------------- | -------------------------- |
| `condoc init [--add]`                         | `CHARTER.md`, `DESIGN.md`, `ROADMAP.md`, `CHANGELOG.md` | `docs: add <names>`        |
| `condoc new charter` \| `design` \| `roadmap` | that artifact                                           | `docs: add charter` (etc.) |
| `condoc new runbook <trigger>`                | `docs/runbooks/<slug>.md`                               | `docs: add runbook <slug>` |
| `condoc new incident <slug>`                  | `docs/incidents/<today>-<slug>.md`                      | `docs: add incident <id>`  |
| `condoc dec draft <title>`                    | a record in the **draft** state                         | `decision: draft <id>`     |
| `condoc dec propose <title>` \| `<id>`        | a new **proposed** record, or promotes a draft          | `decision: propose <id>`   |
| `condoc dec accept <id>`                      | freezes the record as **accepted**                      | `decision: accept <id>`    |
| `condoc dec reject <id>`                      | freezes the record as **rejected**                      | `decision: reject <id>`    |
| `condoc dec errata <id> <text>`               | a dated line in `## Errata`                             | `docs: errata <id>`        |

- `dec` aliases `decision`.
- `--no-commit` is global: it writes the files and leaves the tree dirty.
- `dec propose` takes `--extends <id>` and `--supersedes <id>`, and writes
  both files in one commit — the new record's Issue lead sentence and the
  reciprocal link on the record it points at. `--supersedes` requires a
  frozen target, appending an erratum; naming a draft or proposed target
  refuses, since an unfrozen record is edited in place instead.
- `dec accept` and `dec reject` require the record to be `proposed`.
- `dec errata` refuses a draft or proposed record; it is the only write v1
  makes to a frozen record.
- `new` covers only the artifacts that have no lifecycle event — Charter,
  Design, Roadmap, runbooks, incidents. Decisions have their own verbs, so
  the tool cannot write a record whose event was never announced. `init`
  never writes `EVENTS.md` (still proposed), `PLAN.md`, or `TODO.md` (they
  belong to a branch), and re-running it reports what is missing and creates
  nothing without `--add`.
- `init` reports whether a commitlint config exists and whether its
  `type-enum` covers `decision`, `deploy`, `plan`, `release`, `todo`. It
  never edits that config.

## What it will not do

- No configuration file — the filesystem is authoritative. It probes root
  `UPPERCASE.md` then `docs/<lowercase>.md`, and a Charter `## Artifacts`
  table that disagrees is a finding, not a fact.
- Reads go through [gitoxide](https://github.com/GitoxideLabs/gitoxide)
  in-process, while writes exec the `git` binary, so `commit-msg` hooks,
  signing, and `gitconfig` all apply.
- Every commit uses the explicit pathspec form
  (`git commit -m <subject> -- <paths>`), never `add -A` or `commit -a`, so
  an unrelated staged change can never ride along.
- It writes nothing a person could not have written by hand.

## Not yet built

- Bookkeeping: `condoc status`, `condoc todo sync|clear`, `condoc plan start|done`.
- Validation: `condoc lint|fix|check`, including the drift check that diffs a
  frozen record against its freeze commit.
- `condoc release`, `condoc import`, and `condoc graduate`.
- Linux and Windows builds; binaries from a tag job.

See [the decision that specifies it](https://github.com/phatblat/conventional-docs/blob/main/docs/decisions/2026-09-05-condoc-a-binary-for-the-document-lifecycle.md).
