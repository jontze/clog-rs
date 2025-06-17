use clap::{CommandFactory, Subcommand};
use clap_complete::{
    Generator, generate,
    shells::{self},
};

use crate::commands::{Cli, CommandExecutorTrait};

#[derive(Subcommand)]
pub(super) enum CompletionCommand {
    /// Generate Bash completion script
    Bash,
    /// Generate Zsh completion script
    Zsh,
    /// Generate PowerShell completion script
    #[clap(name = "powershell", alias = "ps")]
    PowerShell,
}

impl CommandExecutorTrait for CompletionCommand {
    async fn execute(
        &self,
        _ctx: crate::Context,
        _output_format: crate::commands::OutputFormat,
    ) -> miette::Result<()> {
        let mut command = Cli::command();
        match self {
            CompletionCommand::Bash => generate_completion(shells::Bash, &mut command),
            CompletionCommand::Zsh => generate_completion(shells::Zsh, &mut command),
            CompletionCommand::PowerShell => generate_completion(shells::PowerShell, &mut command),
        }
        Ok(())
    }
}

fn generate_completion<TShell>(shell: TShell, command: &mut clap::Command)
where
    TShell: Generator,
{
    generate(
        shell,
        command,
        env!("CARGO_PKG_NAME"),
        &mut std::io::stdout(),
    );
}
