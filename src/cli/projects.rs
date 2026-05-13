use crate::cli::common::PaginationArgs;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::projects::Project;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

#[derive(clap::Args, Debug)]
pub struct ProjectsCommand {
    #[command(subcommand)]
    pub action: ProjectsAction,
}

#[derive(Subcommand, Debug)]
pub enum ProjectsAction {
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_delimiter = ',')]
        teams: Vec<String>,
        #[arg(long)]
        lead: Option<String>,
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        target_date: Option<String>,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        priority: Option<i32>,
        #[arg(long)]
        status: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        lead: Option<String>,
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        target_date: Option<String>,
        #[arg(long)]
        color: Option<String>,
        #[arg(long)]
        icon: Option<String>,
        #[arg(long)]
        priority: Option<i32>,
        #[arg(long)]
        status: Option<String>,
    },
    Archive {
        id: String,
    },
    Delete {
        id: String,
    },
}

impl ProjectsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            ProjectsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let ia = pagination.include_archived;
                let ob = pagination.order_by.as_str().to_string();
                let result: ListResponse<Project> = paginate(client, &params, |c, ps, cur| {
                    let ob = ob.clone();
                    Box::pin(async move { c.list_projects(ps, cur, ia, &ob).await })
                })
                .await?;
                print_output(&result, format)
            }
            ProjectsAction::Get { id } => print_output(&client.get_project(&id).await?, format),
            ProjectsAction::Create {
                name,
                description,
                teams,
                lead,
                start_date,
                target_date,
                color,
                icon,
                priority,
                status,
            } => {
                let mut input = serde_json::json!({ "name": name, "teamIds": teams });
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                if let Some(v) = lead {
                    obj.insert("leadId".into(), v.into());
                }
                if let Some(v) = start_date {
                    obj.insert("startDate".into(), v.into());
                }
                if let Some(v) = target_date {
                    obj.insert("targetDate".into(), v.into());
                }
                if let Some(v) = color {
                    obj.insert("color".into(), v.into());
                }
                if let Some(v) = icon {
                    obj.insert("icon".into(), v.into());
                }
                if let Some(v) = priority {
                    obj.insert("priority".into(), v.into());
                }
                if let Some(v) = status {
                    obj.insert("statusId".into(), v.into());
                }
                let p = client.create_project(input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: p.project,
                    },
                    format,
                )
            }
            ProjectsAction::Update {
                id,
                name,
                description,
                lead,
                start_date,
                target_date,
                color,
                icon,
                priority,
                status,
            } => {
                let mut input = serde_json::json!({});
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = name {
                    obj.insert("name".into(), v.into());
                }
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                if let Some(v) = lead {
                    obj.insert("leadId".into(), v.into());
                }
                if let Some(v) = start_date {
                    obj.insert("startDate".into(), v.into());
                }
                if let Some(v) = target_date {
                    obj.insert("targetDate".into(), v.into());
                }
                if let Some(v) = color {
                    obj.insert("color".into(), v.into());
                }
                if let Some(v) = icon {
                    obj.insert("icon".into(), v.into());
                }
                if let Some(v) = priority {
                    obj.insert("priority".into(), v.into());
                }
                if let Some(v) = status {
                    obj.insert("statusId".into(), v.into());
                }
                let p = client.update_project(&id, input).await?;
                print_output(
                    &MutationResponse {
                        success: p.success,
                        data: p.project,
                    },
                    format,
                )
            }
            ProjectsAction::Archive { id } => {
                let p = client.archive_project(&id).await?;
                print_output(
                    &MutationResponse::<()> {
                        success: p.success,
                        data: None,
                    },
                    format,
                )
            }
            ProjectsAction::Delete { id } => {
                let p = client.delete_project(&id).await?;
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
