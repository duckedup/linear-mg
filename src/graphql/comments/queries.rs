#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;
use crate::graphql::common::PaginationOrderBy;

#[derive(cynic::QueryVariables, Debug)]
pub struct CommentByIdVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "CommentByIdVariables"
)]
pub struct CommentByIdQuery {
    #[arguments(id: $id)]
    pub comment: Comment,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CommentsListVariables {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "CommentsListVariables"
)]
pub struct CommentsListQuery {
    #[arguments(
        first: $first,
        after: $after,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub comments: CommentConnection,
}
