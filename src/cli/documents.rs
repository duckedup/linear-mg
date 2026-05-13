use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::documents::Document;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct DocumentsCommand {
    #[command(subcommand)]
    pub action: DocumentsAction,
}

#[derive(Subcommand, Debug)]
pub enum DocumentsAction {
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
}

impl DocumentsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            DocumentsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Document> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_documents(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            DocumentsAction::Get { id } => print_output(&client.get_document(&id).await?, format),
            DocumentsAction::Create {
                title,
                content,
                project,
                icon,
                color,
            } => {
                let mut input = serde_json::json!({ "title": title });
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = content {
                    obj.insert("content".into(), v.into());
                }
                if let Some(v) = project {
                    obj.insert("projectId".into(), v.into());
                }
                if let Some(v) = icon {
                    obj.insert("icon".into(), v.into());
                }
                if let Some(v) = color {
                    obj.insert("color".into(), v.into());
                }
                let p = client.create_document(input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.document),
                    },
                    format,
                )
            }
            DocumentsAction::Update {
                id,
                title,
                content,
                project,
                icon,
                color,
            } => {
                let mut input = serde_json::json!({});
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = title {
                    obj.insert("title".into(), v.into());
                }
                if let Some(v) = content {
                    obj.insert("content".into(), v.into());
                }
                if let Some(v) = project {
                    obj.insert("projectId".into(), v.into());
                }
                if let Some(v) = icon {
                    obj.insert("icon".into(), v.into());
                }
                if let Some(v) = color {
                    obj.insert("color".into(), v.into());
                }
                let p = client.update_document(&id, input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.document),
                    },
                    format,
                )
            }
        }
    }
}
