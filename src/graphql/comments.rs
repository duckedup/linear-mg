use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::UserSlim;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    pub edited_at: Option<String>,
    pub archived_at: Option<String>,
    pub resolved_at: Option<String>,
    pub url: String,
    pub user: Option<UserSlim>,
    pub issue: Option<CommentIssueSlim>,
    pub parent: Option<CommentSlim>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentIssueSlim {
    pub id: String,
    pub identifier: String,
    pub title: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentSlim {
    pub id: String,
    pub body: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentPayload {
    pub success: bool,
    pub comment: Option<Comment>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletePayload {
    pub success: bool,
}

const COMMENT_FIELDS: &str = "
    id body createdAt updatedAt editedAt archivedAt resolvedAt url
    user { id name displayName email }
    issue { id identifier title }
    parent { id body }
";

#[derive(Deserialize)]
pub struct CommentQuery {
    pub comment: Comment,
}
#[derive(Deserialize)]
pub struct CommentsQuery {
    pub comments: Connection<Comment>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentCreateResponse {
    pub comment_create: CommentPayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentUpdateResponse {
    pub comment_update: CommentPayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDeleteResponse {
    pub comment_delete: DeletePayload,
}

impl LinearClient {
    pub async fn get_comment(&self, id: &str) -> Result<Comment, CliError> {
        let query = format!("query($id: String!) {{ comment(id: $id) {{ {COMMENT_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: CommentQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.comment)
    }

    pub async fn list_comments(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Comment>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                comments(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {COMMENT_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: CommentsQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.comments)
    }

    pub async fn create_comment(
        &self,
        input: serde_json::Value,
    ) -> Result<CommentPayload, CliError> {
        let query = format!(
            "mutation($input: CommentCreateInput!) {{ commentCreate(input: $input) {{ success comment {{ {COMMENT_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: CommentCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.comment_create)
    }

    pub async fn update_comment(
        &self,
        id: &str,
        input: serde_json::Value,
    ) -> Result<CommentPayload, CliError> {
        let query = format!(
            "mutation($id: String!, $input: CommentUpdateInput!) {{ commentUpdate(id: $id, input: $input) {{ success comment {{ {COMMENT_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "id": id, "input": input });
        let resp: CommentUpdateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.comment_update)
    }

    pub async fn delete_comment(&self, id: &str) -> Result<DeletePayload, CliError> {
        let query = "mutation($id: String!) { commentDelete(id: $id) { success } }";
        let vars = serde_json::json!({ "id": id });
        let resp: CommentDeleteResponse = self.query(query, Some(vars)).await?;
        Ok(resp.comment_delete)
    }
}
