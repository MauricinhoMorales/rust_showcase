use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::auth::hash_password;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<Thing>,
    pub username: String,
    pub password: String,
}

pub async fn register_user(
    db: web::Data<Surreal<Client>>,
    user: web::Json<User>,
) -> impl Responder {
    let password_hash = match hash_password(user.password.as_str()) {
        Ok(hash) => hash,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to hash password: {}", err));
        }
    };

    let mut user_data = user.into_inner();
    user_data.password = password_hash;

    match db.create::<Option<User>>("users").content(user_data).await {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to create user: {}", err))
        }
    }
}

pub async fn get_users(db: web::Data<Surreal<Client>>) -> impl Responder {
    let users: Vec<User> = db.select("users").await.unwrap();
    HttpResponse::Ok().json(users)
}
