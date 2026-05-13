use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::workflow_states::queries::*;
use crate::graphql::workflow_states::types::*;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::QueryBuilder;

#[derive(clap::Args, Debug)]
pub struct WorkflowStatesCommand {
    #[command(subcommand)]
    pub action: WorkflowStatesAction,
}

#[derive(Subcommand, Debug)]
pub enum WorkflowStatesAction {
    /// List workflow states
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

impl WorkflowStatesCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            WorkflowStatesAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<WorkflowState> =
                    paginate(client, &params, |c, page_size, cursor| Box::pin(async move {
                        let vars = WorkflowStatesListVariables {
                            first: Some(page_size),
                            after: cursor,
                            include_archived,
                            order_by,
                        };
                        let op = WorkflowStatesListQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.workflow_states)
                    }))
                    .await?;
                print_output(&result, format)
            }
        }
    }
}
