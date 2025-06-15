use migration::MigratorTrait;
use sea_orm::Database;

use crate::context::Context;

mod commands;
mod context;
mod entity;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let db_path = if let Ok(path) = std::env::var("CLOG_DATABASE_URL") {
        path
    } else {
        let user_database_dir = dirs::data_local_dir()
            .ok_or_else(|| miette::miette!("Failed to get local data directory"))?
            .join(env!("CARGO_PKG_NAME"));
        std::fs::create_dir_all(&user_database_dir)
            .map_err(|e| miette::miette!("Failed to create user database directory: {}", e))?;
        let db_path = user_database_dir.join("db.sqlite");
        format!(
            "sqlite://{}?mode=rwc",
            db_path
                .to_str()
                .ok_or_else(|| miette::miette!("Invalid database path"))?
        )
    };

    let db = Database::connect(db_path)
        .await
        .map_err(|e| miette::miette!("Failed to connect to database: {}", e))?;

    migration::Migrator::up(&db, None)
        .await
        .map_err(|e| miette::miette!("Failed to run migrations: {}", e))?;

    commands::invoke(Context::new(db)).await
}
