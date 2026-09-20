# Ship doc binaries from a tag job

## Problem

`doc` has no release artifact of its own: a user who wants the binary has to
build it from source, even though the npm package already gets published by
`semantic-release` on every tagged release.

## Outcome

A tag job builds and attaches `doc` binaries (for every platform the build
targets) to the GitHub release `semantic-release` already creates, alongside
the npm package.
