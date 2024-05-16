use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let tables = vec![
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(User::Name).string().not_null())
                .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone())
                .to_owned(),

            Table::create()
                .table(Manuscript::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Manuscript::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(Manuscript::UserId)
                        .integer()
                        .not_null()
                )
                .foreign_key(
                    ForeignKey::create()
                        // Unique key name is UUID
                        .name("FK_9E471309-FE3D-4383-8D61-77EBDA6D29BC")
                        .from(Manuscript::Table, Manuscript::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .col(ColumnDef::new(Manuscript::Title).string().not_null())
                .col(ColumnDef::new(Manuscript::Body).string().not_null())
                .col(ColumnDef::new(Manuscript::CreatedAt).timestamp_with_time_zone())
                .col(ColumnDef::new(Manuscript::UpdatedAt).timestamp_with_time_zone())
                .to_owned(),
        ];

        for table in tables {
            manager.create_table(table).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut tables = vec![
            Table::drop().table(User::Table).to_owned(),
            Table::drop().table(Manuscript::Table).to_owned(),
        ];
        tables.reverse();

        for table in tables {
            manager.drop_table(table).await?;
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Manuscript {
    Table,
    Id,
    Title,
    Body,
    UserId,
    CreatedAt,
    UpdatedAt,
}
