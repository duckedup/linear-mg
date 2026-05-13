use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::teams::Team;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct TeamsCommand {
    #[command(subcommand)]
    pub action: TeamsAction,
}

#[derive(Subcommand, Debug)]
pub enum TeamsAction {
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Get {
        id: String,
    },
}

impl TeamsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            TeamsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Team> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_teams(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            TeamsAction::Get { id } => print_output(&client.get_team(&id).await?, format),
        }
    }
}
