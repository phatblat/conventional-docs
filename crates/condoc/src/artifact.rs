use std::path::PathBuf;

use crate::git::Repo;

/// The three artifacts that have both a small-repo root form and a graduated
/// `docs/` form. Decisions, runbooks, and incidents have exactly one home
/// each and need no probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Artifact {
    Charter,
    Design,
    Roadmap,
}

impl Artifact {
    fn root_name(self) -> &'static str {
        match self {
            Artifact::Charter => "CHARTER.md",
            Artifact::Design => "DESIGN.md",
            Artifact::Roadmap => "ROADMAP.md",
        }
    }

    fn graduated_name(self) -> &'static str {
        match self {
            Artifact::Charter => "docs/charter.md",
            Artifact::Design => "docs/design.md",
            Artifact::Roadmap => "docs/roadmap.md",
        }
    }
}

/// Returns the artifact's existing path, root form checked first, or `None`
/// when neither form exists.
pub fn locate(repo: &Repo, a: Artifact) -> Option<PathBuf> {
    let root = PathBuf::from(a.root_name());
    if repo.workdir.join(&root).is_file() {
        return Some(root);
    }
    let graduated = PathBuf::from(a.graduated_name());
    if repo.workdir.join(&graduated).is_file() {
        return Some(graduated);
    }
    None
}

/// The path a newly created artifact is written to: always the root,
/// small-repo form. condoc never guesses that a document should be born
/// graduated — graduation triggers are per-document judgments.
pub fn create_path(a: Artifact) -> PathBuf {
    PathBuf::from(a.root_name())
}
