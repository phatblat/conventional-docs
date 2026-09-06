mod common;

use common::*;

#[test]
fn charter_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["new", "charter"]), 0);
    assert_eq!(read(&f, "CHARTER.md"), golden("charter.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add charter");
    assert_eq!(commit_paths(&f), vec!["CHARTER.md".to_string()]);
}

#[test]
fn design_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["new", "design"]), 0);
    assert_eq!(read(&f, "DESIGN.md"), golden("design.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add design");
}

#[test]
fn roadmap_writes_golden_bytes_and_commits() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["new", "roadmap"]), 0);
    assert_eq!(read(&f, "ROADMAP.md"), golden("roadmap.md"));
    assert_eq!(log_subjects(&f)[0], "docs: add roadmap");
}

#[test]
fn runbook_writes_golden_bytes_at_the_slugged_path() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["new", "runbook", "Disk full"]), 0);
    assert_eq!(read(&f, "docs/runbooks/disk-full.md"), golden("runbook.md"));
    assert!(read(&f, "docs/runbooks/disk-full.md").starts_with("# Runbook: disk-full\n"));
    assert_eq!(log_subjects(&f)[0], "docs: add runbook disk-full");
}

#[test]
fn incident_writes_golden_bytes_at_the_dated_path() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["new", "incident", "Database outage"]), 0);
    let path = "docs/incidents/2026-09-06-database-outage.md";
    assert_eq!(read(&f, path), golden("incident.md"));
    assert_eq!(
        log_subjects(&f)[0],
        "docs: add incident 2026-09-06-database-outage"
    );
}

#[test]
fn charter_already_present_under_docs_is_rejected_and_writes_nothing() {
    let mut f = repo();
    std::fs::create_dir_all(f.dir.path().join("docs")).unwrap();
    std::fs::write(f.dir.path().join("docs/charter.md"), "# Charter\n").unwrap();
    let commits_before = log_subjects(&f).len();

    assert_eq!(run(&mut f, &["new", "charter"]), 1);

    assert!(!exists(&f, "CHARTER.md"));
    assert_eq!(log_subjects(&f).len(), commits_before);
}
