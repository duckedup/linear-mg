#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::{ProjectSlim, TeamSlim, UserSlim};
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Document")]
pub struct Document {
    pub id: cynic::Id,
    pub title: String,
    pub slug_id: String,
    pub content: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub trashed: Option<bool>,
    pub url: String,
    pub creator: Option<UserSlim>,
    pub updated_by: Option<UserSlim>,
    pub project: Option<ProjectSlim>,
    pub team: Option<TeamSlim>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "DocumentConnection")]
pub struct DocumentConnection {
    pub nodes: Vec<Document>,
    pub page_info: PageInfo,
}

impl Paginatable for DocumentConnection {
    type Node = Document;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Document> {
        self.nodes
    }
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "DocumentPayload")]
pub struct DocumentPayload {
    pub success: bool,
    pub document: Document,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "DocumentCreateInput")]
pub struct DocumentCreateInput {
    pub title: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "DocumentUpdateInput")]
pub struct DocumentUpdateInput {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
