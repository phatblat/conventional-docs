---
title: About
description: Status, versioning policy, license, and prior art.
---

## Status

The specification is a **draft**. It MAY change without notice until version
1.0.0. From 1.0.0 onward, each released version keeps a permanent URL under
`/spec/`, and the working draft stays at [`/spec/next/`](../spec/next/index.md).
Changes to the specification itself follow [SemVer](https://semver.org/).

## License

MIT © Ben Chatelain. A conforming repository owes no attribution to this
project.

## Propose a change

Open an issue, or a pull request that adds a `decision:` commit to
[this repository](https://github.com/phatblat/conventional-docs) — the
convention governs its own specification.

## Related and complementary efforts

- **[agentlink](https://github.com/fialhosoft/agentlink)** — solves the
  _placement_ half of the agent-files problem: one canonical `AGENTS.md` and
  `.agents/skills`, with each tool's expected path materialized as a native
  read, a link, or an import stub, chosen per tool from a data-only provider
  manifest. Conventional Docs decides _what_ the canonical documents are and
  how they change; agentlink decides how each agent finds them. Its git
  posture — commit only the canonical layout, never commit symlinks —
  matches the no-mirroring rule here, and its provider manifests are close to
  the shape this project's compatibility matrix wants.
- **[Conventional Commits](https://www.conventionalcommits.org/)** and
  **[conventional-changelog](https://github.com/conventional-changelog)** —
  the namesake. The event prefixes in [Events](../events/index.md) are
  ordinary Conventional Commits with custom types.
- **[Keep a Changelog](https://keepachangelog.com/en/2.0.0/)** — the changelog itself,
  and the model for how small a convention should be.
- **[ADRs](https://adr.github.io/)** / **[MADR](https://adr.github.io/madr/)**
  — the Decisions folder is an ADR log with a status lifecycle; a proposed ADR
  is the spec.
- **[AGENTS.md](https://agents.md/)** — the canonical agent instruction file
  this convention builds on.
- **[GitHub spec-kit](https://github.com/github/spec-kit)** and
  **[Kiro](https://kiro.dev/)** — spec-driven development for agents. Same
  spec → plan → tasks chain, but organized per feature and kept in the repo;
  Conventional Docs is project-level state plus a decision log, with the plan
  discarded at merge.
- **[Diátaxis](https://diataxis.fr/)** — how to organize user documentation;
  complementary, not overlapping.
