use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::labels::mutations::*;
use crate::graphql::labels::queries::*;
use crate::graphql::labels::types::*;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::{MutationBuilder, QueryBuilder};

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
    /// Create a new label
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        description: Option<String>,
        /// Scope to a team (team ID)
        #[arg(long)]
        team: Option<String>,
        /// Parent label ID (for label groups)
        #[arg(long)]
        parent: Option<String>,
    },
}

impl LabelsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            LabelsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<IssueLabel> =
                    paginate(client, &params, |c, page_size, cursor| {
                        Box::pin(async move {
                            let vars = IssueLabelsListVariables {
                                first: Some(page_size),
                                after: cursor,
                                include_archived,
                                order_by,
                            };
                            let op = IssueLabelsListQuery::build(vars);
                            let data = c.run_query(op).await?;
                            Ok(data.issue_labels)
                        })
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
                let input = IssueLabelCreateInput {
                    name,
                    color,
                    description,
                    team_id: team,
                    parent_id: parent,
                };
                let op = IssueLabelCreateMutation::build(IssueLabelCreateVariables { input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.issue_label_create.success,
                    data: Some(data.issue_label_create.issue_label),
                };
                print_output(&resp, format)
            }
        }
    }
}
