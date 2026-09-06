# Rename condoc to doc and group commands by artifact

## Issue

This decision extends
[2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md).

`condoc` is a contraction of the convention's name, so the binary names the
convention rather than the documents it writes, and nothing about it can be
guessed: a reader has to be told what `condoc` is before the surface means
anything. The convention's own noun is the document.

The surface has two shapes as well. Decisions are noun-first — `dec propose`,
mirroring `EVENTS.md`'s `<artifact>: <verb>` — while the artifacts with no
lifecycle event are verb-first under a `new` group, as `new charter`. Learning
one shape does not predict the other, and the two vaguest planned commands,
`import` and `graduate <doc>`, belong to neither.

Nothing has shipped under the old name: no `condoc` crate on crates.io, no
binary in the npm package, no published binaries, and the `CHANGELOG.md` line
announcing the tool is still under `## [Unreleased]`. The name and the shape
are both free to change now and expensive to change later.

## Status

This is a proposal that is **awaiting review**.

## Assumptions and Constraints

- The extended record is accepted, so its body is frozen. This decision
  replaces its clauses 1 and 2; clauses 3 through 19 stay in force and are not
  restated.
- That record keeps its id and its title. An id is fixed at creation and is
  never re-dated or renamed
  ([2026-09-05-identify-decisions-by-date](./2026-09-05-identify-decisions-by-date.md)),
  so the log keeps a slug that names a binary which no longer exists.
- Nothing is published under the old name — no crate, no binary in the npm
  package (it ships `*.md`, `.claude-plugin`, and `skills`), and no released
  binaries — so there is no compatibility surface to preserve.
- `doc` is unavailable as a package name: crates.io has held it since 2018 at
  `0.0.0`, and npm has `doc@0.0.3`. Cargo lets `[[bin]] name` differ from the
  package name, so neither registry constrains what a caller types. Homebrew
  has no `doc` formula or cask.
- `doc` is short enough that a user may already have a command or a shell alias
  by that name; nothing the project ships can decide `$PATH` order for them.
- v1 is built — `init`, the `new` group, and the `dec` verbs, with tests that
  drive the parser in-process — so a change to the surface rewrites code, not
  only a specification.
- The site is not published: `.github/workflows/pages.yml` runs on
  `workflow_dispatch` only, and enabling Pages is still an open `ROADMAP.md`
  item. No page URL is a citation yet.

## Argument

Name the tool after what it operates on. **Chosen.** `doc` is the shortest true
name for a tool whose every command reads or writes one of the convention's
documents, and it is the word the convention itself uses. Keeping `condoc`
(**Rejected**) trades a contraction in every invocation, help line, and
documentation page for protection against a collision the project can survive:
the name is reserved nowhere that matters, and where it does collide the user's
own `$PATH` already decides.

`doc` invites the reading "documentation generator", which this convention
explicitly disclaims — user-facing docs are Diátaxis's territory. The
subcommands are what settle it: `doc decision propose`, `doc plan start`,
`doc todo sync`. A documentation generator has no `decision propose`. The first
token names the object, and the two after it name the artifact and its
lifecycle verb.

One shape for the whole surface. **Chosen.** Once the first token stops naming
the convention, the tokens after it carry all of the meaning, and they should
spell the event the convention already names: `doc <artifact> <verb>` for
anything scoped to one artifact, `doc <verb>` for the docset. Keeping the `new`
group (**Rejected**) encodes a real fact — that Charter, Design, Roadmap,
runbooks, and incidents have no lifecycle event — in the position of a token,
where only the tool's authors read it. Noun-first puts the artifact where a
caller already looks and leaves the verb to say whether an event is announced,
and it gives the two planned commands that had no home a natural one:
`doc decision import` and `doc design graduate`.

## Architectural Decision

1. The binary is `doc`. This replaces the name in clause 1; the rest of that
   clause stands — a Rust binary in a cargo workspace at the repo root, the
   logic in `lib.rs` behind a thin `main.rs`, macOS arm64 first.
2. The cargo package is `conventional-docs`, in `crates/conventional-docs`,
   with `[[bin]] name = "doc"` and the library as `conventional_docs`. It
   matches this repo's npm package name, and `cargo install conventional-docs`
   installs `doc`.
3. A command scoped to exactly one artifact is `doc <artifact> <verb>`, the
   artifact singular and spelled as the convention names it. A command that
   acts on the whole docset, or announces a repo-wide event, is `doc <verb>`.
4. The surface, replacing clause 2's, with the slice each command belongs to:

   ```text
   doc init [--add]                                v1
   doc charter new                                 v1
   doc design new                                  v1
   doc roadmap new                                 v1
   doc runbook new <trigger>                       v1
   doc incident new <slug>                         v1
   doc decision draft <title>                      v1
   doc decision propose <title> | <id>             v1
           [--extends <id>] [--supersedes <id>]
   doc decision accept <id> | reject <id>          v1
   doc decision errata <id> <text>                 v1

   doc status                                      next
   doc todo sync | clear                           next
   doc plan start [<id>] | done                    next
   doc lint | fix | check                          next

   doc release <version>                           later
   doc decision import                             later
   doc <artifact> graduate                         later
   ```

5. `dec` aliases `decision` and is the only alias in the surface; no other
   artifact gets an abbreviation.
6. `new` is a verb on each artifact that has no lifecycle event, never a group.
   Clause 3 of the extended record is unchanged, and the distinction it draws is
   now carried by which verbs an artifact has: `charter`, `design`, `roadmap`,
   `runbook`, and `incident` have `new` and no event verbs, while `decision`,
   `plan`, and `todo` have event verbs and no `new`.
7. `release` stays a top-level verb: `release: v<version>` names no artifact in
   `EVENTS.md`, and the version it announces is the repo's. The same holds for
   `deploy`, if it is ever built.
8. `graduate` is a verb on each artifact that can graduate, and `import` a verb
   on decisions, replacing clause 2's `import | graduate <doc>`. Their
   migration rules are still owed, as the extended record says.
9. An artifact group invoked with no verb is a usage error: it prints that
   group's help and exits 2, which is clause 9's code for a usage error. What
   one artifact's state is remains `doc status`'s answer.
10. Nothing is kept for the old name: no `condoc` alias, no shim, no second
    binary, and `crates/condoc` is renamed rather than copied. The
    `[Unreleased]` line that announces the tool is edited in place rather than
    answered with a `Changed` line, because the tool it announces has not been
    released.
11. The site page moves from `/condoc/` to `/doc/` with no redirect, since the
    site is not published, and the nav entries in `site/hugo.yaml` rename with
    it.
12. The reciprocal pointer this extension owes the extended record is an
    erratum, and it is written in the commit that ends this record's review, not
    in the one that proposes it. A frozen record can only be appended to, and an
    erratum is never edited afterward, so a pointer written at propose time
    would assert an extension review has not granted, and could not be corrected
    if this record were rejected.

## Consequences

One pass renames everything that names the tool: the workspace member and the
crate directory, the package and library names, `#[command(name = ...)]`, the
`condoc:` prefix on error output, doc comments in `artifact.rs`, `error.rs`,
and `git.rs`, the test helpers' argv and git identity, `README.md`'s "Not a
tool" line, `ROADMAP.md`'s `## condoc` section, two `[Unreleased]` lines, and
the site's page, nav, homepage, FAQ, quickstart, and events references.

The reshape rewrites v1's parser and its tests: the `new` group dissolves into
per-artifact groups, `dec` becomes an alias of a `decision` group, and every
test argv changes shape. No path, template, or commit subject changes with it,
so the golden files and the commit-subject assertions stand as written.

`doc` will collide for some users, whether with an alias, another tool, or a
package that takes the name later. The project ships one name and no fallback;
a user who needs another renames or aliases the installed binary.

The convention does not say when a frozen record's reciprocal erratum is
written. Clause 12 answers it for this record; the general question is owed to
`README.md` and the skill.

The unbuilt slices are specified here in their final shape, so bookkeeping,
validation, and `release` are built noun-first and pay nothing for the change.

## Positions

- **Keep `condoc`.** _Rejected._ Unambiguous and already written down, but it
  makes every caller type a contraction to avoid a collision the project can
  survive, and it names the convention instead of the object.
- **`cdoc`, `condocs`, or `conventional-docs` as the binary.** _Rejected._ A
  shorter contraction is still a contraction, and the full name is worse to type
  than `condoc`; it is the package name instead.
- **`docs`, plural.** _Rejected._ Every command acts on one artifact, and
  `docs/` is a directory this convention already owns, so the plural reads like
  an operation on that directory.
- **Package the crate as `doc`.** _Rejected._ crates.io has held `doc` at
  `0.0.0` since 2018, so the package could never be published under it, and a
  package name is not what a caller types.
- **Keep the `new` group.** _Rejected._ See the Argument.
- **Bare `doc <artifact>` prints that artifact's state.** _Rejected._ It reads
  well and would be cheap, but it splits the status answer across every artifact
  group and `doc status`, and it makes an incomplete command do work instead of
  failing.
- **A `condoc` shim, alias, or deprecation window.** _Rejected._ Nothing shipped
  under the old name, so the shim would serve muscle memory only.
- **Two records, one for the name and one for the shape.** _Rejected._ Both
  replace the same two clauses of the same accepted record, and the surface has
  to be restated with the new name either way; two records restate it twice.

## References

- [2026-09-05-condoc-a-binary-for-the-document-lifecycle](./2026-09-05-condoc-a-binary-for-the-document-lifecycle.md)
  — the record this extends; clauses 3 through 19 stand.
- [2026-09-05-identify-decisions-by-date](./2026-09-05-identify-decisions-by-date.md)
  — why the extended record keeps an id that names the old binary.
- [EVENTS.md](../../EVENTS.md) — the `<type>: <verb>` vocabulary the artifact
  groups mirror.
