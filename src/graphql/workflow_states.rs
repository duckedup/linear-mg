use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::TeamSlim;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowState {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub color: String,
    pub description: Option<String>,
    pub position: f64,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub team: TeamSlim,
}

const STATE_FIELDS: &str =
    "id name type color description position createdAt updatedAt archivedAt team { id name key }";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStatesQuery {
    pub workflow_states: Connection<WorkflowState>,
}

impl LinearClient {
    pub async fn list_workflow_states(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<WorkflowState>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                workflowStates(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {STATE_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: WorkflowStatesQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.workflow_states)
    }
}
