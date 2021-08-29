use sqlx::{
    migrate::Migrator, sqlite::SqliteConnectOptions, ConnectOptions, Connection, SqlitePool,
};
use std::path::{Path, PathBuf};

pub fn check_db_file() -> bool {
    get_db_file_location().exists()
}

pub async fn create_db() -> Result<(), sqlx::Error> {
    let db_file = get_db_file_location();
    let mut conn = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .connect()
        .await?;

    let migration_dir = "./migrations";
    let migrator = Migrator::new(Path::new(&migration_dir)).await?;
    migrator.run(&mut conn).await?;
    conn.close();

    Ok(())
}

pub async fn get_db_pool() -> Result<SqlitePool, sqlx::Error> {
    let db_file = get_db_file_location();
    let option = SqliteConnectOptions::new().filename(db_file);
    let pool = SqlitePool::connect_with(option).await?;

    Ok(pool)
}

fn get_db_file_location() -> PathBuf {
    let pwd = std::env::current_dir().expect("Cannot get app directory");
    pwd.join("main.db")
}
