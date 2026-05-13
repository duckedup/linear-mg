use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::comments::mutations::*;
use crate::graphql::comments::queries::*;
use crate::graphql::comments::types::*;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::{MutationBuilder, QueryBuilder};

#[derive(clap::Args, Debug)]
pub struct CommentsCommand {
    #[command(subcommand)]
    pub action: CommentsAction,
}

#[derive(Subcommand, Debug)]
pub enum CommentsAction {
    /// List comments
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single comment by ID
    Get { id: String },
    /// Create a comment on an issue
    Create {
        /// Issue ID or identifier
        #[arg(long)]
        issue: String,
        /// Comment body (markdown)
        #[arg(long)]
        body: String,
        /// Parent comment ID for threading
        #[arg(long)]
        parent: Option<String>,
    },
    /// Update a comment
    Update {
        /// Comment ID
        id: String,
        /// New body (markdown)
        #[arg(long)]
        body: String,
    },
    /// Delete a comment
    Delete {
        /// Comment ID
        id: String,
    },
}

impl CommentsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            CommentsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Comment> =
                    paginate(client, &params, |c, page_size, cursor| {
                        Box::pin(async move {
                            let vars = CommentsListVariables {
                                first: Some(page_size),
                                after: cursor,
                                include_archived,
                                order_by,
                            };
                            let op = CommentsListQuery::build(vars);
                            let data = c.run_query(op).await?;
                            Ok(data.comments)
                        })
                    })
                    .await?;
                print_output(&result, format)
            }
            CommentsAction::Get { id } => {
                let op = CommentByIdQuery::build(CommentByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.comment, format)
            }
            CommentsAction::Create {
                issue,
                body,
                parent,
            } => {
                let input = CommentCreateInput {
                    issue_id: Some(issue),
                    body,
                    parent_id: parent,
                };
                let op = CommentCreateMutation::build(CommentCreateVariables { input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.comment_create.success,
                    data: data.comment_create.comment,
                };
                print_output(&resp, format)
            }
            CommentsAction::Update { id, body } => {
                let input = CommentUpdateInput { body };
                let op = CommentUpdateMutation::build(CommentUpdateVariables { id, input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.comment_update.success,
                    data: data.comment_update.comment,
                };
                print_output(&resp, format)
            }
            CommentsAction::Delete { id } => {
                let op = CommentDeleteMutation::build(CommentDeleteVariables { id });
                let data = client.run_query(op).await?;
                let resp = MutationResponse::<()> {
                    success: data.comment_delete.success,
                    data: None,
                };
                print_output(&resp, format)
            }
        }
    }
}
