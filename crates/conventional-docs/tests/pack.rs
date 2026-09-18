mod common;

use std::process::Command;

use common::*;

#[test]
fn license_new_with_no_holder_takes_it_from_local_git_config() {
    let mut f = repo();
    let status = Command::new("git")
        .args(["config", "user.name", "Ada Lovelace"])
        .current_dir(f.dir.path())
        .status()
        .expect("git config");
    assert!(status.success());

    assert_eq!(run(&mut f, &["license", "new", "--spdx", "MIT"]), 0);

    assert!(read(&f, "LICENSE.md").contains("Copyright (c) 2026 Ada Lovelace"));
}

#[test]
fn apache_license_stays_verbatim_with_its_appendix_bracket() {
    let mut f = repo();
    assert_eq!(
        run(
            &mut f,
            &["license", "new", "--spdx", "Apache-2.0", "--holder", "X"]
        ),
        0
    );
    let body = read(&f, "LICENSE.md");
    assert!(body.contains("Copyright [yyyy] [name of copyright owner]"));
}

#[test]
fn license_new_with_an_unknown_spdx_id_fails_and_writes_nothing() {
    let mut f = repo();
    let (code, message) = run_err(
        &mut f,
        &["license", "new", "--spdx", "GPL-2.0-only", "--holder", "X"],
    );
    assert_eq!(code, 1);
    assert!(message.contains("MIT"), "message: {message}");
    assert!(!exists(&f, "LICENSE.md"));
}

#[test]
fn pack_list_json_reports_embedded_entries() {
    let mut f = repo();
    assert_eq!(run(&mut f, &["pack", "list", "--json"]), 0);
    let out = take_output(&f);
    assert!(out.contains(r#"{"kind":"license","id":"MIT","source":"embedded"}"#));
    assert!(out.contains(r#""kind":"code-of-conduct""#));
}

#[test]
fn explicit_pack_shadows_the_embedded_body() {
    let mut f = repo();
    let pack_dir = f.dir.path().join("custom-pack");
    std::fs::create_dir_all(pack_dir.join("licenses")).unwrap();
    std::fs::write(
        pack_dir.join("licenses/MIT.txt"),
        "custom {{year}} {{holder}}\n",
    )
    .unwrap();

    assert_eq!(
        run(
            &mut f,
            &[
                "license",
                "new",
                "--spdx",
                "MIT",
                "--holder",
                "X",
                "--pack",
                pack_dir.to_str().unwrap(),
            ]
        ),
        0
    );
    assert_eq!(read(&f, "LICENSE.md"), "custom 2026 X\n");

    assert_eq!(
        run(
            &mut f,
            &[
                "pack",
                "list",
                "--json",
                "--pack",
                pack_dir.to_str().unwrap()
            ]
        ),
        0
    );
    let out = take_output(&f);
    assert!(out.contains(&format!(
        r#"{{"kind":"license","id":"MIT","source":"{}"}}"#,
        pack_dir.display()
    )));
    assert!(out.contains(r#"{"kind":"license","id":"ISC","source":"embedded"}"#));
}

#[test]
fn an_unfilled_placeholder_fails_and_writes_nothing() {
    let mut f = repo();
    let pack_dir = f.dir.path().join("custom-pack");
    std::fs::create_dir_all(pack_dir.join("licenses")).unwrap();
    std::fs::write(pack_dir.join("licenses/Weird.txt"), "body {{nope}}\n").unwrap();

    let (code, message) = run_err(
        &mut f,
        &[
            "license",
            "new",
            "--spdx",
            "Weird",
            "--holder",
            "X",
            "--pack",
            pack_dir.to_str().unwrap(),
        ],
    );
    assert_eq!(code, 1);
    assert!(message.contains("{{nope}}"), "message: {message}");
    assert!(!exists(&f, "LICENSE.md"));
}

#[test]
fn a_missing_pack_directory_is_a_usage_error() {
    let mut f = repo();
    let (code, _) = run_err(
        &mut f,
        &[
            "license",
            "new",
            "--spdx",
            "MIT",
            "--holder",
            "X",
            "--pack",
            "does-not-exist",
        ],
    );
    assert_eq!(code, 2);
    assert!(!exists(&f, "LICENSE.md"));
}
