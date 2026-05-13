#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectMilestoneCreateVariables {
    pub input: ProjectMilestoneCreateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "ProjectMilestoneCreateVariables"
)]
pub struct ProjectMilestoneCreateMutation {
    #[arguments(input: $input)]
    pub project_milestone_create: ProjectMilestonePayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectMilestoneUpdateVariables {
    pub id: String,
    pub input: ProjectMilestoneUpdateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "ProjectMilestoneUpdateVariables"
)]
pub struct ProjectMilestoneUpdateMutation {
    #[arguments(id: $id, input: $input)]
    pub project_milestone_update: ProjectMilestonePayload,
}
