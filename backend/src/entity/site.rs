use super::user::User;
use crate::{
    args,
    util::{db, query::Query},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{Pool, Sqlite};
use std::path::Path;

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Site {
    pub version: f64,
    pub first_run: u8,
    pub created_at: String,
    pub secret: String,
    pub storage: String,
}

#[derive(Deserialize, Debug)]
pub struct SetupRequest {
    pub username: String,
    pub password: String,
    pub storage: String,
}

impl Site {
    // The pool value is only passed at the initialization of app.
    // Later on the pool connection has to be acquired from app state.
    pub async fn init_read(pool: &Pool<Sqlite>) -> Self {
        let query = Query::new("SELECT * FROM site", vec![]);
        let mut conn = match pool.acquire().await {
            Ok(conn) => conn,
            Err(_) => panic!("Cannot get db connection"),
        };
        match db::fetch_single::<Site>(query, &mut conn).await {
            Ok(Some(site)) => site,
            Ok(None) => panic!("No site record in db"),
            Err(e) => panic!("Cannot read site info from db: {}", e),
        }
    }
}

impl SetupRequest {
    pub fn validate(&self) -> Result<bool> {
        if self.username.len() < 1 || self.password.len() < 6 || !Path::new(&self.storage).exists()
        {
            eprintln!("Invalid request data: {:?}", self);
            return Ok(false);
        }

        Ok(true)
    }

    pub fn update_site_query(&self, secret: &str) -> Query {
        Query::new(
            "update SITE set first_run = ?1, storage = ?2, secret = ?3",
            args![0, &self.storage, secret],
        )
    }

    pub fn init_admin_query<'a>(&self) -> Result<Query<'a>> {
        let mut user = User::default();
        user.username = self.username.clone();
        user.password = self.password.clone();
        user.permission = 9;

        Ok(user.insert_user_query()?)
    }

    pub fn prepare_root_in_db_query(&self) -> Query {
        let sql = "insert into FILE(filename, size, file_type, owner_id, parent_id) values(?1, ?2, ?3, (select user_id from USER where username = ?4), ?5)";
        Query::new(sql, args!["root", 0, "root", &self.username, 0])
    }
}
