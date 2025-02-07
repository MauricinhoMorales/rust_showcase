use dotenv::dotenv;
use std::env;
use std::process::Stdio;
use tokio::process::Command;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn start_surrealdb() -> tokio::process::Child {
    Command::new("surreal")
        .arg("start")
        .arg("--user")
        .arg("root")
        .arg("--pass")
        .arg("root")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start SurrealDB server")
}

pub async fn setup_db() -> Surreal<Client> {
    let _surrealdb_process = start_surrealdb().await;
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    dotenv().ok();

    let host = env::var("SURREALDB_HOST").expect("SURREALDB_HOST not set");
    let port = env::var("SURREALDB_PORT").expect("SURREALDB_PORT not set");
    let user = env::var("SURREALDB_USER").expect("SURREALDB_USER not set");
    let password = env::var("SURREALDB_PASS").expect("SURREALDB_PASS not set");
    let ns_name = env::var("SURREALDB_NS").expect("SURREALDB_NS not set");
    let db_name = env::var("SURREALDB_DB").expect("SURREALDB_DB not set");

    let db_url = format!("{}:{}", host, port);

    let db = Surreal::new::<Ws>(db_url).await.unwrap();
    db.signin(Root {
        username: &user,
        password: &password,
    })
    .await
    .unwrap();
    db.use_ns(&ns_name).use_db(&db_name).await.unwrap();

    db
}
