use crate::Context;

pub(super) trait CommandExecutorTrait {
    async fn execute(&self, ctx: Context) -> miette::Result<()>;
}
