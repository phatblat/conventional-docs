# Conventional Docs

**What Conventional Commits did for commit messages, Conventional Docs does for
a repository's docs: a predictable shape that tools and agents can rely on.**

> Status: draft. Nothing here is versioned yet. Expect the shape to move.

## The idea

Every project keeps a small set of markdown files in git that describe its
intent, its current state, and how it changes. They are the same files in every
repo, at predictable paths, with predictable lifetimes, so that a human or an
agent can walk into any project cold and know where to look.

Two axes decide everything else:

- **Lifetime** — how long a document stays true. Project, living, append-only,
  until done, branch, session, or per-release.
- **Audience** — who reads it. Outsiders and machines read the repo root;
  maintainers read `docs/`.

Why this convention exists, what it is trying to achieve, and what it is
designed to enable: [CHARTER.md](CHARTER.md).

## Specification

The normative specification lives at
[`site/content/spec/next/index.md`](site/content/spec/next/index.md). It uses
RFC 2119 keywords and is a draft until version 1.0.0.

## The artifacts

| Artifact  | Small repo   | Graduated                           | Lifetime              | Answers                                            |
| --------- | ------------ | ----------------------------------- | --------------------- | -------------------------------------------------- |
| Charter   | `CHARTER.md` | `docs/charter.md`                   | project               | why it exists, goals, route                        |
| Design    | `DESIGN.md`  | `docs/design.md`                    | living                | what the system is and does _now_                  |
| Decisions | —            | `docs/decisions/YYYY-MM-DD-slug.md` | append-only           | what changed, why, what it cost                    |
| Roadmap   | `ROADMAP.md` | `docs/roadmap.md`                   | living                | what's next, in order                              |
| Backlog   | —            | `docs/backlog/<slug>.md`            | until done            | what one queued item is, in full                   |
| Plan      | `PLAN.md`    | —                                   | one branch / worktree | exact steps for the current decision               |
| Events    | `EVENTS.md`  | `docs/events.md`                    | living                | which lifecycle events the repo's commits announce |
| Runbooks  | —            | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                          |
| Incidents | —            | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                        |
| Todo      | `TODO.md`    | —                                   | one branch / worktree | where the work left off                            |

_Events is proposed, not settled: the vocabulary is in use, but `EVENTS.md`
as its home is [still under review](docs/decisions/2026-09-05-give-events-their-own-artifact.md)._

A decision record moves through four states — **draft**, **proposed**,
**accepted**, **rejected** — and each transition is a commit. A _proposed_
decision is the spec. `accept` and `reject` end review and freeze the record's
body: changing your mind is a new decision that supersedes it, and a
correction or a supersession pointer is a dated line in the record's
append-only `## Errata` tail. A rejected record stays in the log; it says
what was considered and why it was refused. The Plan is written from an
accepted decision, committed for backup and handoff, and deleted before
merge. The Todo is the agent's own list, cached in git so a lost session is
recoverable, and deleted the same way.

### Adjacent files

An _adjacent file_ is a document a repository is expected to have that this
convention places but does not define — its content belongs to whatever
standard already owns it. `README.md`, the license, `CHANGELOG.md`, and
`AGENTS.md` are the root-only case: their consumers — registries, GitHub,
coding agents — look only there, and several of them don't go through a
filesystem that could follow a link. `CHANGELOG.md` follows
[Keep a Changelog 2.0.0](https://keepachangelog.com/en/2.0.0/) and answers
both what shipped and what will ship next: it is hand-curated above the
newest released heading, so a reader needs nothing to assemble it.

The rest of the ring may live in `.github/`, the repository root, or `docs/` —
GitHub resolves them in that order, first found wins — but only in one of
those places at a time; a second copy in a lower-precedence location is a file
nobody reads.

| File                              | Content standard              | Locations                 |
| --------------------------------- | ----------------------------- | ------------------------- |
| `CONTRIBUTING.md`                 | this convention (below)       | `.github/`, root, `docs/` |
| `CODE_OF_CONDUCT.md`              | Contributor Covenant, or none | `.github/`, root, `docs/` |
| `SECURITY.md`                     | none                          | `.github/`, root, `docs/` |
| `SUPPORT.md`                      | none                          | `.github/`, root, `docs/` |
| `CODEOWNERS`                      | GitHub syntax                 | `.github/`, root, `docs/` |
| `CITATION.cff`                    | Citation File Format 1.2.0    | root                      |
| `.github/FUNDING.yml`             | GitHub schema                 | `.github/` only           |
| issue / PR / discussion templates | GitHub schema                 | `.github/`                |

An adjacent file never graduates — it keeps its host-recognized name, and a
copy under `docs/` is a placement choice, not a graduation. The Charter's
`## Artifacts` section records its location when it isn't the root.

`CONTRIBUTING.md` is the one file in this ring whose content the convention
does specify: it is the contributor-facing projection of the loop below — the
Decision and Plan thresholds, the changelog rule, the event vocabulary.
Everything else in the table is deferred entirely to its owning standard.

### Graduating to `docs/`

Small repos keep everything as `UPPERCASE.md` at the root, except the per-file
logs — decisions, runbooks, incidents — which live under `docs/` from their
first entry. Move a document to `docs/` when either trigger fires:

1. the root is getting cluttered with top-level files and folders
   (dotfiles don't count — that's where config conventions live), or
2. the document has outgrown a single file: it needs siblings, status, or
   structure (a `ROADMAP.md` that needs per-item status becomes
   `docs/roadmap.md`; a `ROADMAP.md` whose items need real descriptions or
   conflict between concurrent branches decomposes into `docs/backlog/` plus
   a Roadmap that indexes it — see [The backlog](#the-backlog)).

`PLAN.md` and `TODO.md` never move: neither trigger can fire for a document
that lives on one branch and is deleted rather than grown. There is no
`docs/plan.md` and no `docs/todo.md`.

Graduate in one commit, rewrite inbound links in the same commit, and let a
link check in CI catch the rest. No stub files at the old path, and no mirrors
in either direction — the Charter's `## Artifacts` table records where each
document lives. The one exception is renaming a numbered decision log to
dated ids, which leaves a redirect at each old filename; the
`conventional-docs` skill has the procedure.

`docs/decisions/` is the canonical decisions location, and MADR already
defaults to that path. A root `.adr-dir` file containing `docs/decisions`
points tools that only need the location — `adr list`, `adr generate` — at
it. `adr new` allocates the next sequential number, so records are copied
from the skeleton instead.

### The backlog

A Backlog is one file per work item, `docs/backlog/<slug>.md`, with no
small-repo form: a repository either has no Backlog or has the directory.
Its id is a kebab-case slug of the title, never a number or a date — an
allocated id races the same way two branches filing the same week would
race on a decision number, which is why decisions moved off numbers too.

Every item uses exactly this skeleton:

```markdown
# <Title>

## Problem

<What is wrong or missing, and who it costs. For a bug: what happens, what
should happen, and how to reproduce it.>

## Outcome

<What is observably true when this is done.>

## Notes

<Optional, last: evidence, links, a related decision, a suggested direction
the decision record is free to overrule.>
```

No status, priority, assignment, claim, or date field: presence in
`docs/backlog/` is the open state, order lives in the Roadmap, and dates are
`git log`'s. Once a repository has a Backlog, the Roadmap stops describing
work and starts indexing it — an ordered list of links, never a restated
body. A decision record must never link to a Backlog item, because the item
is deleted at implementation and the link would go stale; a Backlog item may
link to a decision.

An item is deleted by the change that implements it, together with its
Roadmap line, in the same pull request — never by a separate bookkeeping
commit. An item that will not be implemented is deleted rather than marked
closed.

A Backlog item never records who is working on it. A branch that implements
one should end with the item's slug, so any prefix a team already uses
(`feat/`, `fix/`, an author or agent name) survives and concurrent work is
found by a suffix match over the refs a clone already fetches:

```bash
git ls-remote --heads origin '*<slug>'   # anyone, anywhere
git branch --all --list '*<slug>'        # what this clone already knows
```

That is detection, not arbitration: two workers who both start get two
branches, and the second to open a pull request loses the work, not the
repo. An unpushed branch is an invisible claim.

### The changelog

Public-facing release notes are written incrementally, alongside the change
itself, not reconstructed from commit messages after the fact. `CHANGELOG.md`
keeps a `## [Unreleased]` section at the top, and a notable user-facing change
adds its line there in the same commit or PR as the change:

```markdown
## [Unreleased]

### Added

- Support for custom output formats.
```

Its shape is [Keep a Changelog 2.0.0](https://keepachangelog.com/en/2.0.0/): a
`# Changelog` heading with the fixed preamble, released versions as
`## [x.y.z] - YYYY-MM-DD` newest first, the six categories as `###`
subsections, `**Breaking:**` markers inside the type they belong to, and
reference-style links at the bottom resolving each version to a compare diff,
with `[Unreleased]` comparing the latest tag to `HEAD`.

Notable is a judgment, not a gate: no check requires a changelog edit on a
change, and nothing validates that one is present. A conflict in
`[Unreleased]` between two branches is resolved by keeping both lines — order
within a category carries no meaning.

The `release:` event renames `[Unreleased]` to the new version in both the
heading and its reference link, adds a fresh empty `[Unreleased]` pointing at
`HEAD`, and leaves the released section untouched thereafter.

A line may cite the decision id it came from — a path in the repo rather than
one host's number, which is the portable form Keep a Changelog recommends
over a bare `(#1234)`.

### The todo cache

`TODO.md` is the working list the agent is already keeping, written to a file
and committed. It is a cache, not a source: the live list is in the session,
and the file carries the session id and the time it was taken so a reader can
tell it from a session that is gone. A committed list is a rollback point; a
pushed one survives the machine. The agent writes it whenever it is tracking a
list at all, commits it with `todo: sync` at each checkpoint, and deletes it
with `todo: clear` before merge.

A `plan:` or `todo:` commit touches only its own artifact. That is what makes
the bookkeeping disposable: `plan: start` adds `PLAN.md` and `plan: done`
deletes it, `todo: sync` writes `TODO.md` and `todo: clear` deletes it, so
dropping every one of those commits from a branch leaves the tree exactly as
it was, and a squash merge erases them for free. A commit that mixes one of
these files with real work cannot be dropped, so it is not one of these
events.

`TODO.md` is not a backlog. Where a repo already keeps one under that name as
a single durable file, it is a Roadmap and is renamed to `ROADMAP.md` when
the convention is adopted; where it is a directory of per-item files, it is a
Backlog and moves to `docs/backlog/`. Either way it is told apart from a Todo
by lifetime: a Roadmap or a Backlog item outlives every branch, and a Todo
does not outlive the one it is on.

## The loop

```text
intent → Decision (proposed) → review → Decision (accepted)
                                              ↓
   Design updated ← PR merged ← execute ← Plan written
                                Todo cached
   Plan and Todo deleted
```

Thresholds: a PR over ~100 lines, or one that changes behavior, an interface,
or a dependency, needs a Decision. Work spanning more than one session, or
handed to another agent, needs a Plan. Anything smaller just happens.

Each transition is announced by a commit; the vocabulary is in
[EVENTS.md](EVENTS.md).

## Agent instruction files

`AGENTS.md` is the canonical, tool-agnostic instruction file. Tools that read a
different filename get a real file at the root that _includes_ it rather than a
copy or a symlink. For Claude Code:

```markdown
[@AGENTS.md](AGENTS.md)

## Claude Code

<!-- anything Claude-specific -->
```

The `@` in the link text is what Claude Code imports; the link is what humans
click on GitHub. The same idiom pulls `docs/` files into an agent's context on
every session, while a plain link without `@` leaves them to be read on demand.

Agent tooling churns weekly, so this layer is kept out of the core convention
and tracked in a compatibility matrix: per tool, the claim, the version tested,
whether the behavior is documented or merely observed, and when it was last
verified. Rows older than ~90 days render as unverified.

## What this is not

- **Not user documentation.** Tutorials, how-tos, and reference docs are
  [Diátaxis](https://diataxis.fr/)'s territory. Where they overlap, Design is
  explanation/reference and runbooks are how-to.
- **Not a tool.** Like Keep a Changelog, this convention should cost only
  attention. CI checks, the agent skill, and the `doc` binary are optional:
  a repo maintained entirely by hand stays conformant.

## Reference implementation

The `conventional-docs` agent skill (`skills/conventional-docs/SKILL.md`)
teaches coding agents to read, write, and graduate these artifacts, and to
pick up a branch cold from its Plan. It is a plain
[Agent Skills](https://agentskills.io/specification) directory, so it installs
into any agent that reads `SKILL.md`:

```bash
npx skills add phatblat/conventional-docs      # this project; -g for all projects
```

Claude Code installs it as a plugin instead:

```text
/plugin marketplace add phatblat/conventional-docs
/plugin install conventional-docs@conventional-docs
```

Or copy `skills/conventional-docs/` into the agent's skills directory by hand.

## Related and complementary efforts

- **[agentlink](https://github.com/fialhosoft/agentlink)** — solves the
  _placement_ half of the agent-files problem: one canonical `AGENTS.md` and
  `.agents/skills`, with each tool's expected path materialized as a native
  read, a link, or an import stub, chosen per tool from a data-only provider
  manifest. Conventional Docs decides _what_ the canonical documents are and how
  they change; agentlink decides how each agent finds them. Its git posture —
  commit only the canonical layout, never commit symlinks — matches the
  no-mirroring rule here, and its provider manifests are close to the shape this
  project's compatibility matrix wants.
- **[Conventional Commits](https://www.conventionalcommits.org/)** and
  **[conventional-changelog](https://github.com/conventional-changelog)** — the
  namesake. The event prefixes above are ordinary Conventional Commits with
  custom types.
- **[Keep a Changelog](https://keepachangelog.com/en/2.0.0/)** — the changelog itself,
  and the model for how small a convention should be.
- **[ADRs](https://adr.github.io/)** / **[MADR](https://adr.github.io/madr/)** —
  the Decisions folder is an ADR log with a status lifecycle; a proposed ADR is
  the spec.
- **[AGENTS.md](https://agents.md/)** — the canonical agent instruction file
  this convention builds on.
- **[GitHub spec-kit](https://github.com/github/spec-kit)** and
  **[Kiro](https://kiro.dev/)** — spec-driven development for agents. Same
  spec → plan → tasks chain, but organized per feature and kept in the repo;
  Conventional Docs is project-level state plus a decision log, with the plan
  discarded at merge.
- **[Diátaxis](https://diataxis.fr/)** — how to organize user documentation;
  complementary, not overlapping.
- **[Contributor Covenant](https://www.contributor-covenant.org/)**,
  **[Citation File Format](https://citation-file-format.github.io/)**,
  **[SPDX](https://spdx.org/licenses/)** / **[REUSE](https://reuse.software/)**,
  and **[GitHub's community health files](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions)** —
  the standards that own the content of the adjacent-files ring. Conventional
  Docs places these; it does not restate what they say.

## Development

```bash
just deps       # install pinned tools and dependencies
just check      # formatting, markdown lint, link check
just site-dev   # preview the website at http://localhost:1313/conventional-docs/
```

`just --list` shows every recipe.

## License

MIT © Ben Chatelain
