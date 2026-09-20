# Decide how agents claim a backlog item

## Problem

A branch ending in the item's slug lets a worker detect that someone else is
on an item, but it does not arbitrate between two who start at once, and it
goes unseen until the branch is pushed. Crews that dispatch work to several
agents at once need first-push-wins arbitration, and the convention does not
say whether that belongs in the repo.

## Outcome

A decision record that either specifies an in-repo claim form — its path, its
compare-and-swap on write, how a dead worker's claim is reaped, and what it
requires of trunk permissions — or records that claiming stays outside the
convention.

## Notes

The worked prior art is phatblat/phathub's `.bus/`: `git mv` from
`inbox/<agent>/` to `claimed/<agent>/` with a `claimant` nonce, a bounded
jittered push retry, `exit 2` on a lost race, and a TTL janitor that recycles
a stale claim three times before quarantining it. It works because every
worker in that repo can push its trunk — a requirement no other part of this
convention makes.
