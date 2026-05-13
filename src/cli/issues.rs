use crate::cli::common::PaginationArgs;
use crate::client::paginator::paginate;
use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::issues::mutations::*;
use crate::graphql::issues::queries::*;
use crate::graphql::issues::types::*;
use crate::graphql::scalars::TimelessDate;
use crate::output::{print_output, OutputFormat};
use clap::Subcommand;
use cynic::{MutationBuilder, QueryBuilder};

#[derive(clap::Args, Debug)]
pub struct IssuesCommand {
    #[command(subcommand)]
    pub action: IssuesAction,
}

#[derive(Subcommand, Debug)]
pub enum IssuesAction {
    /// List issues with optional filters
    List {
        #[command(flatten)]
        pagination: PaginationArgs,
        /// Filter by team key (e.g., "ENG")
        #[arg(long)]
        team: Option<String>,
        /// Filter by assignee user ID (or "me")
        #[arg(long)]
        assignee: Option<String>,
        /// Filter by state name (e.g., "In Progress")
        #[arg(long)]
        state: Option<String>,
        /// Filter by label name
        #[arg(long)]
        label: Option<String>,
        /// Filter by project ID
        #[arg(long)]
        project: Option<String>,
        /// Filter by cycle ID
        #[arg(long)]
        cycle: Option<String>,
        /// Filter by priority (0=none, 1=urgent, 2=high, 3=medium, 4=low)
        #[arg(long)]
        priority: Option<f64>,
    },
    /// Get a single issue by ID or identifier (e.g., "ENG-123")
    Get {
        /// Issue ID (UUID) or identifier
        id: String,
    },
    /// Create a new issue
    Create {
        /// Title of the issue
        #[arg(long)]
        title: String,
        /// Team ID or key
        #[arg(long)]
        team: String,
        /// Issue description (markdown)
        #[arg(long)]
        description: Option<String>,
        /// Assignee user ID
        #[arg(long)]
        assignee: Option<String>,
        /// Priority (0-4)
        #[arg(long)]
        priority: Option<i32>,
        /// State ID
        #[arg(long)]
        state: Option<String>,
        /// Project ID
        #[arg(long)]
        project: Option<String>,
        /// Cycle ID
        #[arg(long)]
        cycle: Option<String>,
        /// Label IDs (comma-separated)
        #[arg(long, value_delimiter = ',')]
        labels: Vec<String>,
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_date: Option<String>,
        /// Estimate points
        #[arg(long)]
        estimate: Option<i32>,
        /// Parent issue ID or identifier
        #[arg(long)]
        parent: Option<String>,
    },
    /// Update an existing issue
    Update {
        /// Issue ID or identifier
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        assignee: Option<String>,
        #[arg(long)]
        priority: Option<i32>,
        #[arg(long)]
        state: Option<String>,
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        cycle: Option<String>,
        /// Label IDs to add (comma-separated)
        #[arg(long, value_delimiter = ',')]
        add_labels: Vec<String>,
        /// Label IDs to remove (comma-separated)
        #[arg(long, value_delimiter = ',')]
        remove_labels: Vec<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long)]
        estimate: Option<i32>,
        #[arg(long)]
        parent: Option<String>,
        #[arg(long)]
        team: Option<String>,
    },
    /// Archive an issue
    Archive {
        /// Issue ID or identifier
        id: String,
    },
    /// Delete (trash) an issue
    Delete {
        /// Issue ID or identifier
        id: String,
    },
    /// Search issues by text
    Search {
        /// Search query
        query: String,
        #[command(flatten)]
        pagination: PaginationArgs,
    },
}

impl IssuesCommand {
    pub async fn run(self, client: &LinearClient, format: &OutputFormat) -> Result<(), CliError> {
        match self.action {
            IssuesAction::List {
                pagination,
                team,
                assignee,
                state,
                label,
                project,
                cycle,
                priority,
            } => {
                let filter = build_issue_filter(team, assignee, state, label, project, cycle, priority);
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());

                let result: ListResponse<Issue> = paginate(client, &params, |c, page_size, cursor| {
                    let filter = filter.clone();
                    Box::pin(async move {
                        let vars = IssuesListVariables {
                            first: Some(page_size),
                            after: cursor,
                            filter,
                            include_archived,
                            order_by,
                        };
                        let op = IssuesListQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.issues)
                    })
                })
                .await?;
                print_output(&result, format)
            }
            IssuesAction::Get { id } => {
                let op = IssueByIdQuery::build(IssueByIdVariables { id });
                let data = client.run_query(op).await?;
                print_output(&data.issue, format)
            }
            IssuesAction::Create {
                title,
                team,
                description,
                assignee,
                priority,
                state,
                project,
                cycle,
                labels,
                due_date,
                estimate,
                parent,
            } => {
                let input = IssueCreateInput {
                    team_id: team,
                    title: Some(title),
                    description,
                    assignee_id: assignee,
                    priority,
                    state_id: state,
                    project_id: project,
                    cycle_id: cycle,
                    label_ids: if labels.is_empty() { None } else { Some(labels) },
                    due_date: due_date.map(TimelessDate::from),
                    estimate,
                    parent_id: parent,
                };
                let op = IssueCreateMutation::build(IssueCreateVariables { input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.issue_create.success,
                    data: data.issue_create.issue,
                };
                print_output(&resp, format)
            }
            IssuesAction::Update {
                id,
                title,
                description,
                assignee,
                priority,
                state,
                project,
                cycle,
                add_labels,
                remove_labels,
                due_date,
                estimate,
                parent,
                team,
            } => {
                let input = IssueUpdateInput {
                    title,
                    description,
                    assignee_id: assignee,
                    priority,
                    state_id: state,
                    project_id: project,
                    cycle_id: cycle,
                    label_ids: None,
                    added_label_ids: if add_labels.is_empty() { None } else { Some(add_labels) },
                    removed_label_ids: if remove_labels.is_empty() { None } else { Some(remove_labels) },
                    due_date: due_date.map(TimelessDate::from),
                    estimate,
                    parent_id: parent,
                    team_id: team,
                };
                let op = IssueUpdateMutation::build(IssueUpdateVariables { id, input });
                let data = client.run_query(op).await?;
                let resp = MutationResponse {
                    success: data.issue_update.success,
                    data: data.issue_update.issue,
                };
                print_output(&resp, format)
            }
            IssuesAction::Archive { id } => {
                let op = IssueArchiveMutation::build(IssueArchiveVariables { id });
                let data = client.run_query(op).await?;
                let resp = MutationResponse::<()> {
                    success: data.issue_archive.success,
                    data: None,
                };
                print_output(&resp, format)
            }
            IssuesAction::Delete { id } => {
                let op = IssueDeleteMutation::build(IssueDeleteVariables { id });
                let data = client.run_query(op).await?;
                let resp = MutationResponse::<()> {
                    success: data.issue_delete.success,
                    data: None,
                };
                print_output(&resp, format)
            }
            IssuesAction::Search { query, pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = Some(pagination.include_archived);
                let order_by = Some(pagination.order_by.into());
                let q = query;

                let result: ListResponse<Issue> = paginate(client, &params, |c, page_size, cursor| {
                    let q = q.clone();
                    Box::pin(async move {
                        let vars = IssueSearchVariables {
                            query: Some(q),
                            first: Some(page_size),
                            after: cursor,
                            include_archived,
                            order_by,
                        };
                        let op = IssueSearchQuery::build(vars);
                        let data = c.run_query(op).await?;
                        Ok(data.issue_search)
                    })
                })
                .await?;
                print_output(&result, format)
            }
        }
    }
}

fn build_issue_filter(
    team: Option<String>,
    assignee: Option<String>,
    state: Option<String>,
    label: Option<String>,
    project: Option<String>,
    cycle: Option<String>,
    priority: Option<f64>,
) -> Option<IssueFilter> {
    let has_any = team.is_some()
        || assignee.is_some()
        || state.is_some()
        || label.is_some()
        || project.is_some()
        || cycle.is_some()
        || priority.is_some();

    if !has_any {
        return None;
    }

    Some(IssueFilter {
        team: team.map(|t| TeamFilter {
            key: Some(StringComparator {
                eq_ignore_case: Some(t),
                ..Default::default()
            }),
            ..Default::default()
        }),
        assignee: assignee.map(|a| NullableUserFilter {
            id: Some(IdComparator {
                eq: Some(a.into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        state: state.map(|s| WorkflowStateFilter {
            name: Some(StringComparator {
                eq_ignore_case: Some(s),
                ..Default::default()
            }),
            ..Default::default()
        }),
        labels: label.map(|l| IssueLabelCollectionFilter {
            name: Some(StringComparator {
                eq_ignore_case: Some(l),
                ..Default::default()
            }),
            ..Default::default()
        }),
        project: project.map(|p| NullableProjectFilter {
            id: Some(IdComparator {
                eq: Some(p.into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        cycle: cycle.map(|c| NullableCycleFilter {
            id: Some(IdComparator {
                eq: Some(c.into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        priority: priority.map(|p| NullableNumberComparator {
            eq: Some(p),
            ..Default::default()
        }),
    })
}
