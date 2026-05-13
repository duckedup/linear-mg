use crate::client::LinearClient;
use crate::config::{AuthConfig, Config};
use crate::error::CliError;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;
use std::io::{BufRead, Write};

#[derive(clap::Args, Debug)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub action: AuthAction,
}

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Set up authentication with a Linear API key
    Init {
        /// API key (if omitted, you will be prompted)
        #[arg(long)]
        key: Option<String>,
    },
    /// Show the currently authenticated user
    Status,
    /// Remove stored authentication
    Revoke,
}

impl AuthCommand {
    pub async fn run(self, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            AuthAction::Init { key } => {
                let api_key = match key {
                    Some(k) => k,
                    None => prompt_for_key()?,
                };

                let client = LinearClient::new(api_key.clone());
                let user = client.get_viewer().await.map_err(|_| {
                    CliError::AuthError("Invalid API key - could not authenticate".into())
                })?;

                let mut config = Config::load()?;
                config.auth = Some(AuthConfig {
                    api_key: Some(api_key),
                });
                config.save()?;

                match format {
                    OutputFormat::Pretty => {
                        println!(
                            "Authenticated as {} <{}>",
                            user.display_name, user.email
                        );
                        println!("API key saved to {}", Config::path().display());
                    }
                    _ => {
                        print_output(
                            &serde_json::json!({
                                "message": "Authenticated",
                                "user": user.display_name,
                                "email": user.email,
                                "path": Config::path().display().to_string()
                            }),
                            format,
                        )?;
                    }
                }
                Ok(())
            }
            AuthAction::Revoke => {
                let mut config = Config::load()?;
                config.auth = None;
                config.save()?;
                match format {
                    OutputFormat::Pretty => println!("API key removed."),
                    _ => {
                        print_output(
                            &serde_json::json!({ "message": "API key removed" }),
                            format,
                        )?;
                    }
                }
                Ok(())
            }
            AuthAction::Status => {
                let config = Config::load()?;
                let api_key = crate::config::auth::resolve_api_key(None, config.api_key())?;
                let client = LinearClient::new(api_key);
                let user = client.get_viewer().await?;
                print_output(&user, format)
            }
        }
    }
}

fn prompt_for_key() -> Result<String, CliError> {
    eprintln!("Create a personal API key at: https://linear.app/settings/api\n");
    eprint!("API key: ");
    std::io::stderr().flush()?;

    let key = std::io::stdin()
        .lock()
        .lines()
        .next()
        .ok_or_else(|| CliError::InvalidInput("No input received".into()))??
        .trim()
        .to_string();

    if key.is_empty() {
        return Err(CliError::InvalidInput("API key cannot be empty".into()));
    }

    Ok(key)
}
