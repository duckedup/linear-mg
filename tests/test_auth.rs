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

    let user = client.get_viewer().await.unwrap();
    assert_eq!(user.name, "Test User");
    assert_eq!(user.email, "test@example.com");
    assert!(user.is_me);
}

#[tokio::test]
async fn test_unauthorized_returns_auth_error() {
    let (server, client) = setup_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/graphql"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&server)
        .await;

    let result = client.get_viewer().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().exit_code(), 2);
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

    let result = client.get_viewer().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().exit_code(), 6);
}
