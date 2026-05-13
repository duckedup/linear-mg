use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::attachments::Attachment;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct AttachmentsCommand {
    #[command(subcommand)]
    pub action: AttachmentsAction,
}

#[derive(Subcommand, Debug)]
pub enum AttachmentsAction {
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
        title: String,
        #[arg(long)]
        url: String,
        #[arg(long)]
        subtitle: Option<String>,
        #[arg(long)]
        icon_url: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        subtitle: Option<String>,
        #[arg(long)]
        icon_url: Option<String>,
    },
    Delete {
        id: String,
    },
}

impl AttachmentsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            AttachmentsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Attachment> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_attachments(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            AttachmentsAction::Get { id } => {
                print_output(&client.get_attachment(&id).await?, format)
            }
            AttachmentsAction::Create {
                issue,
                title,
                url,
                subtitle,
                icon_url,
            } => {
                let mut input = serde_json::json!({ "issueId": issue, "title": title, "url": url });
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = subtitle {
                    obj.insert("subtitle".into(), v.into());
                }
                if let Some(v) = icon_url {
                    obj.insert("iconUrl".into(), v.into());
                }
                let p = client.create_attachment(input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.attachment),
                    },
                    format,
                )
            }
            AttachmentsAction::Update {
                id,
                title,
                subtitle,
                icon_url,
            } => {
                let mut input = serde_json::json!({ "title": title });
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = subtitle {
                    obj.insert("subtitle".into(), v.into());
                }
                if let Some(v) = icon_url {
                    obj.insert("iconUrl".into(), v.into());
                }
                let p = client.update_attachment(&id, input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.attachment),
                    },
                    format,
                )
            }
            AttachmentsAction::Delete { id } => {
                let p = client.delete_attachment(&id).await?;
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
