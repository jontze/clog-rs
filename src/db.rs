use std::path::PathBuf;

pub(crate) fn db_path() -> miette::Result<PathBuf> {
    let db_path = if let Ok(path) = std::env::var("CLOG_DATABASE_PATH") {
        PathBuf::from(path)
    } else {
        let user_database_dir = dirs::data_local_dir()
            .ok_or_else(|| miette::miette!("Failed to get local data directory"))?
            .join(env!("CARGO_PKG_NAME"));
        std::fs::create_dir_all(&user_database_dir)
            .map_err(|e| miette::miette!("Failed to create user database directory: {}", e))?;
        user_database_dir.join("db.sqlite")
    };
    Ok(db_path)
}
