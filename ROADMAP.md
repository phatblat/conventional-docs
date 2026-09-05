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
