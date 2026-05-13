use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::users::User;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct UsersCommand {
    #[command(subcommand)]
    pub action: UsersAction,
}

#[derive(Subcommand, Debug)]
pub enum UsersAction {
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Get {
        id: String,
    },
    Me,
}

impl UsersCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            UsersAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<User> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_users(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            UsersAction::Get { id } => print_output(&client.get_user(&id).await?, format),
            UsersAction::Me => print_output(&client.get_viewer().await?, format),
        }
    }
}
