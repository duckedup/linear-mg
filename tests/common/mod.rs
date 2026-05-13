use linear_mg::client::LinearClient;
use wiremock::MockServer;

pub async fn setup_mock_server() -> (MockServer, LinearClient) {
    let server = MockServer::start().await;
    let client = LinearClient::with_endpoint(
        "test-api-key".to_string(),
        format!("{}/graphql", server.uri()),
    );
    (server, client)
}

#[allow(dead_code)]
pub fn graphql_response(data: serde_json::Value) -> String {
    serde_json::json!({ "data": data }).to_string()
}

#[allow(dead_code)]
pub fn graphql_error_response(message: &str) -> String {
    serde_json::json!({
        "errors": [{ "message": message }]
    })
    .to_string()
}
