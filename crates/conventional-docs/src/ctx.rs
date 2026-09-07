use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;

/// The state every command runs against.
///
/// Injected rather than read from the environment directly, so tests can pin
/// `today` and isolate the `git` child's identity without touching the
/// process environment (which would make tests order-dependent).
pub struct Ctx {
    /// Where the command was invoked; repository discovery starts here.
    pub cwd: PathBuf,
    /// Injected so ids and errata dates are deterministic in tests.
    pub today: jiff::civil::Date,
    /// Extra environment variables for the `git` child; empty in production.
    pub git_env: Vec<(OsString, OsString)>,
    /// Human-readable output sink.
    pub out: Box<dyn Write>,
}

impl Ctx {
    /// Builds the real context: current directory, local calendar date, no
    /// extra git environment, and stdout as the output sink.
    pub fn live() -> Result<Self, crate::error::Error> {
        let today = jiff::Zoned::now().date();
        Ok(Ctx {
            cwd: std::env::current_dir()?,
            today,
            git_env: Vec::new(),
            out: Box::new(std::io::stdout()),
        })
    }
}
