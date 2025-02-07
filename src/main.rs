use dotenv::dotenv;
use std::env;

use actix_web::{web, App, HttpServer};
use db::setup_db;
use routes::{create_user, get_users};

pub mod db;
pub mod models;
pub mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = setup_db().await;

    let api_port = env::var("API_PORT").expect("API_PORT not set");
    let api_bind = format!("127.0.0.1:{}", api_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
    })
    .bind(&api_bind)?
    .run()
    .await
}
