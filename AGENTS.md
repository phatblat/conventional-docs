# Repository Guidelines

## Toolchain

`mise.toml` pins every tool; `just` is the only command surface. Prefer adding a
recipe over documenting a raw command, and run tools through the recipes so the
pinned versions are the ones that execute.

## Commands

- `just deps` — install pinned tools and project dependencies
- `just check` — the full gate: formatting, markdown lint, fragment format, link check
- `just test` — link check only
- `just outdated` / `just upgrade` — report, then apply, tool and dependency updates

## Conventions

- Formatting is owned by the formatter. Run `just format`; never hand-format.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/),
  plus this project's own `decision:`/`plan:`/`release:`/`deploy:` event types
  (see `commitlint.config.js`). `.husky/commit-msg` enforces this on every commit.
- Releases are automated by `semantic-release` on push to `main`; never hand-edit
  `CHANGELOG.md` or bump `package.json`'s version. Every user-facing change adds
  a `.changes/<slug>.md` fragment in the same commit (see `.changes/README.md`);
  `just lint-changes` validates the format and the release plugin
  (`scripts/semantic-release-changes.mjs`) consumes and deletes fragments.
- `package.json`'s `scripts.prepare` is the one exception to "no scripts": it is
  never run by hand, only by the package manager on install, to wire up Husky's
  git hooks.
