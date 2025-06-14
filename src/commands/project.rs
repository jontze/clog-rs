use clap::{Parser, Subcommand};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, TryIntoModel, prelude::*};
use serde::Serialize;
use tabled::Tabled;

use super::CommandExecutorTrait;
use crate::{
    Context,
    commands::{
        OutputFormat,
        command_output::{CommandOutput, NoTable},
    },
    entity::projects,
};

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
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()> {
        match self {
            ProjectCommand::Create(cmd) => {
                create(&ctx, &cmd.name, cmd.description.as_deref(), output_format).await
            }
            ProjectCommand::Remove(cmd) => remove(&ctx, &cmd.name, output_format).await,
            ProjectCommand::List(_) => list(&ctx, output_format).await,
        }
    }
}

async fn create(
    ctx: &Context,
    name: &str,
    description: Option<&str>,
    output_format: OutputFormat,
) -> miette::Result<()> {
    let created_project = projects::ActiveModel {
        name: Set(name.to_string()),
        description: Set(description.map(|d| d.to_string())),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .map_err(|e| miette::miette!("Failed to create project: {}", e))?
    .try_into_model()
    .map_err(|e| miette::miette!("Failed to convert active model to project model: {}", e))?;

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(output_format)
        .with_prefix_message(format!(
            "Project '{name}' created successfully",
            name = created_project.name
        ))
        .build()
        .print();
    Ok(())
}

async fn remove(ctx: &Context, name: &str, output_format: OutputFormat) -> miette::Result<()> {
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

    CommandOutput::<Vec<NoTable>, NoTable>::builder()
        .with_mode(output_format)
        .with_prefix_message("Project was removed successfully".to_string())
        .build()
        .print();
    Ok(())
}

async fn list(ctx: &Context, output_format: OutputFormat) -> miette::Result<()> {
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

    CommandOutput::<Vec<ProjectTable>, ProjectTable>::builder()
        .with_table_rows(projects_table)
        .with_mode(output_format)
        .with_prefix_message("All Projects in the database".to_string())
        .build()
        .print();

    Ok(())
}

#[derive(Tabled, Serialize, Clone)]
struct ProjectTable {
    id: i32,
    name: String,
    description: String,
    tasks: u64,
}
