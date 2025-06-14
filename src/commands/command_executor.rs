use crate::{Context, commands::OutputFormat};

pub(super) trait CommandExecutorTrait {
    async fn execute(&self, ctx: Context, output_format: OutputFormat) -> miette::Result<()>;
}
