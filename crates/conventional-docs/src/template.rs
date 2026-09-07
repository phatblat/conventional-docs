use crate::record::{Status, status_line};

/// The decision record skeleton, byte-for-byte from
/// `skills/conventional-docs/SKILL.md`'s `### Structure` fenced block, with
/// `<Decision title>` and the `## Status` body substituted.
pub fn decision(title: &str, status: Status) -> String {
    format!(
        "# {title}

## Issue

<The problem requiring a decision, with links to the motivating issue and PRs.
If this decision extends another, the first sentence is `This decision extends
[YYYY-MM-DD-slug](./YYYY-MM-DD-slug.md).` and nothing from that decision is
restated.>

## Status

{status_line}

## Assumptions and Constraints

- <Facts bounding the choice: environment, compatibility guarantees, prior
  decisions. For a change to a public surface, state the compatibility
  guarantee it has to keep here.>

## Argument

<Why the chosen direction beats the alternatives. Name any alternative that
shapes the choice, with its verdict (**Chosen.** / **Rejected.**), and reserve
full reasoning for Positions. `N/A` is acceptable when the constraints make the
decision self-evident.>

## Architectural Decision

<The decision itself, as numbered clauses a reviewer can point at. Include code
or YAML only where it pins down a contract — a field name, a struct variant,
one representative manifest — never to reproduce the implementation.>

## Positions

<Alternatives considered and rejected, each with its reason, or `N/A`.>
",
        status_line = status_line(status)
    )
}

pub fn charter() -> &'static str {
    "# Charter

## Why this exists

<!-- Why this project exists, and what it is for. -->

## Goals

<!-- One bullet per goal. -->

## Artifacts

<!-- A table of where this project keeps each Conventional Docs artifact. -->

## Route

<!-- The current stage, and links to what is next. -->
"
}

pub fn design() -> &'static str {
    "# Design

## What this is

<!-- What the system is and does now. -->

## How it works

<!-- The parts, their responsibilities, and how they fit together. -->
"
}

pub fn roadmap() -> &'static str {
    "# Roadmap

What's next, in order. This is a living document — check items off in place
rather than filing a separate issue for each one, and delete anything that no
longer applies.

<!-- One `- [ ]` item per thing to do, in order. -->
"
}

pub fn runbook(slug: &str) -> String {
    format!(
        "# Runbook: {slug}

## Trigger

<!-- What fires this runbook. -->

## Checks

<!-- What to look at, in order. -->

## Actions

<!-- What to do, in order. -->

## Escalation

<!-- Who to involve when the actions do not resolve it. -->
"
    )
}

pub fn incident(id: &str) -> String {
    format!(
        "# Incident: {id}

## What happened

<!-- The timeline, in UTC. -->

## Impact

<!-- Who and what was affected, and for how long. -->

## Cause

<!-- The root cause. -->

## What we learned

<!-- What this incident changes in the system or the process. -->
"
    )
}

/// `CHANGELOG.md`'s fixed preamble, byte-identical to this repo's own
/// `CHANGELOG.md` lines 1-8 when `semver` is `"2.0.0"`. `semver` sets the
/// Semantic Versioning spec version the preamble links to (`init --semver`,
/// default `2.0.0`); it does not need to match this project's own version.
/// When `unreleased_link` is `Some`, a `[Unreleased]: <url>` reference link
/// is appended as the file's last line.
pub fn changelog(unreleased_link: Option<&str>, semver: &str) -> String {
    let mut body = format!(
        "# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v{semver}.html).

## [Unreleased]
"
    );
    if let Some(url) = unreleased_link {
        body.push_str(&format!("\n[Unreleased]: {url}\n"));
    }
    body
}

pub fn readme() -> &'static str {
    "# Project Name

<!-- Replace with the actual project name. -->

## Install

<!-- How to get it. -->

## Usage

<!-- How to use it. -->

## Development

<!-- How to build, test, and run it from a clone. -->

## License

<!-- The license name; the full text lives in the license file. -->
"
}

pub fn contributing() -> &'static str {
    "# Contributing

This project's documentation follows
[Conventional Docs](https://github.com/phatblat/conventional-docs).

## How a change flows

1. **Decide.** A change that alters behavior, a published interface, or a
   dependency gets a decision record at `docs/decisions/YYYY-MM-DD-slug.md`
   before it merges. A **proposed** record is the spec; **accept**ing it
   freezes the record's body.
2. **Plan.** Work spanning more than one session, or handed off to someone
   else, gets a `PLAN.md` on the branch — the ordered steps to carry out the
   accepted decision — deleted before merge.
3. **Commit.** Commits follow
   [Conventional Commits](https://www.conventionalcommits.org/). Lifecycle
   transitions are announced as `decision:`, `plan:`, `todo:`, `release:`,
   and `deploy:` commits.
4. **Announce.** A notable user-facing change adds its line to
   `CHANGELOG.md`'s `## [Unreleased]` section in the same pull request as the
   change.

## Where documents live

<!-- Link the Charter's `## Artifacts` section here; it records where every
document in this repository currently lives. -->

<!-- Project-specific setup, tests, and review expectations. -->
"
}

pub fn security() -> &'static str {
    "# Security Policy

## Reporting a vulnerability

<!-- A private channel: a GitHub security advisory, an email address, or a
security.txt contact. Do not ask for vulnerability reports to be filed as
public issues. -->

## Supported versions

<!-- Which versions currently receive security fixes. -->
"
}

pub fn support() -> &'static str {
    "# Support

<!-- Where to ask for help, and where not to (e.g. issues are for bugs, not
questions). -->

## Before asking

<!-- What to check first: docs, existing issues, a FAQ. -->
"
}
