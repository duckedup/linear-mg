use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;

// -- Response types --

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub identifier: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: f64,
    pub priority_label: String,
    pub estimate: Option<f64>,
    pub due_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
    pub canceled_at: Option<String>,
    pub archived_at: Option<String>,
    pub started_at: Option<String>,
    pub branch_name: String,
    pub number: f64,
    pub url: String,
    pub trashed: Option<bool>,
    pub state: WorkflowStateSlim,
    pub assignee: Option<UserSlim>,
    pub creator: Option<UserSlim>,
    pub team: TeamSlim,
    pub project: Option<ProjectSlim>,
    pub cycle: Option<CycleSlim>,
    pub parent: Option<IssueSlim>,
    pub labels: LabelConnection,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueSlim {
    pub id: String,
    pub identifier: String,
    pub title: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStateSlim {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub color: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSlim {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub email: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamSlim {
    pub id: String,
    pub name: String,
    pub key: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSlim {
    pub id: String,
    pub name: String,
    pub slug_id: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CycleSlim {
    pub id: String,
    pub number: f64,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelConnection {
    pub nodes: Vec<LabelSlim>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelSlim {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssuePayload {
    pub success: bool,
    pub issue: Option<Issue>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchivePayload {
    pub success: bool,
}

// -- Fragments --

const ISSUE_FIELDS: &str = "
    id identifier title description priority priorityLabel estimate dueDate
    createdAt updatedAt completedAt canceledAt archivedAt startedAt
    branchName number url trashed
    state { id name type color }
    assignee { id name displayName email }
    creator { id name displayName email }
    team { id name key }
    project { id name slugId }
    cycle { id number name }
    parent { id identifier title }
    labels { nodes { id name color } }
";

// -- Queries --

#[derive(Deserialize)]
pub struct IssueQuery {
    pub issue: Issue,
}

#[derive(Deserialize)]
pub struct IssuesQuery {
    pub issues: Connection<Issue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueSearchQuery {
    pub issue_search: Connection<Issue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCreateResponse {
    pub issue_create: IssuePayload,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueUpdateResponse {
    pub issue_update: IssuePayload,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueArchiveResponse {
    pub issue_archive: ArchivePayload,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueDeleteResponse {
    pub issue_delete: ArchivePayload,
}

impl LinearClient {
    pub async fn get_issue(&self, id: &str) -> Result<Issue, CliError> {
        let query = format!("query($id: String!) {{ issue(id: $id) {{ {ISSUE_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: IssueQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.issue)
    }

    pub async fn list_issues(
        &self,
        first: u32,
        after: Option<String>,
        filter: Option<serde_json::Value>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Issue>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $filter: IssueFilter, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                issues(first: $first, after: $after, filter: $filter, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {ISSUE_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({
            "first": first,
            "after": after,
            "filter": filter,
            "includeArchived": include_archived,
            "orderBy": order_by,
        });
        let resp: IssuesQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.issues)
    }

    pub async fn search_issues(
        &self,
        term: &str,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Issue>, CliError> {
        let query = format!(
            "query($query: String, $first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                issueSearch(query: $query, first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {ISSUE_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({
            "query": term,
            "first": first,
            "after": after,
            "includeArchived": include_archived,
            "orderBy": order_by,
        });
        let resp: IssueSearchQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.issue_search)
    }

    pub async fn create_issue(&self, input: serde_json::Value) -> Result<IssuePayload, CliError> {
        let query = format!(
            "mutation($input: IssueCreateInput!) {{ issueCreate(input: $input) {{ success issue {{ {ISSUE_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: IssueCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.issue_create)
    }

    pub async fn update_issue(
        &self,
        id: &str,
        input: serde_json::Value,
    ) -> Result<IssuePayload, CliError> {
        let query = format!(
            "mutation($id: String!, $input: IssueUpdateInput!) {{ issueUpdate(id: $id, input: $input) {{ success issue {{ {ISSUE_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "id": id, "input": input });
        let resp: IssueUpdateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.issue_update)
    }

    pub async fn archive_issue(&self, id: &str) -> Result<ArchivePayload, CliError> {
        let query = "mutation($id: String!) { issueArchive(id: $id) { success } }";
        let vars = serde_json::json!({ "id": id });
        let resp: IssueArchiveResponse = self.query(query, Some(vars)).await?;
        Ok(resp.issue_archive)
    }

    pub async fn delete_issue(&self, id: &str) -> Result<ArchivePayload, CliError> {
        let query = "mutation($id: String!) { issueDelete(id: $id) { success } }";
        let vars = serde_json::json!({ "id": id });
        let resp: IssueDeleteResponse = self.query(query, Some(vars)).await?;
        Ok(resp.issue_delete)
    }
}
