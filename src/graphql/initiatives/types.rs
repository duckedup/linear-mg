#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::UserSlim;
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::Enum, Debug, Clone, Copy)]
#[cynic(
    schema = "linear",
    graphql_type = "InitiativeStatus",
    rename_all = "PascalCase"
)]
pub enum InitiativeStatus {
    Active,
    Completed,
    Planned,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Initiative")]
pub struct Initiative {
    pub id: cynic::Id,
    pub name: String,
    pub slug_id: String,
    pub description: Option<String>,
    pub status: InitiativeStatus,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub target_date: Option<TimelessDate>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub trashed: Option<bool>,
    pub url: String,
    pub owner: Option<UserSlim>,
    pub creator: Option<UserSlim>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "InitiativeConnection")]
pub struct InitiativeConnection {
    pub nodes: Vec<Initiative>,
    pub page_info: PageInfo,
}

impl Paginatable for InitiativeConnection {
    type Node = Initiative;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Initiative> {
        self.nodes
    }
}
