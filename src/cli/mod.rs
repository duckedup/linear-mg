pub mod attachments;
pub mod auth;
pub mod comments;
pub mod common;
pub mod cycles;
pub mod documents;
pub mod initiatives;
pub mod issues;
pub mod labels;
pub mod milestones;
pub mod projects;
pub mod teams;
pub mod users;
pub mod workflow_states;

use clap::{Parser, Subcommand};
use common::GlobalArgs;

#[derive(Parser, Debug)]
#[command(name = "linear-mg", version, about = "CLI for the Linear API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub global: GlobalArgs,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage authentication
    Auth(auth::AuthCommand),
    /// Manage issues
    Issues(issues::IssuesCommand),
    /// Manage teams
    Teams(teams::TeamsCommand),
    /// Manage projects
    Projects(projects::ProjectsCommand),
    /// Manage users
    Users(users::UsersCommand),
    /// Manage comments
    Comments(comments::CommentsCommand),
    /// Manage issue labels
    Labels(labels::LabelsCommand),
    /// Manage cycles
    Cycles(cycles::CyclesCommand),
    /// Manage workflow states
    #[command(name = "states")]
    WorkflowStates(workflow_states::WorkflowStatesCommand),
    /// Manage documents
    Documents(documents::DocumentsCommand),
    /// Manage initiatives
    Initiatives(initiatives::InitiativesCommand),
    /// Manage project milestones
    Milestones(milestones::MilestonesCommand),
    /// Manage attachments
    Attachments(attachments::AttachmentsCommand),
}
