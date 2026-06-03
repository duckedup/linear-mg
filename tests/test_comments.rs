mod common;

use common::{graphql_response, setup_mock_server};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_list_comments() {
    let (server, client) = setup_mock_server().await;
    let fixture = std::fs::read_to_string("tests/fixtures/comments_list.json").unwrap();

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&server)
        .await;

    let result = client
        .list_comments(50, None, false, "createdAt", None)
        .await
        .unwrap();

    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].id, "comment-1");
    assert_eq!(result.nodes[0].body, "This looks good to me");
    assert!(result.nodes[0].user.is_some());
    assert!(result.nodes[0].issue.is_some());
    assert!(!result.page_info.has_next_page);
}

#[tokio::test]
async fn test_get_comment() {
    let (server, client) = setup_mock_server().await;

    let response = graphql_response(serde_json::json!({
        "comment": {
            "id": "comment-1",
            "body": "This looks good to me",
            "createdAt": "2024-06-01T00:00:00.000Z",
            "updatedAt": "2024-06-01T00:00:00.000Z",
            "editedAt": null,
            "archivedAt": null,
            "resolvedAt": null,
            "url": "https://linear.app/test/issue/ENG-1#comment-1",
            "user": {
                "id": "user-123",
                "name": "Test User",
                "displayName": "Test User",
                "email": "test@example.com"
            },
            "issue": {
                "id": "issue-1",
                "identifier": "ENG-1",
                "title": "Fix bug in login"
            },
            "parent": null
        }
    }));

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response))
        .mount(&server)
        .await;

    let comment = client.get_comment("comment-1").await.unwrap();

    assert_eq!(comment.id, "comment-1");
    assert_eq!(comment.body, "This looks good to me");
    assert_eq!(comment.url, "https://linear.app/test/issue/ENG-1#comment-1");
}

#[tokio::test]
async fn test_create_comment() {
    let (server, client) = setup_mock_server().await;

    let response = graphql_response(serde_json::json!({
        "commentCreate": {
            "success": true,
            "comment": {
                "id": "comment-2",
                "body": "New comment",
                "createdAt": "2024-06-02T00:00:00.000Z",
                "updatedAt": "2024-06-02T00:00:00.000Z",
                "editedAt": null,
                "archivedAt": null,
                "resolvedAt": null,
                "url": "https://linear.app/test/issue/ENG-1#comment-2",
                "user": {
                    "id": "user-123",
                    "name": "Test User",
                    "displayName": "Test User",
                    "email": "test@example.com"
                },
                "issue": {
                    "id": "issue-1",
                    "identifier": "ENG-1",
                    "title": "Fix bug in login"
                },
                "parent": null
            }
        }
    }));

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response))
        .mount(&server)
        .await;

    let input = serde_json::json!({ "issueId": "issue-1", "body": "New comment" });
    let payload = client.create_comment(input).await.unwrap();

    assert!(payload.success);
    assert!(payload.comment.is_some());
    assert_eq!(payload.comment.unwrap().body, "New comment");
}

#[tokio::test]
async fn test_update_comment() {
    let (server, client) = setup_mock_server().await;

    let response = graphql_response(serde_json::json!({
        "commentUpdate": {
            "success": true,
            "comment": {
                "id": "comment-1",
                "body": "Updated body",
                "createdAt": "2024-06-01T00:00:00.000Z",
                "updatedAt": "2024-06-02T00:00:00.000Z",
                "editedAt": "2024-06-02T00:00:00.000Z",
                "archivedAt": null,
                "resolvedAt": null,
                "url": "https://linear.app/test/issue/ENG-1#comment-1",
                "user": {
                    "id": "user-123",
                    "name": "Test User",
                    "displayName": "Test User",
                    "email": "test@example.com"
                },
                "issue": {
                    "id": "issue-1",
                    "identifier": "ENG-1",
                    "title": "Fix bug in login"
                },
                "parent": null
            }
        }
    }));

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response))
        .mount(&server)
        .await;

    let input = serde_json::json!({ "body": "Updated body" });
    let payload = client.update_comment("comment-1", input).await.unwrap();

    assert!(payload.success);
    assert_eq!(payload.comment.unwrap().body, "Updated body");
}

#[tokio::test]
async fn test_delete_comment() {
    let (server, client) = setup_mock_server().await;

    let response = graphql_response(serde_json::json!({
        "commentDelete": {
            "success": true
        }
    }));

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response))
        .mount(&server)
        .await;

    let payload = client.delete_comment("comment-1").await.unwrap();

    assert!(payload.success);
}
