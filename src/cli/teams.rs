use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::teams::queries::*;
use crate::graphql::teams::types::*;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::QueryBuilder;

#[derive(clap::Args, Debug)]
pub struct TeamsCommand {
    #[command(subcommand)]
    pub action: TeamsAction,
}

#[derive(Subcommand, Debug)]
pub enum TeamsAction {
    /// List all teams
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single team by ID
    Get {
        /// Team ID
        id: String,
    },
}

impl TeamsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            TeamsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Team> =
                    paginate(client, &params, |c, page_size, cursor| {
                        Box::pin(async move {
                            let vars = TeamsListVariables {
                                first: Some(page_size),
                                after: cursor,
                                include_archived,
                                order_by,
                            };
                            let op = TeamsListQuery::build(vars);
                            let data = c.run_query(op).await?;
                            Ok(data.teams)
                        })
                    })
                    .await?;
                print_output(&result, format)
            }
            TeamsAction::Get { id } => {
                let op = TeamByIdQuery::build(TeamByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.team, format)
            }
        }
    }
}
