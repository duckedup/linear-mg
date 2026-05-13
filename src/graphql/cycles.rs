use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::TeamSlim;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cycle {
    pub id: String,
    pub number: f64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub starts_at: String,
    pub ends_at: String,
    pub completed_at: Option<String>,
    pub progress: f64,
    pub is_active: bool,
    pub is_next: bool,
    pub is_previous: bool,
    pub is_past: bool,
    pub is_future: bool,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub team: TeamSlim,
}

const CYCLE_FIELDS: &str = "
    id number name description startsAt endsAt completedAt progress
    isActive isNext isPrevious isPast isFuture createdAt updatedAt archivedAt
    team { id name key }
";

#[derive(Deserialize)]
pub struct CycleQuery {
    pub cycle: Cycle,
}
#[derive(Deserialize)]
pub struct CyclesQuery {
    pub cycles: Connection<Cycle>,
}

impl LinearClient {
    pub async fn get_cycle(&self, id: &str) -> Result<Cycle, CliError> {
        let query = format!("query($id: String!) {{ cycle(id: $id) {{ {CYCLE_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: CycleQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.cycle)
    }

    pub async fn list_cycles(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Cycle>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                cycles(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {CYCLE_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: CyclesQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.cycles)
    }
}
