#[macro_use] extern crate log;
use std::path::PathBuf;
use sqlx::{Connection, ConnectOptions};
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env("LOG_LEVEL").init();

    let dir = get_config_dir();
    debug!("Using directory: {:?}", &dir);

    let _pool = get_db_conn(&dir).await?;

    Ok(())
}

fn get_config_dir() -> PathBuf {
    match dirs::home_dir() {    // IMPORTANT! This directory should be changed to config dir later!
        Some(d) => d.join("oasis"),
        None => std::env::current_dir()
            .expect("Cannot get the config directory or the current working directory"),
    }
}

async fn get_db_conn(dir: &PathBuf) -> anyhow::Result<SqlitePool>{
    let db_file = dir.join("main.db");
    if !db_file.as_path().exists() {
        create_database(&db_file).await?;
    }

    Ok(get_conn_pool(&db_file).await?)
}

async fn create_database(db_file: &PathBuf) -> anyhow::Result<()> {
    let prefix = db_file.parent().expect("Cannot retrieve the parent directory");
    tokio::fs::create_dir_all(&prefix).await?;
    
    let mut conn = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .log_statements(log::LevelFilter::Trace)
        .connect()
        .await?;

    let sql = tokio::fs::read_to_string("./assets/sql/init.sql").await?;
    sqlx::query(&sql).execute(&mut conn).await?;
    let version = get_version()?;
    sqlx::query("UPDATE site SET version = ?")
        .bind(version)
        .execute(&mut conn)
        .await?;

    debug!("Database created at {:?}", db_file);
    conn.close();

    Ok(())
}

async fn get_conn_pool(db_file: &PathBuf) -> anyhow::Result<SqlitePool> {
    let db_file_str = db_file.to_str().expect("Cannot parse database filename");
    let mut option = SqliteConnectOptions::new()
        .filename(db_file_str);
    option.log_statements(log::LevelFilter::Trace);
    let pool = SqlitePool::connect_with(option).await?;

    Ok(pool)
}

fn get_version() -> anyhow::Result<f64> {
    let version: f64 = match std::env::var("VERSION")?.parse() {
        Ok(v) => v,
        _ => 0.1,
    };

    Ok(version)
}