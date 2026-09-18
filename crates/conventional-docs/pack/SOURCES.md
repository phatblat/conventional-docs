# Pack sources

Every body in this directory is vendored from a canonical upstream source,
retrieved on 2026-09-07. The only edits made to any file are the copyright-line
and reporting-placeholder substitutions listed below. Re-fetch the URL and diff
against the vendored file to review.

## Licenses

Source: `https://raw.githubusercontent.com/spdx/license-list-data/main/text/<SPDX-ID>.txt`
(mirrors `https://spdx.org/licenses/<SPDX-ID>.txt`). License: each license's own
text is itself the license; SPDX's list data is CC0.

| File                   | Edit                                                                                                                                                                     |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Apache-2.0.txt`       | none — verbatim. The `[yyyy] [name of copyright owner]` tokens sit inside the appendix "How to apply the Apache License to your work" and are not a body copyright line. |
| `BSD-2-Clause.txt`     | line 1: `<year>`, `<owner>` → `{{year}}`, `{{holder}}`.                                                                                                                  |
| `BSD-3-Clause.txt`     | line 1: `<year>`, `<owner>` → `{{year}}`, `{{holder}}`.                                                                                                                  |
| `GPL-3.0-or-later.txt` | none — verbatim. The `<year>`/`<name of author>` tokens sit inside the appendix "How to Apply These Terms to Your New Programs" and are not a body copyright line.       |
| `ISC.txt`              | line 3: `<year>`, `<owner>` → `{{year}}`, `{{holder}}`.                                                                                                                  |
| `MIT.txt`              | line 3: `<year>`, `<copyright holders>` → `{{year}}`, `{{holder}}`.                                                                                                      |
| `MPL-2.0.txt`          | none — verbatim; MPL-2.0 carries no copyright-line placeholder.                                                                                                          |
| `Unlicense.txt`        | none — verbatim; the Unlicense carries no copyright-line placeholder.                                                                                                    |

## Codes of conduct

| File                           | Source                                                                                | License      | Edit                                                                                                                                                        |
| ------------------------------ | ------------------------------------------------------------------------------------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `contributor-covenant-3.0.txt` | `https://www.contributor-covenant.org/version/3/0/code_of_conduct/code_of_conduct.md` | CC BY-SA 4.0 | `## Reporting an Issue`: `**[NOTE: describe your means of reporting here.]**` → `**{{contact}}**`. The document's own attribution section is kept verbatim. |
