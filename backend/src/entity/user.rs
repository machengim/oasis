use crate::{api::sys::SetupRequest, args, util::db, util::db::Query};
use anyhow::Result as AnyResult;
use bcrypt::{hash, DEFAULT_COST};
use sqlx::{Sqlite, Transaction};

pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password: String,
    pub permission: i8,
    pub created_at: i64,
}

impl User {
    pub fn from_setup_req(req: &SetupRequest, created_at: i64) -> Self {
        Self {
            user_id: 0,
            username: req.username.to_string(),
            password: req.password.to_string(),
            permission: 9,
            created_at,
        }
    }

    pub async fn insert_query(&self, tx: &mut Transaction<'_, Sqlite>) -> AnyResult<i64> {
        let encrypt_password = hash(&self.password, DEFAULT_COST)?;

        let sql =
            "insert into USER (username, password, permission, created_at) values(?1, ?2, ?3, ?4)";
        let query = Query::new(
            sql,
            args![
                &self.username,
                &encrypt_password,
                self.permission,
                self.created_at
            ],
        );

        let uid = db::execute(query, tx).await?;

        Ok(uid)
    }
}
