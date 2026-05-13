use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::{ProjectSlim, TeamSlim, UserSlim};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub title: String,
    pub slug_id: String,
    pub content: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub trashed: Option<bool>,
    pub url: String,
    pub creator: Option<UserSlim>,
    pub updated_by: Option<UserSlim>,
    pub project: Option<ProjectSlim>,
    pub team: Option<TeamSlim>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentPayload {
    pub success: bool,
    pub document: Document,
}

const DOC_FIELDS: &str = "
    id title slugId content icon color createdAt updatedAt archivedAt trashed url
    creator { id name displayName email }
    updatedBy { id name displayName email }
    project { id name slugId }
    team { id name key }
";

#[derive(Deserialize)]
pub struct DocumentQuery {
    pub document: Document,
}
#[derive(Deserialize)]
pub struct DocumentsQuery {
    pub documents: Connection<Document>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentCreateResponse {
    pub document_create: DocumentPayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentUpdateResponse {
    pub document_update: DocumentPayload,
}

impl LinearClient {
    pub async fn get_document(&self, id: &str) -> Result<Document, CliError> {
        let query = format!("query($id: String!) {{ document(id: $id) {{ {DOC_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: DocumentQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.document)
    }

    pub async fn list_documents(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Document>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                documents(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {DOC_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: DocumentsQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.documents)
    }

    pub async fn create_document(
        &self,
        input: serde_json::Value,
    ) -> Result<DocumentPayload, CliError> {
        let query = format!(
            "mutation($input: DocumentCreateInput!) {{ documentCreate(input: $input) {{ success document {{ {DOC_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: DocumentCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.document_create)
    }

    pub async fn update_document(
        &self,
        id: &str,
        input: serde_json::Value,
    ) -> Result<DocumentPayload, CliError> {
        let query = format!(
            "mutation($id: String!, $input: DocumentUpdateInput!) {{ documentUpdate(id: $id, input: $input) {{ success document {{ {DOC_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "id": id, "input": input });
        let resp: DocumentUpdateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.document_update)
    }
}
