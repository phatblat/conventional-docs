---
title: Adjacent files
description: The files this convention places but does not define.
---

Every project also carries a set of files that are not part of this
convention's ten artifacts, but that a repository is still expected to have —
`CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, and the rest. This
convention calls that set **adjacent files**: it fixes where each one may
live, and leaves what it says to whatever standard already owns that content.

## Root-only

`README.md`, the license file, and `CITATION.cff` are the ring's root-only
members: GitHub's "Cite this repository" feature reads `CITATION.cff` from
the default branch root only, and the license and `README.md` have the same
single-location expectation. `CHANGELOG.md` and `AGENTS.md` are **not**
adjacent files — they are artifacts this convention itself defines and binds
to the root; see [Artifacts](../artifacts/).

## The rest of the ring

These may live in `.github/`, the repository root, or `docs/` — GitHub
resolves them in that order, first found wins — but only in **one** of those
locations at a time. A copy in a lower-precedence location is a file nobody
reads, which is exactly the duplication the artifact rules already forbid; this
extends that rule to the ring GitHub itself recognizes.

| File                              | Content standard              | Locations                 |
| --------------------------------- | ----------------------------- | ------------------------- |
| `CONTRIBUTING.md`                 | this convention (below)       | `.github/`, root, `docs/` |
| `CODE_OF_CONDUCT.md`              | Contributor Covenant, or none | `.github/`, root, `docs/` |
| `SECURITY.md`                     | none                          | `.github/`, root, `docs/` |
| `SUPPORT.md`                      | none                          | `.github/`, root, `docs/` |
| `CODEOWNERS`                      | GitHub syntax                 | `.github/`, root, `docs/` |
| `.github/FUNDING.yml`             | GitHub schema                 | `.github/` only           |
| issue / PR / discussion templates | GitHub schema                 | `.github/`                |

## No graduation

An adjacent file keeps its host-recognized uppercase name forever. A copy
under `docs/` is a placement choice among the host's recognized locations, not
a graduation — the [graduation](../graduating/) triggers never apply to it,
and there is no lowercase `docs/contributing.md` form to move to. The Charter's
`## Artifacts` section records an adjacent file's location only when it isn't
the repository root.

## `CONTRIBUTING.md` is the one exception

Every other file in the ring defers entirely to its owning standard — this
convention adds no content rules for a code of conduct, a security policy, or
`CODEOWNERS` syntax. `CONTRIBUTING.md` is different: it is the
contributor-facing projection of [the loop](../#the-loop) itself — the
Decision and Plan thresholds, the changelog rule, the lifecycle event
vocabulary — content this convention already owns and no external standard
supplies. A `CONTRIBUTING.md` in a Conventional Docs repository should say how
a change moves through Decision → Plan → commit → changelog, or link the
documents that do.

## Related standards

- **[Contributor Covenant](https://www.contributor-covenant.org/)** — the code
  of conduct text this convention's [standards pack](../doc/#standards-pack)
  vendors.
- **[Citation File Format](https://citation-file-format.github.io/)** — the
  `CITATION.cff` schema.
- **[SPDX](https://spdx.org/licenses/)** / **[REUSE](https://reuse.software/)** —
  license identifiers and, for REUSE, the `LICENSES/<SPDX>.txt` layout.
- **[GitHub: community health files](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions)** —
  the precedence order this page's placement rule follows.
