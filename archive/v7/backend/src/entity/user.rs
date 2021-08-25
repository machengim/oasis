use crate::service::token::Token;
use crate::util::db;
use crate::util::query::Query;
use crate::{args, request::site::SetupRequest};
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
    pub created_at: i64,
}

impl User {
    pub fn from_setup_req(req: &SetupRequest) -> Self {
        Self {
            user_id: -1,
            username: req.username.to_string(),
            password: req.password.to_string(),
            permission: 9,
            created_at: req.time.unwrap(),
        }
    }

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
            "insert into USER (username, password, permission, created_at) values (?1, ?2, ?3, ?4)",
            args![
                &self.username,
                encrypt_password,
                self.permission,
                self.created_at
            ],
        ))
    }

    pub fn generate_token(&self) -> Token {
        Token::new(self.user_id, self.permission)
    }
}
