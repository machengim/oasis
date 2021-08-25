use crate::util::env;
use anyhow::Result;
use async_std::fs;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::{ConnectOptions, Connection};
use std::path::{Path, PathBuf};

pub fn check_app_installed() -> bool {
    get_db_file().exists()
}

pub async fn init_app_db() -> Result<()> {
    let db_dir = get_db_dir();
    if !db_dir.exists() {
        fs::create_dir(&db_dir).await?
    }

    let db_file = get_db_file();
    run_migrations(&db_file).await?;

    Ok(())
}

pub async fn get_db_pool() -> Result<SqlitePool> {
    let db_file = get_db_file();
    let option = SqliteConnectOptions::new().filename(db_file);
    let pool = SqlitePool::connect_with(option).await?;

    Ok(pool)
}

pub fn get_listen_address() -> String {
    let port = env::must_get_env_value("PORT", 8000);
    let stage = env::must_get_env_value("STAGE", "dev".to_string());

    let address = match &stage[..] {
        "prod" => "0.0.0.0",
        _ => "127.0.0.1",
    };

    format!("{}:{}", address, port)
}

pub async fn create_site_dir(folder: &str) -> Result<PathBuf> {
    let root_dir_name = env::must_get_env_value("APP_NAME", "oasis".to_string());
    let root = PathBuf::from(folder).join(root_dir_name);
    if root.exists() {
        return Err(anyhow::anyhow!("Directory already existed"));
    }
    fs::create_dir_all(&root).await?;

    Ok(root)
}

fn get_db_dir() -> PathBuf {
    let sub_dir_name = env::must_get_env_value("APP_NAME", "oasis".to_string());
    match dirs::home_dir() {
        Some(dir) => dir.join(sub_dir_name),
        None => panic!("Cannot get config dir or home dir"),
    }
}

fn get_db_file() -> PathBuf {
    let main_db_name = env::must_get_env_value("MAIN_DB", "main.db".to_string());
    get_db_dir().join(&main_db_name)
}

async fn run_migrations(db_file: &PathBuf) -> anyhow::Result<()> {
    let mut conn = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .connect()
        .await?;

    let migration_dir: String = env::try_get_env_value("MIGRATION_DIR")?;
    let migrator = Migrator::new(Path::new(&migration_dir)).await?;
    migrator.run(&mut conn).await?;
    conn.close();

    Ok(())
}
