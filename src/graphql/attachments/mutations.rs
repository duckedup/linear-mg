#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct AttachmentCreateVariables {
    pub input: AttachmentCreateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "AttachmentCreateVariables"
)]
pub struct AttachmentCreateMutation {
    #[arguments(input: $input)]
    pub attachment_create: AttachmentPayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct AttachmentUpdateVariables {
    pub id: String,
    pub input: AttachmentUpdateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "AttachmentUpdateVariables"
)]
pub struct AttachmentUpdateMutation {
    #[arguments(id: $id, input: $input)]
    pub attachment_update: AttachmentPayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct AttachmentDeleteVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Mutation",
    variables = "AttachmentDeleteVariables"
)]
pub struct AttachmentDeleteMutation {
    #[arguments(id: $id)]
    pub attachment_delete: AttachmentDeletePayload,
}
