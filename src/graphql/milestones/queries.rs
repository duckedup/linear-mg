#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;
use crate::graphql::common::PaginationOrderBy;

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectMilestoneByIdVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "ProjectMilestoneByIdVariables"
)]
pub struct ProjectMilestoneByIdQuery {
    #[arguments(id: $id)]
    pub project_milestone: ProjectMilestone,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectMilestonesListVariables {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "ProjectMilestonesListVariables"
)]
pub struct ProjectMilestonesListQuery {
    #[arguments(
        first: $first,
        after: $after,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub project_milestones: ProjectMilestoneConnection,
}
