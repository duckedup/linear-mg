use serde::Serialize;

#[allow(unused_imports)]
use crate::graphql::scalars::schema;

#[derive(cynic::QueryFragment, Debug, Serialize, Clone)]
#[cynic(schema = "linear", graphql_type = "PageInfo")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub end_cursor: Option<String>,
    pub start_cursor: Option<String>,
}

#[derive(cynic::Enum, Debug, Clone, Copy)]
#[cynic(schema = "linear", graphql_type = "PaginationOrderBy", rename_all = "camelCase")]
pub enum PaginationOrderBy {
    CreatedAt,
    UpdatedAt,
}

pub trait Paginatable {
    type Node;
    fn page_info(&self) -> &PageInfo;
    fn into_nodes(self) -> Vec<Self::Node>;
}

#[derive(Serialize)]
pub struct ListResponse<T: Serialize> {
    pub nodes: Vec<T>,
    pub page_info: PageInfoResponse,
}

#[derive(Serialize)]
pub struct PageInfoResponse {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

impl From<&PageInfo> for PageInfoResponse {
    fn from(pi: &PageInfo) -> Self {
        Self {
            has_next_page: pi.has_next_page,
            end_cursor: pi.end_cursor.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct MutationResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
}
