use std::path::PathBuf;

use crate::error::Error;

/// A decision record's four states, moving one direction:
/// `Draft -> Proposed -> Accepted | Rejected`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Draft,
    Proposed,
    Accepted,
    Rejected,
}

/// The convention's four `## Status` body lines, verbatim
/// (`skills/conventional-docs/SKILL.md`, "Status lifecycle").
const STATUS_LINES: &[(Status, &str)] = &[
    (
        Status::Draft,
        "This is a **draft**; it is not ready for review.",
    ),
    (
        Status::Proposed,
        "This is a proposal that is **awaiting review**.",
    ),
    (Status::Accepted, "This is a proposal that is **accepted**."),
    (Status::Rejected, "This proposal was **rejected**."),
];

pub fn status_line(s: Status) -> &'static str {
    STATUS_LINES
        .iter()
        .find(|(status, _)| *status == s)
        .map(|(_, line)| *line)
        .unwrap()
}

/// Lowercase state name for error messages (`"draft"`, `"proposed"`, …).
pub fn describe(s: Status) -> &'static str {
    match s {
        Status::Draft => "draft",
        Status::Proposed => "proposed",
        Status::Accepted => "accepted",
        Status::Rejected => "rejected",
    }
}

/// `docs/decisions/<id>.md`.
pub fn path_for(id: &str) -> PathBuf {
    PathBuf::from("docs/decisions").join(format!("{id}.md"))
}

const STATUS_ERROR: &str = "## Status is not one of the convention's four lines";

/// Finds the `## Status` heading, takes its section body up to the next `## `
/// heading, trims it, and matches it against the convention's four lines. A
/// missing heading and an unrecognized line are the same error.
pub fn read_status(body: &str) -> Result<Status, Error> {
    let lines: Vec<&str> = body.split('\n').collect();
    let (start, end) = section_bounds(&lines, "## Status")
        .ok_or_else(|| Error::Convention(STATUS_ERROR.to_string()))?;
    let section = lines[start + 1..end].join("\n");
    let trimmed = section.trim();
    STATUS_LINES
        .iter()
        .find(|(_, line)| *line == trimmed)
        .map(|(status, _)| *status)
        .ok_or_else(|| Error::Convention(STATUS_ERROR.to_string()))
}

/// Replaces the `## Status` section body with a blank line, `status_line(s)`,
/// and a blank line — the skeleton's exact shape.
pub fn set_status(body: &mut String, s: Status) -> Result<(), Error> {
    let lines: Vec<&str> = body.split('\n').collect();
    let (start, end) = section_bounds(&lines, "## Status")
        .ok_or_else(|| Error::Convention(STATUS_ERROR.to_string()))?;

    let mut new_lines: Vec<String> = lines[..=start].iter().map(|s| s.to_string()).collect();
    new_lines.push(String::new());
    new_lines.push(status_line(s).to_string());
    new_lines.push(String::new());
    new_lines.extend(lines[end..].iter().map(|s| s.to_string()));

    *body = new_lines.join("\n");
    Ok(())
}

/// Appends `- <date>: <text>` as the last line of `## Errata`, creating the
/// section at end of file when absent. Never touches an existing line.
pub fn append_erratum(body: &mut String, date: &str, text: &str) {
    let entry = format!("- {date}: {text}");
    let trimmed = body.trim_end_matches('\n');
    if body.contains("## Errata") {
        *body = format!("{trimmed}\n{entry}\n");
    } else {
        *body = format!("{trimmed}\n\n## Errata\n\n{entry}\n");
    }
}

/// Appends `sentence` as its own paragraph at the end of `## Issue`'s body.
/// A no-op when the sentence is already present in that section.
pub fn append_to_issue(body: &mut String, sentence: &str) {
    splice_section(body, "## Issue", |section| {
        if section.iter().any(|l| l == sentence) {
            return section;
        }
        let mut out = section;
        out.push(sentence.to_string());
        out.push(String::new());
        out
    });
}

/// Prepends `sentence` as its own paragraph at the start of `## Issue`'s
/// body. A no-op when the sentence is already present in that section.
pub fn prepend_to_issue(body: &mut String, sentence: &str) {
    splice_section(body, "## Issue", |section| {
        if section.iter().any(|l| l == sentence) {
            return section;
        }
        let rest: &[String] = if section.first().map(String::as_str) == Some("") {
            &section[1..]
        } else {
            &section[..]
        };
        let mut out = vec![String::new(), sentence.to_string(), String::new()];
        out.extend_from_slice(rest);
        out
    });
}

/// `^\d{4}-\d{2}-\d{2}-[a-z0-9-]+$`, hand-rolled so the crate needs no regex
/// dependency for a single fixed-shape check.
pub fn is_id(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() < 12 {
        return false;
    }
    let digit = |b: u8| b.is_ascii_digit();
    let date_shape = digit(bytes[0])
        && digit(bytes[1])
        && digit(bytes[2])
        && digit(bytes[3])
        && bytes[4] == b'-'
        && digit(bytes[5])
        && digit(bytes[6])
        && bytes[7] == b'-'
        && digit(bytes[8])
        && digit(bytes[9])
        && bytes[10] == b'-';
    if !date_shape {
        return false;
    }
    let rest = &s[11..];
    !rest.is_empty()
        && rest
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Finds `heading`'s line index and the index of the next `## `-prefixed
/// heading (or the end of `lines`), i.e. the `(start, end)` such that the
/// section body is `lines[start + 1..end]`.
fn section_bounds(lines: &[&str], heading: &str) -> Option<(usize, usize)> {
    let start = lines.iter().position(|l| *l == heading)?;
    let end = lines[start + 1..]
        .iter()
        .position(|l| l.starts_with("## "))
        .map(|i| start + 1 + i)
        .unwrap_or(lines.len());
    Some((start, end))
}

/// Replaces `heading`'s section body with `f`'s result, leaving the rest of
/// the document untouched. A no-op when `heading` is absent.
fn splice_section(body: &mut String, heading: &str, f: impl FnOnce(Vec<String>) -> Vec<String>) {
    let lines: Vec<&str> = body.split('\n').collect();
    let Some((start, end)) = section_bounds(&lines, heading) else {
        return;
    };
    let section: Vec<String> = lines[start + 1..end]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let new_section = f(section);

    let mut new_lines: Vec<String> = lines[..=start].iter().map(|s| s.to_string()).collect();
    new_lines.extend(new_section);
    new_lines.extend(lines[end..].iter().map(|s| s.to_string()));

    *body = new_lines.join("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECORD: &str = "# Cache the resolver output\n\n\
## Issue\n\n\
The resolver recomputes on every call.\n\n\
## Status\n\n\
This is a proposal that is **awaiting review**.\n\n\
## Assumptions and Constraints\n\n\
- N/A\n";

    #[test]
    fn read_status_finds_proposed() {
        assert_eq!(read_status(RECORD).unwrap(), Status::Proposed);
    }

    #[test]
    fn read_status_rejects_unrecognized_line() {
        let body = RECORD.replace(
            "This is a proposal that is **awaiting review**.",
            "This is somehow both accepted and rejected.",
        );
        let err = read_status(&body).unwrap_err();
        assert_eq!(err.to_string(), STATUS_ERROR);
    }

    #[test]
    fn read_status_rejects_missing_heading() {
        let body = RECORD.replace("## Status", "## Statuses");
        let err = read_status(&body).unwrap_err();
        assert_eq!(err.to_string(), STATUS_ERROR);
    }

    #[test]
    fn set_status_normalizes_section_shape() {
        let mut body = RECORD.to_string();
        set_status(&mut body, Status::Accepted).unwrap();
        assert_eq!(read_status(&body).unwrap(), Status::Accepted);
        assert!(
            body.contains(
                "## Status\n\nThis is a proposal that is **accepted**.\n\n## Assumptions"
            )
        );
    }

    #[test]
    fn append_erratum_creates_section_then_appends() {
        let mut body = RECORD.to_string();
        append_erratum(&mut body, "2026-09-06", "First correction.");
        assert!(body.ends_with("## Errata\n\n- 2026-09-06: First correction.\n"));
        append_erratum(&mut body, "2026-09-07", "Second correction.");
        assert!(
            body.ends_with("- 2026-09-06: First correction.\n- 2026-09-07: Second correction.\n")
        );
    }

    #[test]
    fn append_to_issue_adds_trailing_paragraph_once() {
        let mut body = RECORD.to_string();
        let sentence = "This decision is extended by [2026-09-06-x](./2026-09-06-x.md).";
        append_to_issue(&mut body, sentence);
        assert!(body.contains("recomputes on every call.\n\nThis decision is extended by"));
        let before = body.clone();
        append_to_issue(&mut body, sentence);
        assert_eq!(body, before, "second call must be a no-op");
    }

    #[test]
    fn prepend_to_issue_adds_leading_paragraph_once() {
        let mut body = RECORD.to_string();
        let sentence = "This decision extends [2026-09-06-x](./2026-09-06-x.md).";
        prepend_to_issue(&mut body, sentence);
        assert!(body.contains(
            "## Issue\n\nThis decision extends [2026-09-06-x](./2026-09-06-x.md).\n\nThe resolver"
        ));
        let before = body.clone();
        prepend_to_issue(&mut body, sentence);
        assert_eq!(body, before, "second call must be a no-op");
    }

    #[test]
    fn is_id_matches_the_convention_shape() {
        assert!(is_id("2026-09-06-cache-the-resolver-output"));
        assert!(!is_id("Cache the resolver output"));
        assert!(!is_id("2026-09-06-"));
        assert!(!is_id("2026-9-06-x"));
    }
}
