#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Team")]
pub struct Team {
    pub id: cynic::Id,
    pub name: String,
    pub key: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub cycles_enabled: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "TeamConnection")]
pub struct TeamConnection {
    pub nodes: Vec<Team>,
    pub page_info: PageInfo,
}

impl Paginatable for TeamConnection {
    type Node = Team;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Team> {
        self.nodes
    }
}
