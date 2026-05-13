use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::TeamSlim;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLabel {
    pub id: String,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub is_group: bool,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub team: Option<TeamSlim>,
    pub parent: Option<IssueLabelSlim>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLabelSlim {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueLabelPayload {
    pub success: bool,
    pub issue_label: IssueLabel,
}

const LABEL_FIELDS: &str = "
    id name color description isGroup createdAt updatedAt archivedAt
    team { id name key }
    parent { id name color }
";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelsQuery {
    pub issue_labels: Connection<IssueLabel>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelCreateResponse {
    pub issue_label_create: IssueLabelPayload,
}

impl LinearClient {
    pub async fn list_labels(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<IssueLabel>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                issueLabels(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {LABEL_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: LabelsQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.issue_labels)
    }

    pub async fn create_label(
        &self,
        input: serde_json::Value,
    ) -> Result<IssueLabelPayload, CliError> {
        let query = format!(
            "mutation($input: IssueLabelCreateInput!) {{ issueLabelCreate(input: $input) {{ success issueLabel {{ {LABEL_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: LabelCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.issue_label_create)
    }
}
