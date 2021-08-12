pub mod custom_error;
pub mod db;
pub mod file_system;
use async_std::fs;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use sqlx::{ConnectOptions, Connection};
use std::path::{Path, PathBuf};
const SECRET_LENGTH: usize = 32;

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
    let init_db_file =
        std::env::var("INIT_SQLITE_FILE").expect("Cannot get init SQL file from env");
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
    let path_env = std::env::var("FRONT_DIR")?;
    let path = Path::new(&path_env);

    Ok(path.to_path_buf())
}

pub fn get_front_index() -> anyhow::Result<PathBuf> {
    let front_dir = get_front_dir()?;
    let path = Path::new(&front_dir).join("index.html");

    Ok(path.to_path_buf())
}

pub fn get_listen_address() -> String {
    let port = std::env::var("PORT").unwrap_or("8000".to_owned());
    let stage = std::env::var("STAGE").unwrap_or("dev".to_owned());

    let address = match &stage[..] {
        "prod" => "0.0.0.0",
        _ => "127.0.0.1",
    };

    format!("{}:{}", address, port)
}

pub fn generate_secret_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SECRET_LENGTH)
        .map(char::from)
        .collect()
}

fn get_db_dir() -> PathBuf {
    let sub_dir_name = std::env::var("APP_NAME").unwrap_or("oasis".to_owned());
    match (dirs::config_dir(), dirs::home_dir()) {
        (Some(dir), _) => dir.join(sub_dir_name),
        (_, Some(dir)) => dir.join(sub_dir_name),
        (None, None) => panic!("Cannot get config dir or home dir"),
    }
}

fn get_db_file() -> PathBuf {
    let main_db_name = std::env::var("MAIN_DB").unwrap_or("main.db".to_owned());
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
