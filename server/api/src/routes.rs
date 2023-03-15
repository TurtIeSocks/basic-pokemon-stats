use actix_web::{post, web, Error, HttpResponse};
use entity::Webhook;
use sea_orm::DatabaseConnection;

#[post("/")]
async fn process(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<Vec<Webhook>>,
) -> Result<HttpResponse, Error> {
    let payload = payload.into_inner();
    log::info!("Received Webhooks: {}", payload.len());

    entity::pokemon::Query::insert_many(&db, payload)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
