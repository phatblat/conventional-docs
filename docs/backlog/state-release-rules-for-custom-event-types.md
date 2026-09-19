# State a release rule for every custom event type

## Problem

`.releaserc.json` states a release rule for `docs` only. The custom lifecycle
types this convention adds — `decision`, `deploy`, `plan`, `release`, `todo`
— have no explicit rule, so they are release-neutral only because
`semantic-release`'s commit analyzer happens to ignore types it does not
recognize; nothing states that on purpose.

## Outcome

`.releaserc.json` carries an explicit release rule for `decision`, `deploy`,
`plan`, `release`, and `todo`, so their release behavior is a stated
decision rather than an accident of the default analyzer's ignore list.
