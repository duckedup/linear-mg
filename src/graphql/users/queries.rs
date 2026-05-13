#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;
use crate::graphql::common::PaginationOrderBy;

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Query")]
pub struct ViewerQuery {
    pub viewer: User,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UserByIdVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "UserByIdVariables"
)]
pub struct UserByIdQuery {
    #[arguments(id: $id)]
    pub user: User,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UsersListVariables {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "UsersListVariables"
)]
pub struct UsersListQuery {
    #[arguments(
        first: $first,
        after: $after,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub users: UserConnection,
}
