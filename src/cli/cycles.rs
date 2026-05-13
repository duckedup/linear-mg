use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::cycles::Cycle;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct CyclesCommand {
    #[command(subcommand)]
    pub action: CyclesAction,
}

#[derive(Subcommand, Debug)]
pub enum CyclesAction {
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Get {
        id: String,
    },
}

impl CyclesCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            CyclesAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Cycle> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_cycles(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            CyclesAction::Get { id } => print_output(&client.get_cycle(&id).await?, format),
        }
    }
}
