use crate::util;
use rocket::tokio::fs;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqliteRow};
use sqlx::{ConnectOptions, Connection, FromRow, Sqlite};
use std::path::PathBuf;

pub async fn get_db_conn() -> SqlitePool {
    let dir = util::get_config_dir();
    let db_file = dir.join("main.db");
    if !db_file.as_path().exists() {
        if let Err(e) = create_db(&db_file).await {
            panic!("Cannot create database\n {}", e);
        }
    }

    match get_conn_pool(&db_file).await {
        Ok(pool) => pool,
        Err(e) => panic!("Cannot get database connection pool\n {}", e),
    }
}

async fn create_db(db_file: &PathBuf) -> anyhow::Result<()> {
    let prefix = db_file
        .parent()
        .expect("Cannot retrieve the parent dir of db file");
    fs::create_dir_all(&prefix).await?;

    let mut conn = SqliteConnectOptions::new()
        .filename(db_file)
        .create_if_missing(true)
        //.log_statements(log::LevelFilter::Debug)
        .connect()
        .await?;

    let sql_init_file =
        std::env::var("INIT_SQLITE_FILE").expect("Cannot get init SQL file from env");
    let sql = fs::read_to_string(sql_init_file).await?;
    sqlx::query(&sql).execute(&mut conn).await?;
    debug!("Database created at {:?}", db_file);
    conn.close();

    Ok(())
}

async fn get_conn_pool(db_file: &PathBuf) -> anyhow::Result<SqlitePool> {
    let db_file_str = db_file.to_str().expect("Cannot parse database filename");
    let option = SqliteConnectOptions::new().filename(db_file_str);
    //option.log_statements(log::LevelFilter::Trace);
    let pool = SqlitePool::connect_with(option).await?;

    Ok(pool)
}

pub async fn fetch_single<T>(sql: &str, args: Vec<&str>, pool: &SqlitePool) -> anyhow::Result<T>
where
    T: Send + Unpin + for<'a> FromRow<'a, SqliteRow>,
{
    let stmt = prepare_sql(sql, args);
    Ok(stmt.fetch_one(pool).await?)
}

pub async fn fetch_multiple<T>(
    sql: &str,
    args: Vec<&str>,
    pool: &SqlitePool,
) -> anyhow::Result<Vec<T>>
where
    T: Send + Unpin + for<'a> FromRow<'a, SqliteRow>,
{
    let stmt = prepare_sql(sql, args);
    Ok(stmt.fetch_all(pool).await?)
}

pub async fn tx_execute(
    sql_list: Vec<&str>,
    args_list: Vec<Vec<String>>,
    pool: &SqlitePool,
) -> anyhow::Result<()> {
    if sql_list.len() != args_list.len() {
        return Err(anyhow::anyhow!("Wrong query input for transaction."));
    }
    let mut tx = pool.begin().await?;
    for i in 0..sql_list.len() {
        let stmt = prepare_exec_sql(sql_list[i], args_list[i].to_owned());
        stmt.execute(&mut tx).await?;
    }
    tx.commit().await?;

    Ok(())
}

pub async fn execute(
    sql: &str,
    args: Vec<String>,
    pool: &mut PoolConnection<Sqlite>,
) -> anyhow::Result<()> {
    let stmt = prepare_exec_sql(sql, args);
    stmt.execute(pool).await?;

    Ok(())
}

fn prepare_sql<'a, T>(
    sql: &'a str,
    args: Vec<&'a str>,
) -> sqlx::query::QueryAs<'a, sqlx::Sqlite, T, sqlx::sqlite::SqliteArguments<'a>>
where
    T: Send + Unpin + for<'b> FromRow<'b, SqliteRow>,
{
    let mut stmt = sqlx::query_as(sql);
    for arg in args.iter() {
        stmt = stmt.bind(arg.to_owned());
    }

    stmt
}

fn prepare_exec_sql<'a>(
    sql: &'a str,
    args: Vec<String>,
) -> sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>> {
    let mut stmt = sqlx::query(sql);
    for arg in args.iter() {
        stmt = stmt.bind(arg.to_owned());
    }

    stmt
}
