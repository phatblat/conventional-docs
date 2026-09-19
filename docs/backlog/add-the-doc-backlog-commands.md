# Add the doc backlog commands

## Problem

`doc` writes every other artifact's skeleton but has no command for a
Backlog item, and `doc status`'s docset table (owed by
[2026-09-07-stamp-the-bookkeeping-report-the-docset](../decisions/2026-09-07-stamp-the-bookkeeping-report-the-docset.md))
reports every artifact in `README.md`'s table, which now includes Backlog.

## Outcome

`doc backlog new <title>` writes `docs/backlog/<slug>.md` from the skeleton
and commits it, and `doc status`'s docset table carries a `backlog` row with
the item count.
