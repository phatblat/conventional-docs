# Enable GitHub Pages

## Problem

The site builds locally but is not published anywhere: `Settings → Pages` has
no source configured, and `.github/workflows/pages.yml` only builds on pull
requests.

## Outcome

GitHub Pages is set to deploy from GitHub Actions, `pages.yml` also runs on
`push: branches: [main]`, and the built site is reachable at its `github.io`
URL.
