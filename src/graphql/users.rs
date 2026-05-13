use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::Connection;

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub active: bool,
    pub admin: bool,
    pub guest: bool,
    pub is_me: bool,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: Option<String>,
    pub last_seen: Option<String>,
}

const USER_FIELDS: &str = "id name displayName email active admin guest isMe avatarUrl description createdAt updatedAt archivedAt lastSeen";

#[derive(Deserialize)]
pub struct ViewerQuery {
    pub viewer: User,
}

#[derive(Deserialize)]
pub struct UserQuery {
    pub user: User,
}

#[derive(Deserialize)]
pub struct UsersQuery {
    pub users: Connection<User>,
}

impl LinearClient {
    pub async fn get_viewer(&self) -> Result<User, CliError> {
        let query = format!("{{ viewer {{ {USER_FIELDS} }} }}");
        let resp: ViewerQuery = self.query(&query, None).await?;
        Ok(resp.viewer)
    }

    pub async fn get_user(&self, id: &str) -> Result<User, CliError> {
        let query = format!("query($id: String!) {{ user(id: $id) {{ {USER_FIELDS} }} }}");
        let vars = serde_json::json!({ "id": id });
        let resp: UserQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.user)
    }

    pub async fn list_users(
        &self,
        first: u32,
        after: Option<String>,
        include_archived: bool,
        order_by: &str,
    ) -> Result<Connection<User>, CliError> {
        let query = format!(
            "query($first: Int, $after: String, $includeArchived: Boolean, $orderBy: PaginationOrderBy) {{
                users(first: $first, after: $after, includeArchived: $includeArchived, orderBy: $orderBy) {{
                    nodes {{ {USER_FIELDS} }}
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
        let resp: UsersQuery = self.query(&query, Some(vars)).await?;
        Ok(resp.users)
    }
}
