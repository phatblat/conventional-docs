# Set the NPM_TOKEN repository secret

## Problem

The `release` CI job publishes to npm through `semantic-release`, but the
repository has no `NPM_TOKEN` secret, so a release run fails at the publish
step.

## Outcome

`gh secret set NPM_TOKEN --repo phatblat/conventional-docs` has been run, and a
release run publishes the package without a credential error.
