#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct DocumentCreateVariables {
    pub input: DocumentCreateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "DocumentCreateVariables")]
pub struct DocumentCreateMutation {
    #[arguments(input: $input)]
    pub document_create: DocumentPayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct DocumentUpdateVariables {
    pub id: String,
    pub input: DocumentUpdateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "DocumentUpdateVariables")]
pub struct DocumentUpdateMutation {
    #[arguments(id: $id, input: $input)]
    pub document_update: DocumentPayload,
}
