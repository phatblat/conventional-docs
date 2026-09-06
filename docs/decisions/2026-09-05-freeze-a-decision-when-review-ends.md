# Freeze a decision when review ends

## Issue

`README.md` says an accepted decision is frozen: "changing your mind is a new
decision that supersedes it." The `conventional-docs` skill does not implement
that. Its status lifecycle keeps writing to the record after review —
`implemented` at merge, `superseded` later, and an
`- Updated: YYYY-MM-DD (<what changed>)` line under `## Dates` for "material
later edits" — so an accepted record is a live document whose text can drift
from what review approved, with no way to tell that it has.

The status set is wrong at both ends. It carries three states that exist only
after review and require editing a frozen record, and it has no state for a
record still being written, or for one that review refused. A refused proposal
is worth keeping: it records what was considered and why it was turned down,
which is the question a later reader most often asks.

`- Published:` is the sharpest symptom. Its instruction is "set at merge", but
`decision: accept` is committed on a branch where no merge date exists, and no
later step is specified to come back for it, so the field is either `TBD`
forever or invented. Moving it to release time would make the date computable
without fixing the real error: what shipped in a release is `CHANGELOG.md`'s
question, and a decision record answering it again is a second copy that can
disagree with the first.

`CHARTER.md` already states the rule to follow. The lifecycle commit types
"make phase transitions greppable events rather than parsed state" — and a
status line in the record is that state, parsed. The artifacts table gives
Decisions the lifetime "append-only", which is precisely the lifetime that
rewriting a record in place violates.

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- A record is written over one or more sessions before anyone should review it,
  and review is where the text stops moving. Freezing can only begin when review
  ends, and the commit that ends it is itself a write.
- Four facts currently live in a record after acceptance: that it was
  implemented, when it was published, that it was superseded, and that it was
  materially edited. Each needs a home outside the record, or an argument for
  staying in.
- Git is content-addressed. The bytes of a file at a commit are already a
  checksum, already immutable, and already in every clone, so any integrity
  scheme that stores its own digest is storing a second copy of something git
  holds better.
- `git log --grep` over the event vocabulary is available to anyone with a full
  clone. A shallow clone is not: an integrity check has to degrade honestly
  rather than pass silently.
- A reader on a rendered page — GitHub, an editor preview, a file copied out of
  the repo — sees the file and not the log. Whatever leaves the file becomes
  invisible to that reader.
- Ids are fixed at creation and may be cited from outside the repo, so nothing
  here may require renaming a record.
- Decisions are append-only. Nothing here may require rewriting one.
- [2026-09-05-one-file-decision-records](./2026-09-05-one-file-decision-records.md)
  is accepted: a decision record is one file, with no single-file form and no
  graduation step.

## Argument

Freeze the body when review ends, and give the file an append-only tail.
**Chosen.** It is the only shape that satisfies both of the convention's own
claims at once: the reviewed text stays exactly as reviewed, and everything
learned afterwards is still attached to the record a reader lands on. An
append-only tail is also the artifact's declared lifetime, so the mechanism and
the lifetime finally agree.

Keeping the full status lifecycle and dropping the freeze claim from `README.md`
(**Rejected**) is the coherent alternative, and it is what MADR does. It gives
up the sentence the whole decision model rests on. Deleting the status lifecycle
with no replacement (**Rejected**) leaves a reader of a superseded record no way
to discover that it is superseded.

Drift needs no new field. The freeze commit's blob **is** the checksum
(**Chosen**), so a check reads the record as of that commit and diffs it against
the working copy, reporting what moved rather than that something did. A digest
written into the record (**Rejected**) is self-referential, duplicates git, and
is editable by whoever edits the body — it catches accidents, not intent.

Everything the tail does not carry moves to the artifact that owns it.
`CHANGELOG.md` already answers what shipped and when, and `plan: done` already
announces that an accepted decision's work is finished.

### Open before acceptance

- Whether an erratum pointing at a superseding record is the same kind of thing
  as a correction. Clause 7 says yes, because neither changes the decision. If
  review disagrees, the section is named `## Addenda` and both kinds stay, or
  the pointer is dropped and a reader of a superseded record is on their own.
- Whether `decision: draft` earns an event, or a draft is simply a record whose
  propose event has not fired yet. Clause 1 gives it one, so that a hook can
  refuse to plan against a record nobody has submitted.

## Architectural Decision

1. A record has four states and moves through them in one direction:
   **draft → proposed → accepted | rejected**. Each transition is a commit.

   ```text
   decision: draft 2026-02-11-split-the-scheduler
   decision: propose 2026-02-11-split-the-scheduler
   decision: accept 2026-02-11-split-the-scheduler
   decision: reject 2026-02-11-split-the-scheduler
   ```

   `draft` means the record exists and is being written; it is not the spec and
   nothing may be planned against it. It is optional — a record written in one
   sitting is proposed directly, and most are — but it is the only honest state
   for a record committed before it is ready to be read. `propose` submits it
   for review and makes it the spec. `accept` and `reject` both end review. A
   rejected record stays in the log: it says what was considered and why it was
   refused.

2. `## Status` carries one line, exactly one of:

   ```text
   This is a **draft**; it is not ready for review.
   This is a proposal that is **awaiting review**.
   This is a proposal that is **accepted**.
   This proposal was **rejected**.
   ```

   The `implemented`, `superseded`, and `deprecated` states are removed.

3. A record is mutable while `draft` or `proposed`, and frozen by `accept` or
   `reject`. That commit is the last write to the record's body.
4. `## Dates` is deleted from the skeleton. A record's date is its id, and every
   other date it carried is a commit date that `git log` already holds.
5. `decision: implement <id> (#PR)` is removed from the vocabulary. Finishing an
   accepted decision's work is already announced by `plan: done <id>`, which
   fires on the branch and deletes `PLAN.md`. Work below the Plan threshold gets
   no completion event; its changelog line and its merge are the record.
6. A changelog line may cite the decision id it came from. That is a portable
   reference — a path in the repo rather than one host's number — which is the
   form Keep a Changelog 2.0.0 recommends over a bare `(#1234)`.
7. The skeleton gains an optional `## Errata` as its last section, append-only,
   one dated line per entry, newest last:

   ```markdown
   ## Errata

   - 2026-03-04: The second clause named `--strict`; the flag shipped as
     `--pedantic`. The decision is unchanged.
   ```

   Exactly two kinds of entry are admissible: a correction of fact or expression
   that leaves the decision itself unchanged, and a pointer to a record that
   supersedes or extends this one. Anything that changes the decision is a new
   decision.

8. Reciprocal cross-linking splits on the target's state. A draft or proposed
   target is edited in place, as today. A frozen target receives an erratum.
9. Supersession is stated by the superseding record's Issue and by an erratum on
   the superseded one. Neither record's status changes.
10. Drift is detected from git, and no digest is written into any record. A
    check resolves the freeze commit for an id with
    `git log --all --grep='^decision: \(accept\|reject\) <id>$'`, reads the
    record at that commit, and compares everything above the `## Errata` heading
    — the whole file when there is none — against the working copy. It reports
    the diff, not a boolean. When no freeze commit is reachable, because the
    clone is shallow or history was rewritten, the check is skipped with that
    reason stated; it never passes silently.
11. Implementing this deletes `## Dates` from every existing record and rewrites
    the skill's status-and-date lifecycle, `README.md`, and `EVENTS.md` — which
    gains `decision: draft` and `decision: reject` and loses
    `decision: implement` — in one commit, with `type-enum` unchanged because
    `decision` is already listed. That commit edits accepted records: it is the
    last edit made under the old rule, and it is not logged as errata, because
    it changes no decision.

## Consequences

"Is this implemented?" and "which release carried it?" leave the record. A
reader on a rendered page loses both at a glance and has to consult
`CHANGELOG.md` or `git log --grep`. That is the price of one source per fact,
and it is the same trade the convention already makes when it insists a
transition is a commit rather than a field.

An accepted record plus its errata is exactly what review approved, followed by
a dated list of what was learned since. That is a stronger artifact than a
record whose text has been edited an unrecorded number of times.

Clause 5 shortens the longest subject that carries an id from
`decision: implement <id> (#PR)` to `decision: propose <id>`, which relaxes the
slug budget from 60 characters to 71. The sibling `condoc` decision states that
cap and changes with this one.

Clause 10 is a `lint` rule, so the integrity property is only as available as
the checker. Nothing enforces it at commit time, by design: a pre-commit hook
that reads history would make every commit in a large repo slower to serve a
rule that fires rarely.

This record is written under the convention in force, so it still carries a
`## Dates` section it proposes to delete. Clause 11 removes it with the rest.

## Positions

- **Move `Published` to release time.** _Rejected._ Makes the date computable,
  and was the first proposal on this branch, which this record replaces. It
  keeps a decision record answering a question `CHANGELOG.md` owns, and it keeps
  a write to a record review had already closed.
- **Keep a single `- Accepted: YYYY-MM-DD` line.** _Rejected._ One date is cheap
  and a rendered reader would use it, but it is the `decision: accept` commit's
  own date, restated in the file that commit just froze.
- **Keep the full status lifecycle; drop the freeze claim from `README.md`.**
  _Rejected._ Internally consistent, and it is what MADR does. It gives up "a
  proposed decision is the spec, and once accepted it is frozen", which is the
  sentence the rest of the decision model is built on.
- **Keep `decision: implement`.** _Rejected._ With the record frozen it has
  nothing to write, and the phase it announces belongs to the Plan, which
  already announces its own completion and has a file to delete while doing it.
- **A content checksum stored in the record.** _Rejected._ It has to exclude
  itself from its own input, it duplicates a hash git already computes and
  distributes, and it is editable by exactly the person the check is aimed at —
  so it detects accident and not intent. The freeze commit cannot be edited
  without rewriting history, which is a louder signal than a mismatched digest.
- **A separate `<id>-errata.md` file.** _Rejected._ It buys strict byte
  immutability for the record, which is genuinely cleaner, but it splits one
  decision across two files against an accepted decision of this repo, hides the
  errata from anyone reading the record, and cannot be linked from the record in
  advance without committing a link to a file that may never exist. Clause 10
  recovers the immutability property without the split.
- **`## Updates` instead of `## Errata`.** _Rejected._ "Update" is the word for
  the operation freeze forbids, and a section named Updates invites it.
  `## Addenda` is the accurate word if the section keeps carrying supersession
  pointers as well as corrections, and is the fallback named in Open before
  acceptance.
- **An index file carrying live status for every record.** _Rejected._ Restores
  at-a-glance state, but it is a mirror of the log that has to be maintained,
  and a stale index misleads worse than no index.
- **No errata; every correction is a new decision.** _Rejected._ A typo or a
  dead link would need a whole record, and a reader of the original would still
  meet the error with nothing pointing away from it.

## References

- [CHARTER.md](../../CHARTER.md) — "greppable events rather than parsed state",
  and the append-only lifetime this aligns with.
- [MADR](https://adr.github.io/madr/) — the status set this narrows at one end
  and widens at the other.
- [Keep a Changelog 2.0.0](https://keepachangelog.com/en/2.0.0/) — portable
  references over host-specific numbers.

## Dates

- Published: TBD (set at merge).
