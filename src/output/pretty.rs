use super::PrettyPrint;
use crate::graphql::attachments::Attachment;
use crate::graphql::comments::{Comment, CommentThreads};
use crate::graphql::common::{ListResponse, MutationResponse};
use crate::graphql::cycles::Cycle;
use crate::graphql::documents::Document;
use crate::graphql::initiatives::Initiative;
use crate::graphql::issues::Issue;
use crate::graphql::labels::IssueLabel;
use crate::graphql::milestones::ProjectMilestone;
use crate::graphql::projects::Project;
use crate::graphql::relations::{IssueRelation, IssueRelationList};
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
    if s.len() >= 10 { &s[..10] } else { s }
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
        if let Some(ref m) = self.project_milestone {
            match m.target_date {
                Some(ref d) => out.push_str(&format!("\n  Milestone: {} (due {d})", m.name)),
                None => out.push_str(&format!("\n  Milestone: {}", m.name)),
            }
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
        for rel in &self.relations.nodes {
            let label = match rel.relation_type.as_str() {
                "blocks" => "Blocking",
                "duplicate" => "Duplicate of",
                "similar" => "Similar to",
                _ => "Related to",
            };
            out.push_str(&format!(
                "\n  {label}: {}: {}",
                rel.related_issue.identifier, rel.related_issue.title
            ));
        }
        for rel in &self.inverse_relations.nodes {
            let label = match rel.relation_type.as_str() {
                "blocks" => "Blocked by",
                "duplicate" => "Duplicated by",
                "similar" => "Similar to",
                _ => "Related to",
            };
            out.push_str(&format!(
                "\n  {label}: {}: {}",
                rel.issue.identifier, rel.issue.title
            ));
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
        out.push_str(&format!("\n  Updated:   {}", short_date(&self.updated_at)));
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
            && !d.is_empty()
        {
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
        out.push_str(&format!("\n  Created:  {}", short_date(&self.created_at)));
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
            self.target_date.as_deref().unwrap_or("-").into(),
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
            && !d.is_empty()
        {
            out.push_str(&format!("\n  Bio:       {}", truncate(d, 80)));
        }
        if let Some(ref ls) = self.last_seen {
            out.push_str(&format!("\n  Last seen: {}", short_date(ls)));
        }
        out.push_str(&format!("\n  Created:   {}", short_date(&self.created_at)));
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

fn comment_author(c: &Comment) -> String {
    c.user
        .as_ref()
        .map_or("unknown".into(), |u| u.display_name.clone())
}

/// A single indented reply line: `↳ Bob (2024-06-01): Actually, one concern..`
fn reply_line(c: &Comment) -> String {
    format!(
        "    ↳ {} ({}): {}",
        comment_author(c),
        short_date(&c.created_at),
        truncate(c.body.lines().next().unwrap_or("").trim(), 70)
    )
}

/// Render a comment's replies: a `Replies (N):` header, one line per fetched
/// reply, and — when the thread has more replies than were pulled inline — a
/// hint pointing at the command that pages through the rest. Empty when there
/// are no replies.
fn replies_section(c: &Comment) -> String {
    if c.children.nodes.is_empty() {
        return String::new();
    }
    let mut out = format!("\n  Replies ({}):", c.children.nodes.len());
    for reply in &c.children.nodes {
        out.push_str(&format!("\n{}", reply_line(reply)));
    }
    if c.children.has_more() {
        out.push_str(&format!(
            "\n    … more replies — comments list --parent {}",
            c.id
        ));
    }
    out
}

impl PrettyPrint for Comment {
    fn pretty(&self) -> String {
        let issue_ref = self.issue.as_ref().map_or("(unknown)".into(), |i| {
            format!("{}: {}", i.identifier, i.title)
        });
        let mut out = format!("Comment on {issue_ref}");
        out.push_str(&format!("\n\n  Author:   {}", comment_author(self)));
        out.push_str(&format!("\n  Created:  {}", short_date(&self.created_at)));
        if self.resolved_at.is_some() {
            out.push_str("\n  Resolved: yes");
        }
        // Flag when this comment is itself a reply, so a `get <reply-id>` makes
        // its place in the thread obvious.
        if let Some(ref p) = self.parent {
            out.push_str(&format!(
                "\n  In reply to: {}",
                truncate(p.body.lines().next().unwrap_or("").trim(), 60)
            ));
        }
        out.push_str(&format!("\n  URL:      {}", self.url));
        out.push_str(&format!("\n\n  {}", truncate(self.body.trim(), 500)));
        if !self.children.nodes.is_empty() {
            out.push('\n');
            out.push_str(&replies_section(self));
        }
        out
    }
}

// -- CommentThreads (list) --

/// Compact thread block used by `comments list`: a root comment header, its body
/// first line, and any replies indented beneath it.
fn thread_block(c: &Comment) -> String {
    let issue_ref = c
        .issue
        .as_ref()
        .map_or("(unknown)".into(), |i| i.identifier.clone());
    let mut out = format!(
        "Comment on {} · {} · {}",
        issue_ref,
        comment_author(c),
        short_date(&c.created_at)
    );
    out.push_str(&format!(
        "\n  {}",
        truncate(c.body.lines().next().unwrap_or("").trim(), 100)
    ));
    out.push_str(&replies_section(c));
    out
}

impl PrettyPrint for CommentThreads {
    fn pretty(&self) -> String {
        let list = &self.0;
        if list.nodes.is_empty() {
            return "No results.".into();
        }
        let mut out = list
            .nodes
            .iter()
            .map(thread_block)
            .collect::<Vec<_>>()
            .join("\n\n");
        let n = list.nodes.len();
        let total: usize = n + list
            .nodes
            .iter()
            .map(|c| c.children.nodes.len())
            .sum::<usize>();
        if list.page_info.has_next_page {
            out.push_str(&format!(
                "\n\n{n} threads ({total} comments, more available)"
            ));
        } else {
            out.push_str(&format!("\n\n{n} threads ({total} comments)"));
        }
        out
    }
}

// -- IssueLabel --

impl PrettyPrint for IssueLabel {
    fn pretty(&self) -> String {
        let mut out = format!("{} ({})", self.name, self.color);
        if let Some(ref d) = self.description
            && !d.is_empty()
        {
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
            && !d.is_empty()
        {
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
            && !d.is_empty()
        {
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
        out.push_str(&format!("\n  Created:  {}", short_date(&self.created_at)));
        out.push_str(&format!("\n  URL:      {}", self.url));
        if let Some(ref c) = self.content
            && !c.is_empty()
        {
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
        out.push_str(&format!("\n  Created:  {}", short_date(&self.created_at)));
        out.push_str(&format!("\n  URL:      {}", self.url));
        if let Some(ref d) = self.description
            && !d.is_empty()
        {
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
        out.push_str(&format!("\n  Created:   {}", short_date(&self.created_at)));
        if let Some(ref d) = self.description
            && !d.is_empty()
        {
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
        out.push_str(&format!("\n  Created:  {}", short_date(&self.created_at)));
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

// -- IssueRelation --

impl PrettyPrint for IssueRelation {
    fn pretty(&self) -> String {
        let mut out = format!(
            "{} {} {}",
            self.issue.identifier, self.relation_type, self.related_issue.identifier
        );
        out.push_str(&format!(
            "\n\n  {}: {}",
            self.issue.identifier, self.issue.title
        ));
        out.push_str(&format!(
            "\n  {}: {}",
            self.related_issue.identifier, self.related_issue.title
        ));
        out.push_str(&format!("\n  Type: {}", self.relation_type));
        out.push_str(&format!("\n  ID:   {}", self.id));
        out
    }
}

// -- IssueRelationList --

impl PrettyPrint for IssueRelationList {
    fn pretty(&self) -> String {
        if self.nodes.is_empty() {
            return "No relations.".into();
        }
        let rows: Vec<Vec<String>> = self
            .nodes
            .iter()
            .map(|r| {
                vec![
                    r.direction.clone(),
                    r.other.identifier.clone(),
                    truncate(&r.other.title, 50),
                    r.id.clone(),
                ]
            })
            .collect();
        let mut out = align_columns(&rows);
        let n = self.nodes.len();
        write!(out, "\n{n} relations").unwrap();
        out
    }
}

use std::fmt::Write;
