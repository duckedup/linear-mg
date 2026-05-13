use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::projects::mutations::*;
use crate::graphql::projects::queries::*;
use crate::graphql::projects::types::*;
use crate::graphql::scalars::TimelessDate;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::{MutationBuilder, QueryBuilder};

#[derive(clap::Args, Debug)]
pub struct ProjectsCommand {
    #[command(subcommand)]
    pub action: ProjectsAction,
}

#[derive(Subcommand, Debug)]
pub enum ProjectsAction {
    /// List projects
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
    },
    /// Get a single project by ID
    Get { id: String },
    /// Create a new project
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
        /// Team IDs (comma-separated)
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
    /// Update an existing project
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
    /// Archive a project
    Archive { id: String },
    /// Delete a project
    Delete { id: String },
}

impl ProjectsCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            ProjectsAction::List { pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Project> =
                    paginate(client, &params, |c, page_size, cursor| Box::pin(async move {
                        let vars = ProjectsListVariables {
                            first: Some(page_size),
                            after: cursor,
                            include_archived,
                            order_by,
                        };
                        let op = ProjectsListQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.projects)
                    }))
                    .await?;
                print_output(&result, format)
            }
            ProjectsAction::Get { id } => {
                let op = ProjectByIdQuery::build(ProjectByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.project, format)
            }
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
                let input = ProjectCreateInput {
                    name,
                    description,
                    team_ids: teams,
                    lead_id: lead,
                    start_date: start_date.map(TimelessDate::from),
                    target_date: target_date.map(TimelessDate::from),
                    color,
                    icon,
                    priority,
                    status_id: status,
                };
                let op = ProjectCreateMutation::build(ProjectCreateVariables { input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.project_create.success,
                    data: data.project_create.project,
                };
                print_output(&resp, format)
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
                let input = ProjectUpdateInput {
                    name,
                    description,
                    lead_id: lead,
                    start_date: start_date.map(TimelessDate::from),
                    target_date: target_date.map(TimelessDate::from),
                    color,
                    icon,
                    priority,
                    status_id: status,
                };
                let op = ProjectUpdateMutation::build(ProjectUpdateVariables { id, input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.project_update.success,
                    data: data.project_update.project,
                };
                print_output(&resp, format)
            }
            ProjectsAction::Archive { id } => {
                let op = ProjectArchiveMutation::build(ProjectArchiveVariables { id });
                let data = client.run_query(op).await?;
                let resp = MutationResponse::<()> {
                    success: data.project_archive.success,
                    data: None,
                };
                print_output(&resp, format)
            }
            ProjectsAction::Delete { id } => {
                let op = ProjectDeleteMutation::build(ProjectDeleteVariables { id });
                let data = client.run_query(op).await?;
                let resp = MutationResponse::<()> {
                    success: data.project_delete.success,
                    data: None,
                };
                print_output(&resp, format)
            }
        }
    }
}
