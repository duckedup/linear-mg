use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::workflow_states::WorkflowState;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct WorkflowStatesCommand {
    #[command(subcommand)]
    pub action: WorkflowStatesAction,
}

#[derive(Subcommand, Debug)]
pub enum WorkflowStatesAction {
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
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<WorkflowState> =
                    paginate(client, &params, |c, ps, cur| {
                        let ob = ob.clone();
                        Box::pin(async move { c.list_workflow_states(ps, cur, ia, &ob).await })
                    })
                    .await?;
                print_output(&result, format)
            }
        }
    }
}
