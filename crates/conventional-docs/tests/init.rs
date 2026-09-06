mod common;

use common::*;

#[test]
fn fresh_repo_gets_four_files_with_golden_bytes() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["init"]), 0);

    assert_eq!(read(&f, "CHARTER.md"), golden("charter.md"));
    assert_eq!(read(&f, "DESIGN.md"), golden("design.md"));
    assert_eq!(read(&f, "ROADMAP.md"), golden("roadmap.md"));
    assert_eq!(read(&f, "CHANGELOG.md"), golden("changelog.md"));

    assert_eq!(
        log_subjects(&f)[0],
        "docs: add charter, design, roadmap, changelog"
    );

    let output = take_output(&f);
    assert!(output.contains("CHANGELOG.md: no [Unreleased] compare link"));
    assert!(output.contains("commitlint: no config found"));
}

#[test]
fn rerun_with_nothing_missing_writes_nothing() {
    let mut f = repo();
    run(&mut f, &["init"]);
    take_output(&f);
    let commits_before = log_subjects(&f).len();

    assert_eq!(run(&mut f, &["init"]), 0);

    assert_eq!(log_subjects(&f).len(), commits_before);
    assert!(take_output(&f).starts_with("nothing missing"));
}

#[test]
fn add_after_deleting_design_creates_only_design() {
    let mut f = repo();
    run(&mut f, &["init"]);
    take_output(&f);
    run_git(&f, &["rm", "-q", "DESIGN.md"]);
    run_git(
        &f,
        &[
            "commit",
            "-q",
            "-m",
            "chore: remove design",
            "--",
            "DESIGN.md",
        ],
    );

    assert_eq!(run(&mut f, &["init", "--add"]), 0);

    assert!(exists(&f, "DESIGN.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add design");
    assert_eq!(commit_paths(&f), vec!["DESIGN.md".to_string()]);
}

#[test]
fn charter_present_under_docs_is_treated_as_present() {
    let mut f = repo();
    std::fs::create_dir_all(f.dir.path().join("docs")).unwrap();
    std::fs::write(f.dir.path().join("docs/charter.md"), "# Charter\n").unwrap();

    assert_eq!(run(&mut f, &["init", "--add"]), 0);

    assert!(
        !exists(&f, "CHARTER.md"),
        "root CHARTER.md must not be created when docs/charter.md exists"
    );
    assert!(exists(&f, "DESIGN.md"));
    assert!(exists(&f, "ROADMAP.md"));
    assert!(exists(&f, "CHANGELOG.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add design, roadmap, changelog");
}

#[test]
fn github_origin_with_a_tag_produces_the_linked_changelog() {
    let mut f = repo();
    run_git(
        &f,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/example/repo.git",
        ],
    );
    run_git(&f, &["tag", "v1.2.0"]);

    assert_eq!(run(&mut f, &["init"]), 0);

    assert_eq!(read(&f, "CHANGELOG.md"), golden("changelog-linked.md"));
    let output = take_output(&f);
    assert!(!output.contains("no [Unreleased] compare link"));
}

#[test]
fn commitlint_config_missing_four_types_is_reported() {
    let mut f = repo();
    std::fs::write(
        f.dir.path().join("commitlint.config.js"),
        "module.exports = { rules: { 'type-enum': [2, 'always', ['feat', 'fix', 'decision']] } };\n",
    )
    .unwrap();

    assert_eq!(run(&mut f, &["init"]), 0);

    let output = take_output(&f);
    assert!(output.contains("commitlint: type-enum is missing deploy, plan, release, todo"));
}

fn run_git(f: &Fixture, args: &[&str]) {
    let status = std::process::Command::new("git")
        .args(args)
        .current_dir(f.dir.path())
        .envs(
            f.ctx
                .git_env
                .iter()
                .map(|(k, v)| (k.as_os_str(), v.as_os_str())),
        )
        .status()
        .expect("spawn git");
    assert!(status.success(), "git {args:?} failed");
}
