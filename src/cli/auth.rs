use crate::client::LinearClient;
use crate::config::{AuthConfig, Config};
use crate::error::CliError;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Store an API key for authentication
    Login {
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
                config.auth = Some(AuthConfig { api_key: Some(key) });
                config.save()?;
                let msg = serde_json::json!({ "message": "API key saved", "path": Config::path().display().to_string() });
                print_output(&msg, format)
            }
            AuthAction::Logout => {
                let mut config = Config::load()?;
                config.auth = None;
                config.save()?;
                print_output(&serde_json::json!({ "message": "API key removed" }), format)
            }
            AuthAction::Whoami => {
                let config = Config::load()?;
                let api_key = crate::config::auth::resolve_api_key(None, config.api_key())?;
                let client = LinearClient::new(api_key);
                let user = client.get_viewer().await?;
                print_output(&user, format)
            }
        }
    }
}
