use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::login;
use users::{get_users, register_user};

use crate::auth::validator;

pub mod auth;
pub mod users;

async fn protected(user_id: web::ReqData<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Authorized {}!", user_id.into_inner()))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/api")
            .route("/users", web::post().to(register_user))
            .route("/users", web::get().to(get_users))
            .route("/login", web::post().to(login))
            .service(
                web::scope("/protected")
                    .wrap(auth)
                    .route("", web::get().to(protected)),
            ),
    );
}
