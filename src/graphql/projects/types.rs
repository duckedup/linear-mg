#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::UserSlim;
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Project")]
pub struct Project {
    pub id: cynic::Id,
    pub name: String,
    pub slug_id: String,
    pub description: String,
    pub icon: Option<String>,
    pub color: String,
    pub priority: i32,
    pub priority_label: String,
    pub progress: f64,
    pub scope: f64,
    pub start_date: Option<TimelessDate>,
    pub target_date: Option<TimelessDate>,
    pub started_at: Option<DateTime>,
    pub completed_at: Option<DateTime>,
    pub canceled_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub trashed: Option<bool>,
    pub url: String,
    pub content: Option<String>,
    pub lead: Option<UserSlim>,
    pub creator: Option<UserSlim>,
    pub status: ProjectStatusSlim,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "ProjectStatus")]
pub struct ProjectStatusSlim {
    pub id: cynic::Id,
    pub name: String,
    pub color: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "ProjectConnection")]
pub struct ProjectConnection {
    pub nodes: Vec<Project>,
    pub page_info: PageInfo,
}

impl Paginatable for ProjectConnection {
    type Node = Project;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Project> {
        self.nodes
    }
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "ProjectPayload")]
pub struct ProjectPayload {
    pub success: bool,
    pub project: Option<Project>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "ProjectArchivePayload")]
pub struct ProjectArchivePayload {
    pub success: bool,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "ProjectCreateInput")]
pub struct ProjectCreateInput {
    pub name: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub team_ids: Vec<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lead_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<TimelessDate>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<TimelessDate>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "ProjectUpdateInput")]
pub struct ProjectUpdateInput {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub lead_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<TimelessDate>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub target_date: Option<TimelessDate>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
}
