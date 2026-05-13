use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::UserSlim;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Initiative {
    pub id: String,
    pub name: String,
    pub slug_id: String,
    pub description: Option<String>,
    pub status: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub target_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub trashed: Option<bool>,
    pub url: String,
    pub owner: Option<UserSlim>,
    pub creator: Option<UserSlim>,
}

const INITIATIVE_FIELDS: &str = "
    id name slugId description status color icon targetDate
    createdAt updatedAt archivedAt trashed url
    owner { id name displayName email }
    creator { id name displayName email }
";

#[derive(Deserialize)]
pub struct InitiativeQuery {
    pub initiative: Initiative,
}
#[derive(Deserialize)]
pub struct InitiativesQuery {
    pub initiatives: Connection<Initiative>,
}

impl LinearClient {
    pub async fn get_initiative(&self, id: &str) -> Result<Initiative, CliError> {
        let query =
            format!("query($id: String!) {{ initiative(id: $id) {{ {INITIATIVE_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: InitiativeQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.initiative)
    }

    pub async fn list_initiatives(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Initiative>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                initiatives(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {INITIATIVE_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: InitiativesQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.initiatives)
    }
}
