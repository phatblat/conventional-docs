//! The standards pack: third-party license and code-of-conduct bodies, kept
//! as plain data so they version independently of the binary (decision
//! clauses 10-13 of
//! `docs/decisions/2026-09-07-adjacent-files-and-the-standards-pack.md`).

use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Error;

/// The two kinds of body a pack carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    License,
    CodeOfConduct,
}

impl Kind {
    /// The subdirectory a pack keeps this kind's bodies under, in both an
    /// external pack directory and the embedded pack.
    fn dir_name(self) -> &'static str {
        match self {
            Kind::License => "licenses",
            Kind::CodeOfConduct => "codes-of-conduct",
        }
    }

    /// The word used in a `docs: add <name> <id>` commit subject and in
    /// `pack list` output.
    pub fn label(self) -> &'static str {
        match self {
            Kind::License => "license",
            Kind::CodeOfConduct => "code-of-conduct",
        }
    }
}

/// Where a resolved entry's body came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Embedded,
    Dir(PathBuf),
}

impl Source {
    pub fn describe(&self) -> String {
        match self {
            Source::Embedded => "embedded".to_string(),
            Source::Dir(dir) => dir.display().to_string(),
        }
    }
}

/// One body a resolved pack can serve.
#[derive(Debug, Clone)]
pub struct Entry {
    pub kind: Kind,
    pub id: String,
    pub source: Source,
}

/// The embedded license bodies: `(SPDX id, verbatim or `{{year}}`/`{{holder}}`
/// normalized text)`. See `pack/SOURCES.md` for the exact edit each carries.
const EMBEDDED_LICENSES: &[(&str, &str)] = &[
    (
        "Apache-2.0",
        include_str!("../pack/licenses/Apache-2.0.txt"),
    ),
    (
        "BSD-2-Clause",
        include_str!("../pack/licenses/BSD-2-Clause.txt"),
    ),
    (
        "BSD-3-Clause",
        include_str!("../pack/licenses/BSD-3-Clause.txt"),
    ),
    (
        "GPL-3.0-or-later",
        include_str!("../pack/licenses/GPL-3.0-or-later.txt"),
    ),
    ("ISC", include_str!("../pack/licenses/ISC.txt")),
    ("MIT", include_str!("../pack/licenses/MIT.txt")),
    ("MPL-2.0", include_str!("../pack/licenses/MPL-2.0.txt")),
    ("Unlicense", include_str!("../pack/licenses/Unlicense.txt")),
];

/// The embedded code-of-conduct bodies: `(id, `{{contact}}`-normalized text)`.
const EMBEDDED_CODES_OF_CONDUCT: &[(&str, &str)] = &[(
    "contributor-covenant-3.0",
    include_str!("../pack/codes-of-conduct/contributor-covenant-3.0.txt"),
)];

fn embedded_table(kind: Kind) -> &'static [(&'static str, &'static str)] {
    match kind {
        Kind::License => EMBEDDED_LICENSES,
        Kind::CodeOfConduct => EMBEDDED_CODES_OF_CONDUCT,
    }
}

/// A resolved standards pack: zero or more external directories, checked
/// before the embedded pack, in order.
pub struct Pack {
    dirs: Vec<PathBuf>,
}

/// The default user data directory: `$XDG_DATA_HOME/conventional-docs/pack`,
/// or `$HOME/.local/share/conventional-docs/pack` when `XDG_DATA_HOME` is
/// unset.
fn user_pack_dir() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share"))
        })?;
    Some(base.join("conventional-docs/pack"))
}

/// Resolves the pack: `explicit`, then the user data directory (if it
/// exists), then the embedded pack (always present). `explicit` naming a path
/// that is not a directory is a usage error (exit 2); a missing user data
/// directory is not an error — it simply carries nothing.
pub fn resolve(explicit: Option<&Path>) -> Result<Pack, Error> {
    let mut dirs = Vec::new();

    if let Some(dir) = explicit {
        if !dir.is_dir() {
            return Err(Error::Usage(format!(
                "--pack {} is not a directory",
                dir.display()
            )));
        }
        dirs.push(dir.to_path_buf());
    }

    if let Some(dir) = user_pack_dir()
        && dir.is_dir()
    {
        dirs.push(dir);
    }

    Ok(Pack { dirs })
}

impl Pack {
    /// The body for `(kind, id)` from the first source that carries it:
    /// each external directory in order, then the embedded pack.
    pub fn body(&self, kind: Kind, id: &str) -> Result<Cow<'static, str>, Error> {
        if !is_valid_id(id) {
            return Err(Error::Usage(format!(
                "invalid {} id '{id}': expected [A-Za-z0-9][A-Za-z0-9.+-]*",
                kind.label()
            )));
        }

        for dir in &self.dirs {
            let path = dir.join(kind.dir_name()).join(format!("{id}.txt"));
            if path.is_file() {
                let body = fs::read_to_string(&path)?;
                return Ok(Cow::Owned(body));
            }
        }

        if let Some((_, body)) = embedded_table(kind).iter().find(|(k, _)| *k == id) {
            return Ok(Cow::Borrowed(body));
        }

        let available: Vec<String> = self
            .entries()
            .into_iter()
            .filter(|e| e.kind == kind)
            .map(|e| e.id)
            .collect();
        let user_dir = user_pack_dir()
            .map(|d| d.display().to_string())
            .unwrap_or_else(|| "<user pack directory>".to_string());
        Err(Error::Convention(format!(
            "no {} '{id}' in the resolved pack (have: {}); add {}/{}/{id}.txt to carry it",
            kind.label(),
            available.join(", "),
            user_dir,
            kind.dir_name()
        )))
    }

    /// Every entry the resolved pack can serve, sorted by kind then id, with
    /// each `(kind, id)` reported once from its first (highest-priority)
    /// source.
    pub fn entries(&self) -> Vec<Entry> {
        let mut seen = std::collections::BTreeSet::new();
        let mut entries = Vec::new();

        for kind in [Kind::License, Kind::CodeOfConduct] {
            for dir in &self.dirs {
                let sub = dir.join(kind.dir_name());
                let Ok(read_dir) = fs::read_dir(&sub) else {
                    continue;
                };
                let mut ids: Vec<String> = read_dir
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let name = e.file_name();
                        let name = name.to_str()?;
                        name.strip_suffix(".txt").map(str::to_string)
                    })
                    .collect();
                ids.sort();
                for id in ids {
                    if seen.insert((kind, id.clone())) {
                        entries.push(Entry {
                            kind,
                            id,
                            source: Source::Dir(dir.clone()),
                        });
                    }
                }
            }

            for (id, _) in embedded_table(kind) {
                if seen.insert((kind, id.to_string())) {
                    entries.push(Entry {
                        kind,
                        id: id.to_string(),
                        source: Source::Embedded,
                    });
                }
            }
        }

        entries
    }
}

/// Whether `id` is safe to join onto a pack directory path: matches
/// `^[A-Za-z0-9][A-Za-z0-9.+-]*$`, the charset `cmd/adjacent.rs` already
/// assumes for every id it writes. Rejects `..`, `/`, and any other
/// character that could escape the pack directory.
fn is_valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphanumeric() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '+' | '-'))
}

/// Substitutes each `{{name}}` in `body` with its value from `vars`, then
/// fails if any `{{...}}` token survives — an unset or misspelled
/// placeholder, rather than a silently blank one.
pub fn render(body: &str, vars: &[(&str, &str)]) -> Result<String, Error> {
    let mut out = body.to_string();
    for (name, value) in vars {
        out = out.replace(&format!("{{{{{name}}}}}"), value);
    }

    if let Some(start) = out.find("{{") {
        let end = out[start..].find("}}").map(|i| start + i + 2);
        let token = match end {
            Some(end) => &out[start..end],
            None => &out[start..],
        };
        return Err(Error::Convention(format!(
            "pack body has an unfilled placeholder: {token}"
        )));
    }

    Ok(out)
}
