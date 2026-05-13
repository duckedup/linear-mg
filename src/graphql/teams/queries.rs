#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;
use crate::graphql::common::PaginationOrderBy;

#[derive(cynic::QueryVariables, Debug)]
pub struct TeamByIdVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Query", variables = "TeamByIdVariables")]
pub struct TeamByIdQuery {
    #[arguments(id: $id)]
    pub team: Team,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct TeamsListVariables {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Query", variables = "TeamsListVariables")]
pub struct TeamsListQuery {
    #[arguments(
        first: $first,
        after: $after,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub teams: TeamConnection,
}
