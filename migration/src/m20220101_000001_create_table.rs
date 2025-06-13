use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create projects table
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(pk_auto(Projects::Id))
                    .col(string(Projects::Name).not_null().string_len(255))
                    .col(text(Projects::Description))
                    .col(
                        timestamp_with_time_zone(Projects::CreatedAt)
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        timestamp_with_time_zone(Projects::UpdatedAt)
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await?;

        // Create tasks table
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(pk_auto(Tasks::Id))
                    .col(integer(Tasks::ProjectId).not_null())
                    .col(string(Tasks::Name).not_null().string_len(255))
                    .col(text(Tasks::Description))
                    .col(string(Tasks::Status).not_null().string_len(32))
                    .col(
                        timestamp_with_time_zone(Tasks::CreatedAt)
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        timestamp_with_time_zone(Tasks::UpdatedAt)
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tasks::Table, Tasks::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create time_entries table
        manager
            .create_table(
                Table::create()
                    .table(TimeEntries::Table)
                    .if_not_exists()
                    .col(pk_auto(TimeEntries::Id))
                    .col(integer(TimeEntries::TaskId).not_null())
                    .col(timestamp_with_time_zone(TimeEntries::StartTime).not_null())
                    .col(timestamp_with_time_zone(TimeEntries::EndTime).not_null())
                    .col(integer(TimeEntries::Duration).not_null()) // duration in seconds
                    .col(
                        timestamp_with_time_zone(TimeEntries::CreatedAt)
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        timestamp_with_time_zone(TimeEntries::UpdatedAt)
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TimeEntries::Table, TimeEntries::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(TimeEntries::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tasks {
    Table,
    Id,
    ProjectId,
    Name,
    Description,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TimeEntries {
    Table,
    Id,
    TaskId,
    StartTime,
    EndTime,
    Duration,
    CreatedAt,
    UpdatedAt,
}
