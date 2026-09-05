# Give the event vocabulary its own artifact

## Issue

The lifecycle event vocabulary — `decision:`, `plan:`, `release:`, `deploy:` —
is specified in a `## Events` section of `README.md`, which is also the
convention's overview, its rationale, and its install instructions. An adopting
repo has nowhere predictable to document the events it announces, even though
predictable paths are this convention's whole premise, and the machine-readable
half of the vocabulary, `commitlint.config.js`, cites a heading anchor inside a
README that keeps growing.

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- The vocabulary has two halves that must agree: prose for humans and agents,
  and `type-enum` in a repo's commitlint config, which is what actually rejects
  a bad subject.
- There is no standard filename to adopt. GitHub recognizes `CONTRIBUTING.md`
  for contributor guidance and resolves it from the root, `.github/`, or
  `docs/`; `vuejs/core` keeps a `.github/commit-convention.md`; `COMMITS.md`
  and `CONVENTIONS.md` appear in the wild with nothing specifying them.
- Those files hold contributor etiquette. This vocabulary is spec: hooks,
  dashboards, and release tooling key off it.
- `README.md`, `LICENSE`, `CHANGELOG.md`, and `AGENTS.md` are the only files
  this convention pins to the root permanently; anything else may graduate.

## Argument

A named artifact of its own. **Chosen.** The convention's answer to "where does
this kind of document live" is always a named artifact with a lifetime and a
graduated path, and the event vocabulary is a document of that kind: it is
read by contributors, agents, and dashboards, and it changes as a repo adds
event types. Putting it in a contributor guide (**Rejected**, see Positions)
would make the convention's own spec a subsection of etiquette, and leaving it
in `README.md` gives adopters no path at all.

### Open before acceptance

- Whether an adopting repo needs a file of its own at all. The vocabulary is
  fixed by this convention, so a repo that announces no types of its own has
  nothing to write down that `type-enum` does not already say.
- Whether `docs/events.md` is the right graduated path, or whether the
  vocabulary belongs next to the decisions it announces.
- Whether a repo's own event types belong here or in `CONTRIBUTING.md`
  alongside the rest of its commit etiquette.

## Architectural Decision

1. `EVENTS.md`, graduating to `docs/events.md`, lifetime living, joins the
   artifacts table between Changes and Runbooks, answering "which lifecycle
   events the repo's commits announce".
2. It carries the one-paragraph rationale, a fenced block of example subjects,
   one line per event saying when it fires, and the `type-enum` list that
   enforces them.
3. `README.md` keeps the loop and a single pointer to `EVENTS.md`; its
   `## Events` section is deleted.
4. `commitlint.config.js` and `AGENTS.md` cite `EVENTS.md` instead of a README
   anchor.
5. A repo that announces its own events adds them to `EVENTS.md` and to
   `type-enum` in the same commit.

## Consequences

The artifact set grows to ten. The vocabulary now lives in three places —
`EVENTS.md`, the `conventional-docs` skill, and `type-enum` — which change
together; `EVENTS.md` is the human-facing source and `type-enum` is what
rejects a bad subject. `README.md` shrinks toward an overview and an index of
the artifacts.

Until this decision is accepted, the artifact is marked proposed everywhere it
appears: `EVENTS.md` opens with a status blockquote, the artifacts table
carries a note, and the `conventional-docs` skill reads an existing
`EVENTS.md` but does not create one.

## Positions

- **`CONTRIBUTING.md`.** _Rejected._ GitHub-recognized and auto-linked from the
  issue and PR composer, which makes it the right home for contributor
  etiquette — but this is the convention's spec, and a spec inside a
  contributor guide is hard to cite and easy to bury.
- **`.github/commit-convention.md`.** _Rejected._ Real precedent, and it keeps
  the root clean, but `.github/` is where tooling config hides; nothing
  surfaces it to a reader.
- **`COMMITS.md`.** _Rejected._ Guessable, and some agent tooling reads the
  name, but it frames the content as commit-message formatting rather than the
  lifecycle events those commits announce.
- **Leave it in `README.md`.** _Rejected._ Adopters get no predictable path for
  their own vocabulary, and the config keeps pointing at a heading anchor.

## References

- [Conventional Commits](https://www.conventionalcommits.org/) — the type
  syntax these events extend.
- [vuejs/core commit convention](https://github.com/vuejs/core/blob/main/.github/commit-convention.md) — the precedent weighed in Positions.

## Dates

- Published: TBD (set at merge).
