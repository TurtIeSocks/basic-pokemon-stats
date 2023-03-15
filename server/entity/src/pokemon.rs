use sea_orm::{entity::prelude::*, sea_query::OnConflict, InsertResult, Set};

use crate::Webhook;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pokemon")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub pokemon_id: u32,
    pub form_id: u32,
    #[sea_orm(column_type = "Double")]
    pub lat: f64,
    #[sea_orm(column_type = "Double")]
    pub lon: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct Query;

impl Query {
    pub async fn insert_many(
        db: &DatabaseConnection,
        webhooks: Vec<Webhook>,
    ) -> Result<InsertResult<ActiveModel>, DbErr> {
        Entity::insert_many(webhooks.into_iter().map(|webhook| ActiveModel {
            id: Set(webhook.message.encounter_id),
            pokemon_id: Set(webhook.message.pokemon_id),
            form_id: Set(webhook.message.form),
            lat: Set(webhook.message.latitude),
            lon: Set(webhook.message.longitude),
        }))
        .on_conflict(
            OnConflict::column(Column::Id)
                .update_column(Column::PokemonId)
                .update_column(Column::FormId)
                .update_column(Column::Lat)
                .update_column(Column::Lon)
                .to_owned(),
        )
        .exec(db)
        .await
    }
}
