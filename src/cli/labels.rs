use crate::cli::common::PaginationArgs;
use crate::cli::resolve;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::labels::IssueLabel;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct LabelsCommand {
    #[command(subcommand)]
    pub action: LabelsAction,
}

#[derive(Subcommand, Debug)]
pub enum LabelsAction {
    /// List issue labels
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single label by ID or name
    Get {
        /// Label ID, or name (must be unambiguous)
        id: String,
    },
    /// Create a new label
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        description: Option<String>,
        /// Team key/name/ID to scope the label to (omit for a workspace label)
        #[arg(long)]
        team: Option<String>,
        /// Parent label ID or name (for label groups)
        #[arg(long)]
        parent: Option<String>,
    },
    /// Update an existing label
    Update {
        /// Label ID, or name (must be unambiguous)
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        description: Option<String>,
        /// Parent label ID or name (for label groups)
        #[arg(long)]
        parent: Option<String>,
    },
    /// Delete a label
    Delete {
        /// Label ID, or name (must be unambiguous)
        id: String,
    },
}

impl LabelsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            LabelsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<IssueLabel> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_labels(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            LabelsAction::Get { id } => {
                let label_id = resolve::resolve_label(client, &id, None).await?;
                print_output(&client.get_label(&label_id).await?, format)
            }
            LabelsAction::Create {
                name,
                color,
                description,
                team,
                parent,
            } => {
                let mut input = serde_json::json!({ "name": name });
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = color {
                    obj.insert("color".into(), v.into());
                }
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                let team_id = if let Some(v) = team {
                    let tid = resolve::resolve_team(client, &v).await?;
                    obj.insert("teamId".into(), tid.clone().into());
                    Some(tid)
                } else {
                    None
                };
                if let Some(v) = parent {
                    let pid = resolve::resolve_label(client, &v, team_id.as_deref()).await?;
                    obj.insert("parentId".into(), pid.into());
                }
                let p = client.create_label(input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.issue_label),
                    },
                    format,
                )
            }
            LabelsAction::Update {
                id,
                name,
                color,
                description,
                parent,
            } => {
                let label_id = resolve::resolve_label(client, &id, None).await?;
                let mut input = serde_json::json!({});
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = name {
                    obj.insert("name".into(), v.into());
                }
                if let Some(v) = color {
                    obj.insert("color".into(), v.into());
                }
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                if let Some(v) = parent {
                    let pid = resolve::resolve_label(client, &v, None).await?;
                    obj.insert("parentId".into(), pid.into());
                }
                let p = client.update_label(&label_id, input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.issue_label),
                    },
                    format,
                )
            }
            LabelsAction::Delete { id } => {
                let label_id = resolve::resolve_label(client, &id, None).await?;
                let p = client.delete_label(&label_id).await?;
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
