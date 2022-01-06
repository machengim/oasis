use super::constants;
use crate::util;
use crate::{entity::site::Site, service::migrate_dir::MigrationDir};
use anyhow::Result as AnyResult;
use include_dir::{include_dir, Dir};
use rocket::tokio::fs;
use sqlx::{
    migrate::Migrator, pool::PoolConnection, sqlite::SqliteConnectOptions, ConnectOptions,
    Connection, Sqlite, SqliteConnection, SqlitePool,
};
use std::path::PathBuf;

pub async fn init_app() -> AnyResult<()> {
    // Remove temp dir and create it again at every startup.
    let temp_dir = util::get_temp_path();
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir).await?;
    } else if temp_dir.is_file() {
        return Err(anyhow::anyhow!("temp dir location occupied as a file"));
    } else {
        fs::remove_dir_all(&temp_dir).await?;
        fs::create_dir_all(&temp_dir).await?;
    }

    // Create db and run migration files if db not existed.
    let db_file = get_db_file_location();
    if !db_file.exists() {
        let db_dir = db_file.parent().unwrap();
        if !db_dir.exists() {
            fs::create_dir_all(db_dir).await?;
        }

        let mut conn = get_db_conn(db_file).await?;
        run_migration(&mut conn).await?;
        conn.close();
    }

    Ok(())
}

pub async fn check_update(conn: &mut PoolConnection<Sqlite>) -> AnyResult<()> {
    run_migration(conn).await?;

    let mut site = match Site::read(conn).await? {
        Some(s) => s,
        None => return Ok(()),
    };

    let version_db = site.version;
    let version_app = constants::VERSION;

    if compare_version(version_app, &version_db) > 0 {
        site.version = version_app.to_owned();
        let mut tx = conn.begin().await?;
        site.update(&mut tx).await?;
        tx.commit().await?;
    }

    println!("Oasis version {}", constants::VERSION);
    Ok(())
}

async fn get_db_conn(db_file: PathBuf) -> AnyResult<SqliteConnection> {
    Ok(SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .connect()
        .await?)
}

async fn run_migration(conn: &mut SqliteConnection) -> AnyResult<()> {
    const ASSETS: Dir = include_dir!("./assets");
    let migration_dir = ASSETS
        .get_dir("migrations")
        .ok_or(anyhow::anyhow!("Migration dir not found"))?;
    let migrator = Migrator::new(MigrationDir::new(migration_dir)).await?;
    migrator.run(conn).await?;

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
    // Change in v0.2.4, move db directory to data directory.
    pwd.join("data").join("db").join("main.db")
}

fn compare_version(va: &str, vb: &str) -> i8 {
    use std::cmp::min;

    let parts_a: Vec<u64> = va.split(".").map(|e| e.parse().unwrap()).collect();
    let parts_b: Vec<u64> = vb.split(".").map(|e| e.parse().unwrap()).collect();
    let len_a = parts_a.len();
    let len_b = parts_b.len();

    for i in 0..min(len_a, len_b) {
        if parts_a[i] > parts_b[i] {
            return 1;
        } else if parts_a[i] < parts_b[i] {
            return -1;
        }
    }

    // Assume no version length more than 128
    len_a as i8 - len_b as i8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_version() {
        let va = "0.1.1";
        let vb = "0.1";
        assert!(compare_version(va, vb) > 0);
    }
}
