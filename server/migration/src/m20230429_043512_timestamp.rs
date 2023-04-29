use sea_orm_migration::prelude::*;

use crate::m20230315_001452_initial::Pokemon;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Pokemon::Table)
                    .add_column(
                        ColumnDef::new(Pokemon::FirstSeen)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .add_column(
                        ColumnDef::new(Pokemon::LastSeen)
                            .timestamp()
                            .not_null()
                            .extra(
                                "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_string(),
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Pokemon::Table)
                    .drop_column(Pokemon::FirstSeen)
                    .drop_column(Pokemon::LastSeen)
                    .to_owned(),
            )
            .await
    }
}
