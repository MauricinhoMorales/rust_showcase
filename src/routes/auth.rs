use crate::auth::{generate_token, verify_password};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use super::users::User;

#[derive(Clone, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    login_data: web::Json<LoginRequest>,
    db: web::Data<Surreal<Client>>,
) -> impl Responder {
    let query = r#"
        SELECT * FROM users
        WHERE username = $username;
    "#;

    let mut result = db
        .query(query)
        .bind(("username", login_data.username.clone()))
        .await
        .unwrap();

    let user: Option<User> = result.take(0).unwrap();

    match user {
        Some(user) => {
            if verify_password(&login_data.password, &user.password).unwrap() {
                let token = generate_token(&user.username);
                HttpResponse::Ok().json(LoginResponse { token })
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        None => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}
