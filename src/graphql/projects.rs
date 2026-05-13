use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::UserSlim;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub slug_id: String,
    pub description: String,
    pub icon: Option<String>,
    pub color: String,
    pub priority: i32,
    pub priority_label: String,
    pub progress: f64,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub canceled_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub trashed: Option<bool>,
    pub url: String,
    pub content: Option<String>,
    pub lead: Option<UserSlim>,
    pub creator: Option<UserSlim>,
    pub status: ProjectStatusSlim,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatusSlim {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPayload {
    pub success: bool,
    pub project: Option<Project>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectArchivePayload {
    pub success: bool,
}

const PROJECT_FIELDS: &str = "
    id name slugId description icon color priority priorityLabel progress
    startDate targetDate startedAt completedAt canceledAt createdAt updatedAt archivedAt trashed url content
    lead { id name displayName email }
    creator { id name displayName email }
    status { id name color }
";

#[derive(Deserialize)]
pub struct ProjectQuery {
    pub project: Project,
}
#[derive(Deserialize)]
pub struct ProjectsQuery {
    pub projects: Connection<Project>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCreateResponse {
    pub project_create: ProjectPayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectUpdateResponse {
    pub project_update: ProjectPayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectArchiveResponse {
    pub project_archive: ProjectArchivePayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDeleteResponse {
    pub project_delete: ProjectArchivePayload,
}

impl LinearClient {
    pub async fn get_project(&self, id: &str) -> Result<Project, CliError> {
        let query = format!("query($id: String!) {{ project(id: $id) {{ {PROJECT_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: ProjectQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.project)
    }

    pub async fn list_projects(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Project>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                projects(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {PROJECT_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: ProjectsQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.projects)
    }

    pub async fn create_project(
        &self,
        input: serde_json::Value,
    ) -> Result<ProjectPayload, CliError> {
        let query = format!(
            "mutation($input: ProjectCreateInput!) {{ projectCreate(input: $input) {{ success project {{ {PROJECT_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: ProjectCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.project_create)
    }

    pub async fn update_project(
        &self,
        id: &str,
        input: serde_json::Value,
    ) -> Result<ProjectPayload, CliError> {
        let query = format!(
            "mutation($id: String!, $input: ProjectUpdateInput!) {{ projectUpdate(id: $id, input: $input) {{ success project {{ {PROJECT_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "id": id, "input": input });
        let resp: ProjectUpdateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.project_update)
    }

    pub async fn archive_project(&self, id: &str) -> Result<ProjectArchivePayload, CliError> {
        let query = "mutation($id: String!) { projectArchive(id: $id) { success } }";
        let vars = serde_json::json!({ "id": id });
        let resp: ProjectArchiveResponse = self.query(query, Some(vars)).await?;
        Ok(resp.project_archive)
    }

    pub async fn delete_project(&self, id: &str) -> Result<ProjectArchivePayload, CliError> {
        let query = "mutation($id: String!) { projectDelete(id: $id) { success } }";
        let vars = serde_json::json!({ "id": id });
        let resp: ProjectDeleteResponse = self.query(query, Some(vars)).await?;
        Ok(resp.project_delete)
    }
}
