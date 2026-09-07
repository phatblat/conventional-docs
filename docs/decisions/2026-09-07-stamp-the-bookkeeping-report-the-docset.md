# Stamp the bookkeeping, report the docset

## Issue

This decision extends
[2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact](./2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact.md).

The bookkeeping half is the most repetitive and the most-read part of the
convention — an agent hand-writes an ISO timestamp, a session id, a
`# Plan: <id>` heading, a `Decision:` link, and four commit subjects, and the
one rule that makes the bookkeeping disposable (a `plan:` or `todo:` commit
touches only its own artifact, `README.md:143-147`) is the easiest to break by
folding `TODO.md` into a commit that carries work. This branch's own history is
the worked example: `PLAN.md` was 167 lines with `###` groups inside
`## Steps` (commit `1d53`), `TODO.md` carried a session UUID and a `Synced`
stamp (commit `2cb0`), and both were deleted by `plan: done` / `todo: clear`
(`badc`, `b563`). Both `ROADMAP.md:20-22` and `ROADMAP.md:26-28` name this
work.

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- The extended record is accepted, so its body is frozen; its clause 4 surface
  and clauses 3, 5, and 9 stand and are not restated.
- From `2026-09-05-condoc-a-binary-for-the-document-lifecycle`, these clauses
  stay in force and bound this decision: 6 (explicit-pathspec commits, and
  `--no-commit` writes without committing), 9 (exit 0 success, 1 convention
  violation or bad state, 2 usage, 130 interrupted prompt), 11 (read commands
  take `--json`; default output is for people), 14 (the tool emits the
  skeleton the convention specifies, never a variant of its own), and 16–19
  (`plan start` requires an accepted decision; with no id it prints a numbered
  list of accepted decisions with no plan and reads one number from stdin; the
  prompt appears only when stdin and stdout are both terminals, else exit 2;
  `SIGINT` exits 130).
- The `PLAN.md` and `TODO.md` skeletons are already specified by the
  convention (`skills/conventional-docs/SKILL.md`,
  `site/content/artifacts/plan/index.md`,
  `site/content/artifacts/todo/index.md`); clause 14 makes them the tool's
  output, so this decision adds no skeleton of its own.
- `Ctx` injects `today: jiff::civil::Date` only
  (`crates/conventional-docs/src/ctx.rs:10-19`), a calendar date with no time
  of day, while `Synced` needs `YYYY-MM-DDTHH:MM:SSZ`.
- `artifact.rs` resolves Charter, Design, and Roadmap only
  (`crates/conventional-docs/src/artifact.rs`), while `status` has to resolve
  every artifact in `README.md`'s table.
- Generated markdown passes `just check` unmodified: `prettier` with
  `proseWrap: preserve`, `markdownlint-cli2` with MD013 and MD041 off.
- `.husky/commit-msg` runs commitlint, so every commit the tool makes goes
  through the `git` binary.
- Nothing is published: no crate, no released binary, no `doc` in the npm
  package, so the flags this decision adds are free to change.

## Argument

- _The tool stamps; the caller authors._ **Chosen.** `TODO.md` is "a cache of
  the list an agent is already keeping" and a Plan is engineering judgment, so
  the only parts a tool can own are the mechanical ones: the header fields,
  the skeleton, the subject, and the single-artifact commit. Generating step
  text from a decision's clauses (**Rejected**) would invent work the record
  never specified.
- _`status` reports facts, `lint` reports findings._ **Chosen.** The extended
  chain already draws this line — the filesystem is authoritative and a
  disagreeing Charter table is "a finding, not a fact" (base clause 4). A
  `status` that warns (**Rejected**) becomes a second `lint` with a different
  exit-code contract.
- _`--session` is the only source, and optional._ **Chosen.** A tool cannot
  discover an agent's session id; inventing one from the git author
  (**Rejected**) writes a value that is plausible and wrong. Optional now
  because no agent passes it yet; the skill teaches it, and `lint` gets the
  finding.

## Architectural Decision

1. The bookkeeping slice is these five commands, filling in the extended
   record's clause 4 `next` lines for `status`, `todo`, and `plan`:

   ```text
   doc status [--json]
   doc todo sync [--session <id>] [--stdin] [--skeleton]
   doc todo clear
   doc plan start [<id>] [--stdin] [--skeleton]
   doc plan done [--force]
   ```

2. The tool never authors step text. `todo sync` and `plan start` stamp, then
   commit, a file the caller wrote; `--stdin` replaces the `## Steps` block
   from checkbox lines read on stdin; `--skeleton` writes the convention's
   skeleton and never commits, whatever `--no-commit` says. Given an id,
   `plan start --skeleton` still fills in the real `# Plan: <id>` heading and
   `Decision:` link, the same as a commit would; without one, both are the
   literal placeholder text from the skeleton.
3. A missing artifact file with neither `--stdin` nor `--skeleton` is a
   convention violation (exit 1), and the message names both ways to create
   it. `--skeleton` on a file that already exists is also exit 1.
4. One step-parsing rule serves both files: inside the `## Steps` section, a
   step is a line beginning `- [ ]` or `- [x]` followed by a space, at column
   0; more-indented lines continue the step above them; `###` subsection
   headings are permitted and are not steps. It is what `status` counts and
   what `plan done` checks.
5. `todo sync` refreshes `Synced` on every write. `Session` comes from
   `--session <id>`; without it, the existing file's `Session` line is
   preserved; with neither, the line reads `unknown` and the command prints a
   hint on stderr naming `--session`. `## Notes` is never touched. The commit
   is `todo: sync` with `TODO.md` as the only pathspec.
6. `--session` is optional in this slice deliberately: no caller passes it
   yet, and a required flag would break every hook the moment the slice
   lands. The skill gains the guidance to pass it, and `Session: unknown`
   becomes a `lint` finding when the validation slice lands.
7. `todo clear` deletes `TODO.md` and commits `todo: clear`. A missing file is
   exit 1, never a silent success. There is no step precondition: a cache is
   dropped whatever state it is in.
8. `plan start` writes `PLAN.md` with `# Plan: <id>` and a `Decision:` link to
   `docs/decisions/<id>.md`, then commits `plan: start <id>` with `PLAN.md`
   as the only pathspec. An existing `PLAN.md` whose heading names a
   different id is exit 1 — one branch, one plan — and its `## Steps`,
   `## Verification`, and `## Status` bodies are left exactly as the caller
   wrote them.
9. `plan start --stdin` requires an explicit `<id>`, because base clause 17
   already spends stdin on the numbered prompt; the two cannot share it. `id`
   missing with `--stdin` is exit 2 unconditionally — a usage error, not a
   convention violation — whether or not a terminal is attached; with no id,
   no `--stdin`, and no terminal, clause 18's exit 2 already applies for the
   same reason: no interactive prompt is possible.
10. `plan done` takes the id from `PLAN.md`'s `# Plan: <id>` heading, never
    from the log: the file is the fact, and a rebase or squash must not
    rename the event. A missing `PLAN.md` is exit 1.
11. `plan done` refuses while unchecked steps remain, reporting the count,
    and `--force` is the override. The event asserts the steps are finished,
    so the default must not announce something the file contradicts.
12. Deletions commit through the same explicit-pathspec form as writes (base
    clause 6), so `plan: done` and `todo: clear` stay droppable and still
    fire `commit-msg`.
13. `doc status` reports facts and exits 0 whatever it finds. Its docset
    section lists, in `README.md`'s table order, Charter, Design, Decisions,
    Roadmap, Events, Runbooks, Incidents, and then Changelog, each with the
    path found — root or graduated — or `—` when absent, and Decisions
    carries counts by state. Its branch section names the current branch and
    reports `PLAN.md`'s decision id with steps done over total, and
    `TODO.md`'s `Session` and `Synced` values:

    ```text
    /Users/phatblat/dev/agents/conventional-docs  (feat/doc-bookkeeping)

      charter    CHARTER.md
      design     DESIGN.md
      decisions  docs/decisions  (10: 8 accepted, 1 proposed, 1 rejected)
      roadmap    ROADMAP.md
      events     EVENTS.md
      runbooks   —
      incidents  —
      changelog  CHANGELOG.md

      plan       PLAN.md  2026-09-07-stamp-the-bookkeeping-report-the-docset  0/9 steps
      todo       TODO.md  session 01a078c4-aafc-76b6-891b-d36e846641be  synced 2026-09-07T18:04:11Z
    ```

    The fields and their order are the contract; the exact column widths are
    the implementation's, pinned by a golden test.

14. `status` prints no staleness of its own: it prints `Synced` verbatim and
    lets the caller subtract. A duration formatter buys nothing a timestamp
    does not already answer.
15. `status --json` (base clause 11) prints one object with stable keys,
    absent files as `null`:

    ```json
    {
      "root": "/Users/phatblat/dev/agents/conventional-docs",
      "branch": "feat/doc-bookkeeping",
      "artifacts": [
        { "artifact": "charter", "state": "root", "path": "CHARTER.md" },
        {
          "artifact": "decisions",
          "state": "graduated",
          "path": "docs/decisions"
        },
        { "artifact": "runbooks", "state": "missing", "path": null }
      ],
      "decisions": {
        "draft": 0,
        "proposed": 1,
        "accepted": 8,
        "rejected": 1,
        "total": 10
      },
      "plan": {
        "path": "PLAN.md",
        "decision": "2026-09-07-stamp-the-bookkeeping-report-the-docset",
        "steps": { "done": 0, "total": 9 }
      },
      "todo": {
        "path": "TODO.md",
        "session": "01a078c4-aafc-76b6-891b-d36e846641be",
        "synced": "2026-09-07T18:04:11Z"
      }
    }
    ```

16. `status` needs a repository, since it reports the branch; outside one it
    fails like every other command.
17. `Ctx` gains an injected UTC timestamp beside `today`, and `Synced` is
    `YYYY-MM-DDTHH:MM:SSZ` at second precision. Tests pin it, the way they
    already pin `today`, so the goldens stay byte-stable.

## Consequences

`artifact.rs` grows from three artifacts to `README.md`'s whole table, which
the validation slice needs anyway; `Ctx` grows a clock, and the existing tests
pin it.

The two roadmap items are one piece of work. The `## Convention` paired-CLI
item (`ROADMAP.md:20-22`) is deleted and the `## doc` bookkeeping item
(`ROADMAP.md:26-28`) is checked off when the slice lands, not now.

`lint` inherits two findings this decision defines but does not implement:
`Session: unknown`, and a `PLAN.md` or `TODO.md` still present at merge.

The skill and the `/doc/` page gain the `--session` guidance and the five
commands when the slice lands; no skeleton changes, so clause 14 costs
nothing here.

`doc status` becomes the first read command, so it is the first place the
`--json` contract of base clause 11 is actually spelled out; `lint --json`
will follow its shape.

## Positions

- **Generate the Plan's steps from the decision's clauses.** _Rejected._ A
  clause is a contract, not a task list; the mapping is engineering judgment,
  and a tool that guesses it produces a plan nobody wrote.
- **`--steps <path>` instead of `--stdin`.** _Rejected._ A temp file to hand
  over a list the caller already holds; stdin is the pipe every hook already
  has.
- **Repeated `--step`/`--done` flags.** _Rejected._ Every wrapped,
  multi-line step becomes a quoting problem, and the flag order silently
  becomes the step order.
- **`$DOC_SESSION`, or the git author name, as a fallback `Session`.**
  _Rejected._ Both write a value that looks right and is wrong; the field's
  purpose is to tell a live cache from an abandoned one.
- **`--session` required.** _Rejected._ Correct destination, wrong slice: it
  would exit 2 for every caller that has not been taught the flag, on a file
  whose whole point is to be cheap to refresh.
- **`status` reporting findings — Charter-table drift, an uncleared Plan.**
  _Rejected._ `lint` owns findings and their exit codes; a warning in
  `status` splits that contract across two commands.
- **One command that clears both files.** _Rejected._ The base record
  already rejected `purge` for breaking the net-zero rule; `plan done` and
  `todo clear` stay separate commits for the same reason.
- **`plan done` deleting unconditionally.** _Rejected._ The subject asserts
  the steps are finished; `--force` is the honest way to say otherwise.
- **Taking `plan done`'s id from the log's `plan: start`.** _Rejected._ A
  rebase, a squash, or a cherry-pick would rename the event; the file in the
  tree is the fact.
- **Humanizing `Synced` into "4m ago".** _Rejected._ A duration formatter and
  its locale questions for information the timestamp already carries.

## References

- [2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact](./2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact.md) —
  the record this extends; its clause 4 surface is what this fills in.
- [2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md) —
  clauses 6, 9, 11, 14, and 16–19 stay in force.
- [2026-09-05-commit-the-session-todo](./2026-09-05-commit-the-session-todo.md) —
  the Todo's cache rules and its branch-scoped lifetime.
- [../../EVENTS.md](../../EVENTS.md) — the `plan:` and `todo:` subjects these
  commands write.
- [../../ROADMAP.md](../../ROADMAP.md) — the two items this answers.
