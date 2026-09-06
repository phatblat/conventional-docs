pub mod dec;
pub mod init;
pub mod new;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::ctx::Ctx;
use crate::error::Error;
use crate::git::Repo;

/// Writes every `(path, contents)` pair relative to the worktree root,
/// printing each written path, then commits them all under `subject` unless
/// `no_commit` — in which case the files are written, git is untouched, and
/// the tree is left dirty.
pub fn write_files(
    ctx: &mut Ctx,
    repo: &Repo,
    no_commit: bool,
    subject: &str,
    files: &[(PathBuf, String)],
) -> Result<(), Error> {
    for (path, contents) in files {
        let full = repo.workdir.join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full, contents)?;
        writeln!(ctx.out, "{}", path.display())?;
    }

    if !no_commit {
        let paths: Vec<PathBuf> = files.iter().map(|(p, _)| p.clone()).collect();
        crate::git::commit(ctx, repo, subject, &paths)?;
        writeln!(ctx.out, "{subject}")?;
    }

    Ok(())
}

/// Reads an existing file relative to the worktree root, erroring with a
/// convention-shaped message when it is missing.
pub fn read_existing(repo: &Repo, path: &Path, id: &str) -> Result<String, Error> {
    let full = repo.workdir.join(path);
    if !full.is_file() {
        return Err(Error::Convention(format!("{id} does not exist")));
    }
    Ok(fs::read_to_string(full)?)
}
