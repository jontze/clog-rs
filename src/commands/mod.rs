use clap::{Parser, Subcommand};

use crate::{
    Context,
    commands::{
        command_executor::CommandExecutorTrait, project::ProjectCommand, task::TaskCommand,
    },
};

mod command_executor;
mod command_output;
mod project;
mod report;
mod task;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Interaction with the project management system
    #[clap(subcommand)]
    Project(ProjectCommand),
    /// Interaction with tasks within a project
    #[clap(subcommand)]
    Task(TaskCommand),
    /// Interaction with project reports
    Report,
}

impl CommandExecutorTrait for Command {
    async fn execute(&self, ctx: Context) -> miette::Result<()> {
        match self {
            Command::Project(cmd) => cmd.execute(ctx).await,
            Command::Task(cmd) => cmd.execute(ctx).await,
            Command::Report => report::execute(ctx).await,
        }
    }
}

pub(crate) async fn invoke(ctx: Context) -> miette::Result<()> {
    let cli = Cli::parse();

    cli.command.execute(ctx).await
}
