mod common;

use common::*;

#[test]
fn readme_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["readme", "new"]), 0);
    assert_eq!(read(&f, "README.md"), golden("readme.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add readme");
    assert_eq!(commit_paths(&f), vec!["README.md".to_string()]);
}

#[test]
fn contributing_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["contributing", "new"]), 0);
    assert_eq!(read(&f, "CONTRIBUTING.md"), golden("contributing.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add contributing");
}

#[test]
fn security_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["security", "new"]), 0);
    assert_eq!(read(&f, "SECURITY.md"), golden("security.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add security");
}

#[test]
fn support_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["support", "new"]), 0);
    assert_eq!(read(&f, "SUPPORT.md"), golden("support.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add support");
}

#[test]
fn contributing_refuses_when_a_github_copy_already_exists() {
    let mut f = repo();
    std::fs::create_dir_all(f.dir.path().join(".github")).unwrap();
    std::fs::write(
        f.dir.path().join(".github/CONTRIBUTING.md"),
        "# Contributing\n",
    )
    .unwrap();
    let commits_before = log_subjects(&f).len();

    assert_eq!(run(&mut f, &["contributing", "new"]), 1);

    assert!(!exists(&f, "CONTRIBUTING.md"));
    assert_eq!(log_subjects(&f).len(), commits_before);
}

#[test]
fn license_new_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(
        run(
            &mut f,
            &[
                "license",
                "new",
                "--spdx",
                "MIT",
                "--holder",
                "Ada Lovelace",
                "--year",
                "2031",
            ]
        ),
        0
    );
    assert_eq!(read(&f, "LICENSE.md"), golden("license-mit.txt"));
    assert!(!read(&f, "LICENSE.md").contains("{{"));
    assert_eq!(log_subjects(&f)[0], "docs: add license MIT");
}

#[test]
fn license_new_refuses_when_copying_already_exists() {
    let mut f = repo();
    std::fs::write(f.dir.path().join("COPYING"), "existing license\n").unwrap();
    let commits_before = log_subjects(&f).len();

    assert_eq!(
        run(
            &mut f,
            &["license", "new", "--spdx", "MIT", "--holder", "X"]
        ),
        1
    );

    assert!(!exists(&f, "LICENSE.md"));
    assert_eq!(log_subjects(&f).len(), commits_before);
}

#[test]
fn code_of_conduct_new_writes_the_contact_and_commits() {
    let mut f = repo();
    assert_eq!(
        run(
            &mut f,
            &["coc", "new", "--contact", "conduct@example.invalid"]
        ),
        0
    );
    let body = read(&f, "CODE_OF_CONDUCT.md");
    assert!(body.contains("conduct@example.invalid"));
    assert!(!body.contains("{{"));
    assert_eq!(
        log_subjects(&f)[0],
        "docs: add code-of-conduct contributor-covenant-3.0"
    );
}
