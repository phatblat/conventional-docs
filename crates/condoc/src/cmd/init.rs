use std::fs;
use std::io::Write;
use std::path::PathBuf;

use super::write_files;
use crate::artifact::{self, Artifact};
use crate::ctx::Ctx;
use crate::error::Error;
use crate::git::{self, Repo};
use crate::template;

/// `commitlint.config.*` / `.commitlintrc*` filenames checked, in order,
/// before falling back to a `package.json` containing `"commitlint"`.
const COMMITLINT_CANDIDATES: &[&str] = &[
    "commitlint.config.js",
    "commitlint.config.mjs",
    "commitlint.config.cjs",
    "commitlint.config.ts",
    ".commitlintrc",
    ".commitlintrc.json",
    ".commitlintrc.js",
    ".commitlintrc.yml",
    ".commitlintrc.yaml",
];

/// The convention's custom commit types a `type-enum` must allow.
const REQUIRED_TYPES: &[&str] = &["decision", "deploy", "plan", "release", "todo"];

/// `init [--add]` — creates the artifacts a fresh repo starts with: Charter,
/// Design, Roadmap, and CHANGELOG. Never `EVENTS.md`, `PLAN.md`, or `TODO.md`.
pub fn init(ctx: &mut Ctx, no_commit: bool, add: bool) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;

    let charter = artifact::locate(&repo, Artifact::Charter);
    let design = artifact::locate(&repo, Artifact::Design);
    let roadmap = artifact::locate(&repo, Artifact::Roadmap);
    let changelog_path = PathBuf::from("CHANGELOG.md");
    let changelog_present = repo.workdir.join(&changelog_path).is_file();

    let missing_count = [
        charter.is_none(),
        design.is_none(),
        roadmap.is_none(),
        !changelog_present,
    ]
    .into_iter()
    .filter(|m| *m)
    .count();

    if missing_count == 0 {
        writeln!(ctx.out, "nothing missing")?;
        report_commitlint(ctx, &repo)?;
        return Ok(());
    }

    // A fresh repo (nothing present) always gets all four; a partially
    // adopted repo needs `--add` to write anything.
    if missing_count < 4 && !add {
        if charter.is_none() {
            writeln!(
                ctx.out,
                "missing: {}",
                artifact::create_path(Artifact::Charter).display()
            )?;
        }
        if design.is_none() {
            writeln!(
                ctx.out,
                "missing: {}",
                artifact::create_path(Artifact::Design).display()
            )?;
        }
        if roadmap.is_none() {
            writeln!(
                ctx.out,
                "missing: {}",
                artifact::create_path(Artifact::Roadmap).display()
            )?;
        }
        if !changelog_present {
            writeln!(ctx.out, "missing: {}", changelog_path.display())?;
        }
        report_commitlint(ctx, &repo)?;
        return Ok(());
    }

    let mut files: Vec<(PathBuf, String)> = Vec::new();
    let mut names: Vec<&str> = Vec::new();

    if charter.is_none() {
        files.push((
            artifact::create_path(Artifact::Charter),
            template::charter().to_string(),
        ));
        names.push("charter");
    }
    if design.is_none() {
        files.push((
            artifact::create_path(Artifact::Design),
            template::design().to_string(),
        ));
        names.push("design");
    }
    if roadmap.is_none() {
        files.push((
            artifact::create_path(Artifact::Roadmap),
            template::roadmap().to_string(),
        ));
        names.push("roadmap");
    }
    if !changelog_present {
        let link = unreleased_link(&repo);
        if link.is_none() {
            writeln!(
                ctx.out,
                "CHANGELOG.md: no [Unreleased] compare link (needs a github.com origin and a tag)"
            )?;
        }
        files.push((changelog_path, template::changelog(link.as_deref())));
        names.push("changelog");
    }

    let subject = format!("docs: add {}", names.join(", "));
    write_files(ctx, &repo, no_commit, &subject, &files)?;

    report_commitlint(ctx, &repo)?;
    Ok(())
}

/// The `[Unreleased]` compare link, when the origin is a `github.com` remote
/// and at least one tag exists — otherwise `None`.
fn unreleased_link(repo: &Repo) -> Option<String> {
    let remote = repo.repo.find_remote("origin").ok()?;
    let url = remote.url(gix::remote::Direction::Fetch)?;
    let host = url.host()?;
    if !host.eq_ignore_ascii_case("github.com") {
        return None;
    }

    let path = url.path.to_string();
    let path = path.trim_start_matches('/').trim_end_matches(".git");
    let (owner, repo_name) = path.split_once('/')?;
    if owner.is_empty() || repo_name.is_empty() {
        return None;
    }

    let tag = newest_tag(repo)?;
    Some(format!(
        "https://github.com/{owner}/{repo_name}/compare/{tag}...HEAD"
    ))
}

/// The tag whose peeled commit has the newest committer time.
fn newest_tag(repo: &Repo) -> Option<String> {
    let refs = repo.repo.references().ok()?;
    let tags = refs.tags().ok()?;

    let mut best: Option<(gix::date::Time, String)> = None;
    for tag_ref in tags.flatten() {
        let mut tag_ref = tag_ref;
        let short = tag_ref.name().shorten().to_string();
        let Ok(commit) = tag_ref.peel_to_commit() else {
            continue;
        };
        let Ok(time) = commit.time() else { continue };
        if best.as_ref().is_none_or(|(best_time, _)| time > *best_time) {
            best = Some((time, short));
        }
    }
    best.map(|(_, name)| name)
}

/// Reports (never edits) whether a commitlint config exists and whether its
/// `type-enum` covers the convention's five custom event types.
fn report_commitlint(ctx: &mut Ctx, repo: &Repo) -> Result<(), Error> {
    let mut config_path = COMMITLINT_CANDIDATES
        .iter()
        .map(|name| repo.workdir.join(name))
        .find(|path| path.is_file());

    if config_path.is_none() {
        let pkg = repo.workdir.join("package.json");
        if fs::read_to_string(&pkg).is_ok_and(|contents| contents.contains("\"commitlint\"")) {
            config_path = Some(pkg);
        }
    }

    let Some(config_path) = config_path else {
        writeln!(
            ctx.out,
            "commitlint: no config found; the convention's event types are unenforced"
        )?;
        return Ok(());
    };

    let text = fs::read_to_string(&config_path)?;
    let missing: Vec<&str> = REQUIRED_TYPES
        .iter()
        .filter(|t| !text.contains(&format!("'{t}'")) && !text.contains(&format!("\"{t}\"")))
        .copied()
        .collect();
    if !missing.is_empty() {
        writeln!(
            ctx.out,
            "commitlint: type-enum is missing {}",
            missing.join(", ")
        )?;
    }
    Ok(())
}
