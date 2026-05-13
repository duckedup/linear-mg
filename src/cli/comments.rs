use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::comments::Comment;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct CommentsCommand {
    #[command(subcommand)]
    pub action: CommentsAction,
}

#[derive(Subcommand, Debug)]
pub enum CommentsAction {
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        issue: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        parent: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        body: String,
    },
    Delete {
        id: String,
    },
}

impl CommentsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            CommentsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Comment> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_comments(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            CommentsAction::Get { id } => print_output(&client.get_comment(&id).await?, format),
            CommentsAction::Create {
                issue,
                body,
                parent,
            } => {
                let mut input = serde_json::json!({ "issueId": issue, "body": body });
                if let Some(p) = parent {
                    input
                        .as_object_mut()
                        .unwrap()
                        .insert("parentId".into(), p.into());
                }
                let p = client.create_comment(input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: p.comment,
                    },
                    format,
                )
            }
            CommentsAction::Update { id, body } => {
                let p = client
                    .update_comment(&id, serde_json::json!({ "body": body }))
                    .await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: p.comment,
                    },
                    format,
                )
            }
            CommentsAction::Delete { id } => {
                let p = client.delete_comment(&id).await?;
                print_output(
                    &MutationResponse::<()> {
                        success: p.success,
                        data: None,
                    },
                    format,
                )
            }
        }
    }
}
