use clap::{Parser, Subcommand};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, prelude::*};
use serde::Deserialize;
use tabled::Tabled;

use super::CommandExecutorTrait;
use crate::{Context, commands::command_output::output, entity::projects};

#[derive(Subcommand)]
pub(super) enum ProjectCommand {
    /// Create a new project
    Create(CreateProjectCommand),
    /// Remove an existing project
    Remove(RemoveProjectCommand),
    /// List all projects
    List(ListProjectsCommand),
}

#[derive(Parser)]
pub(super) struct CreateProjectCommand {
    /// Name of the project to create
    #[clap(short, long)]
    name: String,
    /// Description of the project
    #[clap(short, long)]
    description: Option<String>,
}

#[derive(Parser)]
pub(super) struct RemoveProjectCommand {
    /// Name of the project to remove
    #[clap(short, long)]
    name: String,
}

#[derive(Parser)]
pub(super) struct ListProjectsCommand;

impl CommandExecutorTrait for ProjectCommand {
    async fn execute(&self, ctx: Context) -> miette::Result<()> {
        match self {
            ProjectCommand::Create(cmd) => {
                create(&ctx, &cmd.name, cmd.description.as_deref()).await
            }
            ProjectCommand::Remove(cmd) => remove(&ctx, &cmd.name).await,
            ProjectCommand::List(_) => list(&ctx).await,
        }
    }
}

async fn create(ctx: &Context, name: &str, description: Option<&str>) -> miette::Result<()> {
    projects::ActiveModel {
        name: Set(name.to_string()),
        description: Set(description.map(|d| d.to_string())),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .map_err(|e| miette::miette!("Failed to create project: {}", e))?;
    Ok(())
}

async fn remove(ctx: &Context, name: &str) -> miette::Result<()> {
    let project = projects::Entity::find()
        .filter(projects::Column::Name.eq(name))
        .one(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to find project: {}", e))?
        .ok_or_else(|| miette::miette!("Project not found"))?;
    project
        .delete(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to remove project: {}", e))?;
    Ok(())
}

async fn list(ctx: &Context) -> miette::Result<()> {
    let projects = projects::Entity::find()
        .all(&ctx.db)
        .await
        .map_err(|e| miette::miette!("Failed to list projects: {}", e))?;

    let mut projects_table: Vec<ProjectTable> = vec![];
    for project in projects {
        let amount_of_tasks = project
            .find_related(crate::entity::tasks::Entity)
            .count(&ctx.db)
            .await
            .map_err(|e| miette::miette!("Failed to count tasks: {}", e))?;
        projects_table.push(ProjectTable {
            id: project.id,
            name: project.name,
            description: project.description.unwrap_or_default(),
            tasks: amount_of_tasks,
        });
    }

    output(projects_table);
    Ok(())
}

#[derive(Tabled, Deserialize)]
struct ProjectTable {
    id: i32,
    name: String,
    description: String,
    tasks: u64,
}
