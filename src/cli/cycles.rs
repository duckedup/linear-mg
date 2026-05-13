use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::cycles::queries::*;
use crate::graphql::cycles::types::*;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::QueryBuilder;

#[derive(clap::Args, Debug)]
pub struct CyclesCommand {
    #[command(subcommand)]
    pub action: CyclesAction,
}

#[derive(Subcommand, Debug)]
pub enum CyclesAction {
    /// List cycles
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single cycle by ID
    Get { id: String },
}

impl CyclesCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            CyclesAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Cycle> =
                    paginate(client, &params, |c, page_size, cursor| Box::pin(async move {
                        let vars = CyclesListVariables {
                            first: Some(page_size),
                            after: cursor,
                            include_archived,
                            order_by,
                        };
                        let op = CyclesListQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.cycles)
                    }))
                    .await?;
                print_output(&result, format)
            }
            CyclesAction::Get { id } => {
                let op = CycleByIdQuery::build(CycleByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.cycle, format)
            }
        }
    }
}
