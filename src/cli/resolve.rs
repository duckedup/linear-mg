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
    if let Some(project) = projects
        .nodes
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(value) || p.slug_id.eq_ignore_ascii_case(value))
    {
        return Ok(project.id.clone());
    }
    Err(CliError::NotFound(format!("project '{value}'")))
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
