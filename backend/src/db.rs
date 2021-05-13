use std::path::PathBuf;
use sqlx::{Connection, ConnectOptions, FromRow};
use sqlx::sqlite::{SqlitePool, SqliteConnectOptions, SqliteRow};
use tokio::fs;
use crate::utils;

pub async fn get_db_conn(dir: &PathBuf) -> anyhow::Result<SqlitePool>{
    let db_file = dir.join("main.db");
    if !db_file.as_path().exists() {
        create_database(&db_file).await?;
    }

    Ok(get_conn_pool(&db_file).await?)
}

async fn create_database(db_file: &PathBuf) -> anyhow::Result<()> {
    let prefix = db_file.parent()
        .expect("Cannot retrieve the parent directory");
    fs::create_dir_all(&prefix).await?;

    let mut conn = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        .log_statements(log::LevelFilter::Trace)
        .connect()
        .await?;

    let sql = fs::read_to_string("./assets/sql/init.sql").await?;
    sqlx::query(&sql).execute(&mut conn).await?;
    let version = utils::get_version()?;
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

pub async fn fetch_single<T>(sql: &str, args: Vec<String>, pool: &SqlitePool)
    -> anyhow::Result<T>
    where T: Send + Unpin + for<'a> FromRow<'a, SqliteRow> {

    let stmt = prepare_sql(sql, args);
    Ok(stmt.fetch_one(pool).await?)
}

pub async fn fetch_multiple<T>(sql: &str, args: Vec<String>, pool: &SqlitePool)
    -> anyhow::Result<Vec<T>>
    where T: Send + Unpin + for<'a> FromRow<'a, SqliteRow> {

    let stmt = prepare_sql(sql, args);
    Ok(stmt.fetch_all(pool).await?)
}

fn prepare_sql<T>(sql: &str, args: Vec<String>) 
    -> sqlx::query::QueryAs<'_, sqlx::Sqlite, T, sqlx::sqlite::SqliteArguments<'_>>
    where T: Send + Unpin + for<'a> FromRow<'a, SqliteRow> {

    let mut stmt = sqlx::query_as(sql);
    for arg in args.iter() {
        stmt = stmt.bind(arg.to_owned());
    } 

    stmt
}