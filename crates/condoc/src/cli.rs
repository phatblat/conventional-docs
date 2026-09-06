use clap::{Args, Parser, Subcommand};

/// Writes the Conventional Docs artifacts and their lifecycle commits.
#[derive(Debug, Parser)]
#[command(name = "condoc", version)]
pub struct Cli {
    /// Write the files without committing them.
    #[arg(long, global = true)]
    pub no_commit: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create the artifacts that have no lifecycle event.
    #[command(subcommand)]
    New(NewCommand),

    /// Create the artifacts a fresh repo starts with: Charter, Design,
    /// Roadmap, and CHANGELOG.
    Init(InitArgs),

    /// Write a decision record and its lifecycle commits.
    #[command(alias = "decision", subcommand)]
    Dec(DecCommand),
}

#[derive(Debug, Subcommand)]
pub enum NewCommand {
    /// `CHARTER.md`.
    Charter,
    /// `DESIGN.md`.
    Design,
    /// `ROADMAP.md`.
    Roadmap,
    /// `docs/runbooks/<slug>.md`.
    Runbook {
        /// What fires the runbook, slugged for the filename and heading.
        trigger: String,
    },
    /// `docs/incidents/<today>-<slug>.md`.
    Incident {
        /// A short description, slugged for the filename and heading.
        slug: String,
    },
}

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Create only the artifacts that are missing; otherwise report and exit
    /// 0 without writing anything when some already exist.
    #[arg(long)]
    pub add: bool,
}

#[derive(Debug, Subcommand)]
pub enum DecCommand {
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
