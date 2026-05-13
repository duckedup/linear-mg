use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::documents::mutations::*;
use crate::graphql::documents::queries::*;
use crate::graphql::documents::types::*;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::{MutationBuilder, QueryBuilder};

#[derive(clap::Args, Debug)]
pub struct DocumentsCommand {
    #[command(subcommand)]
    pub action: DocumentsAction,
}

#[derive(Subcommand, Debug)]
pub enum DocumentsAction {
    /// List documents
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single document by ID
    Get { id: String },
    /// Create a new document
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
    /// Update a document
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
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Document> =
                    paginate(client, &params, |c, page_size, cursor| {
                        Box::pin(async move {
                            let vars = DocumentsListVariables {
                                first: Some(page_size),
                                after: cursor,
                                include_archived,
                                order_by,
                            };
                            let op = DocumentsListQuery::build(vars);
                            let data = c.run_query(op).await?;
                            Ok(data.documents)
                        })
                    })
                    .await?;
                print_output(&result, format)
            }
            DocumentsAction::Get { id } => {
                let op = DocumentByIdQuery::build(DocumentByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.document, format)
            }
            DocumentsAction::Create {
                title,
                content,
                project,
                icon,
                color,
            } => {
                let input = DocumentCreateInput {
                    title,
                    content,
                    project_id: project,
                    icon,
                    color,
                };
                let op = DocumentCreateMutation::build(DocumentCreateVariables { input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.document_create.success,
                    data: Some(data.document_create.document),
                };
                print_output(&resp, format)
            }
            DocumentsAction::Update {
                id,
                title,
                content,
                project,
                icon,
                color,
            } => {
                let input = DocumentUpdateInput {
                    title,
                    content,
                    project_id: project,
                    icon,
                    color,
                };
                let op = DocumentUpdateMutation::build(DocumentUpdateVariables { id, input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.document_update.success,
                    data: Some(data.document_update.document),
                };
                print_output(&resp, format)
            }
        }
    }
}
