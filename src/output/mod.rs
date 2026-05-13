pub mod pretty;

use crate::error::CliError;

#[derive(Clone, Debug, Default)]
pub enum OutputFormat {
    #[default]
    Pretty,
    Json,
    JsonPretty,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pretty" => Ok(Self::Pretty),
            "json" => Ok(Self::Json),
            "json-pretty" => Ok(Self::JsonPretty),
            _ => Err(format!("unknown format: {s}")),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pretty => write!(f, "pretty"),
            Self::Json => write!(f, "json"),
            Self::JsonPretty => write!(f, "json-pretty"),
        }
    }
}

pub trait PrettyPrint {
    fn pretty(&self) -> String;
    fn pretty_row(&self) -> Vec<String> {
        vec![self.pretty()]
    }
}

pub fn print_output<T: serde::Serialize + PrettyPrint>(
    data: &T,
    format: &OutputFormat,
) -> Result<(), CliError> {
    let output = match format {
        OutputFormat::Pretty => data.pretty(),
        OutputFormat::Json => serde_json::to_string(data)?,
        OutputFormat::JsonPretty => serde_json::to_string_pretty(data)?,
    };
    println!("{output}");
    Ok(())
}
