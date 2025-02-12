use dotenv::dotenv;
use std::env;

use actix_web::{web, App, HttpServer};
use db::{setup_db, SurrealDbManager};

pub mod auth;
pub mod db;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_manager = SurrealDbManager::new();
    let db = setup_db(&db_manager).await;

    let api_port = env::var("API_PORT").expect("API_PORT not set");
    let api_bind = format!("127.0.0.1:{}", api_port);

    let _server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::init_routes)
    })
    .bind(&api_bind)?
    .run()
    .await;

    db_manager.stop_surrealdb().await;

    Ok(())
}
