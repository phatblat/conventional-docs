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

## Site

- [ ] Pick one of the three preview styles, delete the other two
      `site/assets/css/theme-*.css` files, set `params.style` to the winner, and
      set `params.stylePreview` to `false` (removes the switcher partial and its
      bootstrap script).
- [ ] Enable GitHub Pages (Settings → Pages → Source: GitHub Actions), then add
      `push: branches: [main]` to `.github/workflows/pages.yml`.
- [ ] At go-live, add the site URL to `package.json` as `homepage`, add the
      badge and site link to `README.md`, and trim the README sections the site
      now owns.
- [ ] Add a social preview image at `site/static/og.png` and reference it from
      `head.html` once the visual direction is settled.
- [ ] If a custom domain is registered, update `baseURL` in `site/hugo.yaml` and
      add `site/static/CNAME`.
