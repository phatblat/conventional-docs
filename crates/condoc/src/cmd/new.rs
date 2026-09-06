use std::path::Path;

use super::write_files;
use crate::artifact::{self, Artifact};
use crate::ctx::Ctx;
use crate::error::Error;
use crate::git;
use crate::slug::{INCIDENT_SLUG_MAX, RUNBOOK_SLUG_MAX, slug};
use crate::template;

fn already_exists(name: &str, path: &Path) -> Error {
    Error::Convention(format!("{name} already exists at {}", path.display()))
}

fn create_graduating(
    ctx: &mut Ctx,
    no_commit: bool,
    a: Artifact,
    name: &str,
    contents: &'static str,
) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    if let Some(existing) = artifact::locate(&repo, a) {
        return Err(already_exists(name, &existing));
    }
    let path = artifact::create_path(a);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("docs: add {name}"),
        &[(path, contents.to_string())],
    )
}

/// `new charter` — `CHARTER.md`.
pub fn charter(ctx: &mut Ctx, no_commit: bool) -> Result<(), Error> {
    create_graduating(
        ctx,
        no_commit,
        Artifact::Charter,
        "charter",
        template::charter(),
    )
}

/// `new design` — `DESIGN.md`.
pub fn design(ctx: &mut Ctx, no_commit: bool) -> Result<(), Error> {
    create_graduating(
        ctx,
        no_commit,
        Artifact::Design,
        "design",
        template::design(),
    )
}

/// `new roadmap` — `ROADMAP.md`.
pub fn roadmap(ctx: &mut Ctx, no_commit: bool) -> Result<(), Error> {
    create_graduating(
        ctx,
        no_commit,
        Artifact::Roadmap,
        "roadmap",
        template::roadmap(),
    )
}

/// `new runbook <trigger>` — `docs/runbooks/<slug>.md`.
pub fn runbook(ctx: &mut Ctx, no_commit: bool, trigger: &str) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let slug = slug(trigger, RUNBOOK_SLUG_MAX)?;
    let path = std::path::PathBuf::from("docs/runbooks").join(format!("{slug}.md"));
    if repo.workdir.join(&path).is_file() {
        return Err(already_exists("runbook", &path));
    }
    let body = template::runbook(&slug);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("docs: add runbook {slug}"),
        &[(path, body)],
    )
}

/// `new incident <slug>` — `docs/incidents/<today>-<slug>.md`.
pub fn incident(ctx: &mut Ctx, no_commit: bool, raw_slug: &str) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let normalized = slug(raw_slug, INCIDENT_SLUG_MAX)?;
    let id = format!("{}-{normalized}", ctx.today);
    let path = std::path::PathBuf::from("docs/incidents").join(format!("{id}.md"));
    if repo.workdir.join(&path).is_file() {
        return Err(already_exists("incident", &path));
    }
    let body = template::incident(&id);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("docs: add incident {id}"),
        &[(path, body)],
    )
}
