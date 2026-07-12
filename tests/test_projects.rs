mod common;

use common::{graphql_response, setup_mock_server};
use linear_mg::cli::resolve;
use serde_json::{Value, json};
use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, ResponseTemplate};

fn project_node(id: &str, name: &str, slug: &str, status: &str) -> Value {
    json!({
        "id": id,
        "name": name,
        "slugId": slug,
        "description": "",
        "icon": null,
        "color": "#000000",
        "priority": 0,
        "priorityLabel": "No priority",
        "progress": 0.0,
        "startDate": null,
        "targetDate": null,
        "startedAt": null,
        "completedAt": null,
        "canceledAt": null,
        "createdAt": "2024-06-01T00:00:00.000Z",
        "updatedAt": "2024-06-01T00:00:00.000Z",
        "archivedAt": null,
        "trashed": false,
        "url": "https://linear.app/test/project/x",
        "content": null,
        "lead": null,
        "creator": null,
        "status": { "id": "st-1", "name": status, "color": "#abcabc" }
    })
}

fn projects_list_response(nodes: Vec<Value>) -> String {
    graphql_response(json!({
        "projects": {
            "nodes": nodes,
            "pageInfo": {
                "hasNextPage": false,
                "hasPreviousPage": false,
                "endCursor": null,
                "startCursor": null
            }
        }
    }))
}

#[tokio::test]
async fn test_list_projects_with_status_filter() {
    let (server, client) = setup_mock_server().await;

    // The mock only matches if the outgoing GraphQL body actually carries the
    // status filter, so this verifies the filter is wired into the request.
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .and(body_string_contains("eqIgnoreCase"))
        .and(body_string_contains("In Progress"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(projects_list_response(vec![project_node(
                "proj-1",
                "Mobile App",
                "mobile-app",
                "In Progress",
            )])),
        )
        .mount(&server)
        .await;

    let filter = json!({ "status": { "name": { "eqIgnoreCase": "In Progress" } } });
    let result = client
        .list_projects(50, None, false, "updatedAt", Some(filter))
        .await
        .unwrap();

    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].name, "Mobile App");
    assert_eq!(result.nodes[0].status.name, "In Progress");
}

#[tokio::test]
async fn test_resolve_project_by_name_and_slug() {
    let (server, client) = setup_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(projects_list_response(vec![
                project_node("proj-1", "Mobile App", "mobile-app", "In Progress"),
                project_node("proj-2", "Web App", "web-app", "Planned"),
            ])),
        )
        .mount(&server)
        .await;

    // By name (case-insensitive)
    let by_name = resolve::resolve_project(&client, "web app").await.unwrap();
    assert_eq!(by_name, "proj-2");

    // By slug
    let by_slug = resolve::resolve_project(&client, "mobile-app")
        .await
        .unwrap();
    assert_eq!(by_slug, "proj-1");
}

#[tokio::test]
async fn test_resolve_project_not_found() {
    let (server, client) = setup_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(projects_list_response(vec![project_node(
                "proj-1",
                "Mobile App",
                "mobile-app",
                "In Progress",
            )])),
        )
        .mount(&server)
        .await;

    let err = resolve::resolve_project(&client, "Nonexistent")
        .await
        .unwrap_err();
    // Unknown project maps to not-found (exit code 4).
    assert_eq!(err.exit_code(), 4);
}

#[tokio::test]
async fn test_resolve_project_uuid_passthrough() {
    let (_server, client) = setup_mock_server().await;
    let uuid = "abcdef12-3456-7890-abcd-ef1234567890";
    let resolved = resolve::resolve_project(&client, uuid).await.unwrap();
    assert_eq!(resolved, uuid);
}
