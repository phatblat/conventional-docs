# Plan: 2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact

Decision:
[docs/decisions/2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact.md](docs/decisions/2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact.md)

## Steps

### Crate move and Cargo metadata

- [ ] `git mv crates/condoc crates/conventional-docs`.
- [ ] Root `Cargo.toml`: `members = ["crates/conventional-docs"]`.
- [ ] `crates/conventional-docs/Cargo.toml`: `name = "conventional-docs"`, plus a
      `[[bin]]` section with `name = "doc"` and `path = "src/main.rs"`. Leave
      `version`, `publish = false`, and the dependency table alone; the library
      target becomes `conventional_docs` by default.
- [ ] `src/main.rs`: call `conventional_docs::main()`.

### Noun-first command surface

- [ ] `src/cli.rs`: `#[command(name = "doc", version)]`, and replace the `New`
      and `Dec` groups with one group per artifact. `GraduatingCommand` is
      shared by the three artifacts whose `new` takes no argument, so each
      artifact's file name stays on its group's help line:

```rust
pub enum Command {
    Init(InitArgs),
    /// `CHARTER.md`.
    #[command(subcommand)]
    Charter(GraduatingCommand),
    /// `DESIGN.md`.
    #[command(subcommand)]
    Design(GraduatingCommand),
    /// `ROADMAP.md`.
    #[command(subcommand)]
    Roadmap(GraduatingCommand),
    /// `docs/runbooks/<slug>.md`.
    #[command(subcommand)]
    Runbook(RunbookCommand),
    /// `docs/incidents/<today>-<slug>.md`.
    #[command(subcommand)]
    Incident(IncidentCommand),
    /// Write a decision record and its lifecycle commits.
    #[command(alias = "dec", subcommand)]
    Decision(DecisionCommand),
}

/// The only verb on an artifact that announces no lifecycle event.
pub enum GraduatingCommand {
    /// Write it at its root path, committing `docs: add <name>`.
    New,
}

pub enum RunbookCommand {
    New { trigger: String },
}

pub enum IncidentCommand {
    New { slug: String },
}
```

- [ ] Rename `DecCommand` to `DecisionCommand`; its five variants and their
      arguments are unchanged.
- [ ] `src/lib.rs`: dispatch `Command::Charter(GraduatingCommand::New)` to
      `cmd::new::charter` (same for design and roadmap),
      `Command::Runbook(RunbookCommand::New { trigger })`,
      `Command::Incident(IncidentCommand::New { slug })`, and
      `Command::Decision(sub)` to `cmd::decision::*`. Both `eprintln!` prefixes
      print `doc:` instead of `condoc:`.
- [ ] `git mv crates/conventional-docs/src/cmd/dec.rs .../cmd/decision.rs`;
      update `src/cmd/mod.rs`'s module list and every `cmd::dec::` path.
- [ ] Doc comments that spell a command: `decision.rs` gains
      `decision draft|propose|accept|reject|errata`, `new.rs` gains
      `charter new`, `design new`, `roadmap new`, `runbook new <trigger>`,
      `incident new <slug>`. `init.rs`'s `init [--add]` is unchanged.
- [ ] Prose mentions of the old name in `src/artifact.rs`, `src/error.rs`, and
      `src/git.rs` become `doc`. Leave `src/error.rs`'s citation of
      `docs/decisions/2026-09-05-condoc-a-binary-for-the-document-lifecycle.md`
      exactly as written: a record id is never renamed.

### Tests

- [ ] `tests/common/mod.rs`: `use conventional_docs::cli::Cli` and `::ctx::Ctx`,
      `conventional_docs::run(...)`, `vec!["doc"]` as argv[0], the git identity
      becomes `doc tests`, and the golden-path comment points at
      `crates/conventional-docs/tests/golden/`.
- [ ] `tests/new.rs`: the six invocations become `["charter", "new"]`,
      `["design", "new"]`, `["roadmap", "new"]`,
      `["runbook", "new", "Disk full"]`,
      `["incident", "new", "Database outage"]`, and the already-exists case
      `["charter", "new"]`. Golden bytes, commit subjects, and commit paths are
      unchanged.
- [ ] `tests/dec.rs`: all 24 invocations spell `decision` instead of `dec`;
      nothing else changes.
- [ ] Add two parser tests pinning decision clauses 5 and 9. Use
      `Cli::try_parse_from`, never `run` — `Cli::parse_from` exits the process
      on a usage error: `dec` still resolves to `Command::Decision`, and
      `["doc", "charter"]` with no verb fails with
      `ErrorKind::MissingSubcommand` and `err.exit_code() == 2`.

### Root documents

- [ ] `README.md`: "the `doc` binary" in the "Not a tool" bullet.
- [ ] `ROADMAP.md`: `## condoc` becomes `## doc`, and its five items spell the
      accepted surface — `doc status`, `doc todo sync|clear`,
      `doc plan start|done`, `doc lint|fix|check`, `doc release`,
      `doc decision import`, `doc <artifact> graduate`.
- [ ] `CHANGELOG.md`: edit the two `[Unreleased]` lines in place, never adding a
      `Changed` entry — the tool has not been released. The binary line names
      `doc` with the noun-first commands and cites this decision alongside the
      one it extends; the site line becomes `/doc/`.

### Site

- [ ] `git mv site/content/condoc site/content/doc`, then rewrite the page:
      `title: doc`, the install line
      (`cargo install --path crates/conventional-docs`), and the command table
      in the accepted shape, keeping the `dec` alias note.
- [ ] `site/hugo.yaml`: both menu entries become `name: doc`, `pageRef: /doc`.
- [ ] `site/data/cards.yaml`: `title: doc`, `href: /doc`.
- [ ] Inbound links: `site/content/_index.md`, `site/content/events/index.md`,
      `site/content/faq/index.md` (including its `## Do I need doc?` heading),
      and `site/content/quickstart/index.md`, whose two commands become
      `doc init` and `doc decision propose <title>`.

### Commits

- [ ] `refactor(cli): rename the condoc binary to doc and group commands by artifact`
      — crate move, Cargo metadata, sources, tests. Not `feat!`: the npm package
      ships only `*.md`, `.claude-plugin`, and `skills`, so no published surface
      changes.
- [ ] `docs: rename condoc to doc in the README, roadmap, and changelog`.
- [ ] `docs(site): move the condoc page to /doc/ and document the noun-first surface`.

## Verification

```bash
just check
cargo build -p conventional-docs
ls target/debug/doc
```

`just check` runs format-check, markdownlint, skill frontmatter, `cargo fmt
--check`, `cargo clippy --all-targets -- -D warnings`, both link checks, and
`cargo test`. `target/debug/doc` existing is what proves the binary name.

Then smoke-test the renamed binary against a scratch repo, so nothing is
written to this one:

```bash
doc=$PWD/target/debug/doc
tmp=$(mktemp -d) && git -C "$tmp" init -q
cd "$tmp" && "$doc" charter new && "$doc" decision propose "Try the renamed binary"
git log --format=%s
"$doc" charter; echo "exit=$?"
```

Expect `CHARTER.md` and `docs/decisions/2026-09-06-try-the-renamed-binary.md`
written, `git log` showing `decision: propose 2026-09-06-try-the-renamed-binary`
over `docs: add charter`, and the verbless `charter` group exiting 2.

## Status

Written immediately after
`decision: accept 2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact`
on `feat/rename-doc-binary`. No implementation steps have run yet.
