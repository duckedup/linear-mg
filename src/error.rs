#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("No API key configured. Set LINEAR_API_KEY or run `linear-mg auth login`")]
    NoApiKey,

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Rate limited. Retry after {retry_after:?} seconds")]
    RateLimited { retry_after: Option<u64> },

    #[error("HTTP error {status}: {body}")]
    HttpError {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("GraphQL errors: {0:?}")]
    GraphQlErrors(Vec<String>),

    #[error("No data returned from API")]
    NoData,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}

impl CliError {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "error": {
                "type": self.error_type(),
                "message": self.to_string(),
            }
        })
    }

    fn error_type(&self) -> &'static str {
        match self {
            Self::NoApiKey => "no_api_key",
            Self::AuthError(_) => "auth_error",
            Self::RateLimited { .. } => "rate_limited",
            Self::HttpError { .. } => "http_error",
            Self::GraphQlErrors(_) => "graphql_error",
            Self::NoData => "no_data",
            Self::NotFound(_) => "not_found",
            Self::InvalidInput(_) => "invalid_input",
            Self::ConfigError(_) => "config_error",
            Self::Reqwest(_) => "request_error",
            Self::Io(_) => "io_error",
            Self::SerdeJson(_) => "json_error",
            Self::TomlParse(_) => "toml_parse_error",
            Self::TomlSerialize(_) => "toml_serialize_error",
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            Self::NoApiKey | Self::AuthError(_) => 2,
            Self::RateLimited { .. } => 3,
            Self::NotFound(_) => 4,
            Self::InvalidInput(_) => 5,
            Self::HttpError { .. } | Self::GraphQlErrors(_) | Self::NoData => 6,
            _ => 1,
        }
    }
}
