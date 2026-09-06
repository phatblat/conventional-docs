# Roadmap

What's next, in order. This is a living document — check items off in place rather
than filing a separate issue for each one, and delete anything that no longer
applies.

## Setup

- [ ] Set the `NPM_TOKEN` repository secret so the `release` CI job can publish
      to npm (`gh secret set NPM_TOKEN --repo phatblat/conventional-docs`).

## Convention

- [ ] Decide whether `docs:`-only commits should keep triggering a patch
      release indefinitely, or whether that rule needs narrowing once the
      convention stabilizes.
- [ ] State an explicit release rule in `.releaserc.json` for every custom
      event type (`decision`, `deploy`, `plan`, `release`, `todo`) instead of
      relying on the commit analyzer ignoring unknown types.

## condoc

- [ ] Build the bookkeeping slice — `condoc status`, `condoc todo sync|clear`,
      and `condoc plan start|done` — so `TODO.md` and `PLAN.md` are written,
      synced, and cleared mechanically rather than by hand.
- [ ] Build the validation slice — `condoc lint|fix|check` — including the
      drift check that diffs a frozen record against its freeze commit.
- [ ] Build `condoc release`, then `condoc import` and `condoc graduate`.
- [ ] Build condoc for linux amd64/arm64 and windows; v1 is macOS arm64 only.
- [ ] Ship condoc binaries from a tag job, alongside the npm package
      semantic-release publishes.
