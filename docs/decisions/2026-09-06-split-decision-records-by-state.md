# Split decision records by state

## Issue

Every decision record lives in `docs/decisions/`, whatever its state. A reader
who wants to know which records are ready to be acted on — planned against,
built — has to open each one and read its `## Status` line, or grep the commit
log for `decision: accept`. `ls docs/decisions/` cannot answer the question, and
on a busy day the directory also sorts within the date by slug, so listing order
drifts from creation order.

This record proposes moving a record between directories as its state changes:
`docs/proposals/` while it awaits review, `docs/decisions/` once accepted,
`docs/rejections/` once refused. On acceptance the file would also be re-dated
to the acceptance date, and the record would carry both dates in its body.

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- [2026-09-05-identify-decisions-by-date](./2026-09-05-identify-decisions-by-date.md)
  is accepted: an id is `YYYY-MM-DD-slug`, fixed at creation, never re-dated or
  renamed, not when its status changes. Ids are cited from commit subjects, PR
  titles, issue threads, and other repositories, where a rename cannot reach.
- [2026-09-05-freeze-a-decision-when-review-ends](./2026-09-05-freeze-a-decision-when-review-ends.md)
  is accepted: a record carries no dates, its body is frozen by `accept` or
  `reject`, and a frozen record is corrected only by appending to `## Errata`.
  That record rejected an index file carrying live status as a mirror of the log
  that has to be maintained.
- `CHARTER.md` makes phase transitions greppable events rather than parsed
  state.
- Records cross-link with relative paths, `[id](./id.md)`, and a link inside a
  frozen record cannot be rewritten in place.
- [2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md)
  schedules `condoc status`, and its `plan start` with no id already lists only
  accepted decisions that have no plan.

## Argument

Keep one directory and fixed ids, and answer "what is actionable?" from the
tool. **Chosen.** The state a directory split would surface is already
announced by `decision: accept` commits and already listed by `plan start`;
`condoc status` and a stale-proposal listing add the human-facing view without
touching the id contract or any link.

Moving a record on each transition, whether re-dated (**Rejected**) or not
(**Rejected**), buys an at-a-glance answer from `ls` at the cost of every
citation of the record's path. Re-dating additionally gives one record two ids
across its own lifecycle events. Neither cost is recoverable within the
convention's existing rules, and the sort complaint that motivated the split is
not fixed by it: re-dating changes which day a record sorts under, not the
intra-day tie-break.

## Architectural Decision

1. A decision record stays at `docs/decisions/<id>.md` for its whole life,
   whatever its state. No `docs/proposals/` or `docs/rejections/` directory is
   introduced.
2. The id is not re-dated on acceptance, and a record carries no dates, as the
   two accepted records above already require.
3. Which records are actionable is answered by the log and the tool: `decision:
accept <id>` in `git log`, `plan start` with no id, and `condoc status`.
4. A stale proposal is reported, never rejected automatically. Rejection stays a
   human write with a stated reason, because the value of a rejected record is
   why it was refused.

## Positions

- **Split directories and re-date on acceptance** (as proposed). _Rejected._
  `decision: propose` and `decision: accept` would cite different ids for one
  record; branch names, PR titles, and issue comments minted at propose time go
  stale at accept; every inbound relative link breaks, including links inside
  frozen records, which can only be fixed by an erratum per link per
  acceptance; the proposal date has to return to the body, reinstating the
  `## Dates` section that the freeze decision deleted.
- **Split directories but keep the id fixed.** _Rejected._ Keeps one id per
  record and no dates in the body, but still breaks every relative link on each
  transition and makes the directory a second copy of the status line that
  `lint` would have to reconcile — the same ground on which an index file was
  refused. `.adr-dir` can point at only one directory, so `adr list` would go
  blind to proposals.
- **Auto-reject proposals older than a configurable age.** _Rejected._ A timeout
  has no reason to record. Reporting staleness is a `condoc` listing; the
  rejection itself stays manual.
- **A time component in the id to fix intra-day sort order.** _Rejected._ Out
  of scope here, and the accepted id decision already refused a per-day counter
  because the slug disambiguates and no ordering within a day is implied.

## References

- [2026-09-05-identify-decisions-by-date](./2026-09-05-identify-decisions-by-date.md)
  — the fixed-id rule this would have broken.
- [2026-09-05-freeze-a-decision-when-review-ends](./2026-09-05-freeze-a-decision-when-review-ends.md)
  — no dates in a record, and the rejected index-file position.
- [2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md)
  — `condoc status` and `plan start`, which carry the at-a-glance answer.
