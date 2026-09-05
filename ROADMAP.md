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
- [ ] Build the paired CLI that writes, syncs, and clears `TODO.md` and
      `PLAN.md`, so the bookkeeping commits are produced and dropped
      mechanically rather than by hand.
