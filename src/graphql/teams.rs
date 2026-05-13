use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub name: String,
    pub key: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub cycles_enabled: bool,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
}

const TEAM_FIELDS: &str =
    "id name key description color icon cyclesEnabled createdAt updatedAt archivedAt";

#[derive(Deserialize)]
pub struct TeamQuery {
    pub team: Team,
}

#[derive(Deserialize)]
pub struct TeamsQuery {
    pub teams: Connection<Team>,
}

impl LinearClient {
    pub async fn get_team(&self, id: &str) -> Result<Team, CliError> {
        let query = format!("query($id: String!) {{ team(id: $id) {{ {TEAM_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: TeamQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.team)
    }

    pub async fn list_teams(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Team>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                teams(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {TEAM_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({
            "first": first,
            "after": after,
            "includeArchived": include_archived,
            "orderBy": order_by,
        });
        let resp: TeamsQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.teams)
    }
}
