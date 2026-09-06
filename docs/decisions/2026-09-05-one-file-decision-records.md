# Give every decision its own file

## Issue

The convention offers two forms for the decision log: a single root
`DECISIONS.md` for small repos, and one file per decision under
`docs/decisions/` once the log outgrows it. Drafting the first entries in the
single-file form showed the cost. Every entry repeats the same section
headings, which markdownlint's MD024 reports as duplicate headings, so the form
only works if the lint gate is told to look the other way — friction pointed
against the convention it is supposed to serve. The graduation step is a
migration nobody benefits from, and the entry template — Context, Decision,
Consequences — never says where the motivating issue is, what constrained the
choice, what was rejected, or when the decision was published.

## Status

This is a proposal that is **accepted**.

## Assumptions and Constraints

- Decision records are append-only and are read one at a time, usually from a
  link in a commit, a PR, or another record.
- `docs/decisions/` is canonical; MADR defaults to it.
- `markdownlint-cli2` runs over `**/*.md` on every commit via
  `.husky/pre-commit`, with MD024 at its default setting, which flags duplicate
  headings anywhere in a document.
- The convention already has `docs/`-only artifacts — runbooks and incidents —
  so a decisions log that never lives at the root is not a new shape.

## Argument

One file per decision. **Chosen.** Each record's headings are unique inside its
own document, so a fuller skeleton costs nothing at the lint gate and no
configuration has to be relaxed. There is no graduation step, no splitting
migration, and a decision's path is the same on its first day as on its
thousandth. The single-file form bought one thing — a small repo keeping every
document at the root — and it is worth less than the friction it creates.

## Architectural Decision

1. A decision record is always `docs/decisions/YYYY-MM-DD-slug.md`. The
   single-file `DECISIONS.md` form is removed from the convention; the
   artifacts table shows `—` for its small-repo form, as it already does for
   runbooks and incidents.
2. Every record uses exactly this skeleton, with H2 sections in this order:

   ```markdown
   # <Decision title>

   ## Issue

   <The problem requiring a decision, with links to the motivating issue and
   PRs. If this decision extends another, the first sentence is `This decision
   extends [YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` and nothing from that
   decision is restated.>

   ## Status

   This is a proposal that is **awaiting review**.

   ## Assumptions and Constraints

   - <Facts bounding the choice: environment, compatibility guarantees, prior
     decisions. For a change to a public surface, state the compatibility
     guarantee it has to keep here.>

   ## Argument

   <Why the chosen direction beats the alternatives. Name any alternative that
   shapes the choice, with its verdict (**Chosen.** / **Rejected.**), and
   reserve full reasoning for Positions. `N/A` is acceptable when the
   constraints make the decision self-evident.>

   ## Architectural Decision

   <The decision itself, as numbered clauses a reviewer can point at. Include
   code or YAML only where it pins down a contract — a field name, a struct
   variant, one representative manifest — never to reproduce the
   implementation.>

   ## Positions

   <Alternatives considered and rejected, each with its reason, or `N/A`.>

   ## Dates

   - Published: TBD (set at merge).
   ```

3. Optional sections, in position: `## Consequences` after Architectural
   Decision and before Positions (rollout order, breaking changes, migration
   burden, follow-up documentation owed); `## References` before Dates
   (bulleted links with `—` descriptions: tracking issue, implementation PRs,
   related decisions, external specs).
4. H3 subsections are permitted inside Argument, Architectural Decision, and
   Positions when they improve skimmability.
5. Status is prose and moves with the commit events: `**awaiting review**` when
   proposed, `**accepted**` on merge with the Published date set,
   `**implemented**` — or `Implemented in [owner/repo#NN](<url>).` — once the
   change ships, `**superseded** by [YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md)`
   when replaced.
6. Material later edits append `- Updated: YYYY-MM-DD (<what changed>)` under
   Dates. A merged decision is never rewritten silently; it is extended by a
   new decision.
7. An extension decision opens Issue with `This decision extends
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).`, and in the same PR the extended
   decision gains `This decision is extended by
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` under its Issue heading. Prose
   references decisions as `[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md)`, relative.
8. Records carry the decision and its rationale only. Implementation diaries,
   debugging history, delivery staging, and local setup notes belong in PR
   descriptions. Another decision's content is linked, never copied. One
   decision per file when the parts can land independently.

## Consequences

A repo on the single-file form splits it in one commit: one file per entry,
dated from the entry's own date, inbound links rewritten, `DECISIONS.md`
deleted with no stub — the single file was never an id anyone cited.

Nothing in the markdown gate has to be relaxed: `.markdownlint-cli2.jsonc`
needs no MD024 exception. The link check does need widening, because
`linkinator` runs over an explicit glob list that has to include `docs/**/*.md`
for the relative cross-links between records to be checked.

## Positions

- **Keep both forms.** _Rejected._ Two shapes for one artifact means every
  reader and agent asks which one this repo uses, and the graduation path has
  to be specified and maintained.
- **Keep the single-file form and set MD024 to `siblings_only`.** _Rejected._
  It works, but it means the convention's first instruction to an adopting repo
  is to relax a lint rule.
- **Keep the thin Context/Decision/Consequences template.** _Rejected._ It
  records the outcome and drops the evidence: no motivating link, no
  constraints, no rejected options, no date.

## References

- [2026-09-05-identify-decisions-by-date](./2026-09-05-identify-decisions-by-date.md) — the id format these files use.
- [MADR](https://adr.github.io/madr/) — the template family this skeleton
  departs from, and the source of the `docs/decisions/` default path.

## Errata

- 2026-09-06: Clauses 2, 5, and 6 are superseded by
  [2026-09-05-freeze-a-decision-when-review-ends](./2026-09-05-freeze-a-decision-when-review-ends.md):
  the skeleton has no `## Dates` section, the status set is draft, proposed,
  accepted, and rejected, and a later correction is an `## Errata` line rather
  than an `- Updated:` line.
