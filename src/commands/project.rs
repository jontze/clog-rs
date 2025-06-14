use clap::{Parser, Subcommand};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, prelude::*};

use super::CommandExecutorTrait;
use crate::{Context, entity::projects};

#[derive(Subcommand)]
pub(super) enum ProjectCommand {
    /// Create a new project
    Create(CreateProjectCommand),
    /// Remove an existing project
    Remove(RemoveProjectCommand),
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

impl CommandExecutorTrait for ProjectCommand {
    async fn execute(&self, ctx: Context) -> miette::Result<()> {
        match self {
            ProjectCommand::Create(cmd) => {
                create(&ctx, &cmd.name, cmd.description.as_deref()).await
            }
            ProjectCommand::Remove(cmd) => remove(&ctx, &cmd.name).await,
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
