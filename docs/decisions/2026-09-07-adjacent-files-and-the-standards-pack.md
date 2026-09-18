# Adjacent files and the standards pack

## Issue

This decision extends
[2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md)
with new commands and a standards pack; nothing from that decision is
restated here.

The convention defines nine lifecycle artifacts and, in passing, four root
files that never graduate: `README.md`, the license, `CHANGELOG.md`,
`AGENTS.md`. Every other file a repository is expected to carry —
`CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `SUPPORT.md`,
`CODEOWNERS`, `CITATION.cff`, issue and pull request templates — is
unaddressed. The spec has no clause for them, the routing guide has no row for
them, and `doc` cannot see them. A repository can silently carry two copies of
`CONTRIBUTING.md` — one under `.github/`, one at the root — with no rule
forbidding it, while spec clauses 2 and 3 already forbid exactly that
duplication for the nine artifacts. A contributor to a Conventional-Docs
repository also has nowhere to learn the decision → plan → todo loop: it is
written for adopters and for agents, never for a human arriving through
`CONTRIBUTING.md`.

## Status

This is a proposal that is **accepted**.

## Assumptions and Constraints

- GitHub resolves community health files in a fixed precedence order —
  `.github/`, repository root, `docs/` — first found wins, and a public
  `.github` repository at the organization level supplies defaults for any
  repository lacking its own copy, except the license, which GitHub excludes
  from that inheritance because it must travel with a clone, package, or
  download.
- The Contributor Covenant (version 3.0, released 2025-07-28) has one
  placeholder, the literal sentence
  `**[NOTE: describe your means of reporting here.]**` in its reporting
  section. SPDX license texts carry their own placeholder tokens: MIT and ISC
  use `<year>` and `<copyright holders>`; BSD-2-Clause and BSD-3-Clause use
  `<year>` and `<owner>`. Apache-2.0, GPL-3.0-or-later, MPL-2.0, and Unlicense
  carry no copyright-line placeholder of this project's concern — the bracketed
  tokens they do carry sit inside an appendix of instructions for applying the
  license, not in a line this project would fill in.
- [2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md)
  is accepted and binds this one: clause 4 (no configuration file, the
  filesystem is authoritative), clause 6 (every mutating command commits with
  an explicit pathspec, subject `docs: add <name>`), clause 9 (exit codes 0/1/2),
  clause 11 (`--json` for hook consumers), and clause 14 (the tool emits
  whatever the convention specifies, never a variant of its own).
- Generated markdown must pass `just check` unmodified: `prettier` with
  `proseWrap: preserve`, and `markdownlint-cli2` with MD013 and MD041 disabled.
  Vendored third-party legal and conduct text must not be reflowed by either
  gate.
- No document in this repository cites the spec's clause numbers by number
  except a decision record citing its own; renumbering the spec's tail is safe.

## Argument

Name the ring and fix only its placement, deferring its content to whatever
standard already owns it (**Chosen**). The alternative of an umbrella profile
that also mandates content for the whole ring (**Rejected**) would put every
upstream standard's versioning on this project's release schedule, when the
census shows they move on unrelated timescales — the Covenant is on major
version 3, SPDX license texts are frozen forever, CNCF's set only applies to
CNCF-hosted projects. Placement-only keeps the convention's job to the one
thing every one of these files actually needs from a document-layout
convention: a fixed, single location.

`CONTRIBUTING.md` is the one exception, authored by this convention rather than
deferred (**Chosen**), because its content — the Decision and Plan thresholds,
the changelog rule, the event vocabulary — is this convention's own subject
matter and no external standard supplies it. Deferring it too (**Rejected**)
would leave the gap in the Issue unclosed: a contributor still has nowhere to
learn the loop.

Third-party bodies ship as data in a standards pack, not as code
(**Chosen**). A pack entry is a file at `<pack>/<kind>/<id>.txt`; the id
carries the standard's own version, so a repository can hold
`contributor-covenant-3.0` indefinitely while another repository's pack
already carries a later id — independent versioning without either repository
waiting on a `doc` release. Fetching text at run time (**Rejected**) breaks
the convention's offline posture, stated throughout the README and the `doc`
decision. An executable plugin protocol (**Rejected**) would spend the
hot-start budget the `doc` decision's second constraint protects, and would
run arbitrary code out of a data directory for no capability a text file
lacks.

## Architectural Decision

1. **Term.** An _adjacent file_ is a document a repository is expected to
   have that this convention places but does not define. The nine artifacts
   are unaffected; no tenth artifact is created.
2. **Registry.** The convention names the ring and, per file, its owning
   standard and its recognized locations:

   | File                          | Content standard                  | Recognized locations      |
   | ----------------------------- | --------------------------------- | ------------------------- |
   | `README.md`                   | none (root-only, never graduates) | root                      |
   | license file                  | SPDX identifiers                  | root                      |
   | `CODE_OF_CONDUCT.md`          | Contributor Covenant, or none     | `.github/`, root, `docs/` |
   | `CONTRIBUTING.md`             | this convention (clause 7)        | `.github/`, root, `docs/` |
   | `SECURITY.md`                 | none                              | `.github/`, root, `docs/` |
   | `SUPPORT.md`                  | none                              | `.github/`, root, `docs/` |
   | `CODEOWNERS`                  | GitHub syntax                     | `.github/`, root, `docs/` |
   | `CITATION.cff`                | Citation File Format 1.2.0        | root                      |
   | `.github/FUNDING.yml`         | GitHub schema                     | `.github/` only           |
   | issue/PR/discussion templates | GitHub schema                     | `.github/`                |

3. **One location.** An adjacent file exists in at most one of its host's
   recognized locations. GitHub's precedence order means a duplicate in a
   lower-precedence location is a file nobody reads.
4. **No graduation.** An adjacent file keeps its host-recognized uppercase name
   and never becomes a lowercase `docs/` form. `docs/` is one of the host's
   recognized locations, so a file there is a placement choice, not a
   graduation, and the artifact graduation triggers do not apply to it.
5. **The license is in-repo.** It is committed to the repository itself and
   never inherited from an organization-level `.github` default, because it
   must travel with a clone, package, or download.
6. **The Charter records non-root placement.** The `## Artifacts` section MUST
   record an adjacent file's location when it is not the repository root, and
   MAY record it otherwise. This keeps the table small and records exactly the
   placements a reader cannot guess.
7. **`CONTRIBUTING.md` is the exception the convention authors.** It is the
   contributor-facing projection of the loop — the Decision and Plan
   thresholds, the changelog rule, and the lifecycle event vocabulary — content
   this convention already owns, that no external standard supplies.
8. **Everything else defers.** No content clause is added for `README.md`, the
   license, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `SUPPORT.md`, `CODEOWNERS`,
   `CITATION.cff`, funding, or issue/PR/discussion templates. Their owning
   standard is named; their content is not restated or constrained.
9. **`doc` writes them.** `doc readme|contributing|security|support new`
   write convention-owned templates; `doc license new --spdx <id>` and
   `doc code-of-conduct new --contact <method>` write bodies from a standards
   pack. Each commits `docs: add <name>` — `docs: add license MIT`,
   `docs: add code-of-conduct contributor-covenant-3.0` — matching the
   existing `docs: add runbook <slug>` shape.
10. **Standards pack.** Third-party bodies are data, not code:
    `<pack>/licenses/<SPDX-ID>.txt` and
    `<pack>/codes-of-conduct/<standard>-<version>.txt`. The id carries the
    version, so a new standard version is a new file — independently
    versioned from the binary: a repository may hold
    `contributor-covenant-3.0` while another holds a later one, and neither
    waits on a `doc` release.
11. **Resolution order.** `--pack <dir>`, then
    `$XDG_DATA_HOME/conventional-docs/pack` (default
    `~/.local/share/conventional-docs/pack`), then the pack embedded in the
    binary. First hit per `(kind, id)` wins, so a user pack shadows an
    embedded body without replacing the binary.
12. **No manifest, so no configuration file.** A pack's layout is its
    metadata, and each body declares its own placeholders as `{{name}}`
    tokens. The tool substitutes the set fixed for the kind — license: `year`,
    `holder`; code of conduct: `contact` — and fails when a `{{…}}` token
    survives. This keeps clause 4 of the extended decision intact: no config
    file, the filesystem is authoritative.
13. **Embedded contents.** Apache-2.0, BSD-2-Clause, BSD-3-Clause,
    GPL-3.0-or-later, ISC, MIT, MPL-2.0, Unlicense, and
    contributor-covenant-3.0. Bodies are verbatim from the canonical source
    recorded in `pack/SOURCES.md`; the only permitted edit is normalizing a
    single copyright line's tokens to `{{year}}`/`{{holder}}`, and the
    Covenant's reporting placeholder to `{{contact}}`. An unknown id is an
    error naming what the resolved pack carries and where to add a body.
14. **`doc pack list [--json]`** reports every `(kind, id, source)` the
    resolved pack can serve, so a caller can see which standard versions are
    available and where each came from.
15. **`doc init --semver <x.y.z>`** (default `2.0.0`) sets the changelog
    preamble's Semantic Versioning link, so a repository on a different SemVer
    version is not forced to hand-edit a generated file.
16. **Validation is warnings, and lands later.** Presence, duplication, and
    outdated-standard-version findings for adjacent files land with the
    validation slice (`doc lint|fix|check`) and are warnings, never errors: a
    repository with no code of conduct is not nonconformant.
17. **Not written by the tool yet.** `CITATION.cff`, `AGENTS.md`,
    `CODEOWNERS`, funding and template files are in the registry — the
    placement rules bind them — but `doc` does not write them; they need input
    shapes this decision does not fix. Roadmap.
18. **Executable plugins are rejected.** A pack supplies text. A
    process-spawning plugin protocol would spend the hot-start budget the
    `doc` decision's second constraint protects, and would run arbitrary code
    out of a data directory, for no capability a data file lacks.

## Consequences

The binary grows by roughly 90 KB of embedded text. `pack/SOURCES.md` becomes
a maintenance surface: an upstream retext is a new pack file with a new id
rather than an edit to an existing one. The convention now names files it does
not define, so the registry in clause 2 must be revisited when a host changes
its recognized locations. `doc pack list --json` is the binary's first `--json`
output and sets the shape the `doc` decision's clause 11 promises for the read
commands that follow.

## Positions

- **Umbrella profile mandating content for the whole ring.** _Rejected._ Puts
  every upstream standard's versioning on this project's release bill; the
  census shows they move on unrelated timescales.
- **No vendored text at all — placement rules only.** _Rejected._ Legal and
  conduct text is exactly what a new repository lacks; a pointer to an
  external URL cannot stand in for a committed file, and the license
  specifically must be in-repo (clause 5).
- **TOML-manifested packs.** _Rejected._ A parser dependency and a
  configuration surface for four fields the directory layout already encodes.
- **Fetching texts at run time.** _Rejected._ The convention's whole posture
  is that a clone plus the tool works offline.
- **Executable pack plugins.** _Rejected._ See Argument; a data file has every
  capability a plugin would add, without the hot-start cost or the arbitrary
  code execution.

## References

- [2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md)
  — the decision this extends with new commands and clause 4's config-file
  constraint.
- [Contributor Covenant](https://www.contributor-covenant.org/) — the vendored
  code-of-conduct standard.
- [SPDX License List](https://spdx.org/licenses/) — the vendored license
  identifiers and canonical texts.
- [GitHub: Adding a code of conduct to your project](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions) —
  the community-health-file precedence order this decision relies on.
