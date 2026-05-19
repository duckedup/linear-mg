use crate::cli::common::PaginationArgs;
use crate::cli::resolve;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::issues::Issue;
use crate::output::{OutputFormat, print_output};
use clap::Subcommand;

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
    Get { id: String },
    /// Create a new issue
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        team: String,
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
        #[arg(long, value_delimiter = ',')]
        labels: Vec<String>,
        #[arg(long)]
        due_date: Option<String>,
        #[arg(long)]
        estimate: Option<i32>,
        #[arg(long)]
        parent: Option<String>,
    },
    /// Update an existing issue
    Update {
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
        #[arg(long, value_delimiter = ',')]
        add_labels: Vec<String>,
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
    Archive { id: String },
    /// Delete (trash) an issue
    Delete { id: String },
    /// Search issues by text
    Search {
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
                let filter = build_filter(team, assignee, state, label, project, cycle, priority);
                let params = pagination.to_paginator_params();
                let include_archived = pagination.include_archived;
                let order_by = pagination.order_by.as_str().to_string();

                let result: ListResponse<Issue> =
                    paginate(client, &params, |c, page_size, cursor| {
                        let filter = filter.clone();
                        let order_by = order_by.clone();
                        Box::pin(async move {
                            c.list_issues(page_size, cursor, filter, include_archived, &order_by)
                                .await
                        })
                    })
                    .await?;
                print_output(&result, format)
            }
            IssuesAction::Get { id } => {
                let issue = client.get_issue(&id).await?;
                print_output(&issue, format)
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
                let team_id = resolve::resolve_team(client, &team).await?;
                let mut input = serde_json::json!({ "teamId": &team_id, "title": title });
                let obj = input.as_object_mut().unwrap();
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                if let Some(v) = assignee {
                    let id = resolve::resolve_assignee(client, &v).await?;
                    obj.insert("assigneeId".into(), id.into());
                }
                if let Some(v) = priority {
                    obj.insert("priority".into(), v.into());
                }
                if let Some(v) = state {
                    let id = resolve::resolve_state(client, &v, Some(&team_id)).await?;
                    obj.insert("stateId".into(), id.into());
                }
                if let Some(v) = project {
                    obj.insert("projectId".into(), v.into());
                }
                if let Some(v) = cycle {
                    obj.insert("cycleId".into(), v.into());
                }
                if !labels.is_empty() {
                    obj.insert("labelIds".into(), labels.into());
                }
                if let Some(v) = due_date {
                    obj.insert("dueDate".into(), v.into());
                }
                if let Some(v) = estimate {
                    obj.insert("estimate".into(), v.into());
                }
                if let Some(v) = parent {
                    obj.insert("parentId".into(), v.into());
                }

                let payload = client.create_issue(input).await?;
                let resp = MutationResponse {
                    success: payload.success,
                    data: payload.issue,
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
                let mut input = serde_json::json!({});
                let obj = input.as_object_mut().unwrap();

                let resolved_team_id = if let Some(ref v) = team {
                    let tid = resolve::resolve_team(client, v).await?;
                    obj.insert("teamId".into(), tid.clone().into());
                    Some(tid)
                } else {
                    None
                };

                if let Some(v) = title {
                    obj.insert("title".into(), v.into());
                }
                if let Some(v) = description {
                    obj.insert("description".into(), v.into());
                }
                if let Some(v) = assignee {
                    let uid = resolve::resolve_assignee(client, &v).await?;
                    obj.insert("assigneeId".into(), uid.into());
                }
                if let Some(v) = priority {
                    obj.insert("priority".into(), v.into());
                }
                if let Some(v) = state {
                    let team_ctx = match &resolved_team_id {
                        Some(tid) => Some(tid.clone()),
                        None => match resolve::extract_team_key(&id) {
                            Some(key) => resolve::resolve_team(client, key).await.ok(),
                            None => None,
                        },
                    };
                    let sid = resolve::resolve_state(client, &v, team_ctx.as_deref()).await?;
                    obj.insert("stateId".into(), sid.into());
                }
                if let Some(v) = project {
                    obj.insert("projectId".into(), v.into());
                }
                if let Some(v) = cycle {
                    obj.insert("cycleId".into(), v.into());
                }
                if !add_labels.is_empty() {
                    obj.insert("addedLabelIds".into(), add_labels.into());
                }
                if !remove_labels.is_empty() {
                    obj.insert("removedLabelIds".into(), remove_labels.into());
                }
                if let Some(v) = due_date {
                    obj.insert("dueDate".into(), v.into());
                }
                if let Some(v) = estimate {
                    obj.insert("estimate".into(), v.into());
                }
                if let Some(v) = parent {
                    obj.insert("parentId".into(), v.into());
                }

                let payload = client.update_issue(&id, input).await?;
                let resp = MutationResponse {
                    success: payload.success,
                    data: payload.issue,
                };
                print_output(&resp, format)
            }
            IssuesAction::Archive { id } => {
                let payload = client.archive_issue(&id).await?;
                let resp = MutationResponse::<()> {
                    success: payload.success,
                    data: None,
                };
                print_output(&resp, format)
            }
            IssuesAction::Delete { id } => {
                let payload = client.delete_issue(&id).await?;
                let resp = MutationResponse::<()> {
                    success: payload.success,
                    data: None,
                };
                print_output(&resp, format)
            }
            IssuesAction::Search { query, pagination } => {
                let params = pagination.to_paginator_params();
                let include_archived = pagination.include_archived;
                let order_by = pagination.order_by.as_str().to_string();
                let term = query;

                let result: ListResponse<Issue> =
                    paginate(client, &params, |c, page_size, cursor| {
                        let term = term.clone();
                        let order_by = order_by.clone();
                        Box::pin(async move {
                            c.search_issues(&term, page_size, cursor, include_archived, &order_by)
                                .await
                        })
                    })
                    .await?;
                print_output(&result, format)
            }
        }
    }
}

fn build_filter(
    team: Option<String>,
    assignee: Option<String>,
    state: Option<String>,
    label: Option<String>,
    project: Option<String>,
    cycle: Option<String>,
    priority: Option<f64>,
) -> Option<serde_json::Value> {
    let mut filter = serde_json::Map::new();
    if let Some(t) = team {
        filter.insert(
            "team".into(),
            serde_json::json!({ "key": { "eqIgnoreCase": t } }),
        );
    }
    if let Some(a) = assignee {
        filter.insert("assignee".into(), serde_json::json!({ "id": { "eq": a } }));
    }
    if let Some(s) = state {
        filter.insert(
            "state".into(),
            serde_json::json!({ "name": { "eqIgnoreCase": s } }),
        );
    }
    if let Some(l) = label {
        filter.insert(
            "labels".into(),
            serde_json::json!({ "name": { "eqIgnoreCase": l } }),
        );
    }
    if let Some(p) = project {
        filter.insert("project".into(), serde_json::json!({ "id": { "eq": p } }));
    }
    if let Some(c) = cycle {
        filter.insert("cycle".into(), serde_json::json!({ "id": { "eq": c } }));
    }
    if let Some(p) = priority {
        filter.insert("priority".into(), serde_json::json!({ "eq": p }));
    }

    if filter.is_empty() {
        None
    } else {
        Some(serde_json::Value::Object(filter))
    }
}
