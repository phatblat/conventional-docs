# Identify decisions by date

## Issue

A decision record is identified by a 4-digit sequential number
(`docs/decisions/NNNN-slug.md`, `decision: accept 0007`). The number is
allocated when the record is written but has to be unique when it merges, so
two branches opened the same week both take `0008` and one gets renumbered —
after its id is already in commit subjects, PR titles, plan headings, and
links. The number also implies a relationship it cannot have: two records
sharing a number read as one decision recorded twice.

## Status

This is a proposal that is **accepted**.

## Assumptions and Constraints

- Nothing has been released from this repository yet, so no adopter has a
  numbered log to migrate.
- `docs/decisions/` stays the canonical location: MADR defaults to it, and
  `adr-tools` can be pointed at it with a root `.adr-dir` file.
- Decision ids are cited outside the repository — commit subjects, PR titles,
  issue threads, other repos' records — where a rename cannot reach them.
- The convention already dates its other append-only log,
  `docs/incidents/YYYY-MM-DD-slug.md`.

## Argument

A date is not allocated, so it cannot race. **Chosen.** Sequential numbers are
the ADR default and every ADR tool understands them, but they make the id a
shared mutable resource: unique at merge, chosen at write time. Two decisions
dated the same day is a normal thing to say and implies nothing about them,
while two decisions sharing a number read as the same decision. A date also
keeps a lexical sort of the directory chronological, which is what the number
was buying.

## Architectural Decision

1. A decision's id is `YYYY-MM-DD-slug`: the date the record was written plus a
   kebab-case slug of its title. Its file is
   `docs/decisions/YYYY-MM-DD-slug.md`, and the id is that filename without
   `.md`.
2. The id is fixed at creation. It is never re-dated, renumbered, or renamed —
   not when the status changes, not when a later decision supersedes it.
3. Several decisions may share a date; their slugs tell them apart. No counter,
   no suffix, no renumbering to settle a merge.
4. Commit events carry the whole id and no separate title:
   `decision: accept 2026-09-05-identify-decisions-by-date`. The slug is the
   title, so the subject stays inside Conventional Commits' 100-character
   header limit.
5. `adr new` is not used, because it allocates the next sequential number.
   Records are copied from the skeleton instead.
6. A repo adopting this convention with a numbered log renames every record to
   its date in one commit, taking the date from the record's published date or
   from the date the file was added (`git log --diff-filter=A --format=%ad --date=short -1 -- <file>`),
   rewrites inbound links in the same commit, and leaves a redirect at each old
   filename:

   ```markdown
   # Moved

   Moved to [@2026-02-11-split-the-scheduler.md](2026-02-11-split-the-scheduler.md).
   ```

## Consequences

Those redirects are the one place this convention keeps a file at an old path;
everywhere else it forbids stubs and mirrors. They are redirects, never
content, and they stay, because the log is append-only. They are files, not
symlinks: GitHub renders a symlink blob as its target path, and several
consumers of a repo never follow one.

MADR
[sanctions other filename patterns](https://github.com/joelparkerhenderson/architecture-decision-record#file-name-conventions-for-adrs)
with the caveat that some existing tooling stops applying. Here that is exactly
`adr new`; `adr list` and `adr generate` still work off `.adr-dir`.

## Positions

- **Keep 4-digit sequential numbers.** _Rejected._ The race is structural, not
  a discipline problem.
- **Number, with a suffix on collision** (`0008a`). _Rejected._ Keeps the
  allocation problem and adds a second form to parse.
- **Date plus a per-day counter** (`2026-09-05-01-slug`). _Rejected._ Restores
  an allocation race inside the day for no gain; the slug already
  disambiguates.
