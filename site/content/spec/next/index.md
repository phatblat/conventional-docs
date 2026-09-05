---
title: 'next (draft)'
description: The normative Conventional Docs specification.
showStatus: true
specVersion: next
---

## Summary

Conventional Docs is a convention for where a repository's lifecycle
documentation lives, in markdown, in git. It defines a fixed set of artifacts —
Charter, Design, Decisions, Roadmap, Plan, Changes, Runbooks, Incidents, and
Todo — each with a small-repo path and a graduated path, so a human or an
agent can find the right document without asking. Two axes decide everything
else: how long a document stays true (its **lifetime**), and who is expected
to read it (its **audience**).

## The artifacts

| Artifact  | Small repo           | Graduated                           | Lifetime              | Answers                                               |
| --------- | -------------------- | ----------------------------------- | --------------------- | ----------------------------------------------------- |
| Charter   | `CHARTER.md`         | `docs/charter.md`                   | project               | why it exists, goals, route                           |
| Design    | `DESIGN.md`          | `docs/design.md`                    | living                | what the system is and does _now_                     |
| Decisions | `DECISIONS.md`       | `docs/decisions/NNNN-slug.md`       | append-only           | what changed, why, what it cost                       |
| Roadmap   | `ROADMAP.md`         | `docs/roadmap.md`                   | living                | what's next, in order                                 |
| Plan      | `PLAN.md`            | `docs/plan.md`                      | one branch / worktree | exact steps for the current decision                  |
| Changes   | `.changes/<slug>.md` | `.changes/<slug>.md`                | per-release           | what will ship in the next release, in plain language |
| Runbooks  | —                    | `docs/runbooks/<trigger>.md`        | living                | what to do when _x_ fires                             |
| Incidents | —                    | `docs/incidents/YYYY-MM-DD-slug.md` | append-only           | what broke, what we learned                           |
| Todo      | agent memory         | `docs/todo.md` (opt-in)             | one session           | where this session is                                 |

## Example

A conforming small repo:

```text
CHARTER.md
DESIGN.md
DECISIONS.md
ROADMAP.md
AGENTS.md
README.md
.changes/
  add-export-command.md
```

The same repository after graduating:

```text
docs/
  charter.md
  design.md
  decisions/
    0001-choose-storage-backend.md
    0002-drop-plugin-api.md
  roadmap.md
  runbooks/
    high-error-rate.md
  incidents/
    2026-03-02-billing-outage.md
AGENTS.md
README.md
.changes/
  add-export-command.md
```

`README.md`, `AGENTS.md`, and `.changes/` never move. Nothing exists at both
the small-repo path and the graduated path at once.

## Specification

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in
[RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

1. Every artifact MUST be a UTF-8 markdown file committed to the repository.
2. Every artifact MUST exist at exactly one of its two defined paths — root
   form or graduated form — and MUST NOT exist at both.
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
8. A conforming repository MUST have a decision log: one `DECISIONS.md`, or
   one file per decision at `docs/decisions/NNNN-slug.md` with a zero-padded
   four-digit sequence number that MUST NOT be reused.
9. Every decision MUST record a status of `proposed`, `accepted`, `rejected`,
   or `superseded`, and MUST record context, decision, and consequences.
10. A `proposed` decision MAY be edited freely; an `accepted` decision MUST
    NOT be edited except to set its status to `superseded` and reference the
    decision that supersedes it.
11. Reversing an accepted decision MUST be recorded as a new decision that
    references the one it supersedes.
12. A change that alters behavior, a published interface, or a dependency
    SHOULD have an accepted decision before it merges.
13. A Roadmap at `ROADMAP.md` or `docs/roadmap.md` is OPTIONAL; when present
    it MUST list intended work in intended order, and items that no longer
    apply MUST be deleted rather than archived in place.
14. A Plan at `PLAN.md` or `docs/plan.md` is OPTIONAL; when present it MUST
    correspond to exactly one accepted decision, MUST contain the ordered
    steps to execute it, and MUST be deleted no later than the merge of the
    work it describes.
15. Work spanning more than one working session, or handed to another person
    or agent, SHOULD have a Plan.
16. Every user-facing change MUST add a release-note fragment at
    `.changes/<slug>.md` in the same commit or pull request as the change.
17. Each line of a fragment MUST be a markdown unordered list item beginning
    with one of `Added:`, `Changed:`, `Deprecated:`, `Removed:`, `Fixed:`, or
    `Security:`.
18. At release time, fragments MUST be folded into the release notes and
    `CHANGELOG.md`, and the consumed files MUST be deleted in the same commit
    as the version bump.
19. `CHANGELOG.md` MUST follow [Keep a Changelog](https://keepachangelog.com/)
    and MUST NOT be hand-edited.
20. Runbooks are OPTIONAL; when present they MUST live at
    `docs/runbooks/<trigger>.md`, one file per trigger.
21. Incident records are OPTIONAL; when present they MUST live at
    `docs/incidents/YYYY-MM-DD-slug.md` and MUST NOT be rewritten after the
    incident closes, except to append follow-up references.
22. A session Todo is OPTIONAL; when committed it MUST live at `docs/todo.md`,
    and consumers MUST NOT treat it as durable state.
23. An artifact SHOULD graduate from root form to graduated form when the
    repository root has become crowded with top-level files, or when the
    artifact needs siblings, per-item status, or internal structure.
24. A graduation MUST move the file, rewrite every inbound link, and update
    the Charter's `## Artifacts` section in a single commit.
25. `docs/decisions/` is the canonical graduated location for the decision
    log; a repository using `adr-tools` MUST add a root `.adr-dir` file
    containing `docs/decisions`.
26. `AGENTS.md` is the canonical agent instruction file. A tool-specific
    instruction file MUST be a real committed file that includes `AGENTS.md`
    by reference, MUST NOT duplicate its content, and MUST NOT be a symlink.
27. A repository that publishes lifecycle events MUST publish them as
    Conventional Commits with the types `decision`, `plan`, `release`,
    `deploy` and these subject forms: `decision: propose NNNN <title>`,
    `decision: accept NNNN`, `decision: reject NNNN`,
    `decision: implement NNNN (#<pr>)`, `plan: start NNNN`, `plan: done NNNN`,
    `release: v<semver>`, `deploy: <environment> v<semver>`.
28. The commit MUST be the event of record; a notification MAY point at it
    and MUST NOT carry state that is absent from the repository.
29. An artifact MAY graduate out of the repository into an external system;
    the Charter's `## Artifacts` section MUST then record the external
    location and the stale in-repo file MUST be deleted.
30. A repository conforms when every MUST above holds for the artifacts it
    has. Tooling SHOULD report SHOULD violations as warnings and MUST NOT
    reject a repository for them alone.

## Why

- **Cold start.** A human or an agent opening the repository for the first
  time knows exactly where to look, without asking.
- **Portability.** The convention is markdown in git — it survives a change of
  issue tracker, wiki, or coding agent.
- **Review with the code.** A decision, a plan, and a change fragment go
  through the same pull request as the code they describe.
- **A decision log that survives turnover.** Why something was built the way
  it was outlives the person who built it.
- **Release notes written while the change is fresh**, not reconstructed from
  commit messages at release time.

## FAQ

See the [FAQ](../../faq/index.md) for answers to common adoption questions.
