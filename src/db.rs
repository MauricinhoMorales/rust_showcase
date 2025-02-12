use dotenv::dotenv;
use std::env;
use std::fs;
use std::process::Stdio;
use std::sync::Arc;
use surrealdb::opt::auth::Root;
use surrealdb::{engine::remote::ws::Client, engine::remote::ws::Ws, Surreal};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct SurrealDbManager {
    process: Arc<Mutex<Option<Child>>>,
}

impl SurrealDbManager {
    pub fn new() -> Self {
        Self {
            process: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start_surrealdb(&self) {
        let mut process_guard = self.process.lock().await;

        if process_guard.is_none() {
            let process = Command::new("surreal")
                .arg("start")
                .arg("--user")
                .arg("root")
                .arg("--pass")
                .arg("root")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start SurrealDB server");

            *process_guard = Some(process);

            println!("Waiting for SurrealDB to be ready...");
            if Self::wait_for_db().await.is_err() {
                eprintln!("SurrealDB did not start in time.");
            }
        }
    }

    pub async fn stop_surrealdb(&self) {
        let mut process_guard = self.process.lock().await;
        if let Some(mut process) = process_guard.take() {
            let _ = process.kill().await;
        }
    }

    async fn wait_for_db() -> Result<(), ()> {
        let db_url = format!(
            "{}:{}",
            env::var("SURREALDB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            env::var("SURREALDB_PORT").unwrap_or_else(|_| "8000".to_string())
        );

        let mut attempts = 0;
        while attempts < 10 {
            if let Ok(_db) = Surreal::new::<Ws>(&db_url).await {
                return Ok(());
            }
            sleep(Duration::from_millis(500)).await;
            attempts += 1;
        }
        Err(())
    }
}

async fn run_schema_file(db: &Surreal<Client>, file_path: &str) -> Result<(), surrealdb::Error> {
    let schema = fs::read_to_string(file_path).expect("Failed to read schema file");
    db.query(&schema).await?;
    Ok(())
}

pub async fn setup_db(db_manager: &SurrealDbManager) -> Surreal<Client> {
    db_manager.start_surrealdb().await;

    dotenv().ok();

    let host = env::var("SURREALDB_HOST").expect("SURREALDB_HOST not set");
    let port = env::var("SURREALDB_PORT").expect("SURREALDB_PORT not set");
    let user = env::var("SURREALDB_USER").expect("SURREALDB_USER not set");
    let password = env::var("SURREALDB_PASS").expect("SURREALDB_PASS not set");
    let ns_name = env::var("SURREALDB_NS").expect("SURREALDB_NS not set");
    let db_name = env::var("SURREALDB_DB").expect("SURREALDB_DB not set");

    let db_url = format!("{}:{}", host, port);

    let db = Surreal::new::<Ws>(db_url)
        .await
        .expect("Failed to connect to SurrealDB");

    db.signin(Root {
        username: &user,
        password: &password,
    })
    .await
    .expect("Failed to sign in");

    db.use_ns(&ns_name)
        .use_db(&db_name)
        .await
        .expect("Failed to use namespace and database");

    if let Err(err) = run_schema_file(&db, "src/db/schema.surql").await {
        eprintln!("Error running schema file: {}", err);
    } else {
        println!("Schema file executed successfully!");
    }

    db
}
