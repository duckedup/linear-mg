#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::ProjectSlim;
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::Enum, Debug, Clone, Copy)]
#[cynic(schema = "linear", graphql_type = "ProjectMilestoneStatus", rename_all = "camelCase")]
pub enum ProjectMilestoneStatus {
    Done,
    Next,
    Overdue,
    Unstarted,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "ProjectMilestone")]
pub struct ProjectMilestone {
    pub id: cynic::Id,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectMilestoneStatus,
    pub target_date: Option<TimelessDate>,
    pub progress: f64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub project: ProjectSlim,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "ProjectMilestoneConnection")]
pub struct ProjectMilestoneConnection {
    pub nodes: Vec<ProjectMilestone>,
    pub page_info: PageInfo,
}

impl Paginatable for ProjectMilestoneConnection {
    type Node = ProjectMilestone;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<ProjectMilestone> {
        self.nodes
    }
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "ProjectMilestonePayload")]
pub struct ProjectMilestonePayload {
    pub success: bool,
    pub project_milestone: ProjectMilestone,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "ProjectMilestoneCreateInput")]
pub struct ProjectMilestoneCreateInput {
    pub name: String,
    pub project_id: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<TimelessDate>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "ProjectMilestoneUpdateInput")]
pub struct ProjectMilestoneUpdateInput {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<TimelessDate>,
}
