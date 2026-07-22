use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{Connection, ListResponse, PageInfo};
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
    /// Replies to this comment. Linear threads are one level deep, so children
    /// never have children of their own. Populated by `get_comment` and
    /// `list_comments`; empty for mutation payloads (which don't request it).
    #[serde(default, skip_serializing_if = "CommentChildren::is_empty")]
    pub children: CommentChildren,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentChildren {
    pub nodes: Vec<Comment>,
    /// Present whenever replies were requested. `has_next_page` signals that the
    /// thread has more replies than the inline preview fetched; page through them
    /// with `comments list --parent <id>`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_info: Option<PageInfo>,
}

impl CommentChildren {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Whether the thread has replies beyond the ones fetched inline.
    pub fn has_more(&self) -> bool {
        self.page_info.as_ref().is_some_and(|p| p.has_next_page)
    }
}

/// A page of top-level comments, each carrying its replies. Serializes exactly
/// like `ListResponse<Comment>` (nodes + pageInfo); the newtype exists so the
/// pretty output can render threaded blocks instead of a flat table.
#[derive(Serialize)]
#[serde(transparent)]
pub struct CommentThreads(pub ListResponse<Comment>);

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

/// How many replies to pull inline per thread. Threads with more than this are
/// flagged so the caller can page through the rest with `comments list --parent`.
pub const INLINE_REPLY_LIMIT: u32 = 50;

/// Build a comment selection that also pulls in replies. Linear threads are one
/// level deep, so children reuse `COMMENT_FIELDS` and do not recurse further. The
/// child `pageInfo` lets us flag threads with more replies than we fetched inline.
fn comment_thread_fields() -> String {
    format!(
        "{COMMENT_FIELDS} children(first: {INLINE_REPLY_LIMIT}) {{ nodes {{ {COMMENT_FIELDS} }} pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }} }}"
    )
}

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
        let fields = comment_thread_fields();
        let query = format!("query($id: String!) {{ comment(id: $id) {{ {fields} }} }}");
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
        filter: Option<serde_json::Value>,
    ) -> Result<Connection<Comment>, CliError> {
        let fields = comment_thread_fields();
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy, $filter: CommentFilter) {{
                comments(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy, filter: $filter) {{
                    nodes {{ {fields} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by, "filter": filter });
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
