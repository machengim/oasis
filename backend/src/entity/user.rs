use crate::{entity::query::Query, util::db};
use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use serde::Serialize;
use sqlx::{pool::PoolConnection, FromRow, Sqlite};

#[derive(Serialize, FromRow, Debug, Default)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub password: String,
    pub permission: u8,
    pub created_at: String,
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
        let utc = chrono::Utc::now().to_string();

        Ok(Query::from(
            "insert into USER (username, password, permission, created_at) values (?1, ?2, ?3, ?4)",
            vec![&self.username, &encrypt_password, &permission_str, &utc],
        ))
    }
}
