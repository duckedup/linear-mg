use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::initiatives::Initiative;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct InitiativesCommand {
    #[command(subcommand)]
    pub action: InitiativesAction,
}

#[derive(Subcommand, Debug)]
pub enum InitiativesAction {
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Get {
        id: String,
    },
}

impl InitiativesCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            InitiativesAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Initiative> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_initiatives(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            InitiativesAction::Get { id } => {
                print_output(&client.get_initiative(&id).await?, format)
            }
        }
    }
}
