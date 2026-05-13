#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;

#[derive(cynic::QueryVariables, Debug)]
pub struct CommentCreateVariables {
    pub input: CommentCreateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "CommentCreateVariables")]
pub struct CommentCreateMutation {
    #[arguments(input: $input)]
    pub comment_create: CommentPayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CommentUpdateVariables {
    pub id: String,
    pub input: CommentUpdateInput,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "CommentUpdateVariables")]
pub struct CommentUpdateMutation {
    #[arguments(id: $id, input: $input)]
    pub comment_update: CommentPayload,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CommentDeleteVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(schema = "linear", graphql_type = "Mutation", variables = "CommentDeleteVariables")]
pub struct CommentDeleteMutation {
    #[arguments(id: $id)]
    pub comment_delete: DeletePayload,
}
