use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;
use crate::graphql::issues::{IssueSlim, UserSlim};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub url: String,
    pub source_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub creator: Option<UserSlim>,
    pub issue: IssueSlim,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentPayload {
    pub success: bool,
    pub attachment: Attachment,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentDeletePayload {
    pub success: bool,
}

const ATTACHMENT_FIELDS: &str = "
    id title subtitle url sourceType createdAt updatedAt archivedAt
    creator { id name displayName email }
    issue { id identifier title }
";

#[derive(Deserialize)]
pub struct AttachmentQuery {
    pub attachment: Attachment,
}
#[derive(Deserialize)]
pub struct AttachmentsQuery {
    pub attachments: Connection<Attachment>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentCreateResponse {
    pub attachment_create: AttachmentPayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentUpdateResponse {
    pub attachment_update: AttachmentPayload,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentDeleteResponse {
    pub attachment_delete: AttachmentDeletePayload,
}

impl LinearClient {
    pub async fn get_attachment(&self, id: &str) -> Result<Attachment, CliError> {
        let query =
            format!("query($id: String!) {{ attachment(id: $id) {{ {ATTACHMENT_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: AttachmentQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.attachment)
    }

    pub async fn list_attachments(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<Attachment>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                attachments(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {ATTACHMENT_FIELDS} }}
                    pageInfo {{ hasNextPage hasPreviousPage endCursor startCursor }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "first": first, "after": after, "includeArchived": include_archived, "orderBy": order_by });
        let resp: AttachmentsQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.attachments)
    }

    pub async fn create_attachment(
        &self,
        input: serde_json::Value,
    ) -> Result<AttachmentPayload, CliError> {
        let query = format!(
            "mutation($input: AttachmentCreateInput!) {{ attachmentCreate(input: $input) {{ success attachment {{ {ATTACHMENT_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: AttachmentCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.attachment_create)
    }

    pub async fn update_attachment(
        &self,
        id: &str,
        input: serde_json::Value,
    ) -> Result<AttachmentPayload, CliError> {
        let query = format!(
            "mutation($id: String!, $input: AttachmentUpdateInput!) {{ attachmentUpdate(id: $id, input: $input) {{ success attachment {{ {ATTACHMENT_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "id": id, "input": input });
        let resp: AttachmentUpdateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.attachment_update)
    }

    pub async fn delete_attachment(&self, id: &str) -> Result<AttachmentDeletePayload, CliError> {
        let query = "mutation($id: String!) { attachmentDelete(id: $id) { success } }";
        let vars = serde_json::json!({ "id": id });
        let resp: AttachmentDeleteResponse = self.query(query, Some(vars)).await?;
        Ok(resp.attachment_delete)
    }
}
