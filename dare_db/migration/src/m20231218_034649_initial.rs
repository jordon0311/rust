use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // How to apply the migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Dare::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dare::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dare::Title).string().not_null())
                    .col(ColumnDef::new(Dare::Description).string().not_null())
                    .col(ColumnDef::new(Dare::Author).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // How to revert the migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Dare::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Dare {
    Table,
    Id,
    Title,
    Description,
    Author,
}
