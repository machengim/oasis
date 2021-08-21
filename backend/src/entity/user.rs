use crate::args;
use crate::service::token::Token;
use crate::util::db;
use crate::util::query::Query;
use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use serde::Serialize;
use sqlx::{pool::PoolConnection, FromRow, Sqlite};

#[derive(Serialize, FromRow, Debug, Default)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password: String,
    pub permission: i16,
    pub created_at: String,
}

impl User {
    pub async fn find_exist_username(
        username: &str,
        conn: &mut PoolConnection<Sqlite>,
    ) -> Result<Option<User>> {
        let query = Query::new("select * from USER where username = ?1", args![username]);
        let user: Option<User> = db::fetch_single(query, conn).await?;

        Ok(user)
    }

    pub fn insert_user_query<'a>(&self) -> Result<Query<'a>> {
        let encrypt_password = hash(&self.password, DEFAULT_COST)?;

        Ok(Query::new(
            "insert into USER (username, password, permission) values (?1, ?2, ?3)",
            args![&self.username, encrypt_password, self.permission],
        ))
    }

    pub fn generate_token(&self) -> Token {
        Token::new(self.user_id, self.permission)
    }
}
