use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::attachments::mutations::*;
use crate::graphql::attachments::queries::*;
use crate::graphql::attachments::types::*;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::{MutationBuilder, QueryBuilder};

#[derive(clap::Args, Debug)]
pub struct AttachmentsCommand {
    #[command(subcommand)]
    pub action: AttachmentsAction,
}

#[derive(Subcommand, Debug)]
pub enum AttachmentsAction {
    /// List attachments
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single attachment by ID
    Get { id: String },
    /// Create an attachment on an issue
    Create {
        /// Issue ID or identifier
        #[arg(long)]
        issue: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        url: String,
        #[arg(long)]
        subtitle: Option<String>,
        #[arg(long)]
        icon_url: Option<String>,
    },
    /// Update an attachment
    Update {
        id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        subtitle: Option<String>,
        #[arg(long)]
        icon_url: Option<String>,
    },
    /// Delete an attachment
    Delete { id: String },
}

impl AttachmentsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            AttachmentsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Attachment> =
                    paginate(client, &params, |c, page_size, cursor| Box::pin(async move {
                        let vars = AttachmentsListVariables {
                            first: Some(page_size),
                            after: cursor,
                            include_archived,
                            order_by,
                        };
                        let op = AttachmentsListQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.attachments)
                    }))
                    .await?;
                print_output(&result, format)
            }
            AttachmentsAction::Get { id } => {
                let op = AttachmentByIdQuery::build(AttachmentByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.attachment, format)
            }
            AttachmentsAction::Create {
                issue,
                title,
                url,
                subtitle,
                icon_url,
            } => {
                let input = AttachmentCreateInput {
                    issue_id: issue,
                    title,
                    url,
                    subtitle,
                    icon_url,
                };
                let op = AttachmentCreateMutation::build(AttachmentCreateVariables { input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.attachment_create.success,
                    data: Some(data.attachment_create.attachment),
                };
                print_output(&resp, format)
            }
            AttachmentsAction::Update {
                id,
                title,
                subtitle,
                icon_url,
            } => {
                let input = AttachmentUpdateInput {
                    title,
                    subtitle,
                    icon_url,
                };
                let op = AttachmentUpdateMutation::build(AttachmentUpdateVariables { id, input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.attachment_update.success,
                    data: Some(data.attachment_update.attachment),
                };
                print_output(&resp, format)
            }
            AttachmentsAction::Delete { id } => {
                let op = AttachmentDeleteMutation::build(AttachmentDeleteVariables { id });
                let data = client.run_query(op).await?;
                let resp = MutationResponse::<()> {
                    success: data.attachment_delete.success,
                    data: None,
                };
                print_output(&resp, format)
            }
        }
    }
}
