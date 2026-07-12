use serde::{Deserialize, Serialize};

use crate::client::LinearClient;
use crate::error::CliError;
use crate::graphql::issues::IssueSlim;

// -- Response types --

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelation {
    pub id: String,
    #[serde(rename = "type")]
    pub relation_type: String,
    pub created_at: String,
    pub updated_at: String,
    /// The source issue whose relationship is being described.
    pub issue: IssueSlim,
    /// The target issue that the source issue relates to.
    pub related_issue: IssueSlim,
}

/// A connection of relations embedded on an issue. Defaults to empty so that
/// queries which don't request relations (list/search/create) still deserialize.
#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RelationConnection {
    #[serde(default)]
    pub nodes: Vec<IssueRelation>,
}

/// A relation rendered from the perspective of a specific issue, so the
/// direction reads naturally (e.g. "blocked by" rather than raw "blocks").
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationView {
    /// The relation ID (pass to `issues unrelate` to remove it).
    pub id: String,
    /// Human-readable direction from the queried issue's perspective.
    pub direction: String,
    /// The raw relation type (blocks/duplicate/related/similar).
    #[serde(rename = "type")]
    pub relation_type: String,
    /// The issue on the other end of the relation.
    pub other: IssueSlim,
}

impl IssueRelationView {
    /// Build a view where the queried issue is the *source* of the relation.
    pub fn from_source(rel: IssueRelation) -> Self {
        let direction = match rel.relation_type.as_str() {
            "blocks" => "blocks",
            "duplicate" => "duplicate of",
            "similar" => "similar to",
            _ => "related to",
        }
        .to_string();
        Self {
            id: rel.id,
            direction,
            relation_type: rel.relation_type,
            other: rel.related_issue,
        }
    }

    /// Build a view where the queried issue is the *target* of the relation.
    pub fn from_target(rel: IssueRelation) -> Self {
        let direction = match rel.relation_type.as_str() {
            "blocks" => "blocked by",
            "duplicate" => "duplicated by",
            "similar" => "similar to",
            _ => "related to",
        }
        .to_string();
        Self {
            id: rel.id,
            direction,
            relation_type: rel.relation_type,
            other: rel.issue,
        }
    }
}

/// A list of relations touching a single issue, ready for output.
#[derive(Serialize)]
pub struct IssueRelationList {
    pub nodes: Vec<IssueRelationView>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationPayload {
    pub success: bool,
    pub issue_relation: Option<IssueRelation>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationDeletePayload {
    pub success: bool,
}

// -- Fragments --

pub const RELATION_FIELDS: &str = "
    id type createdAt updatedAt
    issue { id identifier title }
    relatedIssue { id identifier title }
";

// -- Queries --

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationsQuery {
    pub issue: IssueRelationsNode,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationsNode {
    pub relations: RelationConnection,
    pub inverse_relations: RelationConnection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationCreateResponse {
    pub issue_relation_create: IssueRelationPayload,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationDeleteResponse {
    pub issue_relation_delete: RelationDeletePayload,
}

impl LinearClient {
    /// Fetch the relations touching an issue. Returns `(relations, inverse_relations)`
    /// where `relations` are those the issue is the source of, and `inverse_relations`
    /// are those it is the target of.
    pub async fn list_issue_relations(
        &self,
        issue: &str,
    ) -> Result<(Vec<IssueRelation>, Vec<IssueRelation>), CliError> {
        let query = format!(
            "query($id: String!) {{
                issue(id: $id) {{
                    relations(first: 250) {{ nodes {{ {RELATION_FIELDS} }} }}
                    inverseRelations(first: 250) {{ nodes {{ {RELATION_FIELDS} }} }}
                }}
            }}"
        );
        let vars = serde_json::json!({ "id": issue });
        let resp: IssueRelationsQuery = self.query(&query, Some(vars)).await?;
        Ok((
            resp.issue.relations.nodes,
            resp.issue.inverse_relations.nodes,
        ))
    }

    pub async fn create_issue_relation(
        &self,
        input: serde_json::Value,
    ) -> Result<IssueRelationPayload, CliError> {
        let query = format!(
            "mutation($input: IssueRelationCreateInput!) {{ issueRelationCreate(input: $input) {{ success issueRelation {{ {RELATION_FIELDS} }} }} }}"
        );
        let vars = serde_json::json!({ "input": input });
        let resp: IssueRelationCreateResponse = self.query(&query, Some(vars)).await?;
        Ok(resp.issue_relation_create)
    }

    pub async fn delete_issue_relation(&self, id: &str) -> Result<RelationDeletePayload, CliError> {
        let query = "mutation($id: String!) { issueRelationDelete(id: $id) { success } }";
        let vars = serde_json::json!({ "id": id });
        let resp: IssueRelationDeleteResponse = self.query(query, Some(vars)).await?;
        Ok(resp.issue_relation_delete)
    }
}
