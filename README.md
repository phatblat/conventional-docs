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
  branch, session, or per-release.
- **Audience** — who reads it. Outsiders and machines read the repo root;
  maintainers read `docs/`.

Why this convention exists, what it is trying to achieve, and what it is
designed to enable: [CHARTER.md](CHARTER.md).

## The artifacts

| Artifact  | Small repo           | Graduated                           | Lifetime              | Answers                                               |
| --------- | -------------------- | ----------------------------------- | --------------------- | ----------------------------------------------------- |
| Charter   | `CHARTER.md`         | `docs/charter.md`                   | project               | why it exists, goals, route                           |
| Design    | `DESIGN.md`          | `docs/design.md`                    | living                | what the system is and does _now_                     |
| Decisions | —                    | `docs/decisions/YYYY-MM-DD-slug.md` | append-only           | what changed, why, what it cost                       |
| Roadmap   | `ROADMAP.md`         | `docs/roadmap.md`                   | living                | what's next, in order                                 |
| Plan      | `PLAN.md`            | `docs/plan.md`                      | one branch / worktree | exact steps for the current decision                  |
| Changes   | `.changes/<slug>.md` | `.changes/<slug>.md`                | per-release           | what will ship in the next release, in plain language |
| Runbooks  | —                    | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                             |
| Incidents | —                    | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                           |
| Todo      | agent memory         | `docs/todo.md` (opt-in)             | one session           | where this session is                                 |

A _proposed_ decision is the spec. Once accepted it is frozen; changing your
mind is a new decision that supersedes it. The Plan is written from an accepted
decision, committed for backup and handoff, and deleted before merge.

### Files that never graduate

`README.md`, `LICENSE`, `CHANGELOG.md`, and `AGENTS.md` stay at the root
permanently. Their consumers — registries, GitHub, coding agents — look only
there, and several of them don't go through a filesystem that could follow a
link. `CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/);
its entries are assembled from `.changes/` fragments at release time, never
hand-written directly.

### Graduating to `docs/`

Small repos keep everything as `UPPERCASE.md` at the root, except the per-file
logs — decisions, runbooks, incidents — which live under `docs/` from their
first entry. Move a document to `docs/` when either trigger fires:

1. the root is getting cluttered with top-level files and folders
   (dotfiles don't count — that's where config conventions live), or
2. the document has outgrown a single file: it needs siblings, status, or
   structure (a `ROADMAP.md` that needs per-item status becomes
   `docs/roadmap.md`).

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

### Release notes

Public-facing release notes are written incrementally, alongside the change
itself, not reconstructed from commit messages after the fact. Each
user-facing change adds a fragment file under `.changes/<slug>.md` in the same
commit or PR. A fragment is one or more lines; each line is a markdown
unordered list item starting with one of the six
[Keep a Changelog](https://keepachangelog.com/) categories:

```markdown
- Added: support for custom output formats.
- Fixed: a race condition when releasing concurrently.
```

At release time, every fragment file is concatenated, grouped by category,
and folded into that release's notes and the `CHANGELOG.md` entry. The
consumed fragment files are deleted in the same commit as the version bump —
`.changes/` holds only what hasn't shipped yet.

## The loop

```text
intent → Decision (proposed) → review → Decision (accepted)
                                              ↓
   Design updated ← PR merged ← execute ← Plan written
   Decision → implemented          (Todo)
   Plan deleted
```

Thresholds: a PR over ~100 lines, or one that changes behavior, an interface,
or a dependency, needs a Decision. Work spanning more than one session, or
handed to another agent, needs a Plan. Anything smaller just happens.

## Events

Lifecycle transitions are commits with Conventional Commits types, so hooks,
dashboards, and chat notifications can key off `git log` without parsing files:

```text
decision: propose 2026-02-11-split-the-scheduler
decision: accept 2026-02-11-split-the-scheduler
decision: implement 2026-02-11-split-the-scheduler (#88)
plan: start 2026-02-11-split-the-scheduler
plan: done 2026-02-11-split-the-scheduler
release: v1.2.0
deploy: prod v1.2.0
```

Notifications are doorbells: they say where to look, never what to do. The
commit is the event.

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
  attention. CI checks and the agent skill are optional.

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
- **[Keep a Changelog](https://keepachangelog.com/)** — the changelog itself,
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

## Development

```bash
just deps    # install pinned tools and dependencies
just check   # formatting, markdown lint, link check
```

`just --list` shows every recipe.

## License

MIT © Ben Chatelain
