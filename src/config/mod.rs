pub mod auth;

use crate::error::CliError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Config {
    pub auth: Option<AuthConfig>,
    pub defaults: Option<DefaultsConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthConfig {
    pub api_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefaultsConfig {
    pub team: Option<String>,
    pub format: Option<String>,
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("linear-mg")
            .join("config.toml")
    }

    pub fn load() -> Result<Self, CliError> {
        let path = Self::path();
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<(), CliError> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn api_key(&self) -> Option<&str> {
        self.auth.as_ref()?.api_key.as_deref()
    }
}
