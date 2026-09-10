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

#[test]
fn test_issues_relate_requires_direction() {
    // No direction flag: fails validation with invalid-input before any network call.
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["--json", "issues", "relate", "ENG-1"])
        .env("LINEAR_API_KEY", "test")
        .assert()
        .failure()
        .code(5)
        .stderr(predicate::str::contains("invalid_input"));
}

#[test]
fn test_issues_relate_rejects_multiple_directions() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args([
            "--json",
            "issues",
            "relate",
            "ENG-1",
            "--blocks",
            "ENG-2",
            "--related-to",
            "ENG-3",
        ])
        .env("LINEAR_API_KEY", "test")
        .assert()
        .failure()
        .code(5)
        .stderr(predicate::str::contains("invalid_input"));
}

#[test]
fn test_issues_relate_help() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["issues", "relate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--blocks"))
        .stdout(predicate::str::contains("--blocked-by"))
        .stdout(predicate::str::contains("--duplicate-of"));
}

#[test]
fn test_projects_list_help() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["projects", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--status"))
        .stdout(predicate::str::contains("--lead"))
        .stdout(predicate::str::contains("--health"));
}

#[test]
fn test_issues_update_help_has_milestone_flags() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["issues", "update", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--milestone"))
        .stdout(predicate::str::contains("--no-milestone"));
}

#[test]
fn test_issues_update_milestone_flags_conflict() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args([
            "issues",
            "update",
            "ENG-1",
            "--milestone",
            "Beta",
            "--no-milestone",
        ])
        .env("LINEAR_API_KEY", "test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_milestones_list_has_project_filter() {
    Command::cargo_bin("linear-mg")
        .unwrap()
        .args(["milestones", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--project"));
}
