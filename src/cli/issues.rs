use crate::cli::common::PaginationArgs;
use crate::cli::resolve;
use crate::client::LinearClient;
use crate::client::paginator::paginate;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::issues::Issue;
use crate::graphql::relations::{IssueRelationList, IssueRelationView};
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
        /// Filter by assignee (user ID, name, email, or "me")
        #[arg(long)]
        assignee: Option<String>,
        /// Filter by state name (e.g., "In Progress")
        #[arg(long)]
        state: Option<String>,
        /// Filter by label name
        #[arg(long)]
        label: Option<String>,
        /// Filter by project (ID, name, or slug)
        #[arg(long)]
        project: Option<String>,
        /// Filter by milestone (ID or name; names are scoped to --project when given)
        #[arg(long)]
        milestone: Option<String>,
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
        /// Project (ID, name, or slug)
        #[arg(long)]
        project: Option<String>,
        /// Milestone to link (ID or name; names are scoped to --project when given)
        #[arg(long)]
        milestone: Option<String>,
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
        /// Project (ID, name, or slug)
        #[arg(long)]
        project: Option<String>,
        /// Milestone to link (ID or name; names are scoped to --project or the
        /// issue's current project)
        #[arg(long, conflicts_with = "no_milestone")]
        milestone: Option<String>,
        /// Unlink the issue from its milestone
        #[arg(long)]
        no_milestone: bool,
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
    /// Create a relation from an issue to another issue.
    ///
    /// Exactly one direction flag must be given. IDs may be UUIDs or
    /// identifiers (e.g., "ENG-123").
    Relate {
        /// The issue to relate from (ID or identifier)
        issue: String,
        /// This issue blocks the given issue
        #[arg(long)]
        blocks: Option<String>,
        /// This issue is blocked by the given issue
        #[arg(long)]
        blocked_by: Option<String>,
        /// This issue is related to the given issue
        #[arg(long)]
        related_to: Option<String>,
        /// This issue is a duplicate of the given issue
        #[arg(long)]
        duplicate_of: Option<String>,
        /// This issue is similar to the given issue
        #[arg(long)]
        similar_to: Option<String>,
    },
    /// List relations for an issue (both blocking and blocked-by directions)
    Relations {
        /// The issue to list relations for (ID or identifier)
        issue: String,
    },
    /// Delete an issue relation by its relation ID
    Unrelate {
        /// The relation ID (from `issues relations`)
        id: String,
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
                milestone,
                cycle,
                priority,
            } => {
                // Resolve human-friendly values (assignee: "me"/name/email;
                // project: name/slug; milestone: name) to IDs for the filter.
                let assignee = match assignee {
                    Some(a) => Some(resolve::resolve_assignee(client, &a).await?),
                    None => None,
                };
                let project = match project {
                    Some(p) => Some(resolve::resolve_project(client, &p).await?),
                    None => None,
                };
                let milestone = match milestone {
                    Some(m) => Some(
                        resolve::resolve_milestone(client, &m, project.as_deref())
                            .await?
                            .id,
                    ),
                    None => None,
                };
                let filter = build_filter(
                    team, assignee, state, label, project, milestone, cycle, priority,
                );
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
                milestone,
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
                resolve::apply_project_and_milestone(client, obj, project, milestone, None).await?;
                if let Some(v) = cycle {
                    obj.insert("cycleId".into(), v.into());
                }
                if !labels.is_empty() {
                    let mut label_ids = Vec::with_capacity(labels.len());
                    for l in labels {
                        label_ids.push(resolve::resolve_label(client, &l, Some(&team_id)).await?);
                    }
                    obj.insert("labelIds".into(), label_ids.into());
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
                milestone,
                no_milestone,
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
                // Resolve a team context once, shared by name-based lookups for
                // state and labels. Falls back to the team key in the issue
                // identifier (e.g. "ENG" from "ENG-123") when --team is absent.
                let team_ctx =
                    if state.is_some() || !add_labels.is_empty() || !remove_labels.is_empty() {
                        match &resolved_team_id {
                            Some(tid) => Some(tid.clone()),
                            None => match resolve::extract_team_key(&id) {
                                Some(key) => resolve::resolve_team(client, key).await.ok(),
                                None => None,
                            },
                        }
                    } else {
                        None
                    };

                if let Some(v) = state {
                    let sid = resolve::resolve_state(client, &v, team_ctx.as_deref()).await?;
                    obj.insert("stateId".into(), sid.into());
                }
                // A milestone name without --project is scoped to the project
                // the issue currently belongs to.
                let current_project = if milestone.is_some() && project.is_none() {
                    client.get_issue(&id).await?.project.map(|p| p.id)
                } else {
                    None
                };
                resolve::apply_project_and_milestone(
                    client,
                    obj,
                    project,
                    milestone,
                    current_project,
                )
                .await?;
                if no_milestone {
                    obj.insert("projectMilestoneId".into(), serde_json::Value::Null);
                }
                if let Some(v) = cycle {
                    obj.insert("cycleId".into(), v.into());
                }
                if !add_labels.is_empty() {
                    let mut ids = Vec::with_capacity(add_labels.len());
                    for l in add_labels {
                        ids.push(resolve::resolve_label(client, &l, team_ctx.as_deref()).await?);
                    }
                    obj.insert("addedLabelIds".into(), ids.into());
                }
                if !remove_labels.is_empty() {
                    let mut ids = Vec::with_capacity(remove_labels.len());
                    for l in remove_labels {
                        ids.push(resolve::resolve_label(client, &l, team_ctx.as_deref()).await?);
                    }
                    obj.insert("removedLabelIds".into(), ids.into());
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
            IssuesAction::Relate {
                issue,
                blocks,
                blocked_by,
                related_to,
                duplicate_of,
                similar_to,
            } => {
                // Each direction maps to (relation type, whether `issue` is the
                // source of the relation). "blocked_by" is the inverse of "blocks".
                let choices = [
                    (blocks, "blocks", true),
                    (blocked_by, "blocks", false),
                    (related_to, "related", true),
                    (duplicate_of, "duplicate", true),
                    (similar_to, "similar", true),
                ];
                let mut selected = choices
                    .into_iter()
                    .filter_map(|(target, ty, is_source)| target.map(|t| (t, ty, is_source)));
                let (target, relation_type, issue_is_source) =
                    selected.next().ok_or_else(|| {
                        CliError::InvalidInput(
                            "exactly one of --blocks, --blocked-by, --related-to, \
                             --duplicate-of, --similar-to is required"
                                .into(),
                        )
                    })?;
                if selected.next().is_some() {
                    return Err(CliError::InvalidInput(
                        "only one relation direction may be specified".into(),
                    ));
                }

                let (issue_id, related_id) = if issue_is_source {
                    (issue, target)
                } else {
                    (target, issue)
                };
                let input = serde_json::json!({
                    "issueId": issue_id,
                    "relatedIssueId": related_id,
                    "type": relation_type,
                });
                let payload = client.create_issue_relation(input).await?;
                let resp = MutationResponse {
                    success: payload.success,
                    data: payload.issue_relation,
                };
                print_output(&resp, format)
            }
            IssuesAction::Relations { issue } => {
                let (relations, inverse) = client.list_issue_relations(&issue).await?;
                let nodes: Vec<IssueRelationView> = relations
                    .into_iter()
                    .map(IssueRelationView::from_source)
                    .chain(inverse.into_iter().map(IssueRelationView::from_target))
                    .collect();
                print_output(&IssueRelationList { nodes }, format)
            }
            IssuesAction::Unrelate { id } => {
                let payload = client.delete_issue_relation(&id).await?;
                let resp = MutationResponse::<()> {
                    success: payload.success,
                    data: None,
                };
                print_output(&resp, format)
            }
        }
    }
}

// One positional argument per `issues list` filter flag; a struct would just be
// destructured straight back into these locals.
#[allow(clippy::too_many_arguments)]
fn build_filter(
    team: Option<String>,
    assignee: Option<String>,
    state: Option<String>,
    label: Option<String>,
    project: Option<String>,
    milestone: Option<String>,
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
    if let Some(m) = milestone {
        filter.insert(
            "projectMilestone".into(),
            serde_json::json!({ "id": { "eq": m } }),
        );
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
