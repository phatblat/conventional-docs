# Build the doc validation slice

## Problem

Nothing checks that a repository's artifacts actually match this convention
— that a decision record uses the right H2 sections, that its status line is
one of the four exact strings, that a frozen record's body has not been
edited since its freeze commit. Violations are only ever caught by a human
reviewer who happens to notice.

## Outcome

`doc lint`, `doc fix`, and `doc check` exist, including a drift check that
diffs a frozen decision record's current body against its freeze commit
(`decision: accept` / `decision: reject`) and fails when they differ, plus
adjacent-file findings: a missing README or license, the same adjacent file
present in two recognized locations, a code of conduct whose attribution
names a superseded Covenant version, and a non-root adjacent file missing
from the Charter's `## Artifacts` table.
