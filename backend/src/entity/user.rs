use crate::db;
use crate::entity::query::Query;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;
use sqlx::{pool::PoolConnection, FromRow, Sqlite};

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password: String,
    pub permission: u8,
    pub created_at: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct UserId {
    pub user_id: i64,
}

pub async fn find_exist_username(
    username: &str,
    conn: &mut PoolConnection<Sqlite>,
) -> Result<bool> {
    let query_user_sql = find_username_sql(username);
    let user_ids: Vec<UserId> = db::fetch_multiple(query_user_sql, conn).await?;
    let find = user_ids.len() > 0;

    Ok(find)
}

// TODO: how to disinguish different error types.
pub async fn login_user(
    username: &str,
    password: &str,
    conn: &mut PoolConnection<Sqlite>,
) -> Result<User> {
    let encrypt_password = hash(password, DEFAULT_COST)?;

    // Not work. Need to select username only and verify password later.
    let login_user_query = Query::new(
        "select * from USER where username = ?1 and password = ?2",
        vec![username.into(), encrypt_password],
    );

    let user: User = db::fetch_single(login_user_query, conn).await?;

    Ok(user)
}

pub fn insert_user_sql<'a>(
    username: &'a str,
    password: &'a str,
    permission: u8,
) -> Result<Query<'a>> {
    let encrypt_password = hash(password, DEFAULT_COST)?;

    let permission_str = permission.to_string();

    Ok(Query::new(
        "insert into USER (username, password, permission) values (?1, ?2, ?3)",
        vec![username.into(), encrypt_password, permission_str],
    ))
}

pub fn find_username_sql<'a>(username: &'a str) -> Query {
    Query::from(
        "select user_id from USER where username = ?1",
        vec![username],
    )
}
