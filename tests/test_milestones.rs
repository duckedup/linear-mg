mod common;

use common::{graphql_response, setup_mock_server};
use linear_mg::cli::resolve;
use linear_mg::error::CliError;
use serde_json::{Value, json};
use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, ResponseTemplate};

fn milestone_node(id: &str, name: &str, project_id: &str, project_name: &str) -> Value {
    json!({
        "id": id,
        "name": name,
        "description": null,
        "status": "unstarted",
        "targetDate": "2024-09-30",
        "progress": 0.0,
        "createdAt": "2024-06-01T00:00:00.000Z",
        "updatedAt": "2024-06-01T00:00:00.000Z",
        "archivedAt": null,
        "project": { "id": project_id, "name": project_name, "slugId": "slug" }
    })
}

fn milestones_list_response(nodes: Vec<Value>) -> String {
    graphql_response(json!({
        "projectMilestones": {
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
async fn test_list_milestones_with_project_filter() {
    let (server, client) = setup_mock_server().await;

    // Only matches when the project filter is actually sent in the request.
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .and(body_string_contains("ProjectMilestoneFilter"))
        .and(body_string_contains("proj-1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(milestones_list_response(vec![
                milestone_node("ms-1", "Beta", "proj-1", "Mobile App"),
            ])),
        )
        .mount(&server)
        .await;

    let filter = json!({ "project": { "id": { "eq": "proj-1" } } });
    let result = client
        .list_milestones(50, None, false, "updatedAt", Some(filter))
        .await
        .unwrap();

    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].name, "Beta");
    assert_eq!(result.nodes[0].target_date.as_deref(), Some("2024-09-30"));
}

#[tokio::test]
async fn test_resolve_milestone_by_name() {
    let (server, client) = setup_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(milestones_list_response(vec![
                milestone_node("ms-1", "Alpha", "proj-1", "Mobile App"),
                milestone_node("ms-2", "Beta", "proj-1", "Mobile App"),
            ])),
        )
        .mount(&server)
        .await;

    let resolved = resolve::resolve_milestone(&client, "beta", None)
        .await
        .unwrap();
    assert_eq!(resolved.id, "ms-2");
    // The milestone's own project comes back so callers can set projectId.
    assert_eq!(resolved.project_id.as_deref(), Some("proj-1"));

    // UUIDs pass through without a lookup and carry no project.
    let uuid = "123e4567-e89b-12d3-a456-426614174000";
    let passthrough = resolve::resolve_milestone(&client, uuid, None)
        .await
        .unwrap();
    assert_eq!(passthrough.id, uuid);
    assert!(passthrough.project_id.is_none());
}

#[tokio::test]
async fn test_resolve_milestone_ambiguous_without_project() {
    let (server, client) = setup_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(milestones_list_response(vec![
                milestone_node("ms-1", "Beta", "proj-1", "Mobile App"),
                milestone_node("ms-2", "Beta", "proj-2", "Web App"),
            ])),
        )
        .mount(&server)
        .await;

    let err = resolve::resolve_milestone(&client, "Beta", None)
        .await
        .unwrap_err();
    match err {
        CliError::InvalidInput(msg) => {
            assert!(msg.contains("Mobile App"), "{msg}");
            assert!(msg.contains("Web App"), "{msg}");
        }
        other => panic!("expected InvalidInput, got {other:?}"),
    }
}

#[tokio::test]
async fn test_resolve_milestone_scoped_to_project_sends_filter() {
    let (server, client) = setup_mock_server().await;
    // The lookup must push both the project and the name into the API filter
    // so the result set is complete rather than a 250-item window.
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .and(body_string_contains("proj-2"))
        .and(body_string_contains("eqIgnoreCase"))
        .and(body_string_contains("Beta"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(milestones_list_response(vec![
                milestone_node("ms-2", "Beta", "proj-2", "Web App"),
            ])),
        )
        .mount(&server)
        .await;

    let resolved = resolve::resolve_milestone(&client, "Beta", Some("proj-2"))
        .await
        .unwrap();
    assert_eq!(resolved.id, "ms-2");
}

#[tokio::test]
async fn test_resolve_milestone_ambiguous_within_project_lists_ids() {
    let (server, client) = setup_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(milestones_list_response(vec![
                milestone_node("ms-1", "Beta", "proj-1", "Mobile App"),
                milestone_node("ms-2", "Beta", "proj-1", "Mobile App"),
            ])),
        )
        .mount(&server)
        .await;

    let err = resolve::resolve_milestone(&client, "Beta", Some("proj-1"))
        .await
        .unwrap_err();
    match err {
        CliError::InvalidInput(msg) => {
            assert!(msg.contains("ms-1") && msg.contains("ms-2"), "{msg}");
            assert!(msg.contains("pass a milestone ID"), "{msg}");
        }
        other => panic!("expected InvalidInput, got {other:?}"),
    }
}

#[tokio::test]
async fn test_apply_milestone_without_project_sets_project_id() {
    let (server, client) = setup_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(milestones_list_response(vec![
                milestone_node("ms-1", "Beta", "proj-1", "Mobile App"),
            ])),
        )
        .mount(&server)
        .await;

    let mut input = json!({});
    let obj = input.as_object_mut().unwrap();
    resolve::apply_project_and_milestone(&client, obj, None, Some("Beta".into()), None)
        .await
        .unwrap();
    assert_eq!(input["projectMilestoneId"], "ms-1");
    // Linear requires the milestone's project, so it is filled in automatically.
    assert_eq!(input["projectId"], "proj-1");
}

#[tokio::test]
async fn test_resolve_milestone_not_found() {
    let (server, client) = setup_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(milestones_list_response(vec![])))
        .mount(&server)
        .await;

    let err = resolve::resolve_milestone(&client, "Nope", None)
        .await
        .unwrap_err();
    assert!(matches!(err, CliError::NotFound(_)), "{err:?}");
}
