# Contributing

This project's documentation follows
[Conventional Docs](https://github.com/phatblat/conventional-docs).

## How a change flows

1. **Decide.** A change that alters behavior, a published interface, or a
   dependency gets a decision record at `docs/decisions/YYYY-MM-DD-slug.md`
   before it merges. A **proposed** record is the spec; **accept**ing it
   freezes the record's body.
2. **Plan.** Work spanning more than one session, or handed off to someone
   else, gets a `PLAN.md` on the branch — the ordered steps to carry out the
   accepted decision — deleted before merge.
3. **Commit.** Commits follow
   [Conventional Commits](https://www.conventionalcommits.org/). Lifecycle
   transitions are announced as `decision:`, `plan:`, `todo:`, `release:`,
   and `deploy:` commits.
4. **Announce.** A notable user-facing change adds its line to
   `CHANGELOG.md`'s `## [Unreleased]` section in the same pull request as the
   change.

## Where documents live

See [`CHARTER.md`'s `## Artifacts` section](CHARTER.md#artifacts); every
document this repository keeps currently lives at its repository-root path.

`mise.toml` pins every tool this repository uses, and `just` is the only
command surface — prefer adding a recipe over documenting a raw command.
Run `just check` before opening a pull request: it formats, lints markdown
and Rust, checks skill frontmatter, and runs the Rust test suite and the
site's link check. Formatting is owned by the formatter (`just format`);
never hand-format. `README.md` and
`skills/conventional-docs/SKILL.md` describe the same convention from two
audiences and move together in the same commit whenever one changes.
