use crate::client::LinearClient;
use crate::config::{AuthConfig, Config};
use crate::error::CliError;
use crate::graphql::users::queries::ViewerQuery;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::QueryBuilder;

#[derive(clap::Args, Debug)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Store an API key for authentication
    Login {
        /// The Linear API key
        #[arg(long)]
        key: String,
    },
    /// Remove stored authentication
    Logout,
    /// Show the currently authenticated user
    Whoami,
}

impl AuthCommand {
    pub async fn run(self, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            AuthAction::Login { key } => {
                let mut config = Config::load()?;
                config.auth = Some(AuthConfig {
                    api_key: Some(key),
                });
                config.save()?;
                let msg = serde_json::json!({ "message": "API key saved", "path": Config::path().display().to_string() });
                print_output(&msg, format)
            }
            AuthAction::Logout => {
                let mut config = Config::load()?;
                config.auth = None;
                config.save()?;
                let msg = serde_json::json!({ "message": "API key removed" });
                print_output(&msg, format)
            }
            AuthAction::Whoami => {
                let config = Config::load()?;
                let api_key = crate::config::auth::resolve_api_key(None, config.api_key())?;
                let client = LinearClient::new(api_key);
                let op = ViewerQuery::build(());
                let data = client.run_query(op).await?;
                print_output(&data.viewer, format)
            }
        }
    }
}
