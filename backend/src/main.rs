#[macro_use] extern crate log;
use std::path::PathBuf;
use sqlx::sqlite::SqlitePool;
use sqlx::ConnectOptions;

#[tokio::main]
async fn main() {
    env_logger::init();

    let dir = get_config_dir();
    info!("Using directory: {:?}", &dir);

    get_db_conn(&dir).await;
}

fn get_config_dir() -> PathBuf {
    match dirs::home_dir() {    // IMPORTANT! This directory should be changed to config dir later!
        Some(d) => d.join("oasis"),
        None => std::env::current_dir()
            .expect("Cannot get the config directory or the current working directory"),
    }
}

async fn get_db_conn(dir: &PathBuf) {
    let db_file = dir.join("main.db");
    if !db_file.as_path().exists() {
        create_database(&db_file).await;
    }
}

async fn create_database(db_file: &PathBuf) -> anyhow::Result<SqlitePool> {
    let prefix = db_file.parent().expect("Cannot retrieve the parent directory");
    tokio::fs::create_dir_all(&prefix).await?;
    /*
    let db = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .connect()
        .await?;*/
    
    tokio::fs::File::create(db_file).await?;

    let pool = get_conn_pool(db_file).await?;
    sqlx::migrate!("assets/migration")
    .run(&pool)
    .await?;
    
    Ok(pool)
}

async fn get_conn_pool(db_file: &PathBuf) -> Result<SqlitePool, sqlx::Error> {
    let db_file_str = db_file.to_str().expect("Cannot parse database filename");
    let pool = SqlitePool::connect(db_file_str).await?;

    Ok(pool)
}