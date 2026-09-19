# Build doc release, decision import, and graduate

## Problem

Three more `doc` capabilities are still unbuilt: cutting a release, importing
an existing numbered ADR log into this convention's dated form, and moving an
artifact from its small-repo path to its graduated `docs/` path. Each is
currently a manual, error-prone procedure this repo's own skill and README
document in prose only.

## Outcome

`doc release`, `doc decision import`, and `doc <artifact> graduate` exist and
perform the procedures the skill and README already specify by hand:
`release` renames `[Unreleased]` and opens a fresh one, `decision import`
renames a numbered log to dated ids with redirects, and `<artifact> graduate`
moves a file, rewrites inbound links, and updates the Charter's
`## Artifacts` table, all in one commit.
