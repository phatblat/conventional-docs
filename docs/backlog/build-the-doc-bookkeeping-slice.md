# Build the doc bookkeeping slice

## Problem

`TODO.md` and `PLAN.md` are written, synced, and cleared by hand: an agent
hand-writes an ISO timestamp, a session id, a `# Plan: <id>` heading, a
`Decision:` link, and the matching commit subjects. That hand-writing is the
easiest part of the convention to get wrong — a folded commit, a stale
timestamp, a missing single-artifact pathspec — and it is also the most
repetitive.

## Outcome

`doc status [--json]`, `doc todo sync [--session <id>] [--stdin] [--skeleton]`,
`doc todo clear`, `doc plan start [<id>] [--stdin] [--skeleton]`, and
`doc plan done [--force]` exist and match
[2026-09-07-stamp-the-bookkeeping-report-the-docset](../decisions/2026-09-07-stamp-the-bookkeeping-report-the-docset.md):
`TODO.md` and `PLAN.md` are written, synced, and cleared mechanically, and
`doc status` reports every artifact in `README.md`'s table plus the current
branch's Plan and Todo state.

## Notes

This item is the two roadmap lines
[2026-09-07-stamp-the-bookkeeping-report-the-docset](../decisions/2026-09-07-stamp-the-bookkeeping-report-the-docset.md)
already names as one piece of work — the Convention section's "paired CLI"
line and the `doc` section's "bookkeeping slice" line both described this
same build.
