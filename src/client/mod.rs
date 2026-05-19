pub mod paginator;

use crate::error::CliError;
use serde::Serialize;

pub const DEFAULT_ENDPOINT: &str = "https://api.linear.app/graphql";

pub struct LinearClient {
    http: reqwest::Client,
    api_key: String,
    endpoint: String,
}

#[derive(Serialize)]
struct GraphqlRequest<'a> {
    query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    variables: Option<serde_json::Value>,
}

#[derive(serde::Deserialize)]
struct GraphqlResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphqlError>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct GraphqlError {
    pub message: String,
}

impl LinearClient {
    pub fn new(api_key: String) -> Self {
        Self::with_endpoint(api_key, DEFAULT_ENDPOINT.to_string())
    }

    pub fn with_endpoint(api_key: String, endpoint: String) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(format!("linear-wp/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            api_key,
            endpoint,
        }
    }

    pub async fn query<T: serde::de::DeserializeOwned>(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
    ) -> Result<T, CliError> {
        let body = GraphqlRequest { query, variables };

        let response = self
            .http
            .post(&self.endpoint)
            .header("Authorization", &self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok());
            return Err(CliError::RateLimited { retry_after });
        }
        if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
            let body = response.text().await.unwrap_or_default();
            return Err(CliError::AuthError(body));
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CliError::HttpError { status, body });
        }

        let gql: GraphqlResponse<T> = response.json().await?;

        if let Some(errors) = &gql.errors
            && !errors.is_empty()
        {
            if gql.data.is_none() {
                return Err(CliError::GraphQlErrors(
                    errors.iter().map(|e| e.message.clone()).collect(),
                ));
            }
            tracing::warn!("GraphQL partial errors: {:?}", errors);
        }

        gql.data.ok_or(CliError::NoData)
    }
}
