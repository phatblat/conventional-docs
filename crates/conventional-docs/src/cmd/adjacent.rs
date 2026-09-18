use std::io::Write;

use super::{already_exists, write_files};
use crate::adjacent::{self, Adjacent};
use crate::ctx::Ctx;
use crate::error::Error;
use crate::git;
use crate::pack::{self, Kind};
use crate::template;

/// Writes one of the four convention-owned templates (`readme`,
/// `contributing`, `security`, `support`) at its root path, refusing when the
/// file already exists anywhere its host recognizes.
fn create_templated(
    ctx: &mut Ctx,
    no_commit: bool,
    a: Adjacent,
    contents: &'static str,
) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let existing = adjacent::locate_all(&repo, a);
    if let Some(path) = existing.first() {
        return Err(already_exists(a.event_name(), path));
    }
    let path = adjacent::create_path(a);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("docs: add {}", a.event_name()),
        &[(path, contents.to_string())],
    )
}

/// `readme new` — `README.md`.
pub fn readme(ctx: &mut Ctx, no_commit: bool) -> Result<(), Error> {
    create_templated(ctx, no_commit, Adjacent::Readme, template::readme())
}

/// `contributing new` — `CONTRIBUTING.md`.
pub fn contributing(ctx: &mut Ctx, no_commit: bool) -> Result<(), Error> {
    create_templated(
        ctx,
        no_commit,
        Adjacent::Contributing,
        template::contributing(),
    )
}

/// `security new` — `SECURITY.md`.
pub fn security(ctx: &mut Ctx, no_commit: bool) -> Result<(), Error> {
    create_templated(ctx, no_commit, Adjacent::Security, template::security())
}

/// `support new` — `SUPPORT.md`.
pub fn support(ctx: &mut Ctx, no_commit: bool) -> Result<(), Error> {
    create_templated(ctx, no_commit, Adjacent::Support, template::support())
}

/// The `user.name` configured for the repository (local, global, or system,
/// whichever `gix` resolves), used as `license new`'s default `--holder`.
fn configured_user_name(repo: &git::Repo) -> Option<String> {
    let value = repo.repo.config_snapshot().string("user.name")?;
    Some(String::from_utf8_lossy(value.as_slice()).into_owned())
}

/// `license new --spdx <id> [--holder] [--year] [--pack]` — `LICENSE.md`,
/// rendered from the standards pack.
pub fn license(
    ctx: &mut Ctx,
    no_commit: bool,
    spdx: &str,
    holder: Option<&str>,
    year: Option<i16>,
    pack_dir: Option<&std::path::Path>,
) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let existing = adjacent::locate_all(&repo, Adjacent::License);
    if let Some(path) = existing.first() {
        return Err(already_exists("license", path));
    }

    let holder = match holder {
        Some(h) => h.to_string(),
        None => configured_user_name(&repo).ok_or_else(|| {
            Error::Usage(
                "no --holder given and no user.name configured; pass --holder or set user.name"
                    .to_string(),
            )
        })?,
    };
    let year = year.unwrap_or_else(|| ctx.today.year());

    let resolved = pack::resolve(pack_dir)?;
    let body = resolved.body(Kind::License, spdx)?;
    let rendered = pack::render(&body, &[("year", &year.to_string()), ("holder", &holder)])?;
    let rendered = ensure_trailing_newline(rendered);

    let path = adjacent::create_path(Adjacent::License);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("docs: add license {spdx}"),
        &[(path, rendered)],
    )?;

    if spdx == "Apache-2.0" {
        writeln!(
            ctx.out,
            "Apache-2.0 expects a separate NOTICE file; this command does not write one."
        )?;
    }
    writeln!(
        ctx.out,
        "LICENSE.md is verbatim legal text; exclude it from formatters that reflow prose."
    )?;

    Ok(())
}

/// `code-of-conduct new --contact <method> [--standard] [--pack]` (alias
/// `coc`) — `CODE_OF_CONDUCT.md`, rendered from the standards pack.
pub fn code_of_conduct(
    ctx: &mut Ctx,
    no_commit: bool,
    contact: &str,
    standard: &str,
    pack_dir: Option<&std::path::Path>,
) -> Result<(), Error> {
    let repo = git::discover(&ctx.cwd)?;
    let existing = adjacent::locate_all(&repo, Adjacent::CodeOfConduct);
    if let Some(path) = existing.first() {
        return Err(already_exists("code-of-conduct", path));
    }

    let resolved = pack::resolve(pack_dir)?;
    let body = resolved.body(Kind::CodeOfConduct, standard)?;
    let rendered = pack::render(&body, &[("contact", contact)])?;
    let rendered = ensure_trailing_newline(rendered);

    let path = adjacent::create_path(Adjacent::CodeOfConduct);
    write_files(
        ctx,
        &repo,
        no_commit,
        &format!("docs: add code-of-conduct {standard}"),
        &[(path, rendered)],
    )
}

fn ensure_trailing_newline(mut body: String) -> String {
    if !body.ends_with('\n') {
        body.push('\n');
    }
    body
}

/// `pack list [--json] [--pack <dir>]` — reports every entry the resolved
/// pack can serve and which source it came from. Writes nothing and commits
/// nothing.
pub fn pack_list(
    ctx: &mut Ctx,
    json: bool,
    pack_dir: Option<&std::path::Path>,
) -> Result<(), Error> {
    let resolved = pack::resolve(pack_dir)?;
    let entries = resolved.entries();

    if json {
        let mut out = String::from("[");
        for (i, entry) in entries.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            out.push_str(&format!(
                "{{\"kind\":\"{}\",\"id\":\"{}\",\"source\":\"{}\"}}",
                entry.kind.label(),
                json_escape(&entry.id),
                json_escape(&entry.source.describe()),
            ));
        }
        out.push(']');
        writeln!(ctx.out, "{out}")?;
    } else {
        for entry in &entries {
            writeln!(
                ctx.out,
                "{}\t{}\t{}",
                entry.kind.label(),
                entry.id,
                entry.source.describe()
            )?;
        }
    }

    Ok(())
}

/// Escapes `"`, `\`, and raw control bytes (`\n`, `\r`, `\t`, and the rest of
/// `U+0000..=U+001F`) for the flat `--json` array. Ids and kinds are
/// `[A-Za-z0-9.+-]` and never need it, but a `--pack <dir>` path can legally
/// contain any of these on Unix, and `pack list --json` is documented for
/// hook consumers expecting valid JSON.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}
