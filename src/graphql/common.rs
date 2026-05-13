use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub end_cursor: Option<String>,
    pub start_cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connection<T> {
    pub nodes: Vec<T>,
    pub page_info: PageInfo,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResponse<T: Serialize> {
    pub nodes: Vec<T>,
    pub page_info: PageInfoResponse,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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
