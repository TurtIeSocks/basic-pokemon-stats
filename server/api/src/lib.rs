use actix_web::{middleware, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

mod routes;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("[INIT] DATABASE_URL not found");

    let connection = match Database::connect(&database_url).await {
        Ok(db) => db,
        Err(err) => panic!("[API] Cannot connect to Scanner DB: {}", err),
    };

    match Migrator::up(&connection, None).await {
        Ok(_) => log::info!("[API] Migrations successful"),
        Err(err) => log::error!("[API] Migration Error {:?}", err),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connection.clone()))
            .app_data(web::JsonConfig::default().limit(20_971_520))
            .wrap(middleware::Logger::new("%s | %r - %b bytes in %D ms (%a)"))
            .service(web::scope("/api").service(routes::process))
    })
    .bind((
        std::env::var("HOST").unwrap_or("0.0.0.0".to_string()),
        std::env::var("PORT")
            .unwrap_or("6000".to_string())
            .parse::<u16>()
            .unwrap(),
    ))?
    .run()
    .await
}
