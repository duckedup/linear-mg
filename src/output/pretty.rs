use super::PrettyPrint;
use crate::graphql::attachments::Attachment;
use crate::graphql::comments::Comment;
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::cycles::Cycle;
use crate::graphql::documents::Document;
use crate::graphql::initiatives::Initiative;
use crate::graphql::issues::Issue;
use crate::graphql::labels::IssueLabel;
use crate::graphql::milestones::ProjectMilestone;
use crate::graphql::projects::Project;
use crate::graphql::teams::Team;
use crate::graphql::users::User;
use crate::graphql::workflow_states::WorkflowState;

fn truncate(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max {
        s.to_string()
    } else if max <= 2 {
        chars[..max].iter().collect()
    } else {
        let t: String = chars[..max - 2].iter().collect();
        format!("{t}..")
    }
}

fn short_date(s: &str) -> &str {
    if s.len() >= 10 {
        &s[..10]
    } else {
        s
    }
}

fn align_columns(rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut widths = vec![0usize; cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }
    let mut out = String::new();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                out.push_str("  ");
            }
            if i < row.len() - 1 {
                out.push_str(cell);
                for _ in 0..widths[i].saturating_sub(cell.len()) {
                    out.push(' ');
                }
            } else {
                out.push_str(cell);
            }
        }
        out.push('\n');
    }
    out
}

// -- Generic wrappers --

impl<T: serde::Serialize + PrettyPrint> PrettyPrint for ListResponse<T> {
    fn pretty(&self) -> String {
        if self.nodes.is_empty() {
            return "No results.".into();
        }
        let rows: Vec<Vec<String>> = self.nodes.iter().map(|n| n.pretty_row()).collect();
        let mut out = align_columns(&rows);
        let n = self.nodes.len();
        if self.page_info.has_next_page {
            write!(out, "\n{n} results (more available)").unwrap();
        } else {
            write!(out, "\n{n} results").unwrap();
        }
        out
    }
}

impl<T: serde::Serialize + PrettyPrint> PrettyPrint for MutationResponse<T> {
    fn pretty(&self) -> String {
        if self.success {
            match &self.data {
                Some(data) => data.pretty(),
                None => "Done.".into(),
            }
        } else {
            "Operation failed.".into()
        }
    }
}

impl PrettyPrint for () {
    fn pretty(&self) -> String {
        String::new()
    }
}

impl PrettyPrint for serde_json::Value {
    fn pretty(&self) -> String {
        match self {
            serde_json::Value::Object(map) => {
                let mut out = String::new();
                if let Some(serde_json::Value::String(msg)) = map.get("message") {
                    out.push_str(msg);
                }
                for (k, v) in map {
                    if k == "message" {
                        continue;
                    }
                    if !out.is_empty() {
                        out.push('\n');
                    }
                    let val = match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    out.push_str(&format!("{k}: {val}"));
                }
                out
            }
            other => other.to_string(),
        }
    }
}

// -- Issue --

impl PrettyPrint for Issue {
    fn pretty(&self) -> String {
        let mut out = format!("{}: {}", self.identifier, self.title);
        out.push_str(&format!(
            "\n\n  State:     {} ({})",
            self.state.name, self.state.state_type
        ));
        out.push_str(&format!("\n  Priority:  {}", self.priority_label));
        out.push_str(&format!(
            "\n  Team:      {} ({})",
            self.team.name, self.team.key
        ));
        if let Some(ref a) = self.assignee {
            out.push_str(&format!("\n  Assignee:  {}", a.display_name));
        }
        if let Some(ref p) = self.project {
            out.push_str(&format!("\n  Project:   {}", p.name));
        }
        if let Some(ref c) = self.cycle {
            let name = c.name.as_deref().unwrap_or("");
            if name.is_empty() {
                out.push_str(&format!("\n  Cycle:     #{}", c.number as i64));
            } else {
                out.push_str(&format!("\n  Cycle:     #{} ({name})", c.number as i64));
            }
        }
        let labels: Vec<&str> = self.labels.nodes.iter().map(|l| l.name.as_str()).collect();
        if !labels.is_empty() {
            out.push_str(&format!("\n  Labels:    {}", labels.join(", ")));
        }
        if let Some(ref d) = self.due_date {
            out.push_str(&format!("\n  Due:       {d}"));
        }
        if let Some(e) = self.estimate {
            out.push_str(&format!("\n  Estimate:  {}", e as i64));
        }
        if let Some(ref p) = self.parent {
            out.push_str(&format!("\n  Parent:    {}: {}", p.identifier, p.title));
        }
        if let Some(ref d) = self.description {
            let desc = truncate(d.trim(), 200);
            if !desc.is_empty() {
                out.push_str(&format!("\n\n  {desc}"));
            }
        }
        out.push_str(&format!(
            "\n\n  Created:   {}",
            short_date(&self.created_at)
        ));
        out.push_str(&format!(
            "\n  Updated:   {}",
            short_date(&self.updated_at)
        ));
        out.push_str(&format!("\n  URL:       {}", self.url));
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            self.identifier.clone(),
            truncate(&self.state.name, 15),
            self.priority_label.clone(),
            truncate(&self.title, 50),
            self.assignee
                .as_ref()
                .map_or("-".into(), |a| a.display_name.clone()),
        ]
    }
}

// -- Team --

impl PrettyPrint for Team {
    fn pretty(&self) -> String {
        let mut out = format!("{}: {}", self.key, self.name);
        if let Some(ref d) = self.description
            && !d.is_empty() {
                out.push_str(&format!("\n\n  {d}"));
            }
        out.push_str(&format!(
            "\n\n  Cycles:   {}",
            if self.cycles_enabled {
                "enabled"
            } else {
                "disabled"
            }
        ));
        out.push_str(&format!(
            "\n  Created:  {}",
            short_date(&self.created_at)
        ));
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            self.key.clone(),
            self.name.clone(),
            self.description
                .as_deref()
                .map_or(String::new(), |d| truncate(d, 40)),
        ]
    }
}

// -- Project --

impl PrettyPrint for Project {
    fn pretty(&self) -> String {
        let mut out = self.name.clone();
        out.push_str(&format!("\n\n  Status:    {}", self.status.name));
        out.push_str(&format!("\n  Priority:  {}", self.priority_label));
        out.push_str(&format!("\n  Progress:  {:.0}%", self.progress * 100.0));
        if let Some(ref l) = self.lead {
            out.push_str(&format!("\n  Lead:      {}", l.display_name));
        }
        if let Some(ref d) = self.start_date {
            out.push_str(&format!("\n  Start:     {d}"));
        }
        if let Some(ref d) = self.target_date {
            out.push_str(&format!("\n  Target:    {d}"));
        }
        if !self.description.is_empty() {
            out.push_str(&format!("\n\n  {}", truncate(&self.description, 200)));
        }
        out.push_str(&format!(
            "\n\n  Created:   {}",
            short_date(&self.created_at)
        ));
        out.push_str(&format!("\n  URL:       {}", self.url));
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            truncate(&self.name, 30),
            self.status.name.clone(),
            format!("{:.0}%", self.progress * 100.0),
            self.priority_label.clone(),
            self.lead
                .as_ref()
                .map_or("-".into(), |l| l.display_name.clone()),
        ]
    }
}

// -- User --

impl PrettyPrint for User {
    fn pretty(&self) -> String {
        let mut out = format!("{} <{}>", self.display_name, self.email);
        out.push_str(&format!("\n\n  Name:      {}", self.name));
        out.push_str(&format!(
            "\n  Active:    {}",
            if self.active { "yes" } else { "no" }
        ));
        out.push_str(&format!(
            "\n  Admin:     {}",
            if self.admin { "yes" } else { "no" }
        ));
        if self.guest {
            out.push_str("\n  Guest:     yes");
        }
        if let Some(ref d) = self.description
            && !d.is_empty() {
                out.push_str(&format!("\n  Bio:       {}", truncate(d, 80)));
            }
        if let Some(ref ls) = self.last_seen {
            out.push_str(&format!("\n  Last seen: {}", short_date(ls)));
        }
        out.push_str(&format!(
            "\n  Created:   {}",
            short_date(&self.created_at)
        ));
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            self.display_name.clone(),
            self.email.clone(),
            if self.active {
                "active".into()
            } else {
                "inactive".into()
            },
            if self.admin {
                "admin".into()
            } else {
                String::new()
            },
        ]
    }
}

// -- Comment --

impl PrettyPrint for Comment {
    fn pretty(&self) -> String {
        let issue_ref = self.issue.as_ref().map_or("(unknown)".into(), |i| {
            format!("{}: {}", i.identifier, i.title)
        });
        let author = self
            .user
            .as_ref()
            .map_or("unknown".into(), |u| u.display_name.clone());
        let mut out = format!("Comment on {issue_ref}");
        out.push_str(&format!("\n\n  Author:   {author}"));
        out.push_str(&format!(
            "\n  Created:  {}",
            short_date(&self.created_at)
        ));
        if self.resolved_at.is_some() {
            out.push_str("\n  Resolved: yes");
        }
        out.push_str(&format!("\n  URL:      {}", self.url));
        out.push_str(&format!("\n\n  {}", truncate(self.body.trim(), 500)));
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            self.issue
                .as_ref()
                .map_or("-".into(), |i| i.identifier.clone()),
            self.user
                .as_ref()
                .map_or("-".into(), |u| u.display_name.clone()),
            truncate(self.body.lines().next().unwrap_or(""), 60),
            short_date(&self.created_at).into(),
        ]
    }
}

// -- IssueLabel --

impl PrettyPrint for IssueLabel {
    fn pretty(&self) -> String {
        let mut out = format!("{} ({})", self.name, self.color);
        if let Some(ref d) = self.description
            && !d.is_empty() {
                out.push_str(&format!("\n\n  {d}"));
            }
        if let Some(ref t) = self.team {
            out.push_str(&format!("\n\n  Team:    {} ({})", t.name, t.key));
        }
        if self.is_group {
            out.push_str("\n  Group:   yes");
        }
        if let Some(ref p) = self.parent {
            out.push_str(&format!("\n  Parent:  {}", p.name));
        }
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.color.clone(),
            self.team.as_ref().map_or("-".into(), |t| t.key.clone()),
            if self.is_group {
                "group".into()
            } else {
                String::new()
            },
        ]
    }
}

// -- Cycle --

impl PrettyPrint for Cycle {
    fn pretty(&self) -> String {
        let name = self.name.as_deref().unwrap_or("");
        let header = if name.is_empty() {
            format!("Cycle #{}", self.number as i64)
        } else {
            format!("Cycle #{} - {name}", self.number as i64)
        };
        let mut out = header;
        out.push_str(&format!(
            "\n\n  Team:      {} ({})",
            self.team.name, self.team.key
        ));
        out.push_str(&format!(
            "\n  Period:    {} to {}",
            short_date(&self.starts_at),
            short_date(&self.ends_at)
        ));
        out.push_str(&format!("\n  Progress:  {:.0}%", self.progress * 100.0));
        let status = if self.is_active {
            "active"
        } else if self.is_next {
            "next"
        } else if self.is_previous {
            "previous"
        } else if self.is_past {
            "past"
        } else if self.is_future {
            "future"
        } else {
            "unknown"
        };
        out.push_str(&format!("\n  Status:    {status}"));
        if let Some(ref d) = self.description
            && !d.is_empty() {
                out.push_str(&format!("\n\n  {}", truncate(d, 200)));
            }
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        let name = self.name.clone().unwrap_or_default();
        let status = if self.is_active {
            "active"
        } else if self.is_next {
            "next"
        } else if self.is_previous {
            "previous"
        } else if self.is_past {
            "past"
        } else if self.is_future {
            "future"
        } else {
            ""
        };
        vec![
            format!("#{}", self.number as i64),
            truncate(&name, 20),
            format!(
                "{} - {}",
                short_date(&self.starts_at),
                short_date(&self.ends_at)
            ),
            status.into(),
            format!("{:.0}%", self.progress * 100.0),
            self.team.key.clone(),
        ]
    }
}

// -- WorkflowState --

impl PrettyPrint for WorkflowState {
    fn pretty(&self) -> String {
        let mut out = format!("{} ({}, {})", self.name, self.state_type, self.color);
        out.push_str(&format!(
            "\n\n  Team:      {} ({})",
            self.team.name, self.team.key
        ));
        out.push_str(&format!("\n  Position:  {}", self.position));
        if let Some(ref d) = self.description
            && !d.is_empty() {
                out.push_str(&format!("\n  Note:      {d}"));
            }
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.state_type.clone(),
            self.color.clone(),
            self.team.key.clone(),
        ]
    }
}

// -- Document --

impl PrettyPrint for Document {
    fn pretty(&self) -> String {
        let mut out = self.title.clone();
        if let Some(ref c) = self.creator {
            out.push_str(&format!("\n\n  Creator:  {}", c.display_name));
        }
        if let Some(ref p) = self.project {
            out.push_str(&format!("\n  Project:  {}", p.name));
        }
        if let Some(ref t) = self.team {
            out.push_str(&format!("\n  Team:     {} ({})", t.name, t.key));
        }
        out.push_str(&format!(
            "\n  Created:  {}",
            short_date(&self.created_at)
        ));
        out.push_str(&format!("\n  URL:      {}", self.url));
        if let Some(ref c) = self.content
            && !c.is_empty() {
                out.push_str(&format!("\n\n  {}", truncate(c.trim(), 500)));
            }
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            truncate(&self.title, 40),
            self.creator
                .as_ref()
                .map_or("-".into(), |c| c.display_name.clone()),
            self.project
                .as_ref()
                .map_or("-".into(), |p| truncate(&p.name, 20)),
            short_date(&self.created_at).into(),
        ]
    }
}

// -- Initiative --

impl PrettyPrint for Initiative {
    fn pretty(&self) -> String {
        let mut out = self.name.clone();
        out.push_str(&format!("\n\n  Status:   {}", self.status));
        if let Some(ref o) = self.owner {
            out.push_str(&format!("\n  Owner:    {}", o.display_name));
        }
        if let Some(ref d) = self.target_date {
            out.push_str(&format!("\n  Target:   {d}"));
        }
        out.push_str(&format!(
            "\n  Created:  {}",
            short_date(&self.created_at)
        ));
        out.push_str(&format!("\n  URL:      {}", self.url));
        if let Some(ref d) = self.description
            && !d.is_empty() {
                out.push_str(&format!("\n\n  {}", truncate(d.trim(), 200)));
            }
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            truncate(&self.name, 30),
            self.status.clone(),
            self.owner
                .as_ref()
                .map_or("-".into(), |o| o.display_name.clone()),
            self.target_date.as_deref().unwrap_or("-").into(),
        ]
    }
}

// -- ProjectMilestone --

impl PrettyPrint for ProjectMilestone {
    fn pretty(&self) -> String {
        let mut out = self.name.clone();
        out.push_str(&format!("\n\n  Status:    {}", self.status));
        out.push_str(&format!("\n  Progress:  {:.0}%", self.progress * 100.0));
        out.push_str(&format!("\n  Project:   {}", self.project.name));
        if let Some(ref d) = self.target_date {
            out.push_str(&format!("\n  Target:    {d}"));
        }
        out.push_str(&format!(
            "\n  Created:   {}",
            short_date(&self.created_at)
        ));
        if let Some(ref d) = self.description
            && !d.is_empty() {
                out.push_str(&format!("\n\n  {}", truncate(d.trim(), 200)));
            }
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            truncate(&self.name, 30),
            self.status.clone(),
            format!("{:.0}%", self.progress * 100.0),
            truncate(&self.project.name, 20),
            self.target_date.as_deref().unwrap_or("-").into(),
        ]
    }
}

// -- Attachment --

impl PrettyPrint for Attachment {
    fn pretty(&self) -> String {
        let mut out = self.title.clone();
        out.push_str(&format!("\n\n  URL:      {}", self.url));
        out.push_str(&format!(
            "\n  Issue:    {}: {}",
            self.issue.identifier, self.issue.title
        ));
        if let Some(ref s) = self.source_type {
            out.push_str(&format!("\n  Source:   {s}"));
        }
        if let Some(ref c) = self.creator {
            out.push_str(&format!("\n  Creator:  {}", c.display_name));
        }
        out.push_str(&format!(
            "\n  Created:  {}",
            short_date(&self.created_at)
        ));
        out
    }

    fn pretty_row(&self) -> Vec<String> {
        vec![
            truncate(&self.title, 30),
            self.issue.identifier.clone(),
            truncate(&self.url, 40),
            self.creator
                .as_ref()
                .map_or("-".into(), |c| c.display_name.clone()),
        ]
    }
}

use std::fmt::Write;
