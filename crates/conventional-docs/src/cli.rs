use clap::{Args, Parser, Subcommand};

/// Writes the Conventional Docs artifacts and their lifecycle commits.
#[derive(Debug, Parser)]
#[command(name = "doc", version)]
pub struct Cli {
    /// Write the files without committing them.
    #[arg(long, global = true)]
    pub no_commit: bool,

    #[command(subcommand)]
    pub command: Command,
}

/// A command scoped to one artifact is `<artifact> <verb>`; a command that
/// acts on the whole docset is a bare verb.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create the artifacts a fresh repo starts with: Charter, Design,
    /// Roadmap, and CHANGELOG.
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

    /// `README.md`.
    #[command(subcommand)]
    Readme(AdjacentCommand),

    /// `CONTRIBUTING.md`.
    #[command(subcommand)]
    Contributing(AdjacentCommand),

    /// `SECURITY.md`.
    #[command(subcommand)]
    Security(AdjacentCommand),

    /// `SUPPORT.md`.
    #[command(subcommand)]
    Support(AdjacentCommand),

    /// `LICENSE.md`, rendered from the standards pack.
    #[command(subcommand)]
    License(LicenseCommand),

    /// `CODE_OF_CONDUCT.md`, rendered from the standards pack.
    #[command(alias = "coc", subcommand)]
    CodeOfConduct(CodeOfConductCommand),

    /// Reports the resolved standards pack's entries.
    #[command(subcommand)]
    Pack(PackCommand),
}

/// The only verb on an artifact that announces no lifecycle event.
#[derive(Debug, Subcommand)]
pub enum GraduatingCommand {
    /// Write it at its root path, committing `docs: add <name>`.
    New,
}

#[derive(Debug, Subcommand)]
pub enum RunbookCommand {
    /// Write it under `docs/runbooks/`, committing `docs: add runbook <slug>`.
    New {
        /// What fires the runbook, slugged for the filename and heading.
        trigger: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum IncidentCommand {
    /// Write it under `docs/incidents/`, committing `docs: add incident <id>`.
    New {
        /// A short description, slugged for the filename and heading.
        slug: String,
    },
}

/// The only verb on an adjacent file with no third-party body: `readme`,
/// `contributing`, `security`, `support`.
#[derive(Debug, Subcommand)]
pub enum AdjacentCommand {
    /// Write it at the repository root, committing `docs: add <name>`.
    New,
}

/// `--pack <dir>`, shared by every command that resolves a standards pack.
#[derive(Debug, Args)]
pub struct PackArgs {
    /// Check this directory before the user data directory and the
    /// embedded pack. Must exist; a missing path is a usage error.
    #[arg(long)]
    pub pack: Option<std::path::PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum LicenseCommand {
    /// Write `LICENSE.md` from the standards pack, committing
    /// `docs: add license <spdx>`.
    New {
        /// The SPDX identifier the standards pack carries a body for.
        #[arg(long)]
        spdx: String,
        /// Defaults to the repository's configured `user.name`.
        #[arg(long)]
        holder: Option<String>,
        /// Defaults to today's year.
        #[arg(long)]
        year: Option<i16>,
        #[command(flatten)]
        pack: PackArgs,
    },
}

#[derive(Debug, Subcommand)]
pub enum CodeOfConductCommand {
    /// Write `CODE_OF_CONDUCT.md` from the standards pack, committing
    /// `docs: add code-of-conduct <standard>`.
    New {
        /// How a violation should be reported: an email address, a URL, or
        /// another contact method.
        #[arg(long)]
        contact: String,
        /// The standards-pack id to render. Defaults to the Contributor
        /// Covenant's current embedded version.
        #[arg(long, default_value = "contributor-covenant-3.0")]
        standard: String,
        #[command(flatten)]
        pack: PackArgs,
    },
}

#[derive(Debug, Subcommand)]
pub enum PackCommand {
    /// List every `(kind, id, source)` the resolved pack can serve. Writes
    /// and commits nothing.
    List {
        /// Emit a flat JSON array instead of tab-separated text.
        #[arg(long)]
        json: bool,
        #[command(flatten)]
        pack: PackArgs,
    },
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Create only the artifacts that are missing; otherwise report and exit
    /// 0 without writing anything when some already exist.
    #[arg(long)]
    pub add: bool,
    /// The changelog preamble's Semantic Versioning link version.
    #[arg(long, default_value = "2.0.0")]
    pub semver: String,
}

#[derive(Debug, Subcommand)]
pub enum DecisionCommand {
    /// Write a new record in the **draft** state.
    Draft {
        /// The record's title.
        title: String,
    },
    /// Write a new record in the **proposed** state, or promote an existing
    /// **draft** record to **proposed**.
    Propose {
        /// A title (creates a new record) or an existing record's id
        /// (promotes it from draft to proposed).
        title_or_id: String,
        /// The existing record this one extends.
        #[arg(long)]
        extends: Option<String>,
        /// The existing record this one supersedes.
        #[arg(long)]
        supersedes: Option<String>,
    },
    /// End review by accepting a **proposed** record.
    Accept {
        /// The record's id.
        id: String,
    },
    /// End review by rejecting a **proposed** record.
    Reject {
        /// The record's id.
        id: String,
    },
    /// Append a dated correction to a frozen record's `## Errata`.
    Errata {
        /// The record's id.
        id: String,
        /// The correction's text.
        text: String,
    },
}
