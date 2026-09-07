pub mod artifact;
pub mod cli;
pub mod cmd;
pub mod ctx;
pub mod error;
pub mod git;
pub mod record;
pub mod slug;
pub mod template;

use clap::Parser;

use cli::{Cli, Command, DecisionCommand, GraduatingCommand, IncidentCommand, RunbookCommand};
use ctx::Ctx;
use error::Error;

/// Parses `argv`, builds a real [`Ctx`], runs the parsed command, and prints
/// any error to stderr. Returns the process exit code.
pub fn main() -> i32 {
    let cli = Cli::parse();
    let mut ctx = match Ctx::live() {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("doc: {err}");
            return err.exit_code();
        }
    };

    match run(&cli, &mut ctx) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("doc: {err}");
            err.exit_code()
        }
    }
}

/// Runs one parsed command against `ctx`. Exposed for in-process testing.
pub fn run(cli: &Cli, ctx: &mut Ctx) -> Result<(), Error> {
    let no_commit = cli.no_commit;

    match &cli.command {
        Command::Init(args) => cmd::init::init(ctx, no_commit, args.add),
        Command::Charter(GraduatingCommand::New) => cmd::new::charter(ctx, no_commit),
        Command::Design(GraduatingCommand::New) => cmd::new::design(ctx, no_commit),
        Command::Roadmap(GraduatingCommand::New) => cmd::new::roadmap(ctx, no_commit),
        Command::Runbook(RunbookCommand::New { trigger }) => {
            cmd::new::runbook(ctx, no_commit, trigger)
        }
        Command::Incident(IncidentCommand::New { slug }) => {
            cmd::new::incident(ctx, no_commit, slug)
        }
        Command::Decision(sub) => match sub {
            DecisionCommand::Draft { title } => cmd::decision::draft(ctx, no_commit, title),
            DecisionCommand::Propose {
                title_or_id,
                extends,
                supersedes,
            } => cmd::decision::propose(
                ctx,
                no_commit,
                title_or_id,
                extends.as_deref(),
                supersedes.as_deref(),
            ),
            DecisionCommand::Accept { id } => cmd::decision::accept(ctx, no_commit, id),
            DecisionCommand::Reject { id } => cmd::decision::reject(ctx, no_commit, id),
            DecisionCommand::Errata { id, text } => cmd::decision::errata(ctx, no_commit, id, text),
        },
    }
}
