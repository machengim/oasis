use crate::request::user::LoginRequest;
use crate::service::query::Query;
use crate::service::token::Token;
use crate::{args, service::db};
use anyhow::{anyhow, Result};
use bcrypt::verify;
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
    pub async fn find_exist_username(
        username: &str,
        conn: &mut PoolConnection<Sqlite>,
    ) -> Result<Option<User>> {
        let query = Query::new("select * from USER where username = ?1", args![username]);
        let user: Option<User> = db::fetch_single(query, conn).await?;

        Ok(user)
    }

    pub async fn login(req: &LoginRequest, conn: &mut PoolConnection<Sqlite>) -> Result<Self> {
        let sql = "select * from USER where username = ?1";
        let query = Query::new(sql, args![&req.username]);
        let user: User = match db::fetch_single(query, conn).await? {
            Some(u) => u,
            None => return Err(anyhow!("Cannot find user in db")),
        };

        match verify(&req.password, &user.password)? {
            true => Ok(user),
            false => Err(anyhow!("User password incorrect")),
        }
    }

    pub fn insert_query<'a>(&self) -> Result<Query<'a>> {
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
        Token::new(self.user_id, &self.username, self.permission)
    }
}
