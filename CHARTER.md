# Charter

## Why this exists

Software moves through phases — intent, proposal, planning, implementation,
integration, release, operation — and each handoff loses the reasoning that
produced it. The next actor reconstructs intent from chat logs, a ticket
someone closed, or a diff. That reconstruction cost is paid by everyone
downstream, and it is paid again by every new actor: a new teammate, a fresh
agent session, a reviewer, a release tool. Conventional Docs makes each phase
hand off a durable artifact: a file at a predictable path with a declared
lifetime, so the next phase starts from a document instead of an excavation.
Because the paths and lifetimes are the same in every adopting repo, a reader
needs no instructions for this repo: knowing the convention plus having a
clone is enough.

## Goals

- **Discoverable from a clone alone** — no service to query, no credentials, no
  onboarding document that explains where things are.
- **Cold start** — any actor (human, agent, or tool) can pick up in-flight work
  from the artifacts on the branch.
- **Deterministic scope** — the phase's own artifacts state what that phase
  touches, so a gate can check it mechanically instead of by judgment.
- **Context locality** — work-relevant context lives on the branch where the
  work happens and is read lazily, at the moment it matters.
- **Costs attention, not tooling** — a project adopts this by writing files;
  validators, adapters, and the agent skill are optional.

## The pattern

Each SDLC phase ends by writing the artifact the next phase reads.

| SDLC phase      | What the phase hands off                   | Artifact            |
| --------------- | ------------------------------------------ | ------------------- |
| Inception       | why this exists, what it is for            | Charter             |
| Prioritization  | what is next, in order                     | Roadmap             |
| Proposal        | what should change, why, and what it costs | Decision (proposed) |
| Review          | the frozen spec                            | Decision (accepted) |
| Planning        | the exact steps and the files they touch   | Plan                |
| Implementation  | where this session left off                | Todo                |
| Integration     | what the system is and does now            | Design              |
| Release         | what shipped, in plain language            | CHANGELOG.md        |
| Operation       | what to do when _x_ fires                  | Runbook             |
| Incident review | what broke and what was learned            | Incident            |

Every row is an artifact already defined in [README.md](README.md) — the
Charter introduces no new document type, it names the handoff each one
carries. The phase list is a shape, not a workflow: a project that has no
operation phase simply has no runbooks.

## Founding rule: every artifact lives in git

Every artifact named by this convention is a file in the repository it
describes.

- A clone is a complete copy of the project's intent, decisions, and plans.
  There is no second system to reach, and read access to the code is read
  access to the reasoning behind it.
- Artifacts branch, review, and merge with the code they describe, so a branch
  carries its own intent and a merge conflict in intent is visible as a
  conflict.
- History is free: `git log` on an artifact's path is that artifact's audit
  trail, and the convention's lifecycle commit types (`decision:`, `plan:`,
  `todo:`, `release:`, `deploy:` — see [README.md](README.md)) make phase
  transitions greppable events rather than parsed state.
- An actor with no network access — a sandboxed agent, an offline reviewer, a
  hermetic CI job — can still answer why this exists, what changed, and what is
  next.

The Todo is the rule's hardest case, because its source is an agent's working
memory, which is not durable at all; committing it makes a rollback point,
pushing it makes the list survive the machine, and nothing else in the repo
can answer where a lost session left off.

## When a project outgrows git

Defect tracking, ticket workflow, and cross-project planning do outgrow flat
files. GitHub Issues, Linear, and Jira win on querying, notification, and
access for people who do not clone repositories. The convention does not
compete with them: the external system stays the source of truth for its own
records.

What stays in the repo is a branch-scoped projection — the tickets and
external documents this branch depends on, cached as files at conventional
paths. They differ from branch to branch on purpose: that difference is the
cheapest available statement of what this branch is for, available before
anyone crawls history and diffs.

A projection must carry the identity and revision of its source (which record,
which version) so a reader can tell current from stale. The exact field names
are deliberately not fixed here; that is a later decision. The Todo is that
rule turned inward, its source being the session that wrote it, so it names
the session and the time the list was taken.

## What the shape is designed to enable

### Scope gates

A Decision names the change and a Plan names the files it touches, so a
validator can fail a Plan that references files outside its Decision's scope,
or that omits files the Decision requires. The same file list feeds a runtime
allowlist: the next phase gets write access to precisely those paths and
nothing else, deny by default. Both checks are deterministic and reviewable
before execution — the gate reads a file, not a model.

### Adapters

Because the artifacts are plain files at known paths, syncing with an external
tracker is a file-writing job rather than an integration. An adapter extracts
ticket and document state into conventional files (with the source identity
and revision, per the rule above), and pushes accepted state back out.

### Context injection ("reverse MCP")

The [Model Context Protocol](https://modelcontextprotocol.io) gives a model
tools to reach out across a network boundary at inference time. This inverts
the direction: the context is written into the working tree before the job
starts, so an agent reads it from disk — cheaply, lazily, with no egress and no
credentials — and every actor on that branch sees exactly the same context.

## Artifacts

Where this project keeps its own artifacts.

| Artifact  | Location     | Notes                                                                |
| --------- | ------------ | -------------------------------------------------------------------- |
| Charter   | `CHARTER.md` | this file                                                            |
| Design    | —            | the convention is the product; its spec is `README.md` and the skill |
| Decisions | —            | none recorded yet                                                    |
| Roadmap   | `ROADMAP.md` | in use                                                               |
| Plan      | —            | written per branch as `PLAN.md`, deleted at `plan: done`             |
| Runbooks  | —            | not used                                                             |
| Incidents | —            | not used                                                             |
| Todo      | —            | written per branch as `TODO.md`, deleted at `todo: clear`            |

The Design row's claim is the repo's stated split (see `AGENTS.md`):
`README.md` is the human-facing spec and rationale, and
`skills/conventional-docs/SKILL.md` is the agent-facing procedure, so a
separate `DESIGN.md` here would only duplicate them.

## Route

Current stage: draft, nothing versioned yet (same posture as the README's
status line); the reference implementation is the agent skill and Claude Code
plugin published from this repo.

- What is next: [ROADMAP.md](ROADMAP.md)
- The convention itself: [README.md](README.md)
- The agent procedure: [skills/conventional-docs/SKILL.md](skills/conventional-docs/SKILL.md)

What this convention is not (user documentation, a tool) is answered in
[README.md](README.md), not repeated here.
