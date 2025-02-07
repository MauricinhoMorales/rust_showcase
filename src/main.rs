use actix_web::{web, App, HttpServer};
use db::setup_db;
use routes::{create_user, get_users};

pub mod models;
pub mod routes;
pub mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = setup_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}