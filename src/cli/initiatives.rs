use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::initiatives::queries::*;
use crate::graphql::initiatives::types::*;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::QueryBuilder;

#[derive(clap::Args, Debug)]
pub struct InitiativesCommand {
    #[command(subcommand)]
    pub action: InitiativesAction,
}

#[derive(Subcommand, Debug)]
pub enum InitiativesAction {
    /// List initiatives
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single initiative by ID
    Get { id: String },
}

impl InitiativesCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            InitiativesAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Initiative> =
                    paginate(client, &params, |c, page_size, cursor| Box::pin(async move {
                        let vars = InitiativesListVariables {
                            first: Some(page_size),
                            after: cursor,
                            include_archived,
                            order_by,
                        };
                        let op = InitiativesListQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.initiatives)
                    }))
                    .await?;
                print_output(&result, format)
            }
            InitiativesAction::Get { id } => {
                let op = InitiativeByIdQuery::build(InitiativeByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.initiative, format)
            }
        }
    }
}
