pub mod paginator;

use crate::error::CliError;

pub const DEFAULT_ENDPOINT: &str = "https://api.linear.app/graphql";

pub struct LinearClient {
    http: reqwest::Client,
    api_key: String,
    endpoint: String,
}

impl LinearClient {
    pub fn new(api_key: String) -> Self {
        Self::with_endpoint(api_key, DEFAULT_ENDPOINT.to_string())
    }

    pub fn with_endpoint(api_key: String, endpoint: String) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(format!("linear-mg/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client");
        Self {
            http,
            api_key,
            endpoint,
        }
    }

    pub async fn run_query<ResponseData, Vars>(
        &self,
        operation: cynic::Operation<ResponseData, Vars>,
    ) -> Result<ResponseData, CliError>
    where
        ResponseData: serde::de::DeserializeOwned + 'static,
        Vars: serde::Serialize,
    {
        let response = self
            .http
            .post(&self.endpoint)
            .header("Authorization", &self.api_key)
            .json(&operation)
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
        if status == reqwest::StatusCode::UNAUTHORIZED
            || status == reqwest::StatusCode::FORBIDDEN
        {
            let body = response.text().await.unwrap_or_default();
            return Err(CliError::AuthError(body));
        }
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(CliError::HttpError { status, body });
        }

        let gql_response: cynic::GraphQlResponse<ResponseData> = response.json().await?;

        if let Some(errors) = &gql_response.errors {
            if !errors.is_empty() {
                if gql_response.data.is_none() {
                    return Err(CliError::GraphQlErrors(errors.clone()));
                }
                tracing::warn!("GraphQL partial errors: {:?}", errors);
            }
        }

        gql_response.data.ok_or(CliError::NoData)
    }
}
