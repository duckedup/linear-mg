#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::scalars::*;
use serde::Serialize;

// -- Issue fragments --

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Issue")]
pub struct Issue {
    pub id: cynic::Id,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: f64,
    pub priority_label: String,
    pub estimate: Option<f64>,
    pub due_date: Option<TimelessDate>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub completed_at: Option<DateTime>,
    pub canceled_at: Option<DateTime>,
    pub archived_at: Option<DateTime>,
    pub started_at: Option<DateTime>,
    pub branch_name: String,
    pub number: f64,
    pub url: String,
    pub trashed: Option<bool>,
    pub state: WorkflowStateSlim,
    pub assignee: Option<UserSlim>,
    pub creator: Option<UserSlim>,
    pub team: TeamSlim,
    pub project: Option<ProjectSlim>,
    pub cycle: Option<CycleSlim>,
    pub parent: Option<IssueSlim>,
    pub labels: IssueLabelConnection,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Issue")]
pub struct IssueSlim {
    pub id: cynic::Id,
    pub identifier: String,
    pub title: String,
}

// -- Related entity slim fragments --

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "WorkflowState")]
pub struct WorkflowStateSlim {
    pub id: cynic::Id,
    pub name: String,
    #[cynic(rename = "type")]
    pub state_type: String,
    pub color: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "User")]
pub struct UserSlim {
    pub id: cynic::Id,
    pub name: String,
    pub display_name: String,
    pub email: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Team")]
pub struct TeamSlim {
    pub id: cynic::Id,
    pub name: String,
    pub key: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Project")]
pub struct ProjectSlim {
    pub id: cynic::Id,
    pub name: String,
    pub slug_id: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Cycle")]
pub struct CycleSlim {
    pub id: cynic::Id,
    pub number: f64,
    pub name: Option<String>,
}

// -- Label connection --

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "IssueLabelConnection")]
pub struct IssueLabelConnection {
    pub nodes: Vec<LabelSlim>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "IssueLabel")]
pub struct LabelSlim {
    pub id: cynic::Id,
    pub name: String,
    pub color: String,
}

// -- Issue connection --

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "IssueConnection")]
pub struct IssueConnection {
    pub nodes: Vec<Issue>,
    pub page_info: PageInfo,
}

impl Paginatable for IssueConnection {
    type Node = Issue;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Issue> {
        self.nodes
    }
}

// -- Payloads --

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "IssuePayload")]
pub struct IssuePayload {
    pub success: bool,
    pub issue: Option<Issue>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "IssueArchivePayload")]
pub struct IssueArchivePayload {
    pub success: bool,
}

// -- Input types --

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "IssueCreateInput")]
pub struct IssueCreateInput {
    pub team_id: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub state_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub cycle_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<String>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<TimelessDate>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<i32>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "IssueUpdateInput")]
pub struct IssueUpdateInput {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub state_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub cycle_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<String>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub added_label_ids: Option<Vec<String>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub removed_label_ids: Option<Vec<String>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<TimelessDate>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub estimate: Option<i32>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
}

// -- Filters --

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "IssueFilter")]
pub struct IssueFilter {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub team: Option<TeamFilter>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<NullableUserFilter>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowStateFilter>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub priority: Option<NullableNumberComparator>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub project: Option<NullableProjectFilter>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<NullableCycleFilter>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub labels: Option<IssueLabelCollectionFilter>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "TeamFilter")]
pub struct TeamFilter {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub key: Option<StringComparator>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdComparator>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "NullableUserFilter")]
pub struct NullableUserFilter {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdComparator>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "WorkflowStateFilter")]
pub struct WorkflowStateFilter {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub name: Option<StringComparator>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdComparator>,
    #[cynic(rename = "type")]
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub state_type: Option<StringComparator>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "NullableProjectFilter")]
pub struct NullableProjectFilter {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdComparator>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "NullableCycleFilter")]
pub struct NullableCycleFilter {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdComparator>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "IssueLabelCollectionFilter")]
pub struct IssueLabelCollectionFilter {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub name: Option<StringComparator>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub id: Option<IdComparator>,
}

// -- Comparators --

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "StringComparator")]
pub struct StringComparator {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub neq: Option<String>,
    #[cynic(rename = "in")]
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub in_: Option<Vec<String>>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eq_ignore_case: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<String>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "IDComparator")]
pub struct IdComparator {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eq: Option<cynic::Id>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub neq: Option<cynic::Id>,
    #[cynic(rename = "in")]
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub in_: Option<Vec<cynic::Id>>,
}

#[derive(cynic::InputObject, Debug, Clone, Default)]
#[cynic(schema = "linear", graphql_type = "NullableNumberComparator")]
pub struct NullableNumberComparator {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub eq: Option<f64>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub neq: Option<f64>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lt: Option<f64>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lte: Option<f64>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gt: Option<f64>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub gte: Option<f64>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
    #[cynic(rename = "in")]
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub in_: Option<Vec<f64>>,
}
