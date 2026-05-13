#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;
use crate::graphql::common::PaginationOrderBy;

// -- Get single issue --

#[derive(cynic::QueryVariables, Debug)]
pub struct IssueByIdVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "IssueByIdVariables"
)]
pub struct IssueByIdQuery {
    #[arguments(id: $id)]
    pub issue: Issue,
}

// -- List issues --

#[derive(cynic::QueryVariables, Debug)]
pub struct IssuesListVariables {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub filter: Option<IssueFilter>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "IssuesListVariables"
)]
pub struct IssuesListQuery {
    #[arguments(
        first: $first,
        after: $after,
        filter: $filter,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub issues: IssueConnection,
}

// -- Search issues --

#[derive(cynic::QueryVariables, Debug)]
pub struct IssueSearchVariables {
    pub query: Option<String>,
    pub first: Option<i32>,
    pub after: Option<String>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "IssueSearchVariables"
)]
pub struct IssueSearchQuery {
    #[arguments(
        query: $query,
        first: $first,
        after: $after,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub issue_search: IssueConnection,
}
