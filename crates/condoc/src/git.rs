use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ctx::Ctx;
use crate::error::Error;

/// A discovered repository: the `gix` handle plus its resolved worktree root.
pub struct Repo {
    pub repo: gix::Repository,
    pub workdir: PathBuf,
}

/// Discovers the repository containing `cwd`.
///
/// `gix::discover` resolves a linked worktree correctly, which matters here:
/// this project itself is developed from a linked worktree.
pub fn discover(cwd: &Path) -> Result<Repo, Error> {
    let repo = gix::discover(cwd)
        .map_err(|err| Error::Convention(format!("not a git repository: {err}")))?;
    let workdir = repo
        .workdir()
        .ok_or_else(|| Error::Convention("repository has no worktree".to_string()))?
        .to_path_buf();
    Ok(Repo { repo, workdir })
}

/// Stages exactly `paths` and commits exactly `paths` under `subject`.
///
/// This is the only write path to git history in condoc (decision clauses 5
/// and 6): never `add -A`, `add .`, or `commit -a`. `add` is required first —
/// a pathspec commit cannot introduce an untracked file — and the pathspec on
/// `commit` is what keeps any other staged change out of this commit.
///
/// `paths` are relative to the worktree root. On a non-zero exit from either
/// child, the written files are left on disk (no rollback) and `Error::Git`
/// is returned; the caller's message should say the files are uncommitted.
pub fn commit(ctx: &Ctx, repo: &Repo, subject: &str, paths: &[PathBuf]) -> Result<(), Error> {
    run(ctx, repo, &{
        let mut args: Vec<std::ffi::OsString> = vec!["add".into(), "--".into()];
        args.extend(paths.iter().map(|p| p.as_os_str().to_owned()));
        args
    })?;

    run(ctx, repo, &{
        let mut args: Vec<std::ffi::OsString> =
            vec!["commit".into(), "-m".into(), subject.into(), "--".into()];
        args.extend(paths.iter().map(|p| p.as_os_str().to_owned()));
        args
    })?;

    Ok(())
}

fn run(ctx: &Ctx, repo: &Repo, args: &[std::ffi::OsString]) -> Result<(), Error> {
    let status = Command::new("git")
        .args(args)
        .current_dir(&repo.workdir)
        .envs(
            ctx.git_env
                .iter()
                .map(|(k, v)| (k.as_os_str(), v.as_os_str())),
        )
        .status()?;

    if !status.success() {
        return Err(Error::Git {
            code: status.code().unwrap_or(1),
        });
    }
    Ok(())
}
