use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::ProjectSlim;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMilestone {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub target_date: Option<String>,
    pub progress: f64,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub project: ProjectSlim,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMilestonePayload {
    pub success: bool,
    pub project_milestone: ProjectMilestone,
}

const MILESTONE_FIELDS: &str = "
    id name description status targetDate progress createdAt updatedAt archivedAt
    project { id name slugId }
";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneQuery {
    pub project_milestone: ProjectMilestone,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestonesQuery {
    pub project_milestones: Connection<ProjectMilestone>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneCreateResponse {
    pub project_milestone_create: ProjectMilestonePayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneUpdateResponse {
    pub project_milestone_update: ProjectMilestonePayload,
}

impl LinearClient {
    pub async fn get_milestone(&self, id: &str) -> Result<ProjectMilestone, CliError> {
        let query =
            format!("query($id: String!) {{ projectMilestone(id: $id) {{ {MILESTONE_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: MilestoneQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.project_milestone)
    }

    pub async fn list_milestones(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<ProjectMilestone>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                projectMilestones(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {MILESTONE_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: MilestonesQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.project_milestones)
    }

    pub async fn create_milestone(
        &self,
        input: serde_json::Value,
    ) -> Result<ProjectMilestonePayload, CliError> {
        let query = format!(
            "mutation($input: ProjectMilestoneCreateInput!) {{ projectMilestoneCreate(input: $input) {{ success projectMilestone {{ {MILESTONE_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: MilestoneCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.project_milestone_create)
    }

    pub async fn update_milestone(
        &self,
        id: &str,
        input: serde_json::Value,
    ) -> Result<ProjectMilestonePayload, CliError> {
        let query = format!(
            "mutation($id: String!, $input: ProjectMilestoneUpdateInput!) {{ projectMilestoneUpdate(id: $id, input: $input) {{ success projectMilestone {{ {MILESTONE_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "id": id, "input": input });
        let resp: MilestoneUpdateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.project_milestone_update)
    }
}
