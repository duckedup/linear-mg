#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::TeamSlim;
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "WorkflowState")]
pub struct WorkflowState {
    pub id: cynic::Id,
    pub name: String,
    #[cynic(rename = "type")]
    pub state_type: String,
    pub color: String,
    pub description: Option<String>,
    pub position: f64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub team: TeamSlim,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "WorkflowStateConnection")]
pub struct WorkflowStateConnection {
    pub nodes: Vec<WorkflowState>,
    pub page_info: PageInfo,
}

impl Paginatable for WorkflowStateConnection {
    type Node = WorkflowState;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<WorkflowState> {
        self.nodes
    }
}
