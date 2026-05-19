use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI for the Linear API"))
        .stdout(predicate::str::contains("issues"))
        .stdout(predicate::str::contains("teams"))
        .stdout(predicate::str::contains("auth"));
}

#[test]
fn test_version() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("linear-mg"));
}

#[test]
fn test_no_api_key_error() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["issues", "list", "--json"])
        .env_remove("LINEAR_API_KEY")
        .env("HOME", tmp.path())
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("no_api_key"));
}

#[test]
fn test_issues_list_help() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["issues", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--team"))
        .stdout(predicate::str::contains("--assignee"))
        .stdout(predicate::str::contains("--state"))
        .stdout(predicate::str::contains("--limit"));
}

#[test]
fn test_issues_create_requires_title_and_team() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["issues", "create"])
        .env("LINEAR_API_KEY", "test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--title"))
        .stderr(predicate::str::contains("--team"));
}

#[test]
fn test_all_subcommands_have_help() {
    let commands = [
        "auth",
        "issues",
        "teams",
        "projects",
        "users",
        "comments",
        "labels",
        "cycles",
        "states",
        "documents",
        "initiatives",
        "milestones",
        "attachments",
    ];
    for cmd in commands {
        Command::cargo_bin("linear-mg")
            .unwrap()
            .args([cmd, "--help"])
            .assert()
            .success();
    }
}
