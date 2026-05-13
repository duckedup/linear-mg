#[allow(unused_imports)]
use crate::graphql::scalars::schema;

use super::types::*;
use crate::graphql::common::PaginationOrderBy;

#[derive(cynic::QueryVariables, Debug)]
pub struct AttachmentByIdVariables {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "AttachmentByIdVariables"
)]
pub struct AttachmentByIdQuery {
    #[arguments(id: $id)]
    pub attachment: Attachment,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct AttachmentsListVariables {
    pub first: Option<i32>,
    pub after: Option<String>,
    pub include_archived: Option<bool>,
    pub order_by: Option<PaginationOrderBy>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema = "linear",
    graphql_type = "Query",
    variables = "AttachmentsListVariables"
)]
pub struct AttachmentsListQuery {
    #[arguments(
        first: $first,
        after: $after,
        includeArchived: $include_archived,
        orderBy: $order_by
    )]
    pub attachments: AttachmentConnection,
}
