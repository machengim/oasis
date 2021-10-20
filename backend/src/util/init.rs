use crate::service::migrate_dir::MigrationDir;
use anyhow::Result as AnyResult;
use include_dir::{include_dir, Dir};
use rocket::tokio::fs;
use sqlx::{
    migrate::Migrator, sqlite::SqliteConnectOptions, ConnectOptions, Connection, SqlitePool,
};
use std::path::PathBuf;

pub async fn init_app() -> AnyResult<()> {
    if !get_db_file_location().exists() {
        create_db().await?;
    }

    Ok(())
}

pub async fn create_db() -> AnyResult<()> {
    let db_file = get_db_file_location();
    let db_dir = db_file.parent().unwrap();
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).await?;
    }

    let mut conn = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .connect()
        .await?;

    const ASSETS: Dir = include_dir!("./assets");
    let migration_dir = ASSETS
        .get_dir("migrations")
        .ok_or(anyhow::anyhow!("Migration dir not found"))?;
    let migrator = Migrator::new(MigrationDir::new(migration_dir)).await?;
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
    let pwd = super::get_pwd();
    pwd.join("db").join("main.db")
}
