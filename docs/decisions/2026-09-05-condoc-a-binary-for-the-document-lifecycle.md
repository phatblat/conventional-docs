# condoc, a binary for the document lifecycle

## Issue

The convention is nine artifacts, nine event verbs, exact status prose, exact
date lines, id rules that forbid renaming, and reciprocal cross-links between
records — all maintained by hand, or by an agent reading a skill and getting the
strings right from memory. Nothing checks the result.

The drift is already here. `CHARTER.md` records
`| Decisions | — | none recorded yet |` against four records in
`docs/decisions/`, and `just check` passes. `ROADMAP.md` already asks for "the
paired CLI that writes, syncs, and clears `TODO.md` and `PLAN.md`, so the
bookkeeping commits are produced and dropped mechanically rather than by hand".

The bookkeeping half is also read constantly. An agent hook that wants to know
which decision this branch implements, where the todo cache left off, or how
stale it is, pays that cost on every invocation.

## Status

This is a proposal that is **accepted**.

## Assumptions and Constraints

- `README.md` states "Not a tool ... this convention should cost only
  attention. CI checks and the agent skill are optional." Whatever ships must
  keep that true: a repo maintained entirely by hand stays conformant.
- The binary is called from agent hooks, repeatedly, so startup cost is a design
  constraint rather than an optimization.
- macOS arm64 only for now.
- This repo publishes an npm package (the skill and the Claude Code plugin) via
  semantic-release, pins tools with `mise`, and exposes every command through
  `just`.
- Generated markdown must pass `just check` unmodified: `prettier` with
  `proseWrap: preserve`, and `markdownlint-cli2` defaults with MD013 and MD041
  disabled.
- `.husky/commit-msg` runs commitlint, so any commit the tool creates has to go
  through a path that fires git hooks.
- Two sibling decisions are accepted and bind this one.
  [2026-09-05-freeze-a-decision-when-review-ends](./2026-09-05-freeze-a-decision-when-review-ends.md)
  fixes the record skeleton, the four states, the `## Errata` tail, and the
  drift check;
  [2026-09-05-replace-change-fragments-with-unreleased](./2026-09-05-replace-change-fragments-with-unreleased.md)
  fixes what `init` puts in `CHANGELOG.md` and what `release` does to it.
  Neither is implemented yet — the skill still carries the old skeleton and
  `.changes/` still exists — so clause 14 is what keeps v1 correct whichever
  lands first.

## Argument

`EVENTS.md` is already the specification of a command surface: every lifecycle
transition is `<artifact>: <verb>`. Naming the subcommands the same way
(**Chosen**) makes the event table the API reference, so a new event and a new
subcommand are the same edit, and a caller who knows the convention can guess
the command. A flat verb list (**Rejected**) loses that, and it invited two
commands that cannot be built correctly: a general `commit` makes the subject
the caller's choice rather than the event's, and a `purge` that deletes
`PLAN.md` and `TODO.md` together breaks the net-zero rule that makes those
commits droppable.

Reads through `gix`, writes through the `git` binary (**Chosen**) is the split
the constraints force. Reads are the hot path and must not spawn; writes must
fire `commit-msg`, or the tool could produce a subject its own commitlint
rejects — the one failure mode a lifecycle tool must not have.

The filesystem is authoritative (**Chosen**). The skill tells an agent to trust
the Charter's `## Artifacts` table because an agent would otherwise guess. A
tool does not guess, so it reads the tree and reports disagreement as a finding
rather than inheriting it as truth.

## Architectural Decision

1. A Rust binary, `condoc`, in a cargo workspace at the repo root:
   `crates/condoc`, with the logic in `lib.rs` and a thin `main.rs`, so tests
   drive the commands in-process. macOS arm64 first; linux amd64/arm64 and
   windows go on `ROADMAP.md`.
2. Subcommands mirror `EVENTS.md`, with `dec` aliasing `decision`:

   ```text
   condoc init                                      v1
   condoc new charter|design|roadmap                v1
   condoc new runbook <trigger>                     v1
   condoc new incident <slug>                       v1
   condoc dec draft <title>                         v1
   condoc dec propose <title> | <id>                v1
           [--extends <id>] [--supersedes <id>]
   condoc dec accept <id> | reject <id>             v1
   condoc dec errata <id> <text>                    v1

   condoc status                                    next
   condoc todo sync | clear                         next
   condoc plan start [<id>] | done                  next
   condoc lint | fix | check                        next
   condoc release <version>                         later
   condoc import | graduate <doc>                   later
   ```

3. `new` creates only the artifacts that have no lifecycle event: Charter,
   Design, Roadmap, runbooks, incidents. Decisions, Plans, and Todos are created
   by their own verbs, so the tool cannot produce a record whose event was never
   announced.
4. No configuration file. The filesystem is authoritative: probe root
   `UPPERCASE.md`, then `docs/<lowercase>.md`, per artifact. The Charter's
   `## Artifacts` table documents that state; disagreement is a `lint` finding.
5. Reads go through `gix` in-process. Writes exec the `git` binary, so hooks,
   signing, and `gitconfig` all apply.
6. Every mutating subcommand commits as
   `git commit -m "<subject>" -- <exact paths>` — the explicit pathspec form, so
   an unrelated staged change can never ride along. Never `add -A`, `add .`, or
   `commit -a`. `--no-commit` writes the files and leaves the tree dirty.
7. A decision id is `YYYY-MM-DD-<slug>`, the slug kebab-case from the title,
   capped at 71 characters. The cap is derived, not chosen: the longest subject
   that carries an id is `decision: propose` plus a space (18 characters) and
   `YYYY-MM-DD-` (11), leaving 71 of the 100-character Conventional Commits
   header. A same-date slug collision is an error; the convention forbids
   counters and suffixes.
8. `--extends` and `--supersedes` write two files in one commit: the new record,
   and the reciprocal line in the record it points at.
9. Status transitions are a state machine over draft, proposed, accepted, and
   rejected. `dec accept` and `dec reject` fail on a record that is not
   `proposed`; re-running a transition that already holds fails. Exit codes are
   0 for success, 1 for a convention violation or a bad state, 2 for a usage
   error, and 130 when an interactive prompt is interrupted.
10. `init` creates `CHARTER.md`, `DESIGN.md`, `ROADMAP.md`, and `CHANGELOG.md`.
    The changelog gets the shape its own decision fixes: the `# Changelog`
    heading and preamble with the format link pinned to `/en/2.0.0/`, an empty
    `## [Unreleased]`, and a reference link resolving it to a compare against
    `HEAD`. Never `EVENTS.md`, which is still proposed, and never `PLAN.md` or
    `TODO.md`, which belong to a branch.
    Re-running reports what is missing and creates nothing without `--add`. A
    missing `type-enum` entry in a repo's commitlint config is reported, never
    edited.
11. Read commands take `--json` for hook consumers; the default output is for
    people.
12. `just` gains `lint-rust` (`cargo fmt --check`, then
    `cargo clippy --all-targets -- -D warnings`) and `test-rust`
    (`cargo test`), both added to `check`; `format` gains `cargo fmt`.
13. v1 is clause 2's marked set. Then bookkeeping (`status`, `todo`, `plan`),
    then validation (`lint`, `fix`, `check`), then `release`, then `import` and
    `graduate`.
14. The tool emits the skeleton the convention specifies at the time it is
    built, and never a variant of its own: `dec propose` writes whatever
    `skills/conventional-docs/SKILL.md` then specifies, and `init` seeds
    `CHANGELOG.md` with whatever the convention then says a fresh changelog
    contains. A skeleton change is a convention change first and a `condoc`
    change second, never the reverse.
15. `dec errata` appends a dated line to a frozen record's `## Errata` tail. It
    is the only write the tool makes to a frozen record, and it never edits an
    existing line. `lint` carries the drift check: it resolves each record's
    freeze commit from the log and diffs everything above `## Errata` against
    the working copy, which is why the check lands with the validation slice and
    not with v1.
16. `plan start` requires an accepted decision, which is the convention's rule
    already: a Plan implements exactly one accepted decision. `plan start <id>`
    fails when that record is draft, proposed, or rejected.
17. `plan start` with no id lists the accepted decisions that have no plan yet —
    accepted, with no `plan: done <id>` in the log, and not cited by a line in
    `CHANGELOG.md` — as a numbered list, and reads one number from stdin.
    `--all` widens it to every accepted decision, including those a
    `plan: start` has already announced on another branch. A decision
    implemented with neither a Plan nor a user-facing changelog line stays on
    the list; nothing in the log distinguishes it from one nobody has started.
18. There is no cancel entry in that list. `SIGINT` exits 130 with nothing
    written and nothing committed. The prompt is shown only when stdin and
    stdout are both a terminal; with no id and no terminal the command exits 2
    instead of blocking, so a hook can never hang on it.
19. The list is a numbered prompt, not a full-screen picker. Arrow-key
    selection needs a terminal-handling dependency for one screen of output,
    and a numbered prompt also survives being run over a pipe or an ssh session
    with a dumb terminal.

## Consequences

The repo gains a second toolchain and a second release train: semantic-release
keeps publishing the npm package, and binaries ship from a tag job. `mise.toml`
pins a Rust version alongside bun.

`README.md`'s "Not a tool" sentence is amended to say that the convention costs
only attention and the binary is optional. The claim that sentence protects is
that a hand-maintained repo stays conformant, and clauses 4 and 6 preserve it:
the tool reads the tree rather than a state file it owns, and it writes nothing
a person could not have written.

The skill and the binary become two implementations of one specification and
have to change together. `lint` is what keeps them honest, which is an argument
for pulling it earlier than clause 13 schedules it if the two start to drift.

`release` cannot be built before its two dependency decisions are accepted, and
`import`/`graduate` need the numbered-log and `DECISIONS.md` migration rules
written as clauses before they are implementable. Neither is in v1.

Clauses 16 and 17 enforce a rule the convention already states, and enforcing
it exposes a gap: the Decision and Plan thresholds are independent, so work
that spans two sessions but changes under ~100 lines of behavior needs a Plan
and has no Decision to name. The Plan skeleton's `Decision:` field has no value
for that case. This decision takes the convention at its word and requires one;
closing the gap properly is a separate question for `README.md` and the skill.

## Positions

- **Extend the existing bun scripts.** _Rejected._ A node process per hook
  invocation is the cost the startup constraint exists to avoid, and the repo
  would ship lifecycle tooling that only runs where node does.
- **Exec `git` for everything.** _Rejected_ for reads, adopted for writes.
  Always correct and trivially simple, but it pays a process spawn on the path
  that hooks call most.
- **Use `gix` for writes too.** _Rejected._ A gitoxide or libgit2 commit does
  not run `commit-msg`, so the tool could silently write a subject its own
  commitlint would reject.
- **Hand-rolled `.git` parsing instead of `gix`.** _Rejected._ Fastest start and
  no dependency, but it puts packed-refs, worktree links, and bare repos on this
  project's maintenance bill for a saving that only matters once the read path
  is in the millisecond range.
- **Trust the Charter's `## Artifacts` table as the resolution source.**
  _Rejected._ That instruction exists so an agent does not guess; a tool stats
  the tree instead, which turns the drift into a finding rather than a fact.
- **A separate repository.** _Rejected._ The specification and its reference
  implementation would need coordinated bumps for every convention change.
- **Flat verbs, including `commit` and `purge`.** _Rejected._ See Argument;
  `purge` cannot be implemented without breaking the droppability rule.

## References

- [EVENTS.md](../../EVENTS.md) — the vocabulary the command surface mirrors.
- [ROADMAP.md](../../ROADMAP.md) — the paired-CLI item this answers.
- [gitoxide](https://github.com/GitoxideLabs/gitoxide) — the read layer.
