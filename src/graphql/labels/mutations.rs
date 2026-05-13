#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct IssueLabelCreateVariables {
    pub input: IssueLabelCreateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "IssueLabelCreateVariables")]
pub struct IssueLabelCreateMutation {
    #[arguments(input: $input)]
    pub issue_label_create: IssueLabelPayload,
}
