# Repository Guidelines

## Toolchain

`mise.toml` pins every tool; `just` is the only command surface. Prefer adding a
recipe over documenting a raw command, and run tools through the recipes so the
pinned versions are the ones that execute.

## Commands

- `just deps` — install pinned tools and project dependencies
- `just check` — the full gate: formatting, markdown lint, skill frontmatter, link check
- `just test` — link check only
- `just outdated` / `just upgrade` — report, then apply, tool and dependency updates

## Conventions

- Formatting is owned by the formatter. Run `just format`; never hand-format.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/),
  plus this project's own `decision:`/`plan:`/`todo:`/`release:`/`deploy:` event
  types (`EVENTS.md`, enforced by `commitlint.config.js`). `.husky/commit-msg`
  runs commitlint on every commit.
- Releases are automated by `semantic-release` on push to `main`; never
  hand-edit a released section of `CHANGELOG.md` or bump `package.json`'s
  version. A notable user-facing change adds its line to `CHANGELOG.md`'s
  `## [Unreleased]` section in the same commit, and the release plugin
  (`scripts/semantic-release-unreleased.mjs`) promotes that section to the new
  version.
- `package.json`'s `scripts.prepare` is the one exception to "no scripts": it is
  never run by hand, only by the package manager on install, to wire up Husky's
  git hooks.
- The operational rules for this project's own convention live in
  `skills/conventional-docs/SKILL.md`. Read it before creating or editing any
  Charter, Design, Decisions, Roadmap, Plan, Todo, or `CHANGELOG.md` artifact.
  `README.md` is the human-facing spec and rationale; the skill is the
  agent-facing procedure, so a change to the convention updates both in the
  same commit.
