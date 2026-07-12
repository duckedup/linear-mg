mod common;

use common::{graphql_response, setup_mock_server};
use linear_mg::cli::resolve;
use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

fn label_node(id: &str, name: &str, team: Option<(&str, &str, &str)>) -> Value {
    let team_val = match team {
        Some((tid, tname, tkey)) => json!({ "id": tid, "name": tname, "key": tkey }),
        None => Value::Null,
    };
    json!({
        "id": id,
        "name": name,
        "color": "#eb5757",
        "description": null,
        "isGroup": false,
        "createdAt": "2024-06-01T00:00:00.000Z",
        "updatedAt": "2024-06-01T00:00:00.000Z",
        "archivedAt": null,
        "team": team_val,
        "parent": null
    })
}

fn labels_list_response(nodes: Vec<Value>) -> String {
    graphql_response(json!({
        "issueLabels": {
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

async fn mount(server: &wiremock::MockServer, body: String) {
    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(server)
        .await;
}

#[tokio::test]
async fn test_get_label() {
    let (server, client) = setup_mock_server().await;
    mount(
        &server,
        graphql_response(json!({
            "issueLabel": label_node("label-1", "Bug", Some(("t1", "Engineering", "ENG")))
        })),
    )
    .await;

    let label = client.get_label("label-1").await.unwrap();
    assert_eq!(label.id, "label-1");
    assert_eq!(label.name, "Bug");
    assert_eq!(label.team.unwrap().key, "ENG");
}

#[tokio::test]
async fn test_update_label() {
    let (server, client) = setup_mock_server().await;
    mount(
        &server,
        graphql_response(json!({
            "issueLabelUpdate": {
                "success": true,
                "issueLabel": label_node("label-1", "Defect", None)
            }
        })),
    )
    .await;

    let input = json!({ "name": "Defect" });
    let payload = client.update_label("label-1", input).await.unwrap();
    assert!(payload.success);
    assert_eq!(payload.issue_label.name, "Defect");
}

#[tokio::test]
async fn test_delete_label() {
    let (server, client) = setup_mock_server().await;
    mount(
        &server,
        graphql_response(json!({ "issueLabelDelete": { "success": true } })),
    )
    .await;

    let payload = client.delete_label("label-1").await.unwrap();
    assert!(payload.success);
}

#[tokio::test]
async fn test_resolve_label_uuid_passthrough() {
    // A UUID resolves to itself without hitting the API.
    let (_server, client) = setup_mock_server().await;
    let uuid = "12345678-1234-1234-1234-123456789012";
    let resolved = resolve::resolve_label(&client, uuid, None).await.unwrap();
    assert_eq!(resolved, uuid);
}

#[tokio::test]
async fn test_resolve_label_by_name() {
    let (server, client) = setup_mock_server().await;
    mount(
        &server,
        labels_list_response(vec![
            label_node("label-bug", "Bug", Some(("t1", "Engineering", "ENG"))),
            label_node("label-feat", "Feature", Some(("t1", "Engineering", "ENG"))),
        ]),
    )
    .await;

    let resolved = resolve::resolve_label(&client, "bug", None).await.unwrap();
    assert_eq!(resolved, "label-bug");
}

#[tokio::test]
async fn test_resolve_label_ambiguous_without_team() {
    let (server, client) = setup_mock_server().await;
    mount(
        &server,
        labels_list_response(vec![
            label_node("label-eng", "Bug", Some(("t-eng", "Engineering", "ENG"))),
            label_node("label-des", "Bug", Some(("t-des", "Design", "DES"))),
        ]),
    )
    .await;

    let err = resolve::resolve_label(&client, "Bug", None)
        .await
        .unwrap_err();
    // Ambiguous name maps to invalid-input (exit code 5).
    assert_eq!(err.exit_code(), 5);
}

#[tokio::test]
async fn test_resolve_label_team_scoped() {
    let (server, client) = setup_mock_server().await;
    mount(
        &server,
        labels_list_response(vec![
            label_node("label-eng", "Bug", Some(("t-eng", "Engineering", "ENG"))),
            label_node("label-des", "Bug", Some(("t-des", "Design", "DES"))),
        ]),
    )
    .await;

    // A team context picks the team-scoped label.
    let resolved = resolve::resolve_label(&client, "Bug", Some("t-des"))
        .await
        .unwrap();
    assert_eq!(resolved, "label-des");
}

#[tokio::test]
async fn test_resolve_label_workspace_fallback() {
    let (server, client) = setup_mock_server().await;
    mount(
        &server,
        labels_list_response(vec![
            label_node("label-ws", "Bug", None),
            label_node("label-des", "Bug", Some(("t-des", "Design", "DES"))),
        ]),
    )
    .await;

    // No team-specific match for this team → fall back to the workspace label.
    let resolved = resolve::resolve_label(&client, "Bug", Some("t-eng"))
        .await
        .unwrap();
    assert_eq!(resolved, "label-ws");
}
