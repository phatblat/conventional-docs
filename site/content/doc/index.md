---
title: doc
description: The optional binary that writes the artifacts and their lifecycle commits.
---

_`doc` is **experimental**. v1 is macOS arm64 only, no binaries are
published yet, and it is optional — a repo maintained entirely by hand stays
conformant._

Build and run from a clone (Rust 1.98+):

```bash
cargo install --path crates/conventional-docs   # or: cargo run -p conventional-docs -- <args>
doc --version
```

## Commands

| Command                                                                          | Writes                                                  | Commit subject                         |
| -------------------------------------------------------------------------------- | ------------------------------------------------------- | -------------------------------------- |
| `doc init [--add] [--semver <x.y.z>]`                                            | `CHARTER.md`, `DESIGN.md`, `ROADMAP.md`, `CHANGELOG.md` | `docs: add <names>`                    |
| `doc charter new` \| `doc design new` \| `doc roadmap new`                       | that artifact                                           | `docs: add charter` (etc.)             |
| `doc runbook new <trigger>`                                                      | `docs/runbooks/<slug>.md`                               | `docs: add runbook <slug>`             |
| `doc incident new <slug>`                                                        | `docs/incidents/<today>-<slug>.md`                      | `docs: add incident <id>`              |
| `doc decision draft <title>`                                                     | a record in the **draft** state                         | `decision: draft <id>`                 |
| `doc decision propose <title>` \| `<id>`                                         | a new **proposed** record, or promotes a draft          | `decision: propose <id>`               |
| `doc decision accept <id>`                                                       | freezes the record as **accepted**                      | `decision: accept <id>`                |
| `doc decision reject <id>`                                                       | freezes the record as **rejected**                      | `decision: reject <id>`                |
| `doc decision errata <id> <text>`                                                | a dated line in `## Errata`                             | `docs: errata <id>`                    |
| `doc readme new` \| `doc security new` \| `doc support new`                      | that adjacent file                                      | `docs: add <name>`                     |
| `doc contributing new`                                                           | `CONTRIBUTING.md`                                       | `docs: add contributing`               |
| `doc license new --spdx <id> [--holder] [--year] [--pack]`                       | `LICENSE.md`, from the standards pack                   | `docs: add license <id>`               |
| `doc code-of-conduct new --contact <method> [--standard] [--pack]` (alias `coc`) | `CODE_OF_CONDUCT.md`, from the standards pack           | `docs: add code-of-conduct <standard>` |
| `doc pack list [--json] [--pack <dir>]`                                          | nothing — reports the resolved pack's entries           | —                                      |

- A command scoped to one artifact is `doc <artifact> <verb>`; a command that
  acts on the whole docset is a bare verb, like `doc init`.
- `dec` aliases `decision`, and is the surface's only alias besides `coc` for
  `code-of-conduct`.
- `--no-commit` is global: it writes the files and leaves the tree dirty.
- `decision propose` takes `--extends <id>` and `--supersedes <id>`, and writes
  both files in one commit — the new record's Issue lead sentence and the
  reciprocal link on the record it points at. `--supersedes` requires a
  frozen target, appending an erratum; naming a draft or proposed target
  refuses, since an unfrozen record is edited in place instead.
- `decision accept` and `decision reject` require the record to be `proposed`.
- `decision errata` refuses a draft or proposed record; it is the only write v1
  makes to a frozen record.
- `new` is a verb only on the artifacts and adjacent files that have no
  lifecycle event — Charter, Design, Roadmap, runbooks, incidents, and the
  four templated adjacent files. Decisions have their own verbs, so the tool
  cannot write a record whose event was never announced. `init` never writes
  `EVENTS.md` (still proposed), `PLAN.md`, or `TODO.md` (they belong to a
  branch), and re-running it reports what is missing and creates nothing
  without `--add`.
- An artifact group invoked with no verb prints that group's help and exits 2.
- `init` reports whether a commitlint config exists and whether its
  `type-enum` covers `decision`, `deploy`, `plan`, `release`, `todo`. It
  never edits that config. `--semver` (default `2.0.0`) sets the changelog
  preamble's Semantic Versioning link.

## Standards pack

`license new` and `code-of-conduct new` write third-party bodies from a
**standards pack**: license texts and codes of conduct kept as plain data,
versioned independently of the binary. An entry is
`<pack>/licenses/<SPDX-ID>.txt` or
`<pack>/codes-of-conduct/<standard>-<version>.txt` — the id carries the
version, so a repository can hold `contributor-covenant-3.0` indefinitely
while another repository's pack already carries a later id, and neither waits
on a `doc` release.

Resolution order: `--pack <dir>`, then
`$XDG_DATA_HOME/conventional-docs/pack` (default
`~/.local/share/conventional-docs/pack`), then the pack embedded in the
binary. First hit per `(kind, id)` wins, so a user pack shadows an embedded
body without replacing the binary. `doc pack list --json` reports every entry
the resolved pack can serve and which of those three sources it came from.

A body declares its own placeholders as `{{name}}` tokens — `{{year}}` and
`{{holder}}` for a license, `{{contact}}` for a code of conduct — and the
command fails if a token survives substitution. There is no pack manifest: the
directory layout is the metadata, matching [what `doc` will not do](#what-it-will-not-do)
below.

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

- Bookkeeping: `doc status`, `doc todo sync|clear`, `doc plan start|done`.
- Validation: `doc lint|fix|check`, including the drift check that diffs a
  frozen record against its freeze commit, and adjacent-file findings — a
  missing README or license, a duplicate copy in two recognized locations, an
  outdated standard version, a non-root adjacent file absent from the Charter
  — reported as warnings, never errors.
- `doc citation new` and `doc agents new`, once their input shapes are fixed.
- `doc release`, `doc decision import`, and `doc <artifact> graduate`.
- Linux and Windows builds; binaries from a tag job.

Three decisions in
[`docs/decisions/`](https://github.com/phatblat/conventional-docs/tree/main/docs/decisions)
specify it: `2026-09-05-condoc-a-binary-for-the-document-lifecycle` for the
binary itself, `2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact`
for its name and command shape, and
`2026-09-07-adjacent-files-and-the-standards-pack` for the adjacent-file
commands and the standards pack.
