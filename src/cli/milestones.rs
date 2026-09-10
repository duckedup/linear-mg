use crate::cli::common::PaginationArgs;
use crate::cli::resolve;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::milestones::{ProjectMilestone, milestone_filter};
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct MilestonesCommand {
    #[command(subcommand)]
    pub action: MilestonesAction,
}

#[derive(Subcommand, Debug)]
pub enum MilestonesAction {
    /// List milestones, optionally scoped to a project
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
        /// Filter by project (ID, name, or slug)
        #[arg(long)]
        project: Option<String>,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        /// Project to create the milestone in (ID, name, or slug)
        #[arg(long)]
        project: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        target_date: Option<String>,
    },
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
            MilestonesAction::List {
                pagination,
                project,
            } => {
                let project_id = match project {
                    Some(p) => Some(resolve::resolve_project(client, &p).await?),
                    None => None,
                };
                let filter = milestone_filter(project_id.as_deref(), None);
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<ProjectMilestone> =
                    paginate(client, &params, |c, ps, cur| {
                        let ob = ob.clone();
                        let filter = filter.clone();
                        Box::pin(async move { c.list_milestones(ps, cur, ia, &ob, filter).await })
                    })
                    .await?;
                print_output(&result, format)
            }
            MilestonesAction::Get { id } => print_output(&client.get_milestone(&id).await?, format),
            MilestonesAction::Create {
                name,
                project,
                description,
                target_date,
            } => {
                let project_id = resolve::resolve_project(client, &project).await?;
                let mut input = serde_json::json!({ "name": name, "projectId": project_id });
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                if let Some(v) = target_date {
                    obj.insert("targetDate".into(), v.into());
                }
                let p = client.create_milestone(input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.project_milestone),
                    },
                    format,
                )
            }
            MilestonesAction::Update {
                id,
                name,
                description,
                target_date,
            } => {
                let mut input = serde_json::json!({});
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = name {
                    obj.insert("name".into(), v.into());
                }
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                if let Some(v) = target_date {
                    obj.insert("targetDate".into(), v.into());
                }
                let p = client.update_milestone(&id, input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: Some(p.project_milestone),
                    },
                    format,
                )
            }
        }
    }
}
