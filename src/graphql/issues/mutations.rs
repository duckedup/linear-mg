#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;

// -- Create issue --

#[derive(cynic::QueryVariables, Debug)]
pub struct IssueCreateVariables {
    pub input: IssueCreateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "IssueCreateVariables")]
pub struct IssueCreateMutation {
    #[arguments(input: $input)]
    pub issue_create: IssuePayload,
}

// -- Update issue --

#[derive(cynic::QueryVariables, Debug)]
pub struct IssueUpdateVariables {
    pub id: String,
    pub input: IssueUpdateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "IssueUpdateVariables")]
pub struct IssueUpdateMutation {
    #[arguments(id: $id, input: $input)]
    pub issue_update: IssuePayload,
}

// -- Archive issue --

#[derive(cynic::QueryVariables, Debug)]
pub struct IssueArchiveVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "IssueArchiveVariables")]
pub struct IssueArchiveMutation {
    #[arguments(id: $id)]
    pub issue_archive: IssueArchivePayload,
}

// -- Delete issue --

#[derive(cynic::QueryVariables, Debug)]
pub struct IssueDeleteVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "IssueDeleteVariables")]
pub struct IssueDeleteMutation {
    #[arguments(id: $id)]
    pub issue_delete: IssueArchivePayload,
}
