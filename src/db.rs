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

    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    db
}