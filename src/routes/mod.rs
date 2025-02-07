use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::models::User;


pub async fn create_user(user: web::Json<User>, db: web::Data<Surreal<Client>>) -> impl Responder {
    let created: Option<User> = db
        .create("user")
        .content(user.into_inner())
        .await
        .unwrap();

    HttpResponse::Ok().json(created)
}

pub async fn get_users(db: web::Data<Surreal<Client>>) -> impl Responder {
    let users: Vec<User> = db.select("user").await.unwrap();
    HttpResponse::Ok().json(users)
}
