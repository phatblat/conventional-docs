# Graduate the Roadmap into a Backlog

## Issue

This decision extends
[2026-09-05-commit-the-session-todo](./2026-09-05-commit-the-session-todo.md).

The convention's second graduation trigger — "the document has outgrown a
single file: it needs siblings, status, or structure" — has exactly one
remedy today: rename `ROADMAP.md` to `docs/roadmap.md`, still a single file.
The trigger promises siblings and delivers a relocation. A Roadmap that is
actually a list of substantial items hits two failures a rename cannot fix:
every concurrent branch that touches an item conflicts on the same file, and
an item worth more than a checklist line has nowhere to carry a real
description without bloating the ordering document itself.

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- Nothing has been released from this repository yet, so no adopter has an
  existing Roadmap shape to migrate other than this repo's own.
- `2026-09-05-commit-the-session-todo` is accepted and frozen; its clause 9
  ("a repo whose existing `TODO.md` is a durable backlog has a Roadmap under
  the wrong name") assumed a single-file Roadmap and needs to stay true when
  the Roadmap is a directory instead.
- `2026-09-05-identify-decisions-by-date` already rejected sequential and
  date-prefixed decision ids for the same reason a Backlog item id must avoid
  them: allocation races on merge, and a date that belongs to an append-only
  log rather than a queue whose entries are deleted.
- `crates/conventional-docs` already lags the convention — it has no
  `plan`/`todo` verbs yet — so this decision adds no CLI surface of its own;
  what the tool owes is recorded as consequences, not clauses.
- `just test-markdown-links` runs linkinator over `docs/**/*.md` and the root
  markdown files, and is the mechanism that turns "delete the item and its
  Roadmap line together" from a convention into something CI can catch.

## Argument

A directory of per-item files, indexed by an ordered Roadmap, is **chosen**
over every alternative that keeps the Roadmap a single file, because the
problem is structural: two branches editing the same list conflict by
construction, and there is no line length that fits both "what's next" and "a
work item's full description." Splitting the Roadmap into `docs/backlog/` is
therefore not one option among several small tweaks — it is the only shape
where each item's file, and each item's churn, is independent of every other
item's.

Slugs alone, with no number and no date prefix, are **chosen** for the item
id, for the same reason `2026-09-05-identify-decisions-by-date` rejected
numbers for decisions: a number or a date is allocated at write time and has
to stay unique at merge, which is exactly the race two concurrent branches
hit. A slug collision is a title collision, caught by the filesystem itself
merging two files, and it means the two items should be merged by hand — not
renumbered.

No new commit type is **chosen** over a `backlog:` event, because the event
already exists: `docs/backlog/<slug>.md` appearing or disappearing under git
is a greppable fact `git log --diff-filter=A` or `--diff-filter=D` finds
without a vocabulary addition, while a new type would force every adopter to
extend `type-enum` for a fact the filesystem already states.

Detecting concurrent work by branch-name suffix is **chosen** over any
in-repo claim record, because a claim is mutable state that two branches
would both want to edit — conflicting exactly when it matters and needing a
reaper for a worker that dies mid-claim. A branch name costs nothing to write
and is visible to `git ls-remote` the moment it is pushed. It is deliberately
weaker than arbitration: it tells a second worker someone is already on an
item, but does not stop two workers from starting at once. Building the
stronger form is out of scope here (see Consequences) because no adopter has
felt the collision yet, and because an arbitrated claim presumes a trunk
every worker can push.

## Architectural Decision

1. A **Backlog** is a directory of work items, one file per item at
   `docs/backlog/<slug>.md`. It has no small-repo (single-file) form; a
   repository either has no Backlog or has the directory.
2. A repository with a Backlog MUST have a Roadmap. The Roadmap's own
   location — `ROADMAP.md` or `docs/roadmap.md` — is independent of whether a
   Backlog exists.
3. An item's id is a kebab-case slug of its title, with no number and no date
   prefix. The slug MAY change while the item exists (a rename is a file
   rename plus a Roadmap link update, in one commit). Two items whose slugs
   collide are the same item and MUST be merged into one file.
4. An item file uses exactly this skeleton — H2 sections `Problem` and
   `Outcome`, in that order, with an optional `Notes` last:

   ```markdown
   # <Title>

   ## Problem

   <What is wrong or missing, and who it costs. For a bug: what happens, what
   should happen, and how to reproduce it.>

   ## Outcome

   <What is observably true when this is done.>

   ## Notes

   <Optional, last: evidence, links, a related decision, a suggested
   direction the decision record is free to overrule.>
   ```

   An item carries no status, priority, assignment, claim, or date field:
   presence in `docs/backlog/` is the open state, order is the Roadmap's, and
   dates are `git log`'s.

5. Once a repository has a Backlog, the Roadmap carries an ordered index of
   links to Backlog items and MUST NOT restate an item's body. A Roadmap with
   no Backlog MAY continue to carry each item's whole description inline, as
   this project's did before this decision.
6. A Backlog item MUST be deleted by the change that implements it, together
   with its Roadmap line, in the same pull request as that change. An item
   that will not be implemented MUST be deleted rather than kept or marked
   closed. A decision record MUST NOT link to a Backlog item, because the
   item is deleted at implementation and the link would go stale; a decision
   that grew out of an item restates the problem in its own `## Issue`. A
   Backlog item MAY link to a decision record.
7. A Backlog item MUST NOT record who is working on it. A branch that
   implements an item SHOULD end with the item's slug, so any prefix a team
   already uses (`feat/`, `fix/`, an author or agent name) survives and
   concurrent work is found by a suffix match over the refs a clone already
   fetches:

   ```bash
   git ls-remote --heads origin '*<slug>'   # anyone, anywhere
   git branch --all --list '*<slug>'        # what this clone already knows
   ```

   This is detection, not arbitration: two workers who both start get two
   branches, and the second to open a pull request loses the work, not the
   repo. An unpushed branch is an invisible claim, and renaming an item's
   slug while a branch is open breaks the match — rename the branch with it.

8. An artifact SHOULD graduate from root form to graduated form when the root
   is crowded, or when the artifact needs siblings, per-item status, or
   internal structure (existing rule). A Roadmap whose items need more than a
   line each, or whose single file conflicts between concurrent branches,
   SHOULD decompose into a Backlog instead of only moving.
9. Decomposing a Roadmap into a Backlog MUST, in a single commit: write one
   `docs/backlog/<slug>.md` per item, rewrite the Roadmap as the ordered
   index over them, and update the Charter's `## Artifacts` section. No stub
   is left at any old anchor inside the Roadmap.

## Consequences

`doc status`'s docset table (owed by
`2026-09-07-stamp-the-bookkeeping-report-the-docset`) gains a `backlog` row
reporting the item count once it exists; `doc backlog new <title>` — writing
`docs/backlog/<slug>.md` from the skeleton and committing it — is new tool
surface. Both are tracked as Backlog items rather than specified here, since
neither is built yet.

This repository's own `ROADMAP.md` is split in the same change that adds this
decision: its 16 existing lines become 17 files (one Roadmap line spanned two
items that are one piece of work, and two new items are filed for the tool
work above), and `ROADMAP.md` is rewritten as the index.

`2026-09-07-stamp-the-bookkeeping-report-the-docset` (proposed, still
editable) cites `ROADMAP.md:20-22` and `ROADMAP.md:26-28` as the roadmap
lines its bookkeeping slice answers; both citations are rewritten to point at
`docs/backlog/build-the-doc-bookkeeping-slice.md` in a `fix:` commit in the
same change, since editing a proposed record's body is this repo's
established practice.

`2026-09-05-commit-the-session-todo` is frozen and receives a dated
`## Errata` entry pointing at this record, rather than a body edit: its
clause 9 rename advice still applies to a single-file durable backlog, and
now also to a directory of per-item files.

An in-repo arbitrated claim (first push wins, a reaped claim for a dead
worker) is not specified by this decision. It is filed as its own Backlog
item, `decide-how-agents-claim-a-backlog-item`, because no adopter has felt
the collision this would solve, and because it presumes a trunk every worker
can push — a stronger requirement than anything else in this convention
makes.

## Positions

- **Number the items, like decisions once were.** _Rejected._ Reintroduces
  the allocation race `2026-09-05-identify-decisions-by-date` already
  rejected for decisions, and parallel work on the Backlog is the exact
  scenario that race breaks.
- **Date-prefix the items.** _Rejected._ A date belongs to an append-only
  log; a Backlog item is deleted when done, and its filed date is already in
  `git log`.
- **`docs/tickets/`.** _Rejected._ Implies a tracker workflow — states,
  assignment, priority — the item deliberately refuses to carry.
- **`docs/issues/`.** _Rejected._ Collides with GitHub Issues by name, and
  with a decision record's own `## Issue` section.
- **Roadmap as a directory with an index file inside it
  (`docs/roadmap/_index.md` plus `docs/roadmap/<slug>.md`).** _Rejected._
  Introduces no new vocabulary, but trades one bikeshed (what the index file
  is named) for hiding a real register change — a living overview versus
  items that are deleted when done — inside one directory.
- **A `backlog:` commit type.** _Rejected._ The item file's add or delete
  under `docs/backlog/` is already the greppable event; a type would make
  every adopter extend `type-enum` for a fact the filesystem already states.
- **A claim field or a claim subdirectory inside the Backlog.** _Rejected._
  Mutable state that two branches would both want to edit, conflicting
  exactly when it matters, and needing a reaper for a worker that dies
  mid-claim. A branch name costs nothing and is visible the moment it is
  pushed.

## References

- [2026-09-05-commit-the-session-todo](./2026-09-05-commit-the-session-todo.md) —
  the record this extends; its clause 9 rename advice is why this decision
  states the Backlog/Roadmap register split explicitly.
- [2026-09-07-stamp-the-bookkeeping-report-the-docset](./2026-09-07-stamp-the-bookkeeping-report-the-docset.md) —
  its `ROADMAP.md` citations are rewritten by this change.
- [2026-09-05-identify-decisions-by-date](./2026-09-05-identify-decisions-by-date.md) —
  the prior rejection of allocated ids this decision reuses for Backlog item
  ids.
