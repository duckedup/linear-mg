#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::UserSlim;
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Comment")]
pub struct Comment {
    pub id: cynic::Id,
    pub body: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub archived_at: Option<DateTime>,
    pub resolved_at: Option<DateTime>,
    pub url: String,
    pub user: Option<UserSlim>,
    pub issue: Option<CommentIssueSlim>,
    pub parent: Option<CommentSlim>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Issue")]
pub struct CommentIssueSlim {
    pub id: cynic::Id,
    pub identifier: String,
    pub title: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Comment")]
pub struct CommentSlim {
    pub id: cynic::Id,
    pub body: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "CommentConnection")]
pub struct CommentConnection {
    pub nodes: Vec<Comment>,
    pub page_info: PageInfo,
}

impl Paginatable for CommentConnection {
    type Node = Comment;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Comment> {
        self.nodes
    }
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "CommentPayload")]
pub struct CommentPayload {
    pub success: bool,
    pub comment: Option<Comment>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "DeletePayload")]
pub struct DeletePayload {
    pub success: bool,
    pub entity_id: String,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "CommentCreateInput")]
pub struct CommentCreateInput {
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    pub body: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "CommentUpdateInput")]
pub struct CommentUpdateInput {
    pub body: String,
}
