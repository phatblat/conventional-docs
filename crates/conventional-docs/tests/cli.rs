//! Parser-level contracts from
//! `docs/decisions/2026-09-06-rename-condoc-to-doc-and-group-commands-by-artifact.md`:
//! `dec` is the surface's only alias (clause 5), and an artifact group with no
//! verb is a usage error (clause 9). These drive the parser directly, because
//! `Cli::parse_from` exits the process on a usage error.

use clap::Parser;
use conventional_docs::cli::{Cli, Command, DecisionCommand};

#[test]
fn dec_resolves_to_the_decision_group() {
    let cli = Cli::try_parse_from(["doc", "dec", "accept", "2026-09-06-a-slug"]).unwrap();
    assert!(matches!(
        cli.command,
        Command::Decision(DecisionCommand::Accept { .. })
    ));
}

#[test]
fn an_artifact_group_without_a_verb_prints_help_and_exits_2() {
    let err = Cli::try_parse_from(["doc", "charter"]).unwrap_err();
    assert_eq!(
        err.kind(),
        clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
    assert_eq!(err.exit_code(), 2);
    assert!(err.to_string().contains("Usage: doc charter"));
}
