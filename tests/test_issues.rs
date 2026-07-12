mod common;

use common::{graphql_response, setup_mock_server};
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
    // Relations default to empty when the response omits them.
    assert!(issue.relations.nodes.is_empty());
    assert!(issue.inverse_relations.nodes.is_empty());
}

fn relation_json(id: &str, ty: &str, source: &str, target: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "type": ty,
        "createdAt": "2024-06-01T00:00:00.000Z",
        "updatedAt": "2024-06-01T00:00:00.000Z",
        "issue": { "id": "u-source", "identifier": source, "title": "Source issue" },
        "relatedIssue": { "id": "u-target", "identifier": target, "title": "Target issue" }
    })
}

#[tokio::test]
async fn test_list_issue_relations() {
    let (server, client) = setup_mock_server().await;

    let response = graphql_response(serde_json::json!({
        "issue": {
            "relations": { "nodes": [relation_json("rel-1", "blocks", "ENG-1", "ENG-2")] },
            "inverseRelations": { "nodes": [relation_json("rel-2", "blocks", "ENG-3", "ENG-1")] }
        }
    }));

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response))
        .mount(&server)
        .await;

    let (relations, inverse) = client.list_issue_relations("ENG-1").await.unwrap();

    assert_eq!(relations.len(), 1);
    assert_eq!(relations[0].id, "rel-1");
    assert_eq!(relations[0].relation_type, "blocks");
    assert_eq!(relations[0].related_issue.identifier, "ENG-2");

    assert_eq!(inverse.len(), 1);
    assert_eq!(inverse[0].id, "rel-2");
    assert_eq!(inverse[0].issue.identifier, "ENG-3");
}

#[tokio::test]
async fn test_create_issue_relation() {
    let (server, client) = setup_mock_server().await;

    let response = graphql_response(serde_json::json!({
        "issueRelationCreate": {
            "success": true,
            "issueRelation": relation_json("rel-1", "blocks", "ENG-1", "ENG-2")
        }
    }));

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response))
        .mount(&server)
        .await;

    let input = serde_json::json!({
        "issueId": "ENG-1", "relatedIssueId": "ENG-2", "type": "blocks"
    });
    let payload = client.create_issue_relation(input).await.unwrap();

    assert!(payload.success);
    let rel = payload.issue_relation.unwrap();
    assert_eq!(rel.relation_type, "blocks");
    assert_eq!(rel.issue.identifier, "ENG-1");
    assert_eq!(rel.related_issue.identifier, "ENG-2");
}

#[tokio::test]
async fn test_delete_issue_relation() {
    let (server, client) = setup_mock_server().await;

    let response = graphql_response(serde_json::json!({
        "issueRelationDelete": { "success": true }
    }));

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response))
        .mount(&server)
        .await;

    let payload = client.delete_issue_relation("rel-1").await.unwrap();
    assert!(payload.success);
}
