use std::path::PathBuf;

use crate::git::Repo;

/// A document a repository is expected to have that this convention places
/// but does not define. Not an [`crate::artifact::Artifact`]: an adjacent
/// file has no lifecycle event of its own and never graduates (decision
/// clause 4 of
/// `docs/decisions/2026-09-07-adjacent-files-and-the-standards-pack.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjacent {
    Readme,
    Contributing,
    Security,
    Support,
    License,
    CodeOfConduct,
}

impl Adjacent {
    /// The host-recognized file name this adjacent file is written as.
    pub fn file_name(self) -> &'static str {
        match self {
            Adjacent::Readme => "README.md",
            Adjacent::Contributing => "CONTRIBUTING.md",
            Adjacent::Security => "SECURITY.md",
            Adjacent::Support => "SUPPORT.md",
            Adjacent::License => "LICENSE.md",
            Adjacent::CodeOfConduct => "CODE_OF_CONDUCT.md",
        }
    }

    /// The word used in the `docs: add <name>` commit subject.
    pub fn event_name(self) -> &'static str {
        match self {
            Adjacent::Readme => "readme",
            Adjacent::Contributing => "contributing",
            Adjacent::Security => "security",
            Adjacent::Support => "support",
            Adjacent::License => "license",
            Adjacent::CodeOfConduct => "code-of-conduct",
        }
    }

    /// Every name this file is recognized under, in the host's precedence
    /// order within a single directory: the file itself first, then any
    /// alternate name the same standard recognizes (only the license has
    /// more than one).
    ///
    /// `License` also probes `LICENSE`, `LICENSE.txt`, `COPYING`, and
    /// `COPYING.md`, so `license new` cannot clobber a license living under
    /// another conventional name.
    fn recognized_names(self) -> &'static [&'static str] {
        match self {
            Adjacent::License => &[
                "LICENSE.md",
                "LICENSE",
                "LICENSE.txt",
                "COPYING",
                "COPYING.md",
            ],
            Adjacent::Readme => &["README.md"],
            Adjacent::Contributing => &["CONTRIBUTING.md"],
            Adjacent::Security => &["SECURITY.md"],
            Adjacent::Support => &["SUPPORT.md"],
            Adjacent::CodeOfConduct => &["CODE_OF_CONDUCT.md"],
        }
    }

    /// The directories this file's host recognizes, in precedence order.
    /// `README.md`, the license, and `CODE_OF_CONDUCT.md` are root-only;
    /// deliberately no lowercase `docs/<name>.md` form is probed for any of
    /// them — clause 4 of the extended decision says an adjacent file never
    /// graduates, so there is no graduated name to look for.
    fn recognized_dirs(self) -> &'static [&'static str] {
        match self {
            Adjacent::Readme | Adjacent::License | Adjacent::CodeOfConduct => &[""],
            Adjacent::Contributing | Adjacent::Security | Adjacent::Support => {
                &[".github", "", "docs"]
            }
        }
    }
}

/// Every path where `a` currently exists, in the host's precedence order.
/// Used to refuse an overwrite in a lower-precedence location, and by the
/// (future) validation slice to report a duplicate across locations.
pub fn locate_all(repo: &Repo, a: Adjacent) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for dir in a.recognized_dirs() {
        for name in a.recognized_names() {
            let rel = if dir.is_empty() {
                PathBuf::from(name)
            } else {
                PathBuf::from(dir).join(name)
            };
            if repo.workdir.join(&rel).is_file() {
                found.push(rel);
            }
        }
    }
    found
}

/// The path a newly created adjacent file is written to: always the
/// repository root, under its primary recognized name.
pub fn create_path(a: Adjacent) -> PathBuf {
    PathBuf::from(a.file_name())
}
