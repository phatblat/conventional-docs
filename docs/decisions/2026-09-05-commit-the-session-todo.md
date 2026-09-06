# Commit the session todo

## Issue

The Todo is the Implementation phase's handoff artifact and the one artifact
this convention still keeps outside git. `CHARTER.md` states "Founding rule:
every artifact lives in git," yet its own `## Artifacts` table lists
`| Todo | agent memory | not committed |`, and the small-repo/graduated axis
for that row is `agent memory` → `docs/todo.md` (opt-in) — memory is not a
path, so the row misuses a column that means "where it lives once the root is
crowded," and `(opt-in)` appears in no other row and is defined nowhere.
`skills/conventional-docs/SKILL.md`'s loop says per-session state goes "in
agent memory or an opt-in `docs/todo.md`, **not a commit**." All three
statements are false for exactly the phase where work is in flight: a crashed,
compacted, or cancelled session loses the list, and nothing else in the repo
records where it left off, so the founding rule and the cold-start promise
both fail at the one point that needs them most.

## Status

This is a proposal that is **accepted**.

## Assumptions and Constraints

- An agent's list lives in volatile session state; a crashed, compacted, or
  cancelled session loses it and nothing else in the repo records it.
- A commit is a rollback point only for what it contains, and it survives the
  machine only once pushed; durability is bounded by the remote.
- `.husky/pre-commit` runs `just lint` (markdownlint over `**/*.md`) on every
  commit, and `just check` adds `prettier --check .` and a link check over
  root `*.md`, so a file the agent commits mid-task must pass those gates or
  the commit fails during the work.
- `TODO.md` is a common filename in the wild, where it usually means a durable
  backlog — which this convention already has as the Roadmap.
- The graduation trigger is a document outgrowing a single file, which neither
  a Plan nor a Todo scoped to a single branch can do.
- Adding an event type is two edits that must agree: prose in `EVENTS.md` and
  `type-enum` in the repo's commitlint config.

## Argument

Commit the cache. **Chosen.** The Todo is the only handoff that cannot survive
its own actor, and the cost of writing it is a file the agent is already
maintaining in memory. Keeping it in memory (**Rejected**, see Positions)
leaves the founding rule with an exception at the one phase that needs it
most; an opt-in path leaves every reader probing for a file that may not be
there.

## Architectural Decision

1. The Todo artifact is always `TODO.md` at the repository root, with the
   lifetime of one branch / worktree — the same lifetime as the Plan, because
   the cache outlives the session that wrote it and that is exactly what makes
   it a handoff. The `agent memory` small-repo form and the opt-in
   `docs/todo.md` graduated form are removed from the convention, with no
   `docs/` location replacing them.
2. `TODO.md` is a cache, not a source: a projection of the agent's live list.
   Per the Charter's projection rule it carries the identity and revision of
   its source — the session id and the UTC time the list was taken.
3. Every `TODO.md` uses exactly this skeleton:

   ```markdown
   # Todo

   - Session: <agent session id, or the agent's name when it has none>
   - Synced: <YYYY-MM-DDTHH:MM:SSZ>
   - Plan: `PLAN.md` — or `none` when the change needs no Plan

   ## Steps

   - [x] A finished step
   - [ ] The step in progress
   - [ ] A step not started

   ## Notes

   What the list cannot carry: a blocked step, a command that failed, a choice
   made mid-flight. Omit this section when there is nothing to say.
   ```

4. An agent writes and refreshes `TODO.md` whenever it is tracking a list at
   all, and commits it at each checkpoint with `todo: sync`. There is no
   threshold below which the cache is skipped: a change too small to need a
   Plan still gets a Todo if the agent is tracking steps for it.
5. `TODO.md` is deleted before merge, in a `todo: clear` commit; `PLAN.md` is
   deleted by its `plan: done` commit, as `EVENTS.md` already states.
6. A `plan:` or `todo:` commit touches only its own artifact — never code,
   never another document. Each pair is therefore net-zero: `plan: start`
   adds `PLAN.md` and `plan: done` deletes it, `todo: sync` writes `TODO.md`
   and `todo: clear` deletes it. Dropping every `plan:` and `todo:` commit
   from a branch leaves the tree identical, so the bookkeeping is erasable by
   rebase and free to squash-merge. A commit that mixes one of these files
   with real work is not droppable and is not one of these events.
7. The vocabulary gains `todo: sync` and `todo: clear`; `type-enum` gains
   `todo`. `todo:` subjects carry no id — the cache belongs to a session, not
   to a decision.
8. Neither `PLAN.md` nor `TODO.md` graduates. The artifacts table shows `—` in
   the Graduated column for both.
9. A repo whose existing `TODO.md` is a durable backlog has a Roadmap under
   the wrong name and renames it to `ROADMAP.md` when adopting. The two are
   told apart by lifetime: the Roadmap outlives every branch, the Todo does
   not outlive the branch it is on.

## Consequences

Every phase's handoff is now a file in git; the founding rule has no exception
left.

Branch history gains bookkeeping commits, one CI run each on push. They are
net-zero as a group: a repo that squash-merges pays nothing at merge, and a
repo that rebase-merges drops them.

`todo:` is release-neutral — `semantic-release`'s commit analyzer produces no
release for a type it does not know, and `.releaserc.json` names only `docs`.
Stating a release rule for every custom type explicitly is a separate cleanup,
tracked on the Roadmap.

The cache passes the same markdown gates as any other file, which is why its
skeleton is pinned rather than free-form.

Two agents on one branch will conflict in `TODO.md`; that is the same conflict
this convention wants visible for intent, resolved the same way — one branch,
one worktree, one list.

`todo:` prose is added to `EVENTS.md`, whose home is still under review by
[2026-09-05-give-events-their-own-artifact](./2026-09-05-give-events-their-own-artifact.md).
Nothing here depends on the filename: if that decision is rejected, `todo:`
moves with the rest of the vocabulary.

## Positions

- **Leave it in agent memory.** _Rejected._ The one phase whose handoff cannot
  survive its own actor, against a Charter that promises a cold start from the
  branch.
- **`docs/todo.md`, opt-in.** _Rejected._ Opt-in means an agent probes two
  paths and may still find nothing; a cache nobody can rely on finding is not
  a handoff. `docs/` is also where documents go when they grow, and this one
  is deleted rather than grown.
- **A dotfile (`.todo.md`) or a gitignored file.** _Rejected._ Ignored files
  are not rollback points, and dotfiles are where config lives, not
  artifacts.
- **`SESSION.md` or `PROGRESS.md`.** _Rejected._ Unfamiliar; `TODO.md` is what
  every reader and every agent already guesses. The backlog collision is
  answered once, at adoption, by the Roadmap rename.
- **Fold the todo into `PLAN.md`'s Status section.** _Rejected._ The Plan
  exists only past its threshold and only for an accepted decision; the todo
  exists whenever an agent is working.

## References

- [2026-09-05-give-events-their-own-artifact](./2026-09-05-give-events-their-own-artifact.md)
  — the proposed home of the vocabulary this decision adds a type to, still
  under review.
- [Conventional Commits](https://www.conventionalcommits.org/) — the type
  syntax `todo:` extends.
