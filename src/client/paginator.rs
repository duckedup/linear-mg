use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::common::{ListResponse, PageInfoResponse, Paginatable};
use std::future::Future;
use std::pin::Pin;

pub struct PaginationParams {
    pub limit: Option<u32>,
    pub after: Option<String>,
    pub all: bool,
    pub page_size: i32,
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

pub async fn paginate<'a, C, F, Node>(
    client: &'a LinearClient,
    params: &PaginationParams,
    fetch: F,
) -> Result<ListResponse<Node>, CliError>
where
    C: Paginatable<Node = Node>,
    Node: serde::Serialize,
    F: Fn(&'a LinearClient, i32, Option<String>) -> Pin<Box<dyn Future<Output = Result<C, CliError>> + Send + 'a>>,
{
    let max = if params.all {
        None
    } else {
        params.limit.or(Some(50))
    };
    let page_size = match max {
        Some(m) if m < params.page_size as u32 => m as i32,
        _ => params.page_size,
    };

    let mut all_nodes = Vec::new();
    let mut cursor = params.after.clone();
    let mut last_page_info;

    loop {
        let connection = fetch(client, page_size, cursor).await?;
        let has_next = connection.page_info().has_next_page;
        let end_cursor = connection.page_info().end_cursor.clone();
        last_page_info = PageInfoResponse::from(connection.page_info());
        all_nodes.extend(connection.into_nodes());

        if let Some(max) = max {
            if all_nodes.len() >= max as usize {
                all_nodes.truncate(max as usize);
                break;
            }
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
