use crate::error::Error;

/// The commit-subject-derived cap for a decision id's slug.
///
/// `decision: propose ` (18) + `YYYY-MM-DD-` (11) + 71 = 100, the
/// `@commitlint/config-conventional` `header-max-length`.
pub const DECISION_SLUG_MAX: usize = 71;

/// `docs: add incident ` (19) + `YYYY-MM-DD-` (11) + 70 = 100.
pub const INCIDENT_SLUG_MAX: usize = 70;

/// `docs: add runbook ` (18) + 71 = 89.
pub const RUNBOOK_SLUG_MAX: usize = 71;

/// Normalizes `text` into a kebab-case slug of at most `max` characters.
///
/// Lowercases, keeps ASCII alphanumerics, maps every other byte (including
/// non-ASCII) to `-`, collapses runs of `-`, trims leading/trailing `-`,
/// truncates to `max` characters, then trims a trailing `-` left by
/// truncation. An empty result is a usage error (exit 2): the caller gave
/// nothing usable as an id.
pub fn slug(text: &str, max: usize) -> Result<String, Error> {
    let mut out = String::with_capacity(text.len());
    let mut last_was_dash = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            out.push('-');
            last_was_dash = true;
        }
    }

    let trimmed = out.trim_matches('-');
    let truncated: String = trimmed.chars().take(max).collect();
    let result = truncated.trim_end_matches('-').to_string();

    if result.is_empty() {
        return Err(Error::Usage(format!(
            "{text:?} contains no characters usable in a slug"
        )));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_and_dashes() {
        assert_eq!(
            slug("Cache the Resolver Output", 71).unwrap(),
            "cache-the-resolver-output"
        );
    }

    #[test]
    fn collapses_punctuation_and_non_ascii() {
        assert_eq!(slug("Café!! --- Déjà vu???", 71).unwrap(), "caf-d-j-vu");
    }

    #[test]
    fn truncates_at_cap_and_trims_trailing_dash() {
        // "a" * 70 + "-b" — truncating at 70 must not leave a trailing dash.
        let text = format!("{}-b", "a".repeat(70));
        assert_eq!(slug(&text, 70).unwrap(), "a".repeat(70));
    }

    #[test]
    fn empty_result_is_usage_error() {
        let err = slug("!!!???", 71).unwrap_err();
        assert!(matches!(err, Error::Usage(_)));
        assert_eq!(err.exit_code(), 2);
    }

    #[test]
    fn decision_incident_runbook_caps_are_100_header() {
        assert_eq!(
            "decision: propose ".len() + "YYYY-MM-DD-".len() + DECISION_SLUG_MAX,
            100
        );
        assert_eq!(
            "docs: add incident ".len() + "YYYY-MM-DD-".len() + INCIDENT_SLUG_MAX,
            100
        );
        assert_eq!("docs: add runbook ".len() + RUNBOOK_SLUG_MAX, 89);
    }
}
