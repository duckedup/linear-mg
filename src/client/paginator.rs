use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{Connection, ListResponse, PageInfoResponse};
use std::future::Future;
use std::pin::Pin;

pub struct PaginationParams {
    pub limit: Option<u32>,
    pub after: Option<String>,
    pub all: bool,
    pub page_size: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: Some(50),
            after: None,
            all: false,
            page_size: 50,
        }
    }
}

pub async fn paginate<'a, T, F>(
    client: &'a LinearClient,
    params: &PaginationParams,
    fetch: F,
) -> Result<ListResponse<T>, CliError>
where
    T: serde::Serialize,
    F: Fn(
        &'a LinearClient,
        u32,
        Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Connection<T>, CliError>> + Send + 'a>>,
{
    let max = if params.all {
        None
    } else {
        params.limit.or(Some(50))
    };
    let page_size = match max {
        Some(m) if m < params.page_size => m,
        _ => params.page_size,
    };

    let mut all_nodes = Vec::new();
    let mut cursor = params.after.clone();
    let mut last_page_info;

    loop {
        let connection = fetch(client, page_size, cursor).await?;
        let has_next = connection.page_info.has_next_page;
        let end_cursor = connection.page_info.end_cursor.clone();
        last_page_info = PageInfoResponse::from(&connection.page_info);
        all_nodes.extend(connection.nodes);

        if let Some(max) = max
            && all_nodes.len() >= max as usize
        {
            all_nodes.truncate(max as usize);
            break;
        }

        if !has_next {
            break;
        }
        cursor = end_cursor;
    }

    Ok(ListResponse {
        nodes: all_nodes,
        page_info: last_page_info,
    })
}
