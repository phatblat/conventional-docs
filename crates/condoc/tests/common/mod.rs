//! Shared test fixture and helpers. Not every test binary (`dec.rs`,
//! `new.rs`, `init.rs`) uses every helper here, so unused-in-this-binary
//! warnings are expected and suppressed.
#![allow(dead_code)]

use std::cell::RefCell;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

use clap::Parser;
use condoc::cli::Cli;
use condoc::ctx::Ctx;

/// A `Write` sink backed by shared, interior-mutable storage, so the test
/// can read `Ctx::out`'s bytes after handing ownership of the `Box<dyn
/// Write>` to the `Ctx`.
#[derive(Clone, Default)]
struct SharedBuf(Rc<RefCell<Vec<u8>>>);

impl std::io::Write for SharedBuf {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// An isolated git repository with one root commit, wired to a `Ctx` with a
/// fixed `today` and an isolated git identity so tests are parallel-safe.
pub struct Fixture {
    pub dir: tempfile::TempDir,
    pub ctx: Ctx,
    output: SharedBuf,
}

/// The git environment every fixture's commits run under: no host config,
/// signing, or ambient identity can leak in.
fn git_env() -> Vec<(OsString, OsString)> {
    [
        ("GIT_CONFIG_GLOBAL", "/dev/null"),
        ("GIT_CONFIG_SYSTEM", "/dev/null"),
        ("GIT_CONFIG_NOSYSTEM", "1"),
        ("GIT_AUTHOR_NAME", "condoc tests"),
        ("GIT_AUTHOR_EMAIL", "tests@example.invalid"),
        ("GIT_COMMITTER_NAME", "condoc tests"),
        ("GIT_COMMITTER_EMAIL", "tests@example.invalid"),
    ]
    .into_iter()
    .map(|(k, v)| (OsString::from(k), OsString::from(v)))
    .collect()
}

fn run_git(dir: &std::path::Path, env: &[(OsString, OsString)], args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .current_dir(dir)
        .envs(env.iter().map(|(k, v)| (k.as_os_str(), v.as_os_str())))
        .status()
        .expect("spawn git");
    assert!(status.success(), "git {args:?} failed");
}

/// Creates a fresh git repository with one root commit (`chore: base`), and a
/// `Ctx` pinned to 2026-09-06 so ids and errata dates are deterministic.
pub fn repo() -> Fixture {
    let dir = tempfile::tempdir().expect("tempdir");
    let env = git_env();

    run_git(dir.path(), &env, &["init", "-q", "."]);
    std::fs::write(dir.path().join(".gitkeep"), "").unwrap();
    run_git(dir.path(), &env, &["add", "--", ".gitkeep"]);
    run_git(
        dir.path(),
        &env,
        &["commit", "-q", "-m", "chore: base", "--", ".gitkeep"],
    );

    let output = SharedBuf::default();
    let ctx = Ctx {
        cwd: dir.path().to_path_buf(),
        today: jiff::civil::date(2026, 9, 6),
        git_env: env,
        out: Box::new(output.clone()),
    };

    Fixture { dir, ctx, output }
}

/// Parses `args` as a condoc invocation (`condoc` is implied) and runs it
/// in-process against the fixture's `Ctx`. Returns the exit code condoc's own
/// `main()` would have returned.
pub fn run(f: &mut Fixture, args: &[&str]) -> i32 {
    let mut argv = vec!["condoc"];
    argv.extend_from_slice(args);
    let cli = Cli::parse_from(argv);
    match condoc::run(&cli, &mut f.ctx) {
        Ok(()) => 0,
        Err(err) => err.exit_code(),
    }
}

/// The output condoc has written so far, then clears the buffer so the next
/// call only sees new output.
pub fn take_output(f: &Fixture) -> String {
    let mut buf = f.output.0.borrow_mut();
    let s = String::from_utf8(buf.clone()).expect("utf8 output");
    buf.clear();
    s
}

/// The current commit subjects, newest first.
pub fn log_subjects(f: &Fixture) -> Vec<String> {
    let output = Command::new("git")
        .args(["log", "--format=%s"])
        .current_dir(f.dir.path())
        .output()
        .expect("git log");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

/// The paths touched by `HEAD`, relative to the worktree root.
pub fn commit_paths(f: &Fixture) -> Vec<String> {
    let output = Command::new("git")
        .args(["show", "--stat", "--format=", "--name-only", "HEAD"])
        .current_dir(f.dir.path())
        .output()
        .expect("git show");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

/// Paths currently staged in the index.
pub fn staged_paths(f: &Fixture) -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(f.dir.path())
        .output()
        .expect("git diff --cached");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

/// Reads a file relative to the fixture's worktree root.
pub fn read(f: &Fixture, path: &str) -> String {
    std::fs::read_to_string(f.dir.path().join(path)).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

/// Whether a path relative to the fixture's worktree root exists.
pub fn exists(f: &Fixture, path: &str) -> bool {
    f.dir.path().join(path).exists()
}

/// A golden file's contents, from `crates/condoc/tests/golden/<name>`.
pub fn golden(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/golden")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read golden {name}: {e}"))
}

/// Stages `path` (writing it first) so a test can prove an unrelated staged
/// change survives a condoc commit untouched.
pub fn stage_unrelated(f: &Fixture, path: &str) {
    std::fs::write(f.dir.path().join(path), "unrelated\n").unwrap();
    run_git(f.dir.path(), &f.ctx.git_env, &["add", "--", path]);
}
