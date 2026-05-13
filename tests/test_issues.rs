mod common;

use common::setup_mock_server;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_list_issues() {
    let (server, client) = setup_mock_server().await;
    let fixture = std::fs::read_to_string("tests/fixtures/issues_list.json").unwrap();

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&server)
        .await;

    let result = client
        .list_issues(50, None, None, false, "createdAt")
        .await
        .unwrap();

    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].identifier, "ENG-1");
    assert_eq!(result.nodes[0].title, "Fix bug in login");
    assert!(!result.page_info.has_next_page);
}

#[tokio::test]
async fn test_get_issue() {
    let (server, client) = setup_mock_server().await;

    let response = serde_json::json!({
        "data": {
            "issue": {
                "id": "issue-1",
                "identifier": "ENG-1",
                "title": "Fix bug in login",
                "description": "Login page crashes",
                "priority": 1.0,
                "priorityLabel": "Urgent",
                "estimate": null,
                "dueDate": null,
                "createdAt": "2024-06-01T00:00:00.000Z",
                "updatedAt": "2024-06-15T00:00:00.000Z",
                "completedAt": null,
                "canceledAt": null,
                "archivedAt": null,
                "startedAt": null,
                "branchName": "fix/eng-1",
                "number": 1.0,
                "url": "https://linear.app/test/issue/ENG-1",
                "trashed": false,
                "state": { "id": "s1", "name": "Todo", "type": "unstarted", "color": "#ccc" },
                "assignee": null,
                "creator": null,
                "team": { "id": "t1", "name": "Engineering", "key": "ENG" },
                "project": null,
                "cycle": null,
                "parent": null,
                "labels": { "nodes": [] }
            }
        }
    });

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&server)
        .await;

    let issue = client.get_issue("issue-1").await.unwrap();
    assert_eq!(issue.identifier, "ENG-1");
    assert_eq!(issue.title, "Fix bug in login");
}
