use std::collections::HashMap;
use std::path::PathBuf;

use super::{read_existing, write_files};
use crate::ctx::Ctx;
use crate::error::Error;
use crate::git;
use crate::record::{self, Status};
use crate::slug::{DECISION_SLUG_MAX, slug};
use crate::template;

/// `dec draft <title>` — writes a new record in the **draft** state.
pub fn draft(ctx: &mut Ctx, no_commit: bool, title: &str) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let (id, path) = new_id_and_path(ctx, &repo, title)?;
    let body = template::decision(title, Status::Draft);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("decision: draft {id}"),
        &[(path, body)],
    )
}

/// `dec propose <title-or-id> [--extends <id>] [--supersedes <id>]`.
pub fn propose(
    ctx: &mut Ctx,
    no_commit: bool,
    title_or_id: &str,
    extends: Option<&str>,
    supersedes: Option<&str>,
) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;

    let (id, path, mut body) = if record::is_id(title_or_id) {
        let path = record::path_for(title_or_id);
        if !repo.workdir.join(&path).is_file() {
            return Err(Error::Convention(format!(
                "{title_or_id} looks like an id but {} does not exist",
                path.display()
            )));
        }
        let body = std::fs::read_to_string(repo.workdir.join(&path))?;
        let status = read_status_at(&path, &body)?;
        if status != Status::Draft {
            return Err(Error::Convention(format!(
                "{title_or_id} is {}; propose requires draft",
                record::describe(status)
            )));
        }
        let mut body = body;
        record::set_status(&mut body, Status::Proposed)?;
        (title_or_id.to_string(), path, body)
    } else {
        let (id, path) = new_id_and_path(ctx, &repo, title_or_id)?;
        let body = template::decision(title_or_id, Status::Proposed);
        (id, path, body)
    };

    // Reciprocal cross-links (clause 8): prepend to the new/promoted
    // record's Issue in extends-then-supersedes order, and mutate each
    // target once, keyed by path so a target used by both flags keeps both
    // edits.
    let mut targets: HashMap<PathBuf, (String, String)> = HashMap::new(); // path -> (id, body)
    let mut lead_sentences: Vec<String> = Vec::new();

    if let Some(target_id) = extends {
        let target_path = record::path_for(target_id);
        let target_body = read_existing_id(&repo, &target_path, target_id)?;
        let target_status = read_status_at(&target_path, &target_body)?;
        let entry = targets
            .entry(target_path.clone())
            .or_insert((target_id.to_string(), target_body));
        match target_status {
            Status::Draft | Status::Proposed => record::append_to_issue(
                &mut entry.1,
                &format!("This decision is extended by [{id}](./{id}.md)."),
            ),
            Status::Accepted | Status::Rejected => record::append_erratum(
                &mut entry.1,
                &ctx.today.to_string(),
                &format!("Extended by [{id}](./{id}.md)."),
            ),
        }
        lead_sentences.push(format!(
            "This decision extends [{target_id}](./{target_id}.md)."
        ));
    }

    if let Some(target_id) = supersedes {
        let target_path = record::path_for(target_id);
        // Reuse the already-loaded (and possibly extends-mutated) body when
        // the same record was also named by --extends.
        let existing = targets.remove(&target_path);
        let target_body = match existing {
            Some((_, body)) => body,
            None => read_existing_id(&repo, &target_path, target_id)?,
        };
        let target_status = read_status_at(&target_path, &target_body)?;
        if matches!(target_status, Status::Draft | Status::Proposed) {
            return Err(Error::Convention(format!(
                "{target_id} is {}; an unfrozen record is edited in place, not superseded",
                record::describe(target_status)
            )));
        }
        let mut target_body = target_body;
        record::append_erratum(
            &mut target_body,
            &ctx.today.to_string(),
            &format!("Superseded by [{id}](./{id}.md)."),
        );
        targets.insert(target_path, (target_id.to_string(), target_body));
        lead_sentences.push(format!(
            "This decision supersedes [{target_id}](./{target_id}.md)."
        ));
    }

    for sentence in lead_sentences.iter().rev() {
        record::prepend_to_issue(&mut body, sentence);
    }

    let mut files: Vec<(PathBuf, String)> = vec![(path, body)];
    for (path, (_, body)) in targets {
        files.push((path, body));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("decision: propose {id}"),
        &files,
    )
}

/// `dec accept <id>` — ends review, freezing the record as **accepted**.
pub fn accept(ctx: &mut Ctx, no_commit: bool, id: &str) -> Result<(), Error> {
    transition(
        ctx,
        no_commit,
        id,
        Status::Accepted,
        "accept",
        "decision: accept",
    )
}

/// `dec reject <id>` — ends review, freezing the record as **rejected**.
pub fn reject(ctx: &mut Ctx, no_commit: bool, id: &str) -> Result<(), Error> {
    transition(
        ctx,
        no_commit,
        id,
        Status::Rejected,
        "reject",
        "decision: reject",
    )
}

fn transition(
    ctx: &mut Ctx,
    no_commit: bool,
    id: &str,
    to: Status,
    verb: &str,
    subject_prefix: &str,
) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let path = record::path_for(id);
    let mut body = read_existing(&repo, &path, id)?;
    let status = read_status_at(&path, &body)?;
    if status != Status::Proposed {
        return Err(Error::Convention(format!(
            "{id} is {}; {verb} requires proposed",
            record::describe(status)
        )));
    }
    record::set_status(&mut body, to)?;
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("{subject_prefix} {id}"),
        &[(path, body)],
    )
}

/// `dec errata <id> <text>` — the only write v1 makes to a frozen record.
pub fn errata(ctx: &mut Ctx, no_commit: bool, id: &str, text: &str) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let path = record::path_for(id);
    let mut body = read_existing(&repo, &path, id)?;
    let status = read_status_at(&path, &body)?;
    if matches!(status, Status::Draft | Status::Proposed) {
        return Err(Error::Convention(format!(
            "{id} is {}; errata are only appended to a frozen record",
            record::describe(status)
        )));
    }
    record::append_erratum(&mut body, &ctx.today.to_string(), text);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("docs: errata {id}"),
        &[(path, body)],
    )
}

/// Computes `<today>-<slug(title)>` and its path, erroring when a record
/// already exists at that id (clause 7: a same-date slug collision is an
/// error, never a counter or suffix).
fn new_id_and_path(ctx: &Ctx, repo: &git::Repo, title: &str) -> Result<(String, PathBuf), Error> {
    let id = format!("{}-{}", ctx.today, slug(title, DECISION_SLUG_MAX)?);
    let path = record::path_for(&id);
    if repo.workdir.join(&path).is_file() {
        return Err(Error::Convention(format!("{id} already exists")));
    }
    Ok((id, path))
}

fn read_existing_id(repo: &git::Repo, path: &std::path::Path, id: &str) -> Result<String, Error> {
    read_existing(repo, path, id)
}

fn read_status_at(path: &std::path::Path, body: &str) -> Result<Status, Error> {
    record::read_status(body).map_err(|e| Error::Convention(format!("{}: {e}", path.display())))
}
