#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use crate::graphql::common::{PageInfo, Paginatable};
use crate::graphql::issues::types::TeamSlim;
use crate::graphql::scalars::*;
use serde::Serialize;

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "IssueLabel")]
pub struct IssueLabel {
    pub id: cynic::Id,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub is_group: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub archived_at: Option<DateTime>,
    pub team: Option<TeamSlim>,
    pub parent: Option<IssueLabelSlim>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "IssueLabel")]
pub struct IssueLabelSlim {
    pub id: cynic::Id,
    pub name: String,
    pub color: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "IssueLabelConnection")]
pub struct IssueLabelConnection {
    pub nodes: Vec<IssueLabel>,
    pub page_info: PageInfo,
}

impl Paginatable for IssueLabelConnection {
    type Node = IssueLabel;
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }
    fn into_nodes(self) -> Vec<IssueLabel> {
        self.nodes
    }
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema = "linear", graphql_type = "IssueLabelPayload")]
pub struct IssueLabelPayload {
    pub success: bool,
    pub issue_label: IssueLabel,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(schema = "linear", graphql_type = "IssueLabelCreateInput")]
pub struct IssueLabelCreateInput {
    pub name: String,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[cynic(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}
