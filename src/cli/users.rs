use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::ListResponse;
use crate::graphql::users::queries::*;
use crate::graphql::users::types::*;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::QueryBuilder;

#[derive(clap::Args, Debug)]
pub struct UsersCommand {
    #[command(subcommand)]
    pub action: UsersAction,
}

#[derive(Subcommand, Debug)]
pub enum UsersAction {
    /// List all users
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single user by ID
    Get {
        /// User ID
        id: String,
    },
    /// Show the currently authenticated user
    Me,
}

impl UsersCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            UsersAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<User> =
                    paginate(client, &params, |c, page_size, cursor| {
                        Box::pin(async move {
                            let vars = UsersListVariables {
                                first: Some(page_size),
                                after: cursor,
                                include_archived,
                                order_by,
                            };
                            let op = UsersListQuery::build(vars);
                            let data = c.run_query(op).await?;
                            Ok(data.users)
                        })
                    })
                    .await?;
                print_output(&result, format)
            }
            UsersAction::Get { id } => {
                let op = UserByIdQuery::build(UserByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.user, format)
            }
            UsersAction::Me => {
                let op = ViewerQuery::build(());
                let data = client.run_query(op).await?;
                print_output(&data.viewer, format)
            }
        }
    }
}
