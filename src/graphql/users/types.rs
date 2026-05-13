#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "User")]
pub struct User {
    pub id: cynic::Id,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub active: bool,
    pub admin: bool,
    pub guest: bool,
    pub is_me: bool,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub last_seen: Option<DateTime>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "UserConnection")]
pub struct UserConnection {
    pub nodes: Vec<User>,
    pub page_info: PageInfo,
}

impl Paginatable for UserConnection {
    type Node = User;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<User> {
        self.nodes
    }
}
