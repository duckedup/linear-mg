use crate::cli::common::PaginationArgs;
use crate::cli::resolve;
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
    /// List comments, optionally filtered by issue
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
        /// Filter by issue ID or identifier (e.g., "ENG-123")
        #[arg(long)]
        issue: Option<String>,
    },
    /// Get a single comment by ID
    Get { id: String },
    /// Create a new comment on an issue
    Create {
        /// Issue ID or identifier (e.g., "ENG-123")
        #[arg(long)]
        issue: String,
        /// Comment body (markdown)
        #[arg(long)]
        body: String,
        /// Parent comment ID to reply to
        #[arg(long)]
        parent: Option<String>,
    },
    /// Update an existing comment
    Update {
        id: String,
        #[arg(long)]
        body: String,
    },
    /// Delete a comment
    Delete { id: String },
}

impl CommentsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            CommentsAction::List { pagination, issue } => {
                let filter = if let Some(ref iss) = issue {
                    let issue_id = resolve::resolve_issue(client, iss).await?;
                    Some(serde_json::json!({ "issue": { "id": { "eq": issue_id } } }))
                } else {
                    None
                };
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Comment> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    let filter = filter.clone();
                    Box::pin(async move { c.list_comments(ps, cur, ia, &ob, filter).await })
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
                let issue_id = resolve::resolve_issue(client, &issue).await?;
                let mut input = serde_json::json!({ "issueId": issue_id, "body": body });
                if let Some(p) = parent
                    && let Some(obj) = input.as_object_mut()
                {
                    obj.insert("parentId".into(), p.into());
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
