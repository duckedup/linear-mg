#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectCreateVariables {
    pub input: ProjectCreateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "ProjectCreateVariables"
)]
pub struct ProjectCreateMutation {
    #[arguments(input: $input)]
    pub project_create: ProjectPayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectUpdateVariables {
    pub id: String,
    pub input: ProjectUpdateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "ProjectUpdateVariables"
)]
pub struct ProjectUpdateMutation {
    #[arguments(id: $id, input: $input)]
    pub project_update: ProjectPayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectArchiveVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "ProjectArchiveVariables"
)]
pub struct ProjectArchiveMutation {
    #[arguments(id: $id)]
    pub project_archive: ProjectArchivePayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ProjectDeleteVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "ProjectDeleteVariables"
)]
pub struct ProjectDeleteMutation {
    #[arguments(id: $id)]
    pub project_delete: ProjectArchivePayload,
}
