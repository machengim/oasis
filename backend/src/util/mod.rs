pub mod db;
pub mod file_system;
pub mod middleware;
use anyhow::{anyhow, Result};
use async_std::fs;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::{ConnectOptions, Connection};
use std::path::{Path, PathBuf};

pub fn must_get_env_value<T: std::str::FromStr>(name: &str, default: T) -> T {
    if let Ok(s) = std::env::var(name) {
        if let Ok(v) = s.parse::<T>() {
            return v;
        }
    }

    default
}

pub fn try_get_env_value<T: std::str::FromStr>(name: &str) -> Result<T> {
    if let Ok(s) = std::env::var(name) {
        if let Ok(v) = s.parse::<T>() {
            return Ok(v);
        }
    }

    Err(anyhow!("Cannot retrieve {} value from .env", name))
}

pub fn check_installed() -> bool {
    let db_file = get_db_file();
    db_file.as_path().exists()
}

pub async fn create_db_file() -> anyhow::Result<()> {
    let db_dir = get_db_dir();
    if !db_dir.exists() {
        fs::create_dir(&db_dir).await?
    }

    let db_file = get_db_file();
    let init_db_file: String = try_get_env_value("INIT_SQLITE_FILE")?;
    let init_sql = fs::read_to_string(init_db_file).await?;
    init_db_tables(&db_file, &init_sql).await?;

    Ok(())
}

pub async fn get_conn_pool() -> anyhow::Result<SqlitePool> {
    let db_file = get_db_file();
    let option = SqliteConnectOptions::new().filename(db_file);
    //option.log_statements(log::LevelFilter::Trace);
    let pool = SqlitePool::connect_with(option).await?;

    Ok(pool)
}

pub fn get_front_dir() -> anyhow::Result<PathBuf> {
    let path_env: String = try_get_env_value("FRONT_DIR")?;
    let path = Path::new(&path_env);

    Ok(path.to_path_buf())
}

pub fn get_front_index() -> anyhow::Result<PathBuf> {
    let front_dir = get_front_dir()?;
    let path = Path::new(&front_dir).join("index.html");

    Ok(path.to_path_buf())
}

pub async fn create_site_dirs(folder: &str) -> Result<PathBuf> {
    let root_dir_name = must_get_env_value("APP_NAME", "oasis".to_string());
    let root = PathBuf::from(folder).join(root_dir_name);
    if root.exists() {
        return Err(anyhow::anyhow!("Directory already existed"));
    }

    let files_path = root.join("files");
    if !files_path.exists() {
        fs::create_dir_all(files_path).await?
    }

    let tmp_path = root.join("tmp");
    if !tmp_path.exists() {
        fs::create_dir_all(tmp_path).await?
    }

    Ok(root)
}

pub fn get_files_dir(storage: &str) -> PathBuf {
    PathBuf::from(storage).join("files")
}

pub fn get_tmp_dir(storage: &str) -> PathBuf {
    PathBuf::from(storage).join("tmp")
}

pub fn get_listen_address() -> String {
    let port = must_get_env_value("PORT", 8000);
    let stage = must_get_env_value("STAGE", "dev".to_string());

    let address = match &stage[..] {
        "prod" => "0.0.0.0",
        _ => "127.0.0.1",
    };

    format!("{}:{}", address, port)
}

pub fn generate_secret_key() -> String {
    let secret_length = must_get_env_value("SECRET_LENGTH", 32);

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(secret_length)
        .map(char::from)
        .collect()
}

// TODO: check folder's availability in different OSes.
fn get_db_dir() -> PathBuf {
    let sub_dir_name = must_get_env_value("APP_NAME", "oasis".to_string());
    match dirs::home_dir() {
        Some(dir) => dir.join(sub_dir_name),
        None => panic!("Cannot get config dir or home dir"),
    }
}

fn get_db_file() -> PathBuf {
    let main_db_name = must_get_env_value("MAIN_DB", "main.db".to_string());
    get_db_dir().join(&main_db_name)
}

async fn init_db_tables(db_file: &PathBuf, init_sql: &str) -> anyhow::Result<()> {
    let mut conn = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .connect()
        .await?;

    sqlx::query(&init_sql).execute(&mut conn).await?;
    conn.close();

    Ok(())
}
