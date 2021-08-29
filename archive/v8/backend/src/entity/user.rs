use crate::request::user::LoginRequest;
use crate::service::query::Query;
use crate::service::token::Token;
use crate::{args, service::db};
use anyhow::{anyhow, Result};
use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use serde::Serialize;
use sqlx::Transaction;
use sqlx::{pool::PoolConnection, FromRow, Sqlite};

use super::file::{File, FileType};

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

    pub fn create_root_file(&self) -> File {
        File {
            file_id: 0,
            filename: String::new(),
            size: 0,
            file_type: FileType::Dir,
            owner_id: self.user_id,
            parent_id: 0,
            last_modified_at: self.created_at,
        }
    }

    pub async fn create_query<'a>(&self, tx: &mut Transaction<'_, Sqlite>) -> Result<i64> {
        let encrypt_password = hash(&self.password, DEFAULT_COST)?;

        let query = Query::new(
            "insert into USER (username, password, permission, created_at) values (?1, ?2, ?3, ?4)",
            args![
                &self.username,
                encrypt_password,
                self.permission,
                self.created_at
            ],
        );

        let user_id = db::tx_execute(query, tx).await?;
        Ok(user_id)
    }

    pub fn generate_token(&self) -> Token {
        Token::new(self.user_id, &self.username, self.permission)
    }
}
