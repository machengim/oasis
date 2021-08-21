use crate::service::state::State;
use crate::util::db;
use crate::util::query::Query;
use crate::{args, entity::user::User};
use anyhow::{anyhow, Result};
use bcrypt::verify;
use serde::Deserialize;
use sqlx::{pool::PoolConnection, Sqlite};
use tide::Request;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl LoginRequest {
    pub async fn from(req: &mut Request<State>) -> tide::Result<Self> {
        Ok(req.body_json().await?)
    }

    pub fn validate(&self) -> bool {
        self.username.len() >= 2 && self.password.len() >= 6
    }

    pub async fn login(&self, conn: &mut PoolConnection<Sqlite>) -> Result<User> {
        let query = Query::new(
            "select * from USER where username = ?1",
            args![&self.username],
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
