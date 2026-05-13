#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::TeamSlim;
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Cycle")]
pub struct Cycle {
    pub id: cynic::Id,
    pub number: f64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub starts_at: DateTime,
    pub ends_at: DateTime,
    pub completed_at: Option<DateTime>,
    pub progress: f64,
    pub is_active: bool,
    pub is_next: bool,
    pub is_previous: bool,
    pub is_past: bool,
    pub is_future: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub team: TeamSlim,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "CycleConnection")]
pub struct CycleConnection {
    pub nodes: Vec<Cycle>,
    pub page_info: PageInfo,
}

impl Paginatable for CycleConnection {
    type Node = Cycle;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Cycle> {
        self.nodes
    }
}
