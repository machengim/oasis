use super::file::File;
use super::token::Token;
use crate::entity::query::Query;
use crate::util::db;
use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, FromRow, Sqlite};

#[derive(Serialize, FromRow, Debug, Default)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password: String,
    pub permission: i16,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn find_exist_username(
        username: &str,
        conn: &mut PoolConnection<Sqlite>,
    ) -> Result<Option<User>> {
        let query = Query::from("select * from USER where username = ?1", vec![username]);
        let user: Option<User> = db::fetch_single(query, conn).await?;

        Ok(user)
    }

    pub fn insert_user_query<'a>(&self) -> Result<Query<'a>> {
        let encrypt_password = hash(&self.password, DEFAULT_COST)?;
        let permission_str = self.permission.to_string();

        Ok(Query::from(
            "insert into USER (username, password, permission) values (?1, ?2, ?3)",
            vec![&self.username, &encrypt_password, &permission_str],
        ))
    }

    pub fn generate_token(&self) -> Token {
        Token::new(self.user_id, self.permission)
    }
}

impl LoginRequest {
    pub async fn login(&self, conn: &mut PoolConnection<Sqlite>) -> Result<User> {
        let query = Query::from(
            "select * from USER where username = ?1",
            vec![&self.username],
        );
        let user_option: Option<User> = db::fetch_single(query, conn).await?;
        match user_option {
            Some(user) => {
                if verify(&self.password, &user.password)? {
                    Ok(user)
                } else {
                    Err(anyhow!("Password not match"))
                }
            }
            None => Err(anyhow!("No such user in database")),
        }
    }
}
