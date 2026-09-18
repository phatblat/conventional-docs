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

#[test]
fn code_of_conduct_new_with_an_unknown_standard_id_fails_and_writes_nothing() {
    let mut f = repo();
    let (code, message) = run_err(
        &mut f,
        &[
            "coc",
            "new",
            "--contact",
            "conduct@example.invalid",
            "--standard",
            "made-up-standard-9.9",
        ],
    );
    assert_eq!(code, 1);
    assert!(
        message.contains("contributor-covenant-3.0"),
        "message: {message}"
    );
    assert!(!exists(&f, "CODE_OF_CONDUCT.md"));
}

#[test]
fn license_new_with_a_unicode_holder_renders_it() {
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
                "Ολυμπία Δέσποινα",
                "--year",
                "2031",
            ]
        ),
        0
    );
    assert!(read(&f, "LICENSE.md").contains("Ολυμπία Δέσποινα"));
}

#[test]
fn code_of_conduct_new_with_a_unicode_contact_renders_it() {
    let mut f = repo();
    assert_eq!(
        run(
            &mut f,
            &["coc", "new", "--contact", "維護者@example.invalid"]
        ),
        0
    );
    assert!(read(&f, "CODE_OF_CONDUCT.md").contains("維護者@example.invalid"));
}

#[test]
fn readme_refuses_when_the_exact_primary_path_already_exists() {
    let mut f = repo();
    std::fs::write(f.dir.path().join("README.md"), "# Existing\n").unwrap();
    let commits_before = log_subjects(&f).len();

    assert_eq!(run(&mut f, &["readme", "new"]), 1);

    assert_eq!(read(&f, "README.md"), "# Existing\n");
    assert_eq!(log_subjects(&f).len(), commits_before);
}

#[test]
fn security_refuses_when_the_exact_primary_path_already_exists() {
    let mut f = repo();
    std::fs::write(f.dir.path().join("SECURITY.md"), "# Existing\n").unwrap();
    let commits_before = log_subjects(&f).len();

    assert_eq!(run(&mut f, &["security", "new"]), 1);

    assert_eq!(read(&f, "SECURITY.md"), "# Existing\n");
    assert_eq!(log_subjects(&f).len(), commits_before);
}

#[test]
fn support_refuses_when_the_exact_primary_path_already_exists() {
    let mut f = repo();
    std::fs::write(f.dir.path().join("SUPPORT.md"), "# Existing\n").unwrap();
    let commits_before = log_subjects(&f).len();

    assert_eq!(run(&mut f, &["support", "new"]), 1);

    assert_eq!(read(&f, "SUPPORT.md"), "# Existing\n");
    assert_eq!(log_subjects(&f).len(), commits_before);
}

#[test]
fn license_new_refuses_when_the_exact_primary_path_already_exists() {
    let mut f = repo();
    std::fs::write(f.dir.path().join("LICENSE.md"), "existing license\n").unwrap();
    let commits_before = log_subjects(&f).len();

    assert_eq!(
        run(
            &mut f,
            &["license", "new", "--spdx", "MIT", "--holder", "X"]
        ),
        1
    );

    assert_eq!(read(&f, "LICENSE.md"), "existing license\n");
    assert_eq!(log_subjects(&f).len(), commits_before);
}
