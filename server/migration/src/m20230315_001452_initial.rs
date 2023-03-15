use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pokemon::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pokemon::Id)
                            .string_len(25)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Pokemon::PokemonId)
                            .small_integer()
                            .unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Pokemon::FormId)
                            .small_integer()
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Pokemon::Lat).double().not_null())
                    .col(ColumnDef::new(Pokemon::Lon).double().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Pokemon::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Pokemon {
    Table,
    Id,
    PokemonId,
    FormId,
    Lat,
    Lon,
}
