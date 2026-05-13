#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::{IssueSlim, UserSlim};
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "Attachment")]
pub struct Attachment {
    pub id: cynic::Id,
    pub title: String,
    pub subtitle: Option<String>,
    pub url: String,
    pub source_type: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub creator: Option<UserSlim>,
    pub issue: IssueSlim,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "AttachmentConnection")]
pub struct AttachmentConnection {
    pub nodes: Vec<Attachment>,
    pub page_info: PageInfo,
}

impl Paginatable for AttachmentConnection {
    type Node = Attachment;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<Attachment> {
        self.nodes
    }
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "AttachmentPayload")]
pub struct AttachmentPayload {
    pub success: bool,
    pub attachment: Attachment,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "AttachmentCreateInput")]
pub struct AttachmentCreateInput {
    pub issue_id: String,
    pub title: String,
    pub url: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "AttachmentUpdateInput")]
pub struct AttachmentUpdateInput {
    pub title: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "DeletePayload")]
pub struct AttachmentDeletePayload {
    pub success: bool,
    pub entity_id: String,
}
