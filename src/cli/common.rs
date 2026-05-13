use crate::output::OutputFormat;
use clap::Args;

#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Linear API key (overrides config and env)
    #[arg(long, env = "LINEAR_API_KEY", global = true, hide_env_values = true)]
    pub api_key: Option<String>,

    /// Enable verbose output
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,
}

impl GlobalArgs {
    pub fn output_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            OutputFormat::Pretty
        }
    }
}

#[derive(Args, Debug, Clone)]
pub struct PaginationArgs {
    /// Maximum number of results to return
    #[arg(long, default_value = "50")]
    pub limit: u32,

    /// Cursor for manual pagination
    #[arg(long)]
    pub after: Option<String>,

    /// Fetch all results (auto-paginate)
    #[arg(long)]
    pub all: bool,

    /// Include archived items
    #[arg(long)]
    pub include_archived: bool,

    /// Order by field
    #[arg(long, value_enum, default_value = "created-at")]
    pub order_by: OrderBy,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum OrderBy {
    CreatedAt,
    UpdatedAt,
}

impl OrderBy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreatedAt => "createdAt",
            Self::UpdatedAt => "updatedAt",
        }
    }
}

impl PaginationArgs {
    pub fn to_paginator_params(&self) -> crate::client::paginator::PaginationParams {
        crate::client::paginator::PaginationParams {
            limit: Some(self.limit),
            after: self.after.clone(),
            all: self.all,
            page_size: std::cmp::min(self.limit.max(1), 250),
        }
    }
}
