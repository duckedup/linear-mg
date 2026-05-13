#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;
use crate::graphql::common::PaginationOrderBy;

#[derive(cynic::QueryVariables, Debug)]
pub struct DocumentByIdVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "DocumentByIdVariables"
)]
pub struct DocumentByIdQuery {
    #[arguments(id: $id)]
    pub document: Document,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DocumentsListVariables {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "DocumentsListVariables"
)]
pub struct DocumentsListQuery {
    #[arguments(
        first: $first,
        after: $after,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub documents: DocumentConnection,
}
