use migration::MigratorTrait;
use sea_orm::Database;

use crate::context::Context;

mod commands;
mod context;
mod db;
mod entity;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let db_path =
        db::db_path().map_err(|e| miette::miette!("Failed to get database path: {}", e))?;
    let connection_string = format!(
        "sqlite://{}?mode=rwc",
        db_path
            .to_str()
            .ok_or_else(|| miette::miette!("Invalid database path: {}", db_path.display()))?
    );
    let db = Database::connect(connection_string)
        .await
        .map_err(|e| miette::miette!("Failed to connect to database: {}", e))?;

    migration::Migrator::up(&db, None)
        .await
        .map_err(|e| miette::miette!("Failed to run migrations: {}", e))?;

    commands::invoke(Context::new(db)).await
}
