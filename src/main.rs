use sea_orm::Database;

use crate::context::Context;

mod commands;
mod context;
mod entity;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let db = Database::connect(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .map_err(|e| miette::miette!("Failed to connect to database: {}", e))?;

    commands::invoke(Context::new(db)).await
}
