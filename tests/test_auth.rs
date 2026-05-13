mod common;

use common::setup_mock_server;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn test_viewer_query() {
    let (server, client) = setup_mock_server().await;
    let fixture = std::fs::read_to_string("tests/fixtures/viewer.json").unwrap();

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .and(header("Authorization", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&server)
        .await;

    use linear_mg::graphql::users::queries::ViewerQuery;
    let op = cynic::QueryBuilder::build(());
    let data: linear_mg::graphql::users::types::User =
        client.run_query::<ViewerQuery, _>(op).await.map(|d| d.viewer).unwrap();

    assert_eq!(data.name, "Test User");
    assert_eq!(data.email, "test@example.com");
    assert!(data.is_me);
}

#[tokio::test]
async fn test_unauthorized_returns_auth_error() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&server)
        .await;

    use linear_mg::graphql::users::queries::ViewerQuery;
    let op = cynic::QueryBuilder::build(());
    let result = client.run_query::<ViewerQuery, _>(op).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[tokio::test]
async fn test_graphql_error_response() {
    let (server, client) = setup_mock_server().await;
    let error_body = common::graphql_error_response("Entity not found");

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_string(error_body))
        .mount(&server)
        .await;

    use linear_mg::graphql::users::queries::ViewerQuery;
    let op = cynic::QueryBuilder::build(());
    let result = client.run_query::<ViewerQuery, _>(op).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().exit_code(), 6);
}
