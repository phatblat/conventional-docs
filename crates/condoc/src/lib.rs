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

use cli::{Cli, Command, DecCommand, NewCommand};
use ctx::Ctx;
use error::Error;

/// Parses `argv`, builds a real [`Ctx`], runs the parsed command, and prints
/// any error to stderr. Returns the process exit code.
pub fn main() -> i32 {
    let cli = Cli::parse();
    let mut ctx = match Ctx::live() {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("condoc: {err}");
            return err.exit_code();
        }
    };

    match run(&cli, &mut ctx) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("condoc: {err}");
            err.exit_code()
        }
    }
}

/// Runs one parsed command against `ctx`. Exposed for in-process testing.
pub fn run(cli: &Cli, ctx: &mut Ctx) -> Result<(), Error> {
    let no_commit = cli.no_commit;

    match &cli.command {
        Command::New(sub) => match sub {
            NewCommand::Charter => cmd::new::charter(ctx, no_commit),
            NewCommand::Design => cmd::new::design(ctx, no_commit),
            NewCommand::Roadmap => cmd::new::roadmap(ctx, no_commit),
            NewCommand::Runbook { trigger } => cmd::new::runbook(ctx, no_commit, trigger),
            NewCommand::Incident { slug } => cmd::new::incident(ctx, no_commit, slug),
        },
        Command::Init(args) => cmd::init::init(ctx, no_commit, args.add),
        Command::Dec(sub) => match sub {
            DecCommand::Draft { title } => cmd::dec::draft(ctx, no_commit, title),
            DecCommand::Propose {
                title_or_id,
                extends,
                supersedes,
            } => cmd::dec::propose(
                ctx,
                no_commit,
                title_or_id,
                extends.as_deref(),
                supersedes.as_deref(),
            ),
            DecCommand::Accept { id } => cmd::dec::accept(ctx, no_commit, id),
            DecCommand::Reject { id } => cmd::dec::reject(ctx, no_commit, id),
            DecCommand::Errata { id, text } => cmd::dec::errata(ctx, no_commit, id, text),
        },
    }
}
