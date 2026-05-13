use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::milestones::mutations::*;
use crate::graphql::milestones::queries::*;
use crate::graphql::milestones::types::*;
use crate::graphql::scalars::TimelessDate;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::{MutationBuilder, QueryBuilder};

#[derive(clap::Args, Debug)]
pub struct MilestonesCommand {
    #[command(subcommand)]
    pub action: MilestonesAction,
}

#[derive(Subcommand, Debug)]
pub enum MilestonesAction {
    /// List project milestones
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single milestone by ID
    Get { id: String },
    /// Create a new milestone
    Create {
        #[arg(long)]
        name: String,
        /// Project ID
        #[arg(long)]
        project: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        target_date: Option<String>,
    },
    /// Update a milestone
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        target_date: Option<String>,
    },
}

impl MilestonesCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            MilestonesAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<ProjectMilestone> =
                    paginate(client, &params, |c, page_size, cursor| Box::pin(async move {
                        let vars = ProjectMilestonesListVariables {
                            first: Some(page_size),
                            after: cursor,
                            include_archived,
                            order_by,
                        };
                        let op = ProjectMilestonesListQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.project_milestones)
                    }))
                    .await?;
                print_output(&result, format)
            }
            MilestonesAction::Get { id } => {
                let op = ProjectMilestoneByIdQuery::build(ProjectMilestoneByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.project_milestone, format)
            }
            MilestonesAction::Create {
                name,
                project,
                description,
                target_date,
            } => {
                let input = ProjectMilestoneCreateInput {
                    name,
                    project_id: project,
                    description,
                    target_date: target_date.map(TimelessDate::from),
                };
                let op =
                    ProjectMilestoneCreateMutation::build(ProjectMilestoneCreateVariables { input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.project_milestone_create.success,
                    data: Some(data.project_milestone_create.project_milestone),
                };
                print_output(&resp, format)
            }
            MilestonesAction::Update {
                id,
                name,
                description,
                target_date,
            } => {
                let input = ProjectMilestoneUpdateInput {
                    name,
                    description,
                    target_date: target_date.map(TimelessDate::from),
                };
                let op =
                    ProjectMilestoneUpdateMutation::build(ProjectMilestoneUpdateVariables {
                        id,
                        input,
                    });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.project_milestone_update.success,
                    data: Some(data.project_milestone_update.project_milestone),
                };
                print_output(&resp, format)
            }
        }
    }
}
