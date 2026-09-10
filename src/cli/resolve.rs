use crate::client::LinearClient;
use crate::error::CliError;

fn is_uuid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    parts.len() == 5
        && parts[0].len() == 8
        && parts[1].len() == 4
        && parts[2].len() == 4
        && parts[3].len() == 4
        && parts[4].len() == 12
        && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

pub fn extract_team_key(identifier: &str) -> Option<&str> {
    let key = identifier.split('-').next()?;
    if !key.is_empty() && key.chars().all(|c| c.is_ascii_alphabetic()) {
        Some(key)
    } else {
        None
    }
}

pub async fn resolve_issue(client: &LinearClient, value: &str) -> Result<String, CliError> {
    if is_uuid(value) {
        return Ok(value.to_string());
    }
    let issue = client.get_issue(value).await?;
    Ok(issue.id)
}

pub async fn resolve_assignee(client: &LinearClient, value: &str) -> Result<String, CliError> {
    if is_uuid(value) {
        return Ok(value.to_string());
    }
    if value.eq_ignore_ascii_case("me") {
        return Ok(client.get_viewer().await?.id);
    }
    let users = client.list_users(250, None, false, "updatedAt").await?;
    if value.contains('@') {
        if let Some(user) = users
            .nodes
            .iter()
            .find(|u| u.email.eq_ignore_ascii_case(value))
        {
            return Ok(user.id.clone());
        }
        return Err(CliError::NotFound(format!("user with email '{value}'")));
    }
    if let Some(user) = users
        .nodes
        .iter()
        .find(|u| u.display_name.eq_ignore_ascii_case(value) || u.name.eq_ignore_ascii_case(value))
    {
        return Ok(user.id.clone());
    }
    Err(CliError::NotFound(format!("user '{value}'")))
}

pub async fn resolve_project(client: &LinearClient, value: &str) -> Result<String, CliError> {
    if is_uuid(value) {
        return Ok(value.to_string());
    }
    let projects = client
        .list_projects(250, None, false, "updatedAt", None)
        .await?;
    // A slug match is unambiguous, so prefer it before falling back to names.
    if let Some(project) = projects
        .nodes
        .iter()
        .find(|p| p.slug_id.eq_ignore_ascii_case(value))
    {
        return Ok(project.id.clone());
    }
    let matching: Vec<_> = projects
        .nodes
        .iter()
        .filter(|p| p.name.eq_ignore_ascii_case(value))
        .collect();
    match matching.len() {
        0 => Err(CliError::NotFound(format!("project '{value}'"))),
        1 => Ok(matching[0].id.clone()),
        _ => {
            let slugs: Vec<&str> = matching.iter().map(|p| p.slug_id.as_str()).collect();
            Err(CliError::InvalidInput(format!(
                "multiple projects named '{}' found (slugs: {}); pass a slug or project ID to disambiguate",
                value,
                slugs.join(", ")
            )))
        }
    }
}

/// A milestone resolved from user input. `project_id` is known only when the
/// milestone was looked up by name (a UUID passes through without a fetch).
#[derive(Debug)]
pub struct ResolvedMilestone {
    pub id: String,
    pub project_id: Option<String>,
}

/// Resolve a milestone by UUID or name. When `project_id` is given, only
/// milestones in that project are considered; otherwise a name that exists in
/// several projects is rejected as ambiguous. The name match is done by the API
/// so the lookup is complete regardless of how many milestones exist.
pub async fn resolve_milestone(
    client: &LinearClient,
    value: &str,
    project_id: Option<&str>,
) -> Result<ResolvedMilestone, CliError> {
    if is_uuid(value) {
        return Ok(ResolvedMilestone {
            id: value.to_string(),
            project_id: None,
        });
    }
    let filter = crate::graphql::milestones::milestone_filter(project_id, Some(value));
    let milestones = client
        .list_milestones(250, None, false, "updatedAt", filter)
        .await?;
    let matching: Vec<_> = milestones
        .nodes
        .iter()
        .filter(|m| m.name.eq_ignore_ascii_case(value))
        .collect();

    match matching.len() {
        0 => Err(CliError::NotFound(format!("milestone '{value}'"))),
        1 => Ok(ResolvedMilestone {
            id: matching[0].id.clone(),
            project_id: Some(matching[0].project.id.clone()),
        }),
        _ if project_id.is_some() => {
            // Same name twice within one project: only an ID can pick one.
            let ids: Vec<String> = matching
                .iter()
                .map(|m| format!("{} (due {})", m.id, m.target_date.as_deref().unwrap_or("-")))
                .collect();
            Err(CliError::InvalidInput(format!(
                "multiple milestones named '{}' in this project: {}; pass a milestone ID",
                value,
                ids.join(", ")
            )))
        }
        _ => {
            let projects: Vec<&str> = matching.iter().map(|m| m.project.name.as_str()).collect();
            Err(CliError::InvalidInput(format!(
                "multiple milestones named '{}' found in projects: {}; pass --project or a milestone ID to disambiguate",
                value,
                projects.join(", ")
            )))
        }
    }
}

/// Resolve `--project` / `--milestone` for an issue mutation and insert
/// `projectId` / `projectMilestoneId` into `obj`.
///
/// A milestone name is looked up within `--project` when given, else within
/// `fallback_project` (the issue's current project on update). When neither is
/// known, the unique match's own project is set as `projectId`, since Linear
/// requires a milestone to belong to the issue's project.
pub async fn apply_project_and_milestone(
    client: &LinearClient,
    obj: &mut serde_json::Map<String, serde_json::Value>,
    project: Option<String>,
    milestone: Option<String>,
    fallback_project: Option<String>,
) -> Result<(), CliError> {
    let mut project_id = match project {
        Some(v) => Some(resolve_project(client, &v).await?),
        None => None,
    };
    if let Some(m) = milestone {
        let scope = project_id.clone().or(fallback_project);
        let resolved = resolve_milestone(client, &m, scope.as_deref()).await?;
        if project_id.is_none() && scope.is_none() {
            project_id = resolved.project_id;
        }
        obj.insert("projectMilestoneId".into(), resolved.id.into());
    }
    if let Some(pid) = project_id {
        obj.insert("projectId".into(), pid.into());
    }
    Ok(())
}

pub async fn resolve_team(client: &LinearClient, value: &str) -> Result<String, CliError> {
    if is_uuid(value) {
        return Ok(value.to_string());
    }
    let teams = client.list_teams(250, None, false, "updatedAt").await?;
    if let Some(team) = teams
        .nodes
        .iter()
        .find(|t| t.key.eq_ignore_ascii_case(value) || t.name.eq_ignore_ascii_case(value))
    {
        return Ok(team.id.clone());
    }
    Err(CliError::NotFound(format!("team '{value}'")))
}

pub async fn resolve_label(
    client: &LinearClient,
    value: &str,
    team_id: Option<&str>,
) -> Result<String, CliError> {
    if is_uuid(value) {
        return Ok(value.to_string());
    }
    let labels = client.list_labels(250, None, false, "updatedAt").await?;
    let matching: Vec<_> = labels
        .nodes
        .iter()
        .filter(|l| l.name.eq_ignore_ascii_case(value))
        .collect();

    match matching.len() {
        0 => Err(CliError::NotFound(format!("label '{value}'"))),
        1 => Ok(matching[0].id.clone()),
        _ => {
            // When a team is known, prefer a label scoped to that team, then
            // fall back to a workspace-level label (team == None).
            if let Some(tid) = team_id {
                if let Some(l) = matching
                    .iter()
                    .find(|l| l.team.as_ref().is_some_and(|t| t.id == tid))
                {
                    return Ok(l.id.clone());
                }
                if let Some(l) = matching.iter().find(|l| l.team.is_none()) {
                    return Ok(l.id.clone());
                }
            }
            let scopes: Vec<&str> = matching
                .iter()
                .map(|l| l.team.as_ref().map_or("workspace", |t| t.key.as_str()))
                .collect();
            Err(CliError::InvalidInput(format!(
                "multiple labels named '{}' found ({}); pass a label ID to disambiguate",
                value,
                scopes.join(", ")
            )))
        }
    }
}

pub async fn resolve_state(
    client: &LinearClient,
    value: &str,
    team_id: Option<&str>,
) -> Result<String, CliError> {
    if is_uuid(value) {
        return Ok(value.to_string());
    }
    let states = client
        .list_workflow_states(250, None, false, "updatedAt")
        .await?;
    let matching: Vec<_> = states
        .nodes
        .iter()
        .filter(|s| {
            s.name.eq_ignore_ascii_case(value) && team_id.is_none_or(|tid| s.team.id == tid)
        })
        .collect();

    match matching.len() {
        0 => Err(CliError::NotFound(format!("workflow state '{value}'"))),
        1 => Ok(matching[0].id.clone()),
        _ => {
            if team_id.is_some() {
                Ok(matching[0].id.clone())
            } else {
                let teams: Vec<_> = matching.iter().map(|s| s.team.key.as_str()).collect();
                Err(CliError::InvalidInput(format!(
                    "multiple workflow states named '{}' found in teams: {}; specify --team to disambiguate",
                    value,
                    teams.join(", ")
                )))
            }
        }
    }
}
