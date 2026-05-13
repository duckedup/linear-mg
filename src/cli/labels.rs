use crate::cli::common::PaginationArgs;
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
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        team: Option<String>,
        #[arg(long)]
        parent: Option<String>,
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
                if let Some(v) = team {
                    obj.insert("teamId".into(), v.into());
                }
                if let Some(v) = parent {
                    obj.insert("parentId".into(), v.into());
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
        }
    }
}
