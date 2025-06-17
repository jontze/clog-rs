use clap::{Parser, Subcommand, ValueEnum};

use crate::{
    Context,
    commands::{
        command_executor::CommandExecutorTrait, completion::CompletionCommand,
        project::ProjectCommand, report::ReportCommand, sync::SyncCommand, task::TaskCommand,
    },
};

mod command_executor;
mod command_output;
mod completion;
mod project;
mod report;
mod sync;
mod task;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "human")]
    /// Set the output format (e.g., json, yaml, table, human)
    output: OutputFormat,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
    Toml,
    Human,
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
    #[clap(subcommand)]
    Report(ReportCommand),
    /// Interactions with the remote storage
    #[clap(subcommand)]
    Sync(SyncCommand),
    /// Setup Shell Completion
    #[clap(subcommand)]
    Completion(CompletionCommand),
}

impl CommandExecutorTrait for Command {
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()> {
        match self {
            Command::Project(cmd) => cmd.execute(ctx, output_format).await,
            Command::Task(cmd) => cmd.execute(ctx, output_format).await,
            Command::Report(cmd) => cmd.execute(ctx, output_format).await,
            Command::Sync(cmd) => cmd.execute(ctx, output_format).await,
            Command::Completion(cmd) => cmd.execute(ctx, output_format).await,
        }
    }
}

pub(crate) async fn invoke(ctx: Context) -> miette::Result<()> {
    let cli = Cli::parse();

    cli.command.execute(ctx, cli.output).await
}
