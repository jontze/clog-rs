pub(crate) struct Context {
    pub(crate) db: sea_orm::DatabaseConnection,
}

impl Context {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }
}
