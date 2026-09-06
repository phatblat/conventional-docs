use std::fmt;

/// The one error type every condoc command returns.
///
/// The exit-code mapping is clause 9 of
/// `docs/decisions/2026-09-05-condoc-a-binary-for-the-document-lifecycle.md`:
/// a usage error clap itself cannot catch exits 2, everything else exits 1.
#[derive(Debug)]
pub enum Error {
    /// A convention violation or a bad state (wrong status, missing record, …).
    Convention(String),
    /// A usage error clap cannot catch — e.g. a title that slugs to nothing.
    Usage(String),
    /// The `git` child exited non-zero; its stderr was already inherited.
    Git {
        code: i32,
    },
    Io(std::io::Error),
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::Usage(_) => 2,
            Error::Convention(_) | Error::Git { .. } | Error::Io(_) => 1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Convention(msg) => write!(f, "{msg}"),
            Error::Usage(msg) => write!(f, "{msg}"),
            Error::Git { code } => write!(f, "git exited with status {code}"),
            Error::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
