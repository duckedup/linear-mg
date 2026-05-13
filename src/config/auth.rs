use crate::error::CliError;

pub fn resolve_api_key(
    cli_key: Option<&str>,
    config_key: Option<&str>,
) -> Result<String, CliError> {
    cli_key
        .map(String::from)
        .or_else(|| std::env::var("LINEAR_API_KEY").ok())
        .or_else(|| config_key.map(String::from))
        .ok_or(CliError::NoApiKey)
}
