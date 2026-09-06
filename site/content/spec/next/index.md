---
title: 'next (draft)'
description: The normative Conventional Docs specification.
showStatus: true
specVersion: next
---

## Summary

Conventional Docs is a convention for where a repository's lifecycle
documentation lives, in markdown, in git. It defines a fixed set of artifacts —
Charter, Design, Decisions, Roadmap, Plan, Events, Runbooks, Incidents, and
Todo — each at a defined path, some with both a small-repo and a graduated
form, so a human or an agent can find the right document without asking. Two
axes decide everything else: how long a document stays true (its
**lifetime**), and who is expected to read it (its **audience**).

See [Artifacts](../../artifacts/index.md) for the full table of small-repo
and graduated paths, lifetimes, and answers, and
[Example](../../example/index.md) for a conforming repository, small and
graduated.

## Specification

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in
[RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

1. Every artifact MUST be a UTF-8 markdown file committed to the repository.
2. Every artifact MUST exist at exactly one of the paths this specification
   assigns it, and MUST NOT exist at both a small-repo and a graduated path
   where both are defined.
3. A repository MUST NOT commit a stub, pointer file, or symlink at an
   artifact's vacated path.
4. `README.md`, the license file, `CHANGELOG.md`, and `AGENTS.md` MUST stay at
   the repository root and MUST NOT move into `docs/`.
5. A conforming repository MUST have a Charter at `CHARTER.md` or
   `docs/charter.md` stating why the project exists, who it is for, and what
   is out of scope.
6. The Charter MUST contain an `## Artifacts` section recording the current
   location of every artifact the repository uses.
7. A conforming repository MUST have a Design document at `DESIGN.md` or
   `docs/design.md` describing the system as it exists now; it MUST NOT
   describe planned or hypothetical behavior.
8. A conforming repository's decision log is one file per decision at
   `docs/decisions/YYYY-MM-DD-slug.md` — the date the record was written plus
   a kebab-case slug of its title. The id MUST be fixed at creation and MUST
   NOT be re-dated, renumbered, or renamed; several decisions MAY share a
   date, distinguished by slug.
9. Every decision record MUST use these H2 sections, in this order: Issue,
   Status, Assumptions and Constraints, Argument, Architectural Decision,
   Positions. `## Consequences` MAY appear between Architectural Decision
   and Positions, `## References` MAY appear after Positions, and
   `## Errata` MAY appear last. A record MUST NOT carry a `## Dates`
   section.
10. A decision record MUST be in exactly one of four states — draft,
    proposed, accepted, rejected — and MUST move through them in one
    direction: draft → proposed → accepted | rejected. Each transition MUST
    be a commit. `## Status` MUST carry exactly one line: `This is a
**draft**; it is not ready for review.`, `This is a proposal that is
**awaiting review**.`, `This is a proposal that is **accepted**.`, or
    `This proposal was **rejected**.`
11. `decision: accept` or `decision: reject` MUST be the last write to a
    record's body. A frozen record MAY be corrected only by appending a
    dated line to `## Errata`, which MUST hold only a correction that leaves
    the decision unchanged or a pointer to a record that supersedes or
    extends it. Reversing an accepted decision MUST be recorded as a new
    decision that supersedes it, stated in the superseding record's Issue
    and as an erratum on the superseded record; neither record's status
    changes. A rejected record MUST remain in the log.
12. A change that alters behavior, a published interface, or a dependency
    SHOULD have an accepted decision before it merges.
13. A Roadmap at `ROADMAP.md` or `docs/roadmap.md` is OPTIONAL; when present
    it MUST list intended work in intended order, and items that no longer
    apply MUST be deleted rather than archived in place.
14. A Plan at `PLAN.md` is OPTIONAL and has no graduated form; when present it
    MUST correspond to exactly one accepted decision, MUST contain the
    ordered steps to execute it, and MUST be deleted no later than the merge
    of the work it describes.
15. Work spanning more than one working session, or handed to another person
    or agent, SHOULD have a Plan.
16. A Todo at `TODO.md` is OPTIONAL and has no graduated form; when an agent
    or contributor is tracking a working list for the branch, `TODO.md` MUST
    be committed and refreshed at each checkpoint, MUST record the session
    identity and the UTC time it was last synced, and MUST be deleted no
    later than the merge of the work it describes. Consumers MUST NOT treat
    it as durable state beyond the branch.
17. A notable user-facing change SHOULD add its line to `CHANGELOG.md`'s
    `## [Unreleased]` section in the same commit or pull request as the
    change. Notability is a judgment: nothing MUST require a changelog
    edit on a change.
18. `CHANGELOG.md` MUST follow
    [Keep a Changelog 2.0.0](https://keepachangelog.com/en/2.0.0/): a
    `# Changelog` heading with its preamble, `## [Unreleased]` first,
    released versions as `## [x.y.z] - YYYY-MM-DD` newest first, the six
    categories as `###` subsections, `**Breaking:**` markers inside the
    category they belong to, and reference-style links resolving each
    version to a compare diff, with `[Unreleased]` comparing the newest
    tag to `HEAD`.
19. A released section MUST NOT be edited after its release, and a version
    heading or version field MUST NOT be hand-bumped. The `release:` event
    MUST rename `[Unreleased]` to the new version in both the heading and
    its reference link, and MUST open a fresh empty `[Unreleased]`.
20. A changelog line MAY cite the decision id it came from.
21. Runbooks are OPTIONAL; when present they MUST live at
    `docs/runbooks/<trigger>.md`, one file per trigger.
22. Incident records are OPTIONAL; when present they MUST live at
    `docs/incidents/YYYY-MM-DD-slug.md` and MUST NOT be rewritten after the
    incident closes, except to append follow-up references.
23. An artifact SHOULD graduate from root form to graduated form when the
    repository root has become crowded with top-level files, or when the
    artifact needs siblings, per-item status, or internal structure.
24. A graduation MUST move the file, rewrite every inbound link, and update
    the Charter's `## Artifacts` section in a single commit.
25. `docs/decisions/` is the only location for decision records; a repository
    using `adr-tools` MUST add a root `.adr-dir` file containing
    `docs/decisions`.
26. `AGENTS.md` is the canonical agent instruction file. A tool-specific
    instruction file MUST be a real committed file that includes `AGENTS.md`
    by reference, MUST NOT duplicate its content, and MUST NOT be a symlink.
27. A repository that publishes lifecycle events MUST publish them as
    Conventional Commits with the types `decision`, `plan`, `todo`,
    `release`, `deploy` and these subject forms: `decision: draft <id>`,
    `decision: propose <id>`, `decision: accept <id>`,
    `decision: reject <id>`, `plan: start <id>`, `plan: done <id>`,
    `todo: sync`, `todo: clear`, `release: v<semver>`,
    `deploy: <environment> v<semver>`, where `<id>` is `YYYY-MM-DD-slug`.
28. A `plan:` or `todo:` commit MUST touch only its own artifact, so that
    dropping or squashing those commits leaves the tree unchanged.
29. The commit MUST be the event of record; a notification MAY point at it
    and MUST NOT carry state that is absent from the repository.
30. An artifact MAY graduate out of the repository into an external system;
    the Charter's `## Artifacts` section MUST then record the external
    location and the stale in-repo file MUST be deleted.
31. A repository conforms when every MUST above holds for the artifacts it
    has. Tooling SHOULD report SHOULD violations as warnings and MUST NOT
    reject a repository for them alone.

## Why

- **Cold start.** A human or an agent opening the repository for the first
  time knows exactly where to look, without asking.
- **Portability.** The convention is markdown in git — it survives a change of
  issue tracker, wiki, or coding agent.
- **Review with the code.** A decision, a plan, and a changelog line go
  through the same pull request as the code they describe.
- **A decision log that survives turnover.** Why something was built the way
  it was outlives the person who built it.
- **Release notes written while the change is fresh**, not reconstructed from
  commit messages at release time.

## FAQ

See the [FAQ](../../faq/index.md) for answers to common adoption questions.
