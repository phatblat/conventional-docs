mod common;

use common::*;

#[test]
fn propose_writes_golden_bytes_and_commits() {
    let mut f = repo();
    let code = run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    assert_eq!(code, 0);

    let id = "2026-09-06-cache-the-resolver-output";
    let path = format!("docs/decisions/{id}.md");
    assert_eq!(read(&f, &path), golden("decision-proposed.md"));

    assert_eq!(log_subjects(&f)[0], format!("decision: propose {id}"));
    assert_eq!(commit_paths(&f), vec![path]);
}

#[test]
fn draft_then_propose_promotes_and_produces_two_commits() {
    let mut f = repo();
    assert_eq!(
        run(&mut f, &["dec", "draft", "Cache the resolver output"]),
        0
    );

    let id = "2026-09-06-cache-the-resolver-output";
    let path = format!("docs/decisions/{id}.md");
    assert_eq!(read(&f, &path), golden("decision-draft.md"));

    assert_eq!(run(&mut f, &["dec", "propose", id]), 0);
    assert_eq!(read(&f, &path), golden("decision-proposed.md"));

    let subjects = log_subjects(&f);
    assert_eq!(subjects[0], format!("decision: propose {id}"));
    assert_eq!(subjects[1], format!("decision: draft {id}"));
}

#[test]
fn accept_on_a_draft_is_rejected() {
    let mut f = repo();
    run(&mut f, &["dec", "draft", "Cache the resolver output"]);
    let id = "2026-09-06-cache-the-resolver-output";
    let before = read(&f, &format!("docs/decisions/{id}.md"));
    let commits_before = log_subjects(&f).len();

    assert_eq!(run(&mut f, &["dec", "accept", id]), 1);

    assert_eq!(read(&f, &format!("docs/decisions/{id}.md")), before);
    assert_eq!(log_subjects(&f).len(), commits_before);
}

#[test]
fn accept_on_proposed_writes_golden_bytes_then_rejects_a_second_accept() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let id = "2026-09-06-cache-the-resolver-output";

    assert_eq!(run(&mut f, &["dec", "accept", id]), 0);
    assert_eq!(
        read(&f, &format!("docs/decisions/{id}.md")),
        golden("decision-accepted.md")
    );

    assert_eq!(run(&mut f, &["dec", "accept", id]), 1);
}

#[test]
fn reject_on_proposed_writes_golden_bytes() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let id = "2026-09-06-cache-the-resolver-output";

    assert_eq!(run(&mut f, &["dec", "reject", id]), 0);
    assert_eq!(
        read(&f, &format!("docs/decisions/{id}.md")),
        golden("decision-rejected.md")
    );
}

#[test]
fn extends_a_proposed_target_edits_it_in_place() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let target_id = "2026-09-06-cache-the-resolver-output";

    let code = run(
        &mut f,
        &[
            "dec",
            "propose",
            "Reuse the cached parse tree",
            "--extends",
            target_id,
        ],
    );
    assert_eq!(code, 0);

    assert_eq!(
        read(&f, &format!("docs/decisions/{target_id}.md")),
        golden("decision-extends-proposed.md")
    );

    let new_id = "2026-09-06-reuse-the-cached-parse-tree";
    let subjects = log_subjects(&f);
    assert_eq!(subjects[0], format!("decision: propose {new_id}"));
    let mut paths = commit_paths(&f);
    paths.sort();
    let mut expected = vec![
        format!("docs/decisions/{new_id}.md"),
        format!("docs/decisions/{target_id}.md"),
    ];
    expected.sort();
    assert_eq!(paths, expected);
}

#[test]
fn extends_a_frozen_target_appends_an_erratum() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let target_id = "2026-09-06-cache-the-resolver-output";
    run(&mut f, &["dec", "accept", target_id]);

    let code = run(
        &mut f,
        &[
            "dec",
            "propose",
            "Reuse the cached parse tree",
            "--extends",
            target_id,
        ],
    );
    assert_eq!(code, 0);

    assert_eq!(
        read(&f, &format!("docs/decisions/{target_id}.md")),
        golden("decision-extends-frozen.md")
    );
}

#[test]
fn supersedes_an_unfrozen_target_is_rejected() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let target_id = "2026-09-06-cache-the-resolver-output";

    let code = run(
        &mut f,
        &[
            "dec",
            "propose",
            "Reuse the cached parse tree",
            "--supersedes",
            target_id,
        ],
    );
    assert_eq!(code, 1);
    assert!(!exists(
        &f,
        "docs/decisions/2026-09-06-reuse-the-cached-parse-tree.md"
    ));
}

#[test]
fn supersedes_a_frozen_target_appends_an_erratum() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let target_id = "2026-09-06-cache-the-resolver-output";
    run(&mut f, &["dec", "accept", target_id]);

    let code = run(
        &mut f,
        &[
            "dec",
            "propose",
            "Reuse the cached parse tree",
            "--supersedes",
            target_id,
        ],
    );
    assert_eq!(code, 0);

    assert_eq!(
        read(&f, &format!("docs/decisions/{target_id}.md")),
        golden("decision-supersedes-frozen.md")
    );

    let new_id = "2026-09-06-reuse-the-cached-parse-tree";
    assert!(
        read(&f, &format!("docs/decisions/{new_id}.md")).contains(&format!(
            "This decision supersedes [{target_id}](./{target_id}.md)."
        ))
    );
    assert_eq!(log_subjects(&f)[0], format!("decision: propose {new_id}"));
}

#[test]
fn extends_a_nonexistent_target_is_rejected() {
    let mut f = repo();

    let code = run(
        &mut f,
        &[
            "dec",
            "propose",
            "Reuse the cached parse tree",
            "--extends",
            "2026-01-01-nope",
        ],
    );
    assert_eq!(code, 1);
    assert!(!exists(
        &f,
        "docs/decisions/2026-09-06-reuse-the-cached-parse-tree.md"
    ));
}

#[test]
fn supersedes_a_nonexistent_target_is_rejected() {
    let mut f = repo();

    let code = run(
        &mut f,
        &[
            "dec",
            "propose",
            "Reuse the cached parse tree",
            "--supersedes",
            "2026-01-01-nope",
        ],
    );
    assert_eq!(code, 1);
    assert!(!exists(
        &f,
        "docs/decisions/2026-09-06-reuse-the-cached-parse-tree.md"
    ));
}

#[test]
fn errata_on_a_frozen_record_appends_dated_lines_in_order() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let id = "2026-09-06-cache-the-resolver-output";
    run(&mut f, &["dec", "accept", id]);

    assert_eq!(
        run(
            &mut f,
            &["dec", "errata", id, "The flag shipped as --pedantic."]
        ),
        0
    );
    let after_first = read(&f, &format!("docs/decisions/{id}.md"));
    assert!(after_first.ends_with("## Errata\n\n- 2026-09-06: The flag shipped as --pedantic.\n"));

    assert_eq!(
        run(&mut f, &["dec", "errata", id, "A second correction."]),
        0
    );
    let after_second = read(&f, &format!("docs/decisions/{id}.md"));
    assert!(after_second.ends_with(
        "- 2026-09-06: The flag shipped as --pedantic.\n- 2026-09-06: A second correction.\n"
    ));
}

#[test]
fn errata_on_a_proposed_record_is_rejected() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    let id = "2026-09-06-cache-the-resolver-output";

    assert_eq!(run(&mut f, &["dec", "errata", id, "Too early."]), 1);
}

#[test]
fn propose_on_an_id_that_already_exists_is_rejected() {
    let mut f = repo();
    run(&mut f, &["dec", "propose", "Cache the resolver output"]);

    let code = run(&mut f, &["dec", "propose", "Cache the resolver output"]);
    assert_eq!(code, 1);
}

#[test]
fn propose_commits_only_its_own_record_leaving_an_unrelated_staged_file_alone() {
    let mut f = repo();
    stage_unrelated(&f, "unrelated.txt");

    assert_eq!(
        run(&mut f, &["dec", "propose", "Cache the resolver output"]),
        0
    );

    let id = "2026-09-06-cache-the-resolver-output";
    assert_eq!(commit_paths(&f), vec![format!("docs/decisions/{id}.md")]);
    assert_eq!(staged_paths(&f), vec!["unrelated.txt".to_string()]);
}

#[test]
fn no_commit_writes_the_file_without_committing() {
    let mut f = repo();
    let commits_before = log_subjects(&f).len();

    assert_eq!(
        run(
            &mut f,
            &["--no-commit", "dec", "propose", "Cache the resolver output"]
        ),
        0
    );

    let id = "2026-09-06-cache-the-resolver-output";
    assert!(exists(&f, &format!("docs/decisions/{id}.md")));
    assert_eq!(log_subjects(&f).len(), commits_before);
}
