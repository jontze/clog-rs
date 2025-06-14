use clap::{Parser, Subcommand, ValueEnum};

use crate::{
    Context,
    commands::{
        command_executor::CommandExecutorTrait, project::ProjectCommand, report::ReportCommand,
        task::TaskCommand,
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
}

impl CommandExecutorTrait for Command {
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()> {
        match self {
            Command::Project(cmd) => cmd.execute(ctx, output_format).await,
            Command::Task(cmd) => cmd.execute(ctx, output_format).await,
            Command::Report(cmd) => cmd.execute(ctx, output_format).await,
        }
    }
}

pub(crate) async fn invoke(ctx: Context) -> miette::Result<()> {
    let cli = Cli::parse();

    cli.command.execute(ctx, cli.output).await
}
